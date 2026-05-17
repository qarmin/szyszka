# Code audit — Szyszka Slint port

Scope: `port/` directory (Slint UI + Rust glue). The reusable rule logic copied from the GTK4 version (`port/src/rule/*.rs`) is reviewed where it has been modified or where its assumptions interact with the new UI flow; pure-logic findings that apply equally to the original GTK4 codebase are flagged with `[shared with original]`.

Severity legend: **CRITICAL** (data loss, undefined behaviour) / **HIGH** (clear bug, broken feature) / **MEDIUM** (incorrect-in-edge-cases, performance) / **LOW** (cosmetic, maintainability, dead code).

---

## 1. Confirmed bugs

### 1.1 [HIGH] Re-entrant `borrow()` panic on "Edit Rule" with no selection
**Location:** `port/src/main.rs:156–166` + `port/src/connect/rules_ops.rs:12–29`

When the user clicks the "Edit Rule" button (which calls `open-rule-editor(0)`), `main.rs` does:

```rust
cb.on_open_rule_editor(move |idx| {
    if let Some(ui) = ui_weak.upgrade() {
        let resolved = if idx == 0 {
            let s = state.borrow();
            s.rule_selected.iter().position(|x| *x).map(|i| i as i32).unwrap_or(-1)
        } else { idx };
        open_editor(&ui, &state, resolved);
    }
});
```

The `s` borrow is **not dropped before `open_editor`** is called — Rust's NLL ends the borrow when `s` is last used, but `s` is used inside the `let resolved = …` block whose final value is `i32`, so the borrow technically ends before `open_editor`. ✅ OK so far.

However, `open_editor` itself does the same dance:

```rust
pub fn open_editor(ui: &MainWindow, state: &SharedState, edit_index: i32) {
    if edit_index >= 0 {
        let state_ref = state.borrow();
        let idx = edit_index as usize;
        if let Some(rule) = state_ref.rules.rules.get(idx).cloned() {
            drop(state_ref);
            load_rule_into_editor(ui, &rule);
            state.borrow_mut().edit_index = Some(idx);
        }
    }
    …
    update_example(ui, state);
```

`update_example` invokes `refresh_future_names`, which calls `state.borrow_mut()` (see `rules_ops.rs:168`). If the early-return `if let Some(rule) … else {}` matches and the rule was found, we `drop(state_ref)` ourselves. **But if `state_ref.rules.rules.get(idx)` is `None`, we skip the `drop(state_ref)` and fall through past the `if`** — then call `update_example(ui, state)` *with `state_ref` still live*. That causes a `RefCell` panic.

```rust
if edit_index >= 0 {
    let state_ref = state.borrow();
    let idx = edit_index as usize;
    if let Some(rule) = state_ref.rules.rules.get(idx).cloned() {
        drop(state_ref);              // only dropped on the Some branch
        load_rule_into_editor(ui, &rule);
        state.borrow_mut().edit_index = Some(idx);
    }
} else { … }
update_example(ui, state);            // panics if state_ref still held
```

**Fix:** end the borrow at the top of the `if` by scoping it, or take the cloned rule out and drop the borrow unconditionally:

```rust
if edit_index >= 0 {
    let idx = edit_index as usize;
    let rule = state.borrow().rules.rules.get(idx).cloned();
    if let Some(rule) = rule {
        load_rule_into_editor(ui, &rule);
        state.borrow_mut().edit_index = Some(idx);
    }
}
```

NB: NLL likely saves the current code in optimized builds because the `if let` consumes `state_ref.rules.rules.get(idx)` and the borrow could be flow-narrowed — but this is brittle and reviewer-hostile. It is also definitely buggy when `idx` is out of bounds, because the fallthrough now happens with `state_ref` still live in the enclosing block.

### 1.2 [HIGH] `open-rule-editor(0)` cannot edit the rule at index 0
**Location:** `port/src/main.rs:158` + `port/ui/main.slint:153–156`

```rust
let resolved = if idx == 0 {
    // treat 0 as "look up first selected"
    …
} else { idx };
```

This means a double-click on the first rule (index 0) cannot tell Rust "open the rule at index 0" — it gets re-interpreted as "find a selected one". The double-click handler in Slint is:

```slint
double-clicked => { Callabler.open-rule-editor(idx); }
```

If the user double-clicks row 0 without checking the box first, the dispatcher will fall back to "first selected", which is `None`, so `resolved = -1` and the editor opens in *add* mode instead of *edit*.

**Fix:** use a separate sentinel (e.g. `-2`) or a separate callback for "edit selected" vs. "open at index". Cleanest is to add `edit-selected-rule()` callback that resolves on the Slint side or returns -1 if nothing is selected:

```rust
cb.on_edit_selected_rule(move || { … });
cb.on_open_rule_editor(move |idx| { open_editor(&ui, &state, idx); });
```

### 1.3 [HIGH] After `perform_renaming`, the file list is cleared even if every rename failed
**Location:** `port/src/connect/renaming.rs:134–141`

```rust
fn finalize_rename(ui: &MainWindow, state: &SharedState, result: RenameResult) {
    {
        let mut state_mut = state.borrow_mut();
        state_mut.files.clear();
        state_mut.file_selected.clear();
        state_mut.result_entries.files.clear();
    }
    …
}
```

`result.failed` contains the entries we should have kept (or at least preserved the originals for). The TODO from the GTK4 original even calls this out. As written, if the user picks a wrong destination once, they lose the entire working set with no Undo.

**Fix:** keep entries whose `(old_name, new_name)` appears in `result.failed`. Update `ItemStruct.name` to the successful renames' `future_name` (so subsequent passes start from the new state), drop the renamed ones, and leave failed ones untouched.

### 1.4 [HIGH] Stale `selected` field in `FileRow`/`RuleRow` after toggle
**Location:** `port/ui/main.slint:84–109` + `port/src/main.rs:329–342`

The Slint template binds `row.selected` directly from the model:

```slint
for row[idx] in root.files : Rectangle {
    background: row.selected ? Palette.accent-background : …;
    CheckBox { checked: row.selected; toggled => { Callabler.set-file-selected(idx, self.checked); } }
}
```

In Rust, `set-file-selected` mutates `state.file_selected[idx]` and then calls `sync_files`, which rebuilds the entire `VecModel<FileRow>` from scratch:

```rust
ui.set_files(ModelRc::new(VecModel::from(rows)));
```

So *every* selection toggle re-allocates the whole file model and replaces the `ModelRc` on the UI side. With 50 000 files this is O(N) per click — visible jank. The original GTK4 used `ListStore` row-level updates; here we should either:
- store `selected: bool` only on the Rust side and bind UI selection through a separate selection model;
- or use a per-row Slint `Model<bool>` rebound on demand.

