# Upper buttons
upper_start_renaming_button = 开始重命名中
upper_add_files_button = 添加文件
upper_add_folders_button = 添加文件夹
upper_remove_selection_button = 移除选择
upper_update_names_button = 更新名称
upper_results_one_up_button = 上一个
upper_results_one_down_button = 下一个
upper_select_popup_button = 选择
# Bottom Buttons
bottom_rule_add_button = 添加规则
bottom_rule_edit_button = 编辑规则
bottom_rule_remove_button = 删除规则
bottom_rule_one_up_button = 上一个
bottom_rule_one_down_button = 下一个
bottom_rule_save_rules_button = 保存规则
bottom_rule_load_rules_button = 加载规则
# Edit names
edit_names_used_in_rules = 规则中使用的名称： { $rules }
edit_names_choose_name = 选择规则名称(如果存在将覆盖)
# Tree View Rules
tree_view_upper_column_type = 类型
tree_view_upper_column_current_name = 当前名称
tree_view_upper_column_future_name = 未来名称
tree_view_upper_column_path = 路径
# Tree View Results
tree_view_bottom_tool_type = 工具类型
tree_view_bottom_usage_name = 用法名称
tree_view_bottom_description = 描述
# Settings
settings_language_label = 语言
settings_open_rules = 打开规则设置文件
settings_open_cache_custom_texts = 打开自定义缓存文件
settings_open_config_dir = 打开缓存目录
# Other in main window
bottom_rule_label_rules = 规则
upper_files_folders_label = 文件/文件夹
upper_files_folders_label_update = 文件/文件夹({ $files_number }) - ### 更新请求### #
upper_files_folders_label_up_to_date = 文件/文件夹({ $files_number }) - 最新版本
# Select popover
button_select_all = 选择所有
button_select_reverse = 反向选择
button_select_custom = 选择自定义
button_unselect_custom = 取消选择自定义
button_select_changed = 选择已更改
button_unselect_changed = 取消选择更改
# Un/Select custom
select_custom_example = 用法：*/folder-nr*/* 或name-version-*.txt
select_custom_path = 路径
select_custom_current_path = 当前路径
select_custom_future_path = 未来路径
select_custom_path_current_name = 路径 + 当前名称
select_custom_path_future_name = 路径 + 未来名称
select_custom_directory_file = 目录/文件
select_custom_select_directory = 选择目录
select_custom_unselect_directory = 取消选择目录
# General
dialog_button_ok = 好的
dialog_button_cancel = 取消
# Dialogs
dialog_name_files_to_include = 要包含的文件
dialog_name_folders_to_include = 要包含的文件夹
dialog_scan_inside = 扫描內部的
dialog_ignore_folders = 忽略文件夹
dialog_confirm_renaming = 确认重命名...
dialog_outdated_results = 过时的结果
dialog_results_of_renaming = 重命名结果
dialog_save_rule = 保存规则
dialog_select_custom = 选择自定义
dialog_unselect_custom = 取消选择自定义

# Rule Window


## Common

label_usage_type = 用法类型：
label_example = example
label_example_text_before = 之前：
label_example_text_after = 之后：
button_rule_window_add = 添加规则

## Custom

label_custom_instruction =
    $(NAME) - 打印文件名
    $(EXT) - 打印扩展名
    $(MODIF) - 打印文件修改日期
    $(CREAT) - 打印文件创建
    $(CURR) - 打印当前文件名并扩展名
    $(PARENT) - 打印父文件夹名称
    $(N)/$(K) - 打印数字(参数是可选的)
    $(N:3:4:5) 打印数字从3, 打印数字。 带第4步
            并填充零到5个位置。
    	K只是列表中的位置，也使用文件夹中的位置项。
menu_button_load_custom_rule = 自定义规则选择器
button_save_custom_rule = 保存自定义规则

## Upper/Lower Case

check_button_letters_type_uppercase = 大写
check_button_letters_type_lowercase = 小写
check_button_letters_usage_name = 仅名称
check_button_letters_usage_extension = 仅扩展
check_button_letters_usage_both = 两者都是
label_letters_tool_type = 工具类型：
# Purge
label_purge_tool_type = 工具类型：
check_button_purge_name = 仅名称
check_button_purge_extension = 仅扩展
check_button_purge_both = 两者都是
# Add number
label_add_number_place = 放置号码：
label_add_number_settings = 号码设置：
check_button_add_number_before_name = 名字前
check_button_add_number_after_name = 名字之后
label_number_start_number = 起始号码
label_number_step = 步骤
label_number_fill_zeros = 用零填充
# Add text
check_button_add_text_before_name = 名字前
check_button_add_text_after_name = 名字之后
label_add_text = 要添加的文本：
# Replace
check_button_replace_name = 仅名称
check_button_replace_extension = 仅扩展
check_button_replace_both = 两者都是
check_button_replace_case_sensitive = 区分大小写
check_button_replace_case_insensitive = 不敏感大小写
check_button_replace_regex = 使用正则表达式
check_button_replace_replace_all = 替换所有事件
label_replace_replacing_strings = 替换字符串：
label_replace_text_to_find = 要查找的文本
label_replace_text_to_replace = 替换的文本
label_replace_captures = 抓取
label_replace_captured_captures = 捕获的抓取
label_replace_captures_number = ({ $capture_number } captures)
label_replace_no_captures = 没有抓取
label_replace_invalid_regex = 无效的凭单
# Trim
check_button_trim_name_start = 名称开始
check_button_trim_name_end = 名称结束
check_button_trim_extension_start = 扩展开始
check_button_trim_extension_end = 扩展结束
check_button_trim_case_sensitive = 区分大小写
check_button_trim_case_insensitive = 不敏感大小写
label_trim_trim_text = 修剪文本
label_trim_case_sensitivity = 案例灵敏度
# Normalize name
label_normalize_name =
    Everything - renames the name to one that contains only the
                              characters `a-z`, `0-9`, `-`, ` `, `.`.
                              e.g. `żółć CZERona.Txt` -> `zolc-czerwona.txt`
    
    Partial - works exactly same as option above, but allows
                      to use spaces ` ` and big letters `A-Z`
                      e.g. ` Źrebię Krokietowe.Rar ` -> `Zrebie Krokietowe.Rar`
check_button_normalize_everything = 全部内容
check_button_normalize_partial = 部分的
# RuleType
rule_type_custom = 自定义
rule_type_case_size = 大小写
rule_type_purge = Purge
rule_type_add_text = 添加文本
rule_type_trim = 修饰
rule_type_replace = 替换
rule_type_add_number = 添加号码
rule_type_normalize = 正常化
# RulePlace
rule_place_none = 无
rule_place_extension = 仅扩展
rule_place_name = 仅名称
rule_place_extension_name = 扩展名和名称
rule_place_before_extension = 在扩展名前
rule_place_after_extension = 扩展后
rule_place_before_name = 名字前
rule_place_after_name = 名字之后
rule_place_from_name_start = 从开始开始
rule_place_from_name_end_reverse = 从名称结束到开始
rule_place_from_extension_start = 从扩展开始
rule_place_from_extension_end_reverse = 从扩展末尾到开始
# Rule Description
rule_description_full_normalize = 完全正常化
rule_description_partial_normalize = 部分正常化
rule_description_zeros = 并以 { $zeros } 零填充，
rule_description_step = 从 { $start } 开始，步骤为 { $step }{ $zeros }
rule_description_lowercase = 小写
rule_description_uppercase = 大写
rule_description_text = 文本
rule_description_added_text = 添加文本：
rule_description_start = 开始
rule_description_end_of_name = 名字结尾
rule_description_extension = 扩展
rule_description_end_of_extension = 扩展结束
rule_description_trimming = 正在从{ $trim_text }“从 { $where_remove } 中尝试” { $trim_text }
rule_description_custom_rule = 自定义规则： { $custom_rule }
rule_description_replace = 正在替换 { $additional_regex_text } "{ $text_to_find }" 为 "{ $text_to_replace }"
# Notebooks
notebook_tab_custom = 自定义
notebook_tab_case_size = 上级/下级案件
notebook_tab_purge = Purge
notebook_tab_add_number = 添加号码
notebook_tab_add_text = 添加文本
notebook_tab_replace = 替换
notebook_tab_trim = 修饰
notebook_tab_normalize = 正常化名称
# Renaming dialog
renaming_question = 您确定要重命名 { $number_of_renamed_files } 文件吗？
renaming_destination_file_exists = 目标文件已存在。
renaming_renamed_files = 正确重命名 { $properly_renamed } 文件
renaming_ignored_files = 忽略 { $ignored } 文件，因为更改前后的名称是相同的。
renaming_failed_files = 重命名 { $failed_vector } 文件失败
renaming_list_of_failed_to_rename = 所有失败重命名列表
renaming_error = 错误
renaming_some_records_not_updated = 一些记录未被更新，您可以点击更新名称按钮进行更新。\n您确定要继续不更新名称吗？
renaming_missing_files = 缺少文件
renaming_require_missing_files = 您需要使用至少 1 个文件
renaming_missing_rules = 缺少规则
renaming_require_missing_rules = 您需要使用至少 1 条规则
