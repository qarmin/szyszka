# Upper buttons
upper_start_renaming_button = 開始重新命名
upper_add_files_button = 新增檔案
upper_add_folders_button = 新增資料夾
upper_remove_selection_button = 移除選擇
upper_update_names_button = 更新檔名
upper_results_one_up_button = 上移一位
upper_results_one_down_button = 下移一位
upper_select_popup_button = 選擇
# Bottom Buttons
bottom_rule_add_button = 新增規則
bottom_rule_edit_button = 編輯規則
bottom_rule_remove_button = 刪除規則
bottom_rule_one_up_button = 上移一位
bottom_rule_one_down_button = 下移一位
bottom_rule_save_rules_button = 儲存規則
bottom_rule_load_rules_button = 載入規則
# Edit names
edit_names_used_in_rules = 規則中使用的名稱：{ $rules }
edit_names_choose_name = 選擇規則名稱（如果存在將覆蓋）
# Tree View Rules
tree_view_upper_column_type = 類型
tree_view_upper_column_current_name = 目前檔名
tree_view_upper_column_future_name = 未來檔名
tree_view_upper_column_path = 路徑
# Tree View Results
tree_view_bottom_tool_type = 工具類型
tree_view_bottom_usage_name = 用途名稱
tree_view_bottom_description = 描述
# Settings
settings_language_label = 語言
settings_open_rules = 開啟規則設定檔案
settings_open_cache_custom_texts = 開啟自訂快取檔案
settings_open_config_dir = 開啟快取目錄
check_button_dark_theme = 深色圖示
# Other in main window
bottom_rule_label_rules = 規則
upper_files_folders_label = 檔案/資料夾
upper_files_folders_label_update = 檔案/資料夾（{ $files_number }）- ##### 需要更新 #####
upper_files_folders_label_up_to_date = 檔案/資料夾（{ $files_number }）- 最新的
# Select popover
button_select_all = 選擇所有
button_select_reverse = 反向選擇
button_select_custom = 自訂選擇
button_unselect_custom = 取消自訂選擇
button_select_changed = 選擇已變更
button_unselect_changed = 取消選擇已變更
# Un/Select custom
select_custom_example = 使用方法：*/folder-nr*/* 或 name-version-*.txt
select_custom_path = 路徑
select_custom_current_path = 目前路徑
select_custom_future_path = 未來路徑
select_custom_path_current_name = 路徑 + 目前檔名
select_custom_path_future_name = 路徑 + 未來檔名
select_custom_directory_file = 目錄/檔案
select_custom_select_directory = 選擇目錄
select_custom_unselect_directory = 取消選擇目錄
# General
dialog_button_ok = 確認
dialog_button_cancel = 取消
# Dialogs
dialog_name_files_to_include = 要包含的檔案
dialog_name_folders_to_include = 要包含的資料夾
dialog_scan_inside = 掃描內部
dialog_ignore_folders = 忽略資料夾
dialog_confirm_renaming = 確認重新命名...
dialog_outdated_results = 過時的結果
dialog_results_of_renaming = 重新命名的結果
dialog_save_rule = 儲存規則
dialog_select_custom = 自訂選擇
dialog_unselect_custom = 取消自訂選擇

# Rule Window


## Common

label_usage_type = 用途類型：
label_example = 範例
label_example_text_before = 原始：
label_example_text_after = 結果：
button_rule_window_add = 新增規則

## Custom

label_custom_instruction =
    $(NAME) - 列印基礎檔名
    $(EXT) - 列印副檔名
    $(MODIF) - 列印檔案修改日期
    $(CREAT) - 列印檔案創建
    $(CURR) - 列印目前基礎檔名及其副檔名
    $(PARENT) - 列印父資料夾名稱
    $(N)/$(K) - 列印數字（參數是可選的）
    $(N:3:4:5) 列印數字，從 3 開始，依次遞增 4，
            並以零填充至 5 位數。
    	K 不只是列表中的次序，也使用資料夾中的次序。
menu_button_load_custom_rule = 自訂規則選擇器
button_save_custom_rule = 儲存自訂規則

## Upper/Lower Case

check_button_letters_type_uppercase = 大寫
check_button_letters_type_lowercase = 小寫
check_button_letters_usage_name = 僅基礎檔名
check_button_letters_usage_extension = 僅副檔名
check_button_letters_usage_both = 兩者皆是
label_letters_tool_type = 工具類型：
# Purge
label_purge_tool_type = 工具類型：
check_button_purge_name = 僅基礎檔名
check_button_purge_extension = 僅副檔名
check_button_purge_both = 兩者皆是
# Add number
label_add_number_place = 插入數字：
label_add_number_settings = 數字設定：
check_button_add_number_before_name = 基礎檔名之前
check_button_add_number_after_name = 基礎檔名之後
label_number_start_number = 起始數字
label_number_step = 遞增值
label_number_fill_zeros = 用零填充
# Add text
check_button_add_text_before_name = 基礎檔名之前
check_button_add_text_after_name = 基礎檔名之後
label_add_text = 要插入的文字：
# Replace
check_button_replace_name = 僅基礎檔名
check_button_replace_extension = 僅副檔名
check_button_replace_both = 兩者皆是
check_button_replace_case_sensitive = 區分大小寫
check_button_replace_case_insensitive = 不區分大小寫
check_button_replace_regex = 使用正規表示式
check_button_replace_replace_all = 取代所有出現項目
label_replace_replacing_strings = 取代字串：
label_replace_text_to_find = 要尋找的文字
label_replace_text_to_replace = 要取代的文字
label_replace_captures = 擷取
label_replace_captured_captures = 捕獲的擷取
label_replace_captures_number = （{ $capture_number } 擷取）
label_replace_no_captures = 沒有擷取
label_replace_invalid_regex = 無效的正規表示式
# Trim
check_button_trim_name_start = 基礎檔名開頭
check_button_trim_name_end = 基礎檔名結尾
check_button_trim_extension_start = 副檔名開頭
check_button_trim_extension_end = 副檔名結尾
check_button_trim_case_sensitive = 區分大小寫
check_button_trim_case_insensitive = 不區分大小寫
label_trim_trim_text = 修剪文字
label_trim_case_sensitivity = 不區分大小寫
# Normalize name
label_normalize_name =
    全部內容 - 重新命名為一個僅包含以下
                              字元 `a-z`, `0-9`, `-`, ` `, `.` 的名稱。
                              例如：`żółć CZERona.Txt` -> `zolc-czerwona.txt`
    
    部分內容 - 與上述選項相同，但允許使用
                      中間空格 ` ` 與大寫字母 `A-Z`。
                      例如：` Źrebię Krokietowe.Rar ` -> `Zrebie Krokietowe.Rar`
check_button_normalize_everything = 全部內容
check_button_normalize_partial = 部分內容
# RuleType
rule_type_custom = 自訂
rule_type_case_size = 大小寫
rule_type_purge = 清理
rule_type_add_text = 插入文字
rule_type_trim = 修剪
rule_type_replace = 取代
rule_type_add_number = 插入數字
rule_type_normalize = 正常化
# RulePlace
rule_place_none = 無
rule_place_extension = 僅副檔名
rule_place_name = 僅基礎檔名
rule_place_extension_name = 副檔名與基礎檔名
rule_place_before_extension = 副檔名之前
rule_place_after_extension = 副檔名之後
rule_place_before_name = 基礎檔名之前
rule_place_after_name = 基礎檔名之後
rule_place_from_name_start = 從基礎檔名開頭
rule_place_from_name_end_reverse = 從基礎檔名結尾到開頭
rule_place_from_extension_start = 從副檔名開頭
rule_place_from_extension_end_reverse = 從副檔名結尾到開頭
# Rule Description
rule_description_full_normalize = 完全正常化
rule_description_partial_normalize = 部分正常化
rule_description_zeros = 並以 { $zeros } 個零填充，
rule_description_step = 從{ $start }開始，遞增值為 { $step }{ $zeros }
rule_description_lowercase = 小寫
rule_description_uppercase = 大寫
rule_description_text = 文字
rule_description_added_text = 插入文字：
rule_description_start = 開頭
rule_description_end_of_name = 基礎檔名結尾
rule_description_extension = 副檔名
rule_description_end_of_extension = 副檔名結尾
rule_description_trimming = 修剪「{ $trim_text }」從{ $where_remove }開始
rule_description_custom_rule = 自訂規則：{ $custom_rule }
rule_description_replace = 取代 { $additional_regex_text } "{ $text_to_find }" 為 "{ $text_to_replace }"
# Notebooks
notebook_tab_custom = 自訂
notebook_tab_case_size = 大寫/小寫字母
notebook_tab_purge = 清理
notebook_tab_add_number = 插入數字
notebook_tab_add_text = 插入文字
notebook_tab_replace = 取代
notebook_tab_trim = 修飾
notebook_tab_normalize = 正常化檔名
# Renaming dialog
renaming_question = 您確定要重新命名 { $number_of_renamed_files } 檔案嗎？
renaming_destination_file_exists = 目標檔案已存在。
renaming_renamed_files = 成功重新命名 { $properly_renamed } 檔案
renaming_ignored_files = 忽略 { $ignored } 檔案，因為變更前後的檔名是相同的。
renaming_failed_files = 重新命名 { $failed_vector } 檔案失敗
renaming_list_of_failed_to_rename = 所有重新命名失敗列表
renaming_error = 錯誤
renaming_some_records_not_updated = 一些記錄未被更新，您可以點擊更新檔名按鈕進行更新。\n您確定要繼續而不更新檔名嗎？
renaming_missing_files = 缺少檔案
renaming_require_missing_files = 您需要使用至少 1 個檔案
renaming_missing_rules = 缺少規則
renaming_require_missing_rules = 您需要使用至少 1 條規則
