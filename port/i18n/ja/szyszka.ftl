# Upper buttons
upper_start_renaming_button = 名前の変更を開始
upper_add_files_button = ファイルを追加
upper_add_folders_button = フォルダを追加
upper_remove_selection_button = 選択範囲を削除
upper_update_names_button = 名前を更新
upper_results_one_up_button = One Up
upper_results_one_down_button = ワンダウン
upper_select_popup_button = 選択
# Bottom Buttons
bottom_rule_add_button = ルールを追加
bottom_rule_edit_button = ルールの編集
bottom_rule_remove_button = ルールを削除
bottom_rule_one_up_button = One Up
bottom_rule_one_down_button = ワンダウン
bottom_rule_save_rules_button = ルールを保存
bottom_rule_load_rules_button = ルールを読み込む
# Edit names
edit_names_used_in_rules = ルールで使用される名前: { $rules }
edit_names_choose_name = ルールの名前を選択してください（存在する場合は上書きされます）
# Tree View Rules
tree_view_upper_column_type = タイプ
tree_view_upper_column_current_name = 現在の名前
tree_view_upper_column_future_name = 将来の名前
tree_view_upper_column_path = パス
# Tree View Results
tree_view_bottom_tool_type = ツールタイプ
tree_view_bottom_usage_name = 使用者名
tree_view_bottom_description = 説明
# Settings
settings_language_label = 言語
settings_open_rules = ルール設定ファイルを開く
settings_open_cache_custom_texts = カスタム キャッシュ ファイルを開く
settings_open_config_dir = キャッシュディレクトリを開く
check_button_dark_theme = ダークアイコン
# Other in main window
bottom_rule_label_rules = ルール
upper_files_folders_label = ファイル/フォルダ
upper_files_folders_label_update = ファイル/フォルダ({ $files_number }) - ##### 要求を更新する #####
upper_files_folders_label_up_to_date = ファイル/フォルダ({ $files_number }) - 日付まで
# Select popover
button_select_all = すべて選択
button_select_reverse = 選択を逆にする
button_select_custom = カスタムを選択
button_unselect_custom = カスタムの選択を解除
button_select_changed = 変更を選択
button_unselect_changed = 選択解除の変更
# Un/Select custom
select_custom_example = 使用法: */folder-nr*/* または name-version-*.txt
select_custom_path = パス
select_custom_current_path = 現在のパス
select_custom_future_path = 将来のパス
select_custom_path_current_name = パス + 現在の名前
select_custom_path_future_name = Path + Future Name
select_custom_directory_file = ディレクトリ/ファイル
select_custom_select_directory = ディレクトリを選択
select_custom_unselect_directory = ディレクトリの選択を解除
# General
dialog_button_ok = OK
dialog_button_cancel = キャンセル
# Dialogs
dialog_name_files_to_include = 含めるファイル
dialog_name_folders_to_include = 含めるフォルダ
dialog_scan_inside = 内側をスキャン
dialog_ignore_folders = フォルダを無視
dialog_confirm_renaming = 名前の変更を確認
dialog_outdated_results = 古い結果
dialog_results_of_renaming = 名前を変更した結果
dialog_save_rule = ルールの保存
dialog_select_custom = カスタムを選択
dialog_unselect_custom = カスタムの選択を解除

# Rule Window


## Common

label_usage_type = 使用タイプ:
label_example = 例
label_example_text_before = 以下を参照：
label_example_text_after = あと:
button_rule_window_add = ルールの追加

## Custom

label_custom_instruction =
    $(NAME) - prints file name
    $(EXT) - prints extension
    $(MODIF) - prints file modification date
    $(CREAT) - prints file creation
    $(CURR) - prints current file name with extension
    $(PARENT) - prints parent folder name
    $(N)/$(K) - prints numbers(arguments are optional)
    $(N:3:4:5) prints numbers from 3, with step 4
            and fills them with zeros to 5 positions.
    	K instead only position in list, also uses position item in folder.
menu_button_load_custom_rule = カスタムルール選択
button_save_custom_rule = カスタムルールを保存

## Upper/Lower Case

check_button_letters_type_uppercase = 大文字・小文字
check_button_letters_type_lowercase = 小文字
check_button_letters_usage_name = 名前のみ
check_button_letters_usage_extension = エクステンションのみ
check_button_letters_usage_both = 両方とも
label_letters_tool_type = ツールタイプ:
# Purge
label_purge_tool_type = ツールタイプ:
check_button_purge_name = 名前のみ
check_button_purge_extension = エクステンションのみ
check_button_purge_both = 両方とも
# Add number
label_add_number_place = 数字を入れる場所:
label_add_number_settings = 番号の設定:
check_button_add_number_before_name = 名前の前
check_button_add_number_after_name = 名前の後
label_number_start_number = 開始番号
label_number_step = Step
label_number_fill_zeros = ゼロで塗りつぶし
# Add text
check_button_add_text_before_name = 名前の前
check_button_add_text_after_name = 名前の後
label_add_text = 追加するテキスト:
# Replace
check_button_replace_name = 名前のみ
check_button_replace_extension = エクステンションのみ
check_button_replace_both = 両方とも
check_button_replace_case_sensitive = 大文字と小文字を区別する
check_button_replace_case_insensitive = 大文字小文字を区別しない
check_button_replace_regex = 正規表現を使用
check_button_replace_replace_all = すべての繰り返しを置き換え
label_replace_replacing_strings = 文字列の置き換え:
label_replace_text_to_find = 検索するテキスト
label_replace_text_to_replace = 置換されたテキスト
label_replace_captures = キャプチャ
label_replace_captured_captures = キャプチャされたキャプチャ
label_replace_captures_number = ({ $capture_number } captures)
label_replace_no_captures = キャプチャなし
label_replace_invalid_regex = 無効な正規表現があります
# Trim
check_button_trim_name_start = 名前の開始
check_button_trim_name_end = 名前 終了
check_button_trim_extension_start = 拡張機能の開始
check_button_trim_extension_end = 拡張機能の終了
check_button_trim_case_sensitive = 大文字と小文字を区別する
check_button_trim_case_insensitive = 大文字小文字を区別しない
label_trim_trim_text = テキストをトリムする
label_trim_case_sensitivity = ケース感度
# Normalize name
label_normalize_name =
    Everything - renames the name to one that contains only the
                              characters `a-z`, `0-9`, `-`, ` `, `.`.
                              e.g. `żółć CZERona.Txt` -> `zolc-czerwona.txt`
    
    Partial - works exactly same as option above, but allows
                      to use spaces ` ` and big letters `A-Z`
                      e.g. ` Źrebię Krokietowe.Rar ` -> `Zrebie Krokietowe.Rar`
check_button_normalize_everything = すべて
check_button_normalize_partial = 部分的な
# RuleType
rule_type_custom = カスタム
rule_type_case_size = 案件サイズ
rule_type_purge = Purge
rule_type_add_text = テキストを追加
rule_type_trim = 切り落とし
rule_type_replace = 置換
rule_type_add_number = 番号を追加
rule_type_normalize = 正規化
# RulePlace
rule_place_none = 該当なし
rule_place_extension = エクステンションのみ
rule_place_name = 名前のみ
rule_place_extension_name = 拡張機能と名前
rule_place_before_extension = 拡張の前
rule_place_after_extension = 拡張機能の後
rule_place_before_name = 名前の前
rule_place_after_name = 名前の後
rule_place_from_name_start = 開始から
rule_place_from_name_end_reverse = 名前の終わりから開始まで
rule_place_from_extension_start = 拡張機能の開始から
rule_place_from_extension_end_reverse = エクステンション終了からスタートまで
# Rule Description
rule_description_full_normalize = 完全な正規化
rule_description_partial_normalize = 部分正規化
rule_description_zeros = そして { $zeros } 個のゼロで満たしています
rule_description_step = { $start } から { $step }で始まる{ $zeros }
rule_description_lowercase = 小文字
rule_description_uppercase = 大文字・小文字
rule_description_text = テキスト
rule_description_added_text = テキストを追加:
rule_description_start = 開始
rule_description_end_of_name = 名前の終わり
rule_description_extension = 拡張
rule_description_end_of_extension = エクステンションの終了
rule_description_trimming = { $trim_text }から " { $where_remove }" をトリミング中
rule_description_custom_rule = カスタムルール: { $custom_rule }
rule_description_replace = { $additional_regex_text } "{ $text_to_find }" を "{ $text_to_replace } " に置き換える
# Notebooks
notebook_tab_custom = カスタム
notebook_tab_case_size = 上位/下位ケース
notebook_tab_purge = Purge
notebook_tab_add_number = 番号を追加
notebook_tab_add_text = テキストを追加
notebook_tab_replace = 置換
notebook_tab_trim = 切り落とし
notebook_tab_normalize = 名前を正規化する
# Renaming dialog
renaming_question = { $number_of_renamed_files } ファイルの名前を変更してもよろしいですか？
renaming_destination_file_exists = 保存先のファイルは既に存在します。
renaming_renamed_files = { $properly_renamed } ファイルの名前を正しく変更しました
renaming_ignored_files = 変更前と変更後の名前が同じであるため、 { $ignored } ファイルを無視しました。
renaming_failed_files = { $failed_vector } ファイルの名前を変更できませんでした
renaming_list_of_failed_to_rename = すべての失敗した名前のリスト
renaming_error = エラー
renaming_some_records_not_updated = 一部のレコードは更新されません。\nボタンをクリックすると更新できます。 \n 名前を更新せずに続行してもよろしいですか？
renaming_missing_files = 不足しているファイル
renaming_require_missing_files = 少なくとも 1 つのファイルを使用する必要があります。
renaming_missing_rules = ルールがありません
renaming_require_missing_rules = 少なくとも1つのルールを使用する必要があります