Right now it works but every checkbox tick triggers a full model rebuild + Slint re-render of all visible rows. See also 2.1.

### 1.5 [MEDIUM] `apply_select(SelectCustom | UnselectCustom)` is silently a no-op
**Location:** `port/src/connect/select.rs:48–50`

```rust
SelectMode::SelectCustom | SelectMode::UnselectCustom => {
    // handled by apply_select_custom
}
```

Nothing calls `apply_select` with these modes (the Slint dialog calls `select-custom-apply` directly), but the dead arm exists and the enum still includes them. If a future caller accidentally dispatches through `apply_select` with `SelectCustom`, it will silently rebuild the model with no change. Either:
- remove these enum variants entirely (they're not used anywhere),
- or `debug_assert!(false, "unreachable")` inside the arm.

### 1.6 [MEDIUM] `apply_language` at startup before `MainWindow::new()` requires Fluent strings, but they're only read once into Slint
**Location:** `port/src/main.rs:31–43`

```rust
let saved_language = load_saved_language();
apply_language(&saved_language);

let ui = MainWindow::new()?;
apply_translations(&ui);
```

`MainWindow::new()` constructs the window, which evaluates the Slint default values in `common.slint` like `"Start Renaming"`. Those strings are baked-in English defaults, then immediately overwritten by `apply_translations` calling 100+ `fls!` macros. This works but means:

1. There is a brief flicker on first paint (mitigated by Slint paint timing, but real on slow startups).
2. Every default string in `Translations { … }` duplicates the English text that `apply_translations` is going to write anyway.

**Fix:** drop the English defaults from `common.slint` (use empty strings or `???`) — the canonical source is the FTL files. Alternatively, push translations *before* the first paint by calling `apply_translations(&ui)` inside an `init =>` callback bound to a Rust callback. Currently the order is fine because `ui.run()` doesn't enter the event loop until later.

### 1.7 [MEDIUM] `pick_files_and_add` and folder picker block the event loop
**Location:** `port/src/connect/files.rs:16–25`

```rust
pub fn pick_files_and_add(ui: &MainWindow, state: &SharedState) {
    let files = rfd::FileDialog::new().set_title("Add files").pick_files();
    let Some(files) = files else { return };
    …
}
```

`rfd::FileDialog::pick_files()` is a synchronous, blocking call. While it is open, the event loop is frozen — that's normally fine for a modal file picker, but it means **the progress overlay shown by `start_async_scan` cannot have appeared first** because we only display it after the picker closes. Result: the overlay's purpose ("no clicks while scanning") is intact, but the user sees the file dialog → instant overlay, with no chance for the click that started the dialog to be visually disabled while the dialog is open. Minor UX issue.

More importantly, on Linux/Wayland, `pick_files()` can occasionally **deadlock** if reentered (it can happen if the user double-clicks the "Add Files" overlay-confirm OK button fast enough to send two `request-add-files` events before the dialog opens). Currently the flow is:

```
Slint dialog OK → set add_files_dialog_open = false → Callabler.request-add-files()
```

If `set` is processed first and the user re-clicks the same area within ~10ms, the second click would fall on the underlying "Add Files" button, which would set the dialog back open and queue another `request-add-files`. Recommend:
- use the async `rfd::AsyncFileDialog` variants and dispatch back via `slint::invoke_from_event_loop`;
- or set a `picker_active: bool` guard in `AppState` before opening.

### 1.8 [MEDIUM] `start_async_scan` clones the entire dedup set per scan
**Location:** `port/src/connect/files.rs:77`

```rust
let progress = Arc::new(ScanProgress::default());
let dedup = state.borrow().result_entries.files.clone();
```

`result_entries.files: BTreeSet<String>` is cloned in full to move into the worker thread. For 10 000 prior entries that's 10 000 string allocations every time the user clicks Add Files. Worse, the worker only does `dedup.contains(&full_str)` reads — so an `Arc<BTreeSet<String>>` or `Arc<HashSet<String>>` would let the worker hold a read-only reference with one allocation. The downside is `Arc<BTreeSet>` cannot be `&mut`-ated mid-scan, but the worker doesn't need to.

**Fix:**

```rust
let dedup = Arc::new(state.borrow().result_entries.files.clone());
// or, even better, wrap result_entries in an Arc itself so the main thread
// can replace it atomically after the scan completes.
```

For larger gains, switch to `HashSet<String>` (or `ahash::HashSet`): `contains` is O(1) instead of O(log n).

### 1.9 [MEDIUM] `apply_select_custom` snapshots all files including `path` twice
**Location:** `port/src/connect/select.rs:59–63`

```rust
let snapshot: Vec<(String, String, String, String, bool)> = state_mut
    .files
    .iter()
    .map(|f| (f.path.clone(), f.name.clone(), f.future_name.clone(), f.path.clone(), f.is_dir))
    .collect();
```

`path` is cloned **twice** per row into the same tuple (positions 0 and 3). The 4th slot is then never read (`_, _, _, _, is_dir` destructuring ignores it on line 68). Pure waste — for 50 000 rows this is 50 000 redundant `String` clones (and the entire snapshot is unnecessary; the loop only needs read access to the `Vec`, which `state_mut.files.iter()` would happily provide if we didn't need `file_selected.get_mut` in the same `state_mut` borrow).

**Fix:** restructure to take an immutable read of `state.files` plus a mutable read of `file_selected`, e.g., by splitting `AppState` into two `RefCell` fields or by using indices + `split_at_mut`. Or just drop the unused slot:

```rust
for (idx, file) in state_mut.files.iter().enumerate() {
    if file.is_dir && !include_dirs { continue; }
    let target = match mode_index {
        0 => format!("{}{}{}", file.path, CHARACTER, file.name),
        1 => format!("{}{}{}", file.path, CHARACTER, file.future_name),
        _ => file.path.clone(),
    };
    if regex_check(pattern, &target) {
        if let Some(s) = state_mut.file_selected.get_mut(idx) { *s = select; }
    }
}
```

This trips the borrow checker because of `state_mut.files.iter()` + `state_mut.file_selected.get_mut(idx)`. The fix is to collect target strings first:

```rust
let targets: Vec<(usize, String)> = state_mut.files.iter().enumerate()
    .filter(|(_, f)| include_dirs || !f.is_dir)
    .map(|(i, f)| (i, match mode_index { … }))
    .collect();
for (idx, target) in targets {
    if regex_check(pattern, &target) {
        if let Some(s) = state_mut.file_selected.get_mut(idx) { *s = select; }
    }
}
```

### 1.10 [MEDIUM] `regex_check` is called inside a hot loop and pre-splits on every call
**Location:** `port/src/files.rs:152–204`

```rust
pub fn regex_check(expression: &str, directory: impl AsRef<Path>) -> bool {
    let temp_splits: Vec<&str> = expression.split('*').collect();
    let mut splits: Vec<&str> = Vec::new();
    for i in temp_splits {
        if !i.is_empty() { splits.push(i); }
    }
    …
}
```

The `expression` (user pattern) is split into glob fragments **once per file**. For 50 000 files and one pattern, that's 50 000 `split` + 50 000 `Vec` allocations. The pattern is fixed for the entire `apply_select_custom` call.

**Fix:** pre-split outside the loop:

```rust
fn compile_glob(expression: &str) -> Vec<&str> {
    expression.split('*').filter(|s| !s.is_empty()).collect()
}
fn matches_glob(splits: &[&str], starts_star: bool, ends_star: bool, directory: &str) -> bool { … }
```

Then `apply_select_custom` calls `compile_glob` once. Also, `directory.find(splits[0]).unwrap()` (line 281 in original / 174 here) panics if `splits[0]` is empty, though the upstream filter prevents it; safer to use the `Option` directly.

Additionally, this function is also reused as-is from the original GTK4 codebase — `[shared with original]` — so any improvement benefits both projects.

### 1.11 [MEDIUM] `update_example` calls `refresh_future_names` on every keystroke
**Location:** `port/src/connect/rules_ops.rs:129–164` + `port/ui/rule_editor.slint`

Every `edited(t) =>` handler in the rule editor calls `Callabler.rule-editor-update-example()`, which calls `update_example` → `refresh_future_names`, which **rebuilds every file's `future_name` from scratch and reuploads the entire `VecModel`** (`sync_files`).

```rust
pub fn update_example(ui: &MainWindow, state: &SharedState) {
    …
    refresh_future_names(ui, state);
}
```

With 50 000 files and a complex rule, every keystroke in the regex field triggers a full pass. The user can't physically type fast enough to outpace the scheduling, but each keystroke will lock the UI for as long as the recompute takes. Recommendations:
- debounce keystroke-driven updates (e.g., a 100ms `slint::Timer` reset on every change);
- separate "preview example only" (cheap) from "preview all files" (expensive), where the latter runs only on `Update Names` click;
- the original GTK4 only updated the example label, not every file's future name.

This is a behaviour regression vs. the GTK4 app, which only previewed the *example field*, not the whole list.

### 1.12 [MEDIUM] `refresh_future_names` clones `Rules` on every call
**Location:** `port/src/connect/rules_ops.rs:166–194`

```rust
let mut state_mut = state.borrow_mut();
let rules_clone = state_mut.rules.clone();
```

`Rules::clone()` deep-clones every `SingleRule` including its `RuleData` strings. Done once per keystroke under the regime above; combined with the prior issue this is several thousand string allocations per second while typing.

**Fix:** drop the borrow before the heavy loop, or take a snapshot through an `Rc<Rules>` (only the regex vec needs to be local; the rules can be borrowed). Or split `AppState` such that `rules` and `files` can be borrowed separately.

### 1.13 [LOW] `sync_files`/`sync_rules` always rebuild the entire model
**Location:** `port/src/connect/sync.rs:8–28, 30–48`

Every state mutation calls `sync_files` (or `sync_rules`), which builds a new `Vec<FileRow>`, wraps it in a fresh `VecModel`, wraps that in a new `ModelRc`, and replaces the property:

```rust
ui.set_files(ModelRc::new(VecModel::from(rows)));
```

Slint's `Model` trait allows row-level mutation (`row_changed`, `push`, `remove`); using these is dramatically faster and avoids tearing down view state. For lists ≤ ~500 rows this is invisible. For lists of 10 000+ it will be the dominant cost.

**Fix:** store the `VecModel<FileRow>` in `AppState` (e.g. `pub files_model: ModelRc<FileRow>`) and call `model.push(row)` / `model.remove(idx)` / `model.set_row_data(idx, row)` for incremental updates.

### 1.14 [LOW] `sync_files` rebuilds even when only `file_selected` changed
**Location:** `port/src/main.rs:329–342`

```rust
cb.on_set_file_selected(move |idx, sel| {
    …
    sync_files(&ui, &state);
});
```

Same observation as 1.13 but specifically for the per-row checkbox toggle. The only field that changed is `FileRow.selected`. Build the new `FileRow` for that row and call `model.set_row_data(idx, new_row)`.

### 1.15 [LOW] `state.active_timer = timer_holder.borrow_mut().take()` discards previous active timer mid-run
**Location:** `port/src/connect/files.rs:62, 135` and `port/src/connect/renaming.rs:131`

```rust
*timer_holder.borrow_mut() = Some(timer);
state.borrow_mut().active_timer = timer_holder.borrow_mut().take();
```

If a previous timer (e.g. an in-flight scan) is still running and the user starts a *second* scan, the assignment `state.borrow_mut().active_timer = …` drops the previous `Timer`, which stops its callback. The previous worker thread keeps running but its results go to an `mpsc::Receiver` whose `Sender` was dropped (because the worker is still alive and will `send`, just nobody to receive). So the user loses the first scan's results silently.

In practice, the Add Files button does not become disabled during a scan, so this is reachable: user clicks Add Files twice in a row → first scan's work is wasted, second scan completes correctly.

**Fix:** disable scan-triggering buttons while `ProgressState.visible` is true (Slint `enabled: !ProgressState.visible` on every "Add Files"/"Add Folders"/"Update Names"/"Start Renaming" button), and/or store a `Vec<Timer>` rather than `Option<Timer>` if you want concurrent operations.

### 1.16 [LOW] `update-names` callback marks `rules.updated = true` even if user just typed into rule editor without applying
**Location:** `port/src/main.rs:122–129`

```rust
cb.on_update_names(move || {
    if let Some(ui) = ui_weak.upgrade() {
        refresh_future_names(&ui, &state);
        state.borrow_mut().rules.updated = true;
        sync_outdated(&ui, &state);
    }
});
```

`refresh_future_names` is also called from `update_example` on every keystroke, which mutates `file.future_name` — but does **not** set `rules.updated = true`. So:
- typing in the rule editor silently updates every file's `future_name` in memory,
- but the "UPDATE REQUIRED" badge stays red because `rules.updated` is still `false`.

It's not technically broken (the badge will reset when the user clicks Update Names), but the previewed future names are already up-to-date — so the badge is misleading.

**Fix:** decide whether the previewed names are "real" or "preview". If real, mark `updated` true inside `refresh_future_names`. If preview-only, don't touch `file.future_name` from `update_example`; preview only the example field.

### 1.17 [LOW] `save_rule_set` reads from disk before writing, doubling I/O
**Location:** `port/src/connect/rules_ops.rs:426–443`

```rust
pub fn save_rule_set(ui: &MainWindow, state: &SharedState, name: &str) {
    …
    let mut all = load_rules();        // reads JSON from disk
    if let Some(existing) = all.iter_mut().find(|m| m.name == name) {
        *existing = new_entry;
    } else {
        all.push(new_entry);
    }
    save_rules_to_file(&all);          // writes JSON to disk
    refresh_rule_sets(ui);             // reads JSON from disk AGAIN
}
```

We just wrote `all` to disk, then `refresh_rule_sets` re-reads the same file. Cache the result:

```rust
save_rules_to_file(&all);
let entries: Vec<RuleSetEntry> = all.into_iter().map(|m| RuleSetEntry { name: m.name.into() }).collect();
ui.global::<Callabler>().set_saved_rule_sets(ModelRc::new(VecModel::from(entries)));
```

Same applies to `save_custom_text` → `refresh_custom_texts`.

### 1.18 [LOW] `delete_rule_set` writes to file then re-reads it via `refresh_rule_sets`
**Location:** `port/src/connect/rules_ops.rs:464–474`

Same pattern, same fix.

### 1.19 [LOW] No `.shrink_to_fit()` after removing files, so memory creeps
Cosmetic in normal use, but if a user repeatedly adds 100 000 files and then removes them, `Vec::remove` doesn't shrink capacity. Same for `BTreeSet`. Probably ignorable.

### 1.20 [LOW] Filename `path` reuses `display()` which is lossy on non-UTF-8
**Location:** `port/src/files.rs:42–46`

```rust
pub fn split_path(path: &Path) -> (String, String) {
    match (path.parent(), path.file_name()) {
        (Some(dir), Some(file)) => (dir.display().to_string(), file.to_string_lossy().into_owned()),
        …
    }
}
```

`dir.display().to_string()` and `file.to_string_lossy()` both replace invalid UTF-8 with `U+FFFD`. Subsequent `fs::rename` uses these strings, which means a file named with invalid UTF-8 bytes will be **renamed to a different path** than its real one — `rename` will fail or rename a phantom file. Granted, the original GTK4 code has the same problem `[shared with original]`. Could fix by storing `PathBuf` rather than `String`, but that's a larger refactor.

### 1.21 [LOW] `f_data.0`/`.1`/`.2`/`.3` magic indices in `rule_custom`
**Location:** `port/src/rule/rule_custom.rs:22–33`

```rust
if let Some(f_data) = file_data {
    modification_date = DateTime::from_timestamp(f_data.0 as i64, 0).expect(…)…;
    creation_date = DateTime::from_timestamp(f_data.1 as i64, 0).expect(…)…;
    size = format_size(f_data.2, BINARY);
    if let Some(last_component) = Path::new(&f_data.3).components().next_back() { … }
}
```

The `(u64, u64, u64, &str)` tuple has no labelled fields; you have to look up the call site to know what `0`, `1`, `2`, `3` mean. The `.expect("Failed to create DateTime(should never happens)")` is also a `[shared with original]` smell: `DateTime::from_timestamp` returns `None` for timestamps outside the valid range (~year 262144). For `u64 as i64` cast, values > `i64::MAX` wrap to negative and *could* legitimately produce `None`. The comment "should never happens" is wrong.

**Fix:** introduce a `struct FileContext { mtime: u64, ctime: u64, size: u64, parent: &str }`. Drop the `.expect` in favour of a fallback string like `""` or `"???"`.

### 1.22 [LOW] `rule_change_size_letters` appends `.` for files whose original contained `.`, even if the dot was a leading dot
**Location:** `port/src/rule/rule_change_size_letters.rs:43–47`

```rust
if !extension.is_empty() || data_to_change.contains('.') {
    format!("{name}.{extension}")
} else {
    name
}
```

For a hidden file like `.config` (which `split_file_name` returns as `name=".config", ext=""`), this produces `.CONFIG.` — an erroneous trailing dot. `[shared with original]`.

### 1.23 [LOW] Hardcoded English strings in Slint UI
**Location:** `port/ui/main.slint:486, 488, 493, 498, 549, 578, 579, 580` etc.

```slint
Text { text: "Usage: */folder*/* or name-version-*.txt"; }
LineEdit { placeholder-text: "Pattern"; … }
CheckBox { text: "Include directories"; … }
Text { text: "Are you sure that you want to rename " + GuiState.file_count + " files?"; … }
Text { text: "Properly renamed: " + GuiState.properly_renamed; }
```

These bypass the `Translations` global. When switching language, these stay English. There's also `"Saved rule sets:"`, `"Saved custom texts:"`, `"Rule name"`, `"Select"`, `"Unselect"`, `"Load"`, `"X"`, the progress overlay's `"Working…"`, `"Adding files…"`, `"Scanning folders…"`, etc.

**Fix:** add the missing FTL keys and wire them through `Translations`.

### 1.24 [LOW] Hardcoded number conversions like `file_count + " files"` ignore plural rules
**Location:** `port/ui/main.slint:549`

```slint
Text { text: "Are you sure that you want to rename " + GuiState.file_count + " files?"; … }
```

The original GTK4 used `fls!("renaming_question", {"number_of_renamed_files": …})` with Fluent plural-aware messages. The Slint version dropped that interpolation — the message is no longer translatable and "1 files" reads wrong in every language. Apply the FTL `renaming_question` template and build the string in Rust before pushing to UI.

### 1.25 [LOW] `LineEdit { text: GuiState.save_rule_name; edited(t) => { GuiState.save_rule_name = t; } }` — circular binding
**Location:** `port/ui/main.slint:448–451, 487–490`

A `LineEdit` already keeps its own text; setting `text: GuiState.save_rule_name` and then writing back to `GuiState.save_rule_name` in `edited` creates a write-during-render cycle. In practice Slint handles this, but the pattern is unidiomatic — prefer `text <=> GuiState.save_rule_name` for two-way binding, or just store the text inside the `LineEdit` and read it in the OK button:

```slint
oklineedit := LineEdit { … }
Button { clicked => { Callabler.rule-set-save(oklineedit.text); } }
```

### 1.26 [LOW] `Translations` default values duplicate English from FTL
**Location:** `port/ui/common.slint:81–204`

As noted in 1.6: every key is initialized with an English string and then overwritten by `apply_translations`. This is double-maintenance — if you add a new FTL key, you must also remember to add it in `common.slint` and `translations.rs`. Drop the defaults from common.slint (`in-out property <string> upper_start_renaming_button;` with no default = empty string).

### 1.27 [LOW] `LANGUAGES_ALL` is duplicated between Rust (`language.rs`) and Slint (`common.slint`)
**Location:** `port/src/language.rs:12–26` + `port/ui/common.slint:209–223`

Two source-of-truth lists for the 13 supported languages. If you add a 14th, you must update both. The Slint list could be populated from Rust at startup via `Settings.set_available_languages(model)`.

### 1.28 [LOW] `select_custom_mode_index` is an int when an enum would do
**Location:** `port/ui/common.slint:296` + `port/src/connect/select.rs:72–77`

```rust
let target = match mode_index {
    0 => format!("{path}{CHARACTER}{current_name}"),
    1 => format!("{path}{CHARACTER}{future_name}"),
    2 => path.clone(),
    _ => path.clone(),  // 3 and beyond all silently fall back
};
```

The slint-reference skill explicitly calls this out: use enums, not magic ints. Define `SelectCustomMode { PathCurrentName, PathFutureName, CurrentPath, FuturePath }`. The string-matching ComboBox handler (`main.slint:500–505`) is also fragile:

```slint
selected(v) => {
    if (v == "Path + Current Name") { GuiState.select_custom_mode_index = 0; }
    else if (v == "Path + Future Name") { … }
    …
}
```

Localising the strings will break this. Bind via `current-index` and emit the index directly.

### 1.29 [LOW] `dest_exists_msg = fls!("renaming_destination_file_exists")` resolved before the user changes language
**Location:** `port/src/connect/renaming.rs:73`

```rust
let dest_exists_msg = fls!("renaming_destination_file_exists");
…
std::thread::spawn(move || {
    …
    rename_one(&old_name, &new_name, …, &dest_exists_msg);
});
```

`dest_exists_msg` is captured at the start of `perform_renaming`, which is fine. But during a long batch the user can't change the language anyway. Just flagging that pulling translations to the worker thread is the only correct way here, since `fls!` is not `Send`-safe to call across threads.

### 1.30 [LOW] `text_err` captured per call in `finalize_rename`
**Location:** `port/src/connect/renaming.rs:149`

```rust
let text_err = fls!("renaming_error");
let mut text = String::new();
for i in result.failed {
    text.push_str(&format!("{} -> {}, {text_err}: {}\n", i.0, i.1, i.2));
}
```

Fine, but for clarity move `text_err` outside the `if result.failed.is_empty()` so re-reading is obvious. Minor.

### 1.31 [LOW] `process_one_item` calls `canonicalize` per file
**Location:** `port/src/files.rs:135`

```rust
let canonical = file_entry.canonicalize().ok()?.to_string_lossy().to_string();
```

`canonicalize` on Linux issues a `realpath` syscall, which resolves symlinks and stats every intermediate component. On network filesystems (NFS, SMB) or large directory trees this can dominate scan time. Not a bug, just a perf note. `[shared with original]`

### 1.32 [LOW] `rule_replace` allocates an O(n²) chain of `to_lowercase` + `format!`
**Location:** `port/src/rule/rule_replace.rs:41–48`

```rust
while let Some(index) = name[start_index..].to_lowercase().find(&text_to_find_lowercase) {
    start_index += index;
    name = format!("{}{}{}", &name[..start_index], text_to_replace, &name[start_index + text_to_find_lowercase.len()..]);
    …
}
```

Every loop iteration reallocates `name` and re-lowercases the whole substring. For long filenames with many replacements this is quadratic. `[shared with original]`

Also: `text_to_find_lowercase.len()` is used as a byte offset into `name` (which is *not* lowercased), so if the case-insensitive match changes byte length (e.g. `'İ'.to_lowercase() == "i\u{307}"` — 1 char → 2 chars), the slice indices will be off and you'll panic with "byte index N is not a char boundary". Rare but real.

### 1.33 [LOW] `rule_trim` indexes by byte length on a string that may not have aligned byte boundaries
**Location:** `port/src/rule/rule_trim.rs:22, 28, 37, 43, 52, 58, 67, 73`

Same class of issue: byte-index slicing of multi-byte UTF-8 strings without confirming char boundaries. For `text_to_trim = "ä"` (2 bytes) inside `extension = "ä"` (also 2 bytes) it works; but for mixed case-insensitive matches where the upper/lower forms have different byte lengths it can panic.

```rust
return_string = data_to_change[text_to_trim_lowercase.len()..data_to_change_lowercase.len()].to_string();
```

`[shared with original]`

### 1.34 [LOW] `rule_add_number` casts `i64 as usize` to make a length
**Location:** `port/src/rule/rule_add_number.rs:29–30`

```rust
if text_to_add.len() < fill_with_zeros as usize {
    let zeros: String = "0".repeat((fill_with_zeros - text_to_add.len() as i64) as usize);
    text_to_add = zeros + text_to_add.as_str();
}
```

`fill_with_zeros: i64` is `min`-capped to 50 (good), but if it's negative the `as usize` underflow produces a huge value. The earlier `min(fill_with_zeros, 50)` doesn't prevent negative values. Add `let fill_with_zeros = fill_with_zeros.clamp(0, 50);`. `[shared with original]`

### 1.35 [LOW] `parse_string_rules` uses `i64::checked_mul(used_number)` and falls back to `0`, silently
**Location:** `port/src/rule/rule_custom.rs:196`

```rust
let mut number = if step_number.checked_mul(used_number).is_none() { 0 } else { step_number * used_number };
```

A failed multiply (overflow) silently produces `0`. Users will see "all my files got 0" with no indication why. Probably fine for typical use, but worth surfacing in the example panel ("OVERFLOW"). `[shared with original]`

### 1.36 [LOW] `slint::Timer` polling at 60–80ms while not updating is wasteful
**Location:** `port/src/connect/files.rs:41, 92`; `port/src/connect/renaming.rs:105`

```rust
timer.start(TimerMode::Repeated, Duration::from_millis(70), move || { … });
```

The timer fires at 70ms intervals even when there is no progress to report. Could use `slint::Weak::upgrade_in_event_loop` from the worker side and a sentinel `Done` message, eliminating the polling entirely. Energy waste on battery; ~14 wakeups/sec.

### 1.37 [LOW] `Timer` lifetime stored in `AppState.active_timer` only supports one in-flight op
**Location:** `port/src/state.rs:15`

```rust
pub active_timer: Option<slint::Timer>,
```

If we want to support background indexing + a separate progress (e.g., scan files then immediately rename in batches), there's no room. See 1.15. Use `Vec<slint::Timer>` keyed by operation id, or a `HashMap`.

### 1.38 [LOW] `connect/progress.rs::poll_scan` is dead code
**Location:** `port/src/connect/progress.rs:28–71`

```rust
pub fn poll_scan<T: 'static, F>(ui: &MainWindow, progress: Arc<ScanProgress>, rx: Receiver<T>, on_done: F) -> TimerHolder
where F: Fn(&MainWindow, T) + 'static,
```

This function is defined but never called — `start_async_scan` and `perform_renaming` both reimplement the same polling timer pattern inline. Either delete the helper or refactor the callers to use it.

### 1.39 [LOW] `language.rs::save_dark_theme_setting` is dead code wrapper
**Location:** `port/src/language.rs:60–63`

```rust
#[allow(dead_code)]
pub fn save_dark_theme_setting(is_dark: bool) {
    save_dark_theme(is_dark);
}
```

`#[allow(dead_code)]` plus a one-line wrapper that adds nothing. Delete.

### 1.40 [LOW] `main.rs::_force_used(_m: SelectMode)` is dead code
**Location:** `port/src/main.rs:369–370`

```rust
#[allow(unused)]
fn _force_used(_m: SelectMode) {}
```

Apparently left over from an earlier import-fixing iteration. Delete.

### 1.41 [LOW] `MainWindow::init` sets `Palette.color-scheme` redundantly with the `changed` handler
**Location:** `port/ui/main.slint:17–19`

```slint
property <bool> theme_sync: Settings.dark_theme;
changed theme_sync => { Palette.color-scheme = self.theme_sync ? ColorScheme.dark : ColorScheme.light; }
init => { Palette.color-scheme = Settings.dark_theme ? ColorScheme.dark : ColorScheme.light; }
```

Both branches do the same. The `init` is needed because `changed` only fires on subsequent changes. OK, but the duplication is fragile if you ever extend the formula. Extract into a callback or inline:

```slint
function apply-palette() { Palette.color-scheme = Settings.dark_theme ? ColorScheme.dark : ColorScheme.light; }
init => { apply-palette(); }
changed theme_sync => { apply-palette(); }
```

### 1.42 [LOW] `LoadDirection` enum declared but unused
**Location:** `port/ui/common.slint:54–58`

```slint
export enum LoadDirection {
    load, save, delete,
}
```

No code on either side uses this enum. Delete.

### 1.43 [LOW] `Callabler` global mixes callbacks AND data properties (`saved_rule_sets`, `saved_custom_rules`)
**Location:** `port/ui/common.slint:328, 333`

```slint
export global Callabler {
    callback request-load-rule-sets();
    in-out property <[RuleSetEntry]> saved_rule_sets: [];
    …
    in-out property <[string]> saved_custom_rules: [];
    …
}
```

`Callabler` was conceptually "callbacks into Rust". Mixing data models into it muddles the meaning. Move `saved_rule_sets` and `saved_custom_rules` into a dedicated `Lists` global (or into `GuiState`).

### 1.44 [LOW] `dialog_outdated_results` translation key is dead
**Location:** `port/src/connect/translations.rs:59` + `port/ui/common.slint:132`

The "Outdated results" pre-rename warning dialog from the GTK4 version is not implemented in this port (per the differences doc). But the translation key is still loaded into the `Translations` global. Either implement the dialog or strip the key. Same for `dialog_select_custom`/`dialog_unselect_custom` which are present but not used as the dialog title (the dialog uses `dialog_select_custom` once; `dialog_unselect_custom` is unreferenced).

### 1.45 [LOW] `SingleRule::new` dead-code warning is real
**Location:** `port/src/rule/rules.rs:27–36`

`SingleRule::new()` is only used in tests in the original codebase; the port doesn't carry tests. Either suppress with `#[cfg(test)]` (matching the original) or delete.

### 1.46 [LOW] `Rules::remove_rule` dead
**Location:** `port/src/rule/rules.rs:64–66`

Same situation — never called. Delete or move to a `#[cfg(test)]` block.

### 1.47 [LOW] `Rules::edit_mode: Option<usize>` field never read
**Location:** `port/src/rule/rules.rs:41` (warning) — port uses `AppState::edit_index` instead.

Two separate "currently editing" markers. The `Rules` field is dead and confusing. Remove.

### 1.48 [LOW] `Rules::add_single_rule` is a one-line method that just does `self.rules.push`
**Location:** `port/src/rule/rules.rs:60–62`

Trivial wrapper. Either inline at call sites or keep the method but remove the per-call `add_single_rule`/`push` inconsistency in the codebase.

### 1.49 [LOW] `apply_translations` is a ~120-line wall of repetitive `set_*(fls!(…).into())` calls
**Location:** `port/src/connect/translations.rs`

Maintainable but boilerplate-heavy. Consider a `macro_rules! tr_set { ($t:ident, $($key:ident),*) => { $( $t.[<set_ $key>](fls!(stringify!($key)).into()); )* } }`, or generate this code from the FTL file at build time. For 100+ keys, the macro pays for itself.

### 1.50 [LOW] Magic numbers in Slint dimensions
**Location:** `port/ui/main.slint:229–230, 247–250, 273–277, 311–313, 360–362, 393–395, 436–438, 472–476, 538–540, 566–568, 627`

Most overlay sizes are hardcoded (`width: 780px; height: 660px;` etc). On small displays (HiDPI laptops in scaled mode) these overlay cards can be larger than the window. Use `max-width: 90%; max-height: 90%;` or compute from `parent.width/height`.

### 1.51 [LOW] No min-size on overlay cards
Same area. With `width: 380px; height: 200px;` and long translated strings (e.g., German), the LineEdit + button row will overflow. Either use `preferred-height: 200px; min-height: ...;` or remove fixed heights altogether and let layout drive size.

### 1.52 [LOW] `start_async_scan` shows "Scanning…" title regardless of source
**Location:** `port/src/connect/files.rs:71`

```rust
show_overlay(ui, "Scanning…", message, false);
```

The `message` parameter is passed dynamically but the title is hardcoded. The caller is the one who knows whether this is files or folders. Pass title too, or just merge title+message.

### 1.53 [LOW] Worker thread send-success is unchecked
**Location:** `port/src/connect/files.rs:32, 83`; `port/src/connect/renaming.rs:91`

```rust
let _ = tx.send(items);
```

If the receiver was dropped (e.g., the UI closed mid-scan), the worker silently drops the result. That's OK; just acknowledge it in a comment. Otherwise this looks like an oversight.

### 1.54 [LOW] No accessibility properties set anywhere
None of the Slint widgets set `accessible-role`/`accessible-label`. Screen readers will see the application as opaque rectangles. Slint reference notes the perf tradeoff, but for buttons + labels in a settings dialog the cost is negligible. Add at minimum `accessible-label` and `accessible-role: button` to all `TouchArea`-wrapped clickables.

### 1.55 [LOW] `apply_language(combo_text)` panics on invalid language identifier
**Location:** `port/src/language.rs:56`

```rust
let requested = vec![lang.short_text.parse().expect("Invalid language identifier")];
```

`expect` on parsing a static string — fine, it's hardcoded. But the function would be more robust if it took an `Option` and silently ignored unknown languages. Currently `get_language_from_combo_box_text` returns `&LANGUAGES_ALL[0]` (English) on lookup miss, so this can't actually panic — but in a future where languages can be added dynamically, this becomes a foot-gun.

### 1.56 [LOW] `LANGUAGES_ALL` ordering not aligned with `Settings.available_languages`
**Location:** `port/src/language.rs:12–26` vs `port/ui/common.slint:209–223`

Both lists exist in the same order today, but nothing enforces this. If the Slint list is ever extended without updating Rust, lookups will silently return English. See 1.27 for the deduplication fix.

### 1.57 [LOW] Inconsistent quote of placeholder text style across overlays
Different overlays use `placeholder-text: "Rule name"` vs none at all; some have a hint Text above the LineEdit. Pick one pattern.

### 1.58 [LOW] `humansize::BINARY` for sizes (KiB/MiB) but `format_size` not used in the UI list
`sync_files` populates `size_text: format_size(item.size, BINARY)` (good) but **no column displays it**. The `FileRow.size_text` field is dead weight. Either show the Size column or drop the field from `FileRow`.

### 1.59 [LOW] `FileRow.date_text` similarly unused in UI
Same as 1.58. Drop `date_text` until the Date column exists.

### 1.60 [LOW] `slint_gen::DirFileTypeUi` is used by Rust but the column it would have driven is gone
The Slint UI uses `row.dir_type == DirFileTypeUi.directory-t ? "Dir" : "File"`. Works, but `DirFileTypeUi` is now used only as a boolean. Could be `is_dir: bool` directly.

### 1.61 [LOW] `mpsc::TryRecvError::Disconnected` handling silently hides overlay
**Location:** `port/src/connect/files.rs:53–58, 126–131`; `port/src/connect/renaming.rs:122–127`

```rust
Err(mpsc::TryRecvError::Disconnected) => {
    hide_overlay(&ui);
    if let Some(t) = th_c.borrow().as_ref() { t.stop(); }
}
```

If the worker panicked, the channel is disconnected. The user sees the overlay disappear with no indication that anything failed. Worth a `eprintln!("worker thread terminated without result")` or a message dialog.

### 1.62 [LOW] `state.rs::AppState` has `active_timer` mixed with data
`active_timer: Option<slint::Timer>` is UI infrastructure, not domain state. Mixing it with `files`, `rules`, etc. blurs the boundary. Move into a separate `UiInfra` struct or use `thread_local!`.

### 1.63 [LOW] `Cargo.toml` has `[workspace]` empty table and no top-level [workspace] excludes
**Location:** `port/Cargo.toml`

```toml
[workspace]
```

This was added to escape the outer workspace. Fine. But empty `[workspace]` invites future confusion ("did the author forget members?"). Add a comment: `# Sub-crate kept out of the parent workspace`.

### 1.64 [LOW] `[profile.dev.package."*"] opt-level = 3` may double build time on first compile
**Location:** `port/Cargo.toml`

This is copied from the original. It's the right choice for Slint apps (compile slow once, run fast forever) but a one-line comment would help newcomers.

### 1.65 [LOW] `i18n.toml` lives in `port/` but the localizer hardcodes `i18n/`
**Location:** `port/src/localizer.rs:8–10`

```rust
#[derive(RustEmbed)]
#[folder = "i18n/"]
struct Localizations;
```

Works because `cargo build` is run from `port/`. If you ever build from the workspace root, this breaks (the folder will be `port/i18n/`). Use `concat!(env!("CARGO_MANIFEST_DIR"), "/i18n")` or document the assumption.

### 1.66 [LOW] `Cargo.toml` pins `slint = 1.13` but Cargo.lock shows 1.16
**Location:** `port/Cargo.toml` vs `port/Cargo.lock`

Not a bug (semver-compatible), but a fixed lockfile drift. Either bump the manifest pin or document the policy.

### 1.67 [LOW] Hardcoded `set_title("Add files")` / `set_title("Add folders")` in `rfd`
**Location:** `port/src/connect/files.rs:17, 24`

```rust
let files = rfd::FileDialog::new().set_title("Add files").pick_files();
```

The dialog title is in English regardless of language setting. Translatable.

### 1.68 [LOW] `pick_folders_and_add` doesn't use the `ignore_folders` flag when folder didn't get scanned-inside
**Location:** `port/src/files.rs:52–69`

```rust
let mut folders = if check_folders_inside {
    for folder in folders_to_check {
        for entry in WalkDir::new(folder).skip_hidden(true)… {
            if ignore_folders { … } else { new_entries.push(entry.path()); }
        }
    }
    new_entries
} else {
    folders_to_check        // ← ignore_folders is silently ignored
};
```

The `ignore_folders` flag is documented in the dialog as "only include files, no directories". When `check_folders_inside` is false, the user selects directories themselves; we add them as-is. If the user picks a folder and toggles "ignore folders" but leaves "scan inside" off, they probably expect the folder to be excluded (`Vec::new()`). Current behaviour is "add the folder anyway". Either disable `ignore_folders` UI when `scan_inside` is off, or honour it:

```rust
} else if ignore_folders {
    return Vec::new();
} else {
    folders_to_check
};
```

### 1.69 [LOW] `enumerate_folder_contents` populates `new_entries` only inside `WalkDir`-loop; never used externally
Fine, but the variable name and the conditional `is_file()` check could be a separate function `walk_inside(folder, ignore_folders)`.

### 1.70 [LOW] `apply_select` resizes `file_selected` to `files.len()` even when files were never removed
**Location:** `port/src/connect/select.rs:9–10`

```rust
let len = state_mut.files.len();
state_mut.file_selected.resize(len, false);
```

This is defensive (in case `file_selected` got out of sync), but if it ever truly needs this resize, that's a bug elsewhere. Either delete it (trust invariants) or `debug_assert_eq!(state_mut.file_selected.len(), state_mut.files.len())` first.

### 1.71 [LOW] `regex_check` returns `false` on the empty pattern, which is surprising
**Location:** `port/src/files.rs:160–162`

```rust
if splits.is_empty() { return false; }
```

An empty pattern matches nothing rather than everything. This is the original behaviour, but the dialog's `LineEdit` allows empty input and the OK button doesn't validate — user clicks "Select" with empty pattern and gets a silent no-op. Either disable OK while pattern is empty, or treat empty as "match all" (`return true`).

### 1.72 [LOW] `path: file_entry.canonicalize().ok()?.to_string_lossy().to_string()` rejects files whose paths cannot be canonicalised
**Location:** `port/src/files.rs:135`

`canonicalize` requires the path to exist *and* fully resolve all symlinks. On Windows, paths that exceed `MAX_PATH` without `\\?\` prefix fail. Result: those files silently disappear from the added set. Worth logging:

```rust
let canonical = match file_entry.canonicalize() {
    Ok(p) => p.to_string_lossy().to_string(),
    Err(e) => { eprintln!("canonicalize failed for {file_entry:?}: {e}"); return None; }
};
```

(Currently the `.ok()?` swallows the error.)

### 1.73 [LOW] No tests at all
The original GTK4 codebase has unit tests in `port/src/rule/*.rs`'s ancestor files (e.g. `rule_replace::test`). The port carries over the logic but not the tests. Add `#[cfg(test)] mod test { … }` per rule file. Many edge cases (UTF-8 panic in 1.32/1.33, integer overflow in 1.34/1.35) would be caught by ports of the existing tests.

### 1.74 [LOW] `cargo build` emits 3 dead-code warnings that the project treats as informational
**Location:** `port/src/rule/rules.rs:28, 41, 64`

The dead-code analysis is correct (see 1.45/1.46/1.47). Pick a resolution.

### 1.75 [LOW] `state.rs::new_shared()` is a one-line wrapper around `Rc::new(RefCell::new(Default::default()))`
Trivial helper. Could inline at the single call site in `main.rs`.

---

## 2. Performance issues (summary, beyond what's above)

| # | Issue | Severity |
|---|---|---|
| 2.1 | `sync_files`/`sync_rules` rebuild entire `ModelRc` on every state change | medium |
| 2.2 | Per-keystroke `refresh_future_names` over all files | medium |
| 2.3 | `Rules::clone()` per keystroke (1.12) | medium |
| 2.4 | `regex_check` pre-splits pattern per file (1.10) | medium |
| 2.5 | `apply_select_custom` snapshots files redundantly (1.9) | low |
| 2.6 | `BTreeSet<String>` dedup clone per scan (1.8) | low |
| 2.7 | `O(n²)` string rebuild in `rule_replace` (1.32) | low |
| 2.8 | `canonicalize` per file on every scan (1.31) | low |
| 2.9 | Timer polling at 70 ms even when idle (1.36) | low |
| 2.10 | I/O re-read after every save (1.17, 1.18) | low |

---

## 3. Maintainability issues

| # | Issue | Severity |
|---|---|---|
| 3.1 | Duplicate language list Rust↔Slint (1.27) | low |
| 3.2 | Translation defaults duplicate FTL (1.6, 1.26) | low |
| 3.3 | Magic ints for `select_custom_mode_index` (1.28) | low |
| 3.4 | Hardcoded English in Slint (1.23, 1.24, 1.67) | low |
| 3.5 | Dead helpers: `poll_scan`, `save_dark_theme_setting`, `_force_used`, `LoadDirection`, `dialog_unselect_custom`, `SingleRule::new`, `Rules::remove_rule`, `Rules::edit_mode`, `FileRow.size_text`, `FileRow.date_text` (1.38–1.42, 1.45–1.47, 1.58–1.59) | low |
| 3.6 | Magic-tuple `(u64, u64, u64, &str)` for file metadata (1.21) | low |
| 3.7 | 120-line `apply_translations` repetition (1.49) | low |
| 3.8 | Two separate "currently editing" markers — `Rules::edit_mode` and `AppState::edit_index` (1.47) | low |
| 3.9 | `i32` for index parameters that should be `Option<usize>` (1.2, `rule-set-load`/`-delete`, `custom-rule-load`/`-delete`) | low |
| 3.10 | `Callabler` mixes callbacks with data models (1.43) | low |

---

## 4. Suggested concrete next steps

Ordered by ROI:

1. **Fix 1.2** (Edit Rule on index 0 broken). One-line change with high user impact.
2. **Fix 1.1** (borrow panic on out-of-bounds edit index). One scope-restructure.
3. **Fix 1.3** (don't clear file list on rename failure). Preserves user data.
4. **Fix 1.11/1.12** (debounce preview, don't clone Rules per keystroke). Major UX win for large lists.
5. **Fix 1.15** (disable buttons during in-flight progress). Prevents lost work.
6. **Fix 1.13/1.14** (incremental model updates). Major perf win for any list > 1000 rows.
7. **Fix 1.23/1.24** (FTL all UI strings). Restores translation completeness.
8. **Fix 1.10/1.8/1.9** (regex_check pre-splitting, dedup Arc, snapshot waste). Easy wins.
9. **Port existing rule unit tests** (1.73). Catches the lurking UTF-8 and overflow issues.
10. **Delete dead code** (1.38–1.47). Reduces noise.

---

## 5. Files with the most issues

| File | Issues |
|---|---|
| `port/src/connect/rules_ops.rs` | 1.1, 1.2 (callee), 1.11, 1.12, 1.16, 1.17, 1.18 |
| `port/src/connect/files.rs` | 1.7, 1.8, 1.15, 1.36, 1.52, 1.53, 1.61 |
| `port/src/connect/renaming.rs` | 1.3, 1.15, 1.30, 1.36, 1.61 |
| `port/src/files.rs` | 1.8, 1.10, 1.20, 1.31, 1.68, 1.71, 1.72 |
| `port/src/rule/rules.rs` | 1.45, 1.46, 1.47, 1.48 |
| `port/ui/main.slint` | 1.4, 1.13, 1.14, 1.23, 1.24, 1.25, 1.28, 1.41, 1.42, 1.50, 1.51 |
| `port/ui/common.slint` | 1.26, 1.27, 1.42, 1.43, 1.44 |
| `port/src/connect/translations.rs` | 1.49 |
| `port/src/main.rs` | 1.2, 1.16, 1.40, 1.55 |
