# Upper buttons
upper_start_renaming_button = Start Renaming
upper_add_files_button = Add Files
upper_add_folders_button = Add Folders
upper_remove_selection_button = Remove Selection
upper_update_names_button = Update Names
upper_results_one_up_button = One Up
upper_results_one_down_button = One Down
upper_select_popup_button = Select

# Bottom Buttons
bottom_rule_add_button = Add Rule
bottom_rule_edit_button = Edit Rule
bottom_rule_remove_button = Remove Rule
bottom_rule_one_up_button = One Up
bottom_rule_one_down_button = One Down
bottom_rule_save_rules_button = Save Rules
bottom_rule_load_rules_button = Load Rules

# Edit names
edit_names_used_in_rules = Names used in rules: { $rules }
edit_names_choose_name = Choose name of rules(if exists, will override it)

# Tree View Rules
tree_view_upper_column_type = Type
tree_view_upper_column_current_name = Current Name
tree_view_upper_column_future_name = Future Name
tree_view_upper_column_path = Path

# Tree View Results
tree_view_bottom_tool_type = Tool Type
tree_view_bottom_usage_name = Usage Name
tree_view_bottom_description = Description

# Settings
settings_language_label = Language
settings_open_rules = Open rules settings file
settings_open_cache_custom_texts = Open custom cache file
settings_open_config_dir = Open cache dir
check_button_dark_theme = Dark theme icons

# Other in main window
bottom_rule_label_rules = Rules
upper_files_folders_label = Files/Folders
upper_files_folders_label_update = Files/Folders({ $files_number }) - ##### UPDATE REQUIRED #####
upper_files_folders_label_up_to_date = Files/Folders({ $files_number }) - up to date

# Select popover
button_select_all = Select All
button_select_reverse = Reverse Selection
button_select_custom = Select Custom
button_unselect_custom = Unselect Custom
button_select_changed = Select Changed
button_unselect_changed = Unselect Changed

