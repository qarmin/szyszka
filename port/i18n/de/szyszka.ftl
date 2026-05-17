# Upper buttons
upper_start_renaming_button = Umbenennen starten
upper_add_files_button = Dateien hinzufügen
upper_add_folders_button = Ordner hinzufügen
upper_remove_selection_button = Auswahl entfernen
upper_update_names_button = Namen aktualisieren
upper_results_one_up_button = Einmal oben
upper_results_one_down_button = Eine Tiefe
upper_select_popup_button = Auswählen
# Bottom Buttons
bottom_rule_add_button = Neue Regel
bottom_rule_edit_button = Regel bearbeiten
bottom_rule_remove_button = Regel entfernen
bottom_rule_one_up_button = Einmal oben
bottom_rule_one_down_button = Eine Tiefe
bottom_rule_save_rules_button = Regeln speichern
bottom_rule_load_rules_button = Lade Regeln
# Edit names
edit_names_used_in_rules = Namen, die in Regeln verwendet werden: { $rules }
edit_names_choose_name = Name der Regeln wählen (falls vorhanden, wird sie überschrieben)
# Tree View Rules
tree_view_upper_column_type = Typ
tree_view_upper_column_current_name = Aktueller Name
tree_view_upper_column_future_name = Zukünftige Name
tree_view_upper_column_path = Pfad
# Tree View Results
tree_view_bottom_tool_type = Werkzeugtyp
tree_view_bottom_usage_name = Nutzungsname
tree_view_bottom_description = Beschreibung
# Settings
settings_language_label = Sprache
settings_open_rules = Öffne Regel-Einstellungsdatei
settings_open_cache_custom_texts = Eigene Cache-Datei öffnen
settings_open_config_dir = Cache-Verzeichnis öffnen
check_button_dark_theme = Dunkle Symbole
# Other in main window
bottom_rule_label_rules = Regeln
upper_files_folders_label = Dateien/Ordner
upper_files_folders_label_update = Files/Folders({ $files_number }) - ##### UPDATE ANFRAGEN #####
upper_files_folders_label_up_to_date = Dateien/Ordner ({ $files_number }) - Aktuell
# Select popover
button_select_all = Alle auswählen
button_select_reverse = Auswahl umkehren
button_select_custom = Eigene auswählen
button_unselect_custom = Eigene Abwählen
button_select_changed = Geändert auswählen
button_unselect_changed = Auswahl abwählen
# Un/Select custom
select_custom_example = Verwendung: */folder-nr*/* oder name-version-*.txt
select_custom_path = Pfad
select_custom_current_path = Aktueller Pfad
select_custom_future_path = Zukünftige Pfad
select_custom_path_current_name = Pfad + aktueller Name
select_custom_path_future_name = Pfad + Zufallsname
select_custom_directory_file = Verzeichnis/Datei
select_custom_select_directory = Verzeichnis auswählen
select_custom_unselect_directory = Verzeichnis deaktivieren
# General
dialog_button_ok = Ok
dialog_button_cancel = Abbrechen
# Dialogs
dialog_name_files_to_include = Dateien zum Einbinden
dialog_name_folders_to_include = Ordner zum Einbinden
dialog_scan_inside = Scannen innerhalb
dialog_ignore_folders = Ordner ignorieren
dialog_confirm_renaming = Umbenennen bestätigen
dialog_outdated_results = Veraltete Ergebnisse
dialog_results_of_renaming = Ergebnisse des Umbenennens
dialog_save_rule = Regel speichern
dialog_select_custom = Eigene auswählen
dialog_unselect_custom = Eigene Abwählen

# Rule Window


## Common

label_usage_type = Verwendungstyp:
label_example = EXAMPLE
label_example_text_before = Vor:
label_example_text_after = Nach:
button_rule_window_add = Regel hinzufügen

## Custom

label_custom_instruction =
    $(NAME) - druckt Dateiname
    $(EXT) - Druckt die Erweiterung
    $(MODIF) - Druckt das Änderungsdatum der Datei
    $(CREAT) - Druckt die Erstellung der Datei
    $(CURR) - Druckt den aktuellen Dateinamen mit der Erweiterung
    $(PARENT) - Druckt den übergeordneten Ordnernamen
    $(N)/$(K) - druckt Zahlen (Argumente sind optional)
    $(N:3:4:5) gibt Zahlen aus 3 aus. mit Schritt 4
            und füllt sie mit Nullen bis 5 Positionen.
    	K statt dessen nur Position in der Liste, verwendet auch Positionselement im Ordner.
menu_button_load_custom_rule = Eigene Regelauswahl
button_save_custom_rule = Eigene Regel speichern

## Upper/Lower Case

check_button_letters_type_uppercase = Großbuchstaben
check_button_letters_type_lowercase = Kleinbuchstaben
check_button_letters_usage_name = Nur Name
check_button_letters_usage_extension = Nur Erweiterung
check_button_letters_usage_both = Beides
label_letters_tool_type = Werkzeugtyp:
# Purge
label_purge_tool_type = Werkzeugtyp:
check_button_purge_name = Nur Name
check_button_purge_extension = Nur Erweiterung
check_button_purge_both = Beides
# Add number
label_add_number_place = Ort für Nummer:
label_add_number_settings = Nummerneinstellungen:
check_button_add_number_before_name = Vorname
check_button_add_number_after_name = Nachname
label_number_start_number = Startnummer
label_number_step = Schritt
label_number_fill_zeros = Mit Null füllen
# Add text
check_button_add_text_before_name = Vorname
check_button_add_text_after_name = Nachname
label_add_text = Zu hinzufügender Text:
# Replace
check_button_replace_name = Nur Name
check_button_replace_extension = Nur Erweiterung
check_button_replace_both = Beides
check_button_replace_case_sensitive = Groß-/Kleinschreibung
check_button_replace_case_insensitive = Groß-/Kleinschreibung
check_button_replace_regex = Regex verwenden
check_button_replace_replace_all = Alle Vorkommnisse ersetzen
label_replace_replacing_strings = Zeichenketten ersetzen:
label_replace_text_to_find = Zu findende Text
label_replace_text_to_replace = Ersetzter Text
label_replace_captures = Aufnahmen
label_replace_captured_captures = Erfasste Aufnahmen
label_replace_captures_number = ({ $capture_number } captures)
label_replace_no_captures = Keine Aufnahmen
label_replace_invalid_regex = UNGÜLTIGE REGEX
# Trim
check_button_trim_name_start = Name Start
check_button_trim_name_end = Name Ende
check_button_trim_extension_start = Erweiterungs-Start
check_button_trim_extension_end = Ende der Erweiterung
check_button_trim_case_sensitive = Groß-/Kleinschreibung
check_button_trim_case_insensitive = Groß-/Kleinschreibung
label_trim_trim_text = Text kürzen
label_trim_case_sensitivity = Groß-/Kleinschreibung
# Normalize name
label_normalize_name =
    Everything - renames the name to one that contains only the
                              characters `a-z`, `0-9`, `-`, ` `, `.`.
                              e.g. `żółć CZERona.Txt` -> `zolc-czerwona.txt`
    
    Partial - works exactly same as option above, but allows
                      to use spaces ` ` and big letters `A-Z`
                      e.g. ` Źrebię Krokietowe.Rar ` -> `Zrebie Krokietowe.Rar`
check_button_normalize_everything = Alles
check_button_normalize_partial = Teilweise
# RuleType
rule_type_custom = Eigene
rule_type_case_size = Fallgröße
rule_type_purge = Purge
rule_type_add_text = Neuer Text
rule_type_trim = Trimmen
rule_type_replace = Ersetzen
rule_type_add_number = Nummer hinzufügen
rule_type_normalize = Normalisieren
# RulePlace
rule_place_none = N/A
rule_place_extension = Nur Erweiterung
rule_place_name = Nur Name
rule_place_extension_name = Erweiterung und Name
rule_place_before_extension = Vor der Erweiterung
rule_place_after_extension = Nach der Erweiterung
rule_place_before_name = Vorname
rule_place_after_name = Nachname
rule_place_from_name_start = Von Anfang
rule_place_from_name_end_reverse = Von Name Ende bis Start
rule_place_from_extension_start = Von Extension Start
rule_place_from_extension_end_reverse = Vom Ende bis zum Start der Erweiterung
# Rule Description
rule_description_full_normalize = Volle Normalisierung
rule_description_partial_normalize = Teilweise normalisieren
rule_description_zeros = und füllen mit { $zeros } Nullen
rule_description_step = Beginnend mit { $start } mit Schritt { $step }{ $zeros }
rule_description_lowercase = Kleinbuchstaben
rule_description_uppercase = Großbuchstaben
rule_description_text = text
rule_description_added_text = Text hinzugefügt:
rule_description_start = anfangen
rule_description_end_of_name = Ende des Namens
rule_description_extension = ausbau
rule_description_end_of_extension = Ende der Erweiterung
rule_description_trimming = Schneiden "{ $trim_text }" von { $where_remove }
rule_description_custom_rule = Eigene Regel: { $custom_rule }
rule_description_replace = Ersetze { $additional_regex_text } "{ $text_to_find }" durch "{ $text_to_replace }"
# Notebooks
notebook_tab_custom = Eigene
notebook_tab_case_size = Ober-/Untere Fälle
notebook_tab_purge = Purge
notebook_tab_add_number = Nummer hinzufügen
notebook_tab_add_text = Neuer Text
notebook_tab_replace = Ersetzen
notebook_tab_trim = Trimmen
notebook_tab_normalize = Normalisiere Name
# Renaming dialog
renaming_question = Sind Sie sicher, dass Sie { $number_of_renamed_files } Dateien umbenennen möchten?
renaming_destination_file_exists = Zieldatei existiert bereits.
renaming_renamed_files = { $properly_renamed } Dateien korrekt umbenannt
renaming_ignored_files = Ignorierte { $ignored } -Dateien, weil der Name vor und nach der Änderung gleich ist.
renaming_failed_files = Umbenennen der { $failed_vector } Dateien fehlgeschlagen
renaming_list_of_failed_to_rename = Liste aller fehlerhaften Umbenennungen
renaming_error = fehlerhaft
renaming_some_records_not_updated = Einige Datensätze werden nicht aktualisiert, Sie können dies durch Klicken auf die Schaltfläche Name aktualisieren.\nSind Sie sicher, dass Sie fortfahren wollen, ohne die Namen zu aktualisieren?
renaming_missing_files = Fehlende Dateien
renaming_require_missing_files = Sie müssen mindestens 1 Datei verwenden
renaming_missing_rules = Fehlende Regeln
renaming_require_missing_rules = Sie müssen mindestens 1 Regel verwenden