# Un/Select custom
select_custom_example = Usage: */folder-nr*/* or name-version-*.txt
select_custom_path = Path
select_custom_current_path = Current Path
select_custom_future_path = Future Path
select_custom_path_current_name = Path + Current Name
select_custom_path_future_name = Path + Future Name
select_custom_directory_file = Directory/File
select_custom_select_directory = Select Directory
select_custom_unselect_directory = Unselect Directory

# General
dialog_button_ok = Ok
dialog_button_cancel = Cancel

# Dialogs
dialog_name_files_to_include = Files to include
dialog_name_folders_to_include = Folders to include
dialog_scan_inside = Scan inside
dialog_ignore_folders = Ignore folders
dialog_confirm_renaming = Confirm renaming
dialog_outdated_results = Outdated results
dialog_results_of_renaming = Results of renaming
dialog_save_rule = Save Rule
dialog_select_custom = Select Custom
dialog_unselect_custom = Unselect Custom

# Rule Window
## Common
label_usage_type = Usage Type:
label_example = EXAMPLE
label_example_text_before = Before:
label_example_text_after = After:
button_rule_window_add = Rule Add

## Custom
label_custom_instruction = $(NAME) - prints file name
                           $(EXT) - prints extension
                           $(MODIF) - prints file modification date
                           $(CREAT) - prints file creation
                           $(CURR) - prints current file name with extension
                           $(PARENT) - prints parent folder name
                           $(N)/$(K) - prints numbers(arguments are optional)
                           $(N:3:4:5) prints numbers from 3, with step 4
                                   and fills them with zeros to 5 positions.
                           	K instead only position in list, also uses position item in folder.

menu_button_load_custom_rule = Custom rule chooser
button_save_custom_rule = Save custom rule



## Upper/Lower Case
check_button_letters_type_uppercase = Uppercase
check_button_letters_type_lowercase = Lowercase
check_button_letters_usage_name = Only Name
check_button_letters_usage_extension = Only Extension
check_button_letters_usage_both = Both
label_letters_tool_type = Tool Type:

# Purge
label_purge_tool_type = Tool Type:
check_button_purge_name = Only Name
check_button_purge_extension = Only Extension
check_button_purge_both = Both

# Add number
label_add_number_place = Place to put number:
label_add_number_settings = Number settings:
check_button_add_number_before_name = Before Name
check_button_add_number_after_name = After Name
label_number_start_number = Start number
label_number_step = Step
label_number_fill_zeros = Fill with zeros

# Add text
check_button_add_text_before_name = Before Name
check_button_add_text_after_name = After Name
label_add_text = Text to add:

# Replace
check_button_replace_name = Only Name
check_button_replace_extension = Only Extension
check_button_replace_both = Both
check_button_replace_case_sensitive = Case Sensitive
check_button_replace_case_insensitive = Case Insensitive
check_button_replace_regex = Use regex
check_button_replace_replace_all = Replace all occurrences
label_replace_replacing_strings = Replacing Strings:
label_replace_text_to_find = Text to find
label_replace_text_to_replace = Replaced text
label_replace_captures = Captures
label_replace_captured_captures = Captured captures
label_replace_captures_number = ({ $capture_number } captures)
label_replace_no_captures = No captures
label_replace_invalid_regex = INVALID REGEX

# Trim
check_button_trim_name_start = Name Start
check_button_trim_name_end = Name End
check_button_trim_extension_start = Extension Start
check_button_trim_extension_end = Extension End
check_button_trim_case_sensitive = Case Sensitive
check_button_trim_case_insensitive = Case Insensitive
label_trim_trim_text = Trim text
label_trim_case_sensitivity = Case Sensitivity

# Normalize name
label_normalize_name = Everything - renames the name to one that contains only the
                                                 characters `a-z`, `0-9`, `-`, ` `, `.`.
                                                 e.g. `żółć CZERona.Txt` -> `zolc-czerwona.txt`

                       Partial - works exactly same as option above, but allows
                                         to use spaces ` ` and big letters `A-Z`
                                         e.g. ` Źrebię Krokietowe.Rar ` -> `Zrebie Krokietowe.Rar`

check_button_normalize_everything = Everything
check_button_normalize_partial = Partial

# RuleType
rule_type_custom = Custom
rule_type_case_size = Case Size
rule_type_purge = Purge
rule_type_add_text = Add Text
rule_type_trim = Trim
rule_type_replace = Replace
rule_type_add_number = Add Number
rule_type_normalize = Normalize

# RulePlace
rule_place_none = N/A
rule_place_extension = Only Extension
rule_place_name = Only Name
rule_place_extension_name = Extension and Name
rule_place_before_extension = Before Extension
rule_place_after_extension = After Extension
rule_place_before_name = Before Name
rule_place_after_name = After Name
rule_place_from_name_start = From Start
rule_place_from_name_end_reverse = From Name End to Start
rule_place_from_extension_start = From Extension Start
rule_place_from_extension_end_reverse = From Extension End to Start

# Rule Description
rule_description_full_normalize = Full normalize
rule_description_partial_normalize = Partial normalize
rule_description_zeros = and filling with { $zeros } zeros,
rule_description_step = Starting with { $start } with step { $step }{ $zeros }
rule_description_lowercase = Lowercase
rule_description_uppercase = Uppercase
rule_description_text = text
rule_description_added_text = Added text:
rule_description_start = start
rule_description_end_of_name = end of name
rule_description_extension = extension
rule_description_end_of_extension = end of extension
rule_description_trimming = Trimming "{ $trim_text }" from { $where_remove }
rule_description_custom_rule = Custom rule: { $custom_rule }
rule_description_replace = Replacing { $additional_regex_text } "{ $text_to_find }" with "{ $text_to_replace }"

# Notebooks
notebook_tab_custom = Custom
notebook_tab_case_size = Upper/Lower Cases
notebook_tab_purge = Purge
notebook_tab_add_number = Add Number
notebook_tab_add_text = Add Text
notebook_tab_replace = Replace
notebook_tab_trim = Trim
notebook_tab_normalize = Normalize Name

# Renaming dialog
renaming_question = Are you sure that you want to rename { $number_of_renamed_files } files?
renaming_destination_file_exists = Destination file already exists.
renaming_renamed_files = Properly renamed { $properly_renamed } files
renaming_ignored_files = Ignored { $ignored } files, because the name before and after the change are the same.
renaming_failed_files = Failed to rename { $failed_vector } files
renaming_list_of_failed_to_rename = List of all failing renames
renaming_error = error
renaming_some_records_not_updated = Some records are not updated, you can do it by clicking at the Update Names button.\nAre you sure that you want to proceed without updating names?
renaming_missing_files = Missing Files
renaming_require_missing_files = You need to use at least 1 file
renaming_missing_rules = Missing Rules
renaming_require_missing_rules = You need to use at least 1 rule




