# Upper buttons
upper_start_renaming_button = Начать переименование
upper_add_files_button = Добавить файлы
upper_add_folders_button = Добавить папки
upper_remove_selection_button = Удалить выделенное
upper_update_names_button = Обновить имена
upper_results_one_up_button = Один Вверх
upper_results_one_down_button = Один вниз
upper_select_popup_button = Выбрать
# Bottom Buttons
bottom_rule_add_button = Добавить правило
bottom_rule_edit_button = Изменить правило
bottom_rule_remove_button = Удалить правило
bottom_rule_one_up_button = Один Вверх
bottom_rule_one_down_button = Один вниз
bottom_rule_save_rules_button = Сохранить правила
bottom_rule_load_rules_button = Загрузить правила
# Edit names
edit_names_used_in_rules = Названия, используемые в правилах: { $rules }
edit_names_choose_name = Выберите имя правил (если существует, будет переопределить его)
# Tree View Rules
tree_view_upper_column_type = Тип
tree_view_upper_column_current_name = Текущее имя
tree_view_upper_column_future_name = Будущее имя
tree_view_upper_column_path = Путь
# Tree View Results
tree_view_bottom_tool_type = Тип инструмента
tree_view_bottom_usage_name = Имя использования
tree_view_bottom_description = Описание
# Settings
settings_language_label = Язык
settings_open_rules = Открыть файл настроек правил
settings_open_cache_custom_texts = Открыть файл кэша
settings_open_config_dir = Открыть каталог кэша
check_button_dark_theme = Темные иконки
# Other in main window
bottom_rule_label_rules = Правила
upper_files_folders_label = Файлы/Папки
upper_files_folders_label_update = Файлы/Папки({ $files_number }) - ##### ОБНОВЛЕНИЕ ТРЕБУЕТСЯ #####
upper_files_folders_label_up_to_date = Файлы/Папки({ $files_number }) - актуальное
# Select popover
button_select_all = Выделить все
button_select_reverse = Обратный выбор
button_select_custom = Выбрать пользовательский
button_unselect_custom = Снять выбор
button_select_changed = Выбрать измененные
button_unselect_changed = Снять отметку с изменений
# Un/Select custom
select_custom_example = Использование: */folder-nr*/* или name-version-*.txt
select_custom_path = Путь
select_custom_current_path = Текущий путь
select_custom_future_path = Будущий путь
select_custom_path_current_name = Путь + текущее имя
select_custom_path_future_name = Путь + Будущее имя
select_custom_directory_file = Каталог/Файл
select_custom_select_directory = Выберите папку
select_custom_unselect_directory = Отменить выбор каталога
# General
dialog_button_ok = Ок
dialog_button_cancel = Отмена
# Dialogs
dialog_name_files_to_include = Включать файлы
dialog_name_folders_to_include = Папки для включения
dialog_scan_inside = Сканировать внутри
dialog_ignore_folders = Игнорировать папки
dialog_confirm_renaming = Подтвердите переименование
dialog_outdated_results = Результаты устарели
dialog_results_of_renaming = Результаты переименования
dialog_save_rule = Сохранить правило
dialog_select_custom = Выбрать пользовательский
dialog_unselect_custom = Снять выбор

# Rule Window


## Common

label_usage_type = Тип использования:
label_example = ИСПОЛЬЗОВАТЬ
label_example_text_before = До:
label_example_text_after = После:
button_rule_window_add = Добавить правило

## Custom

label_custom_instruction =
    $(NAME) - выводит имя файла
    $(EXT) - выводит расширение
    $(MODIF) - выводит дату модификации файла
    $(CREAT) - выводит создание файла
    $(CURR) - выводит текущее имя файла с расширением
    $(PARENT) - выводит имя родительской папки
    $(N)/$(K) - выводит numbers(аргументы являются необязательными)
    $(N:3:4:5) выводит числа из 3, на четвертом шаге
            и заполняет их нулями до 5 позиций.
    	К вместо этого только позиция в списке, также использует элемент позиции в папке.
menu_button_load_custom_rule = Пользовательский выбор правил
button_save_custom_rule = Сохранить пользовательское правило

## Upper/Lower Case

check_button_letters_type_uppercase = Прописные
check_button_letters_type_lowercase = Строчные буквы
check_button_letters_usage_name = Только имя
check_button_letters_usage_extension = Только расширение
check_button_letters_usage_both = Оба
label_letters_tool_type = Тип инструмента:
# Purge
label_purge_tool_type = Тип инструмента:
check_button_purge_name = Только имя
check_button_purge_extension = Только расширение
check_button_purge_both = Оба
# Add number
label_add_number_place = Место для размещения:
label_add_number_settings = Количество настроек:
check_button_add_number_before_name = Перед именем
check_button_add_number_after_name = После имени
label_number_start_number = Начальный номер
label_number_step = Шаг
label_number_fill_zeros = Заполнить нулями
# Add text
check_button_add_text_before_name = Перед именем
check_button_add_text_after_name = После имени
label_add_text = Текст для добавления:
# Replace
check_button_replace_name = Только имя
check_button_replace_extension = Только расширение
check_button_replace_both = Оба
check_button_replace_case_sensitive = Учитывать регистр
check_button_replace_case_insensitive = Без учета регистра
check_button_replace_regex = Использовать регулярные выражения
check_button_replace_replace_all = Заменить все события
label_replace_replacing_strings = Замена строк:
label_replace_text_to_find = Текст для поиска
label_replace_text_to_replace = Заменный текст
label_replace_captures = Захваты
label_replace_captured_captures = Захваченные снимки
label_replace_captures_number = ({ $capture_number } captures)
label_replace_no_captures = Нет снимков
label_replace_invalid_regex = НЕВЕРНЫЙ РЕЖИМ
# Trim
check_button_trim_name_start = Начать имя
check_button_trim_name_end = Конец имени
check_button_trim_extension_start = Начало расширения
check_button_trim_extension_end = Окончание расширения
check_button_trim_case_sensitive = Учитывать регистр
check_button_trim_case_insensitive = Без учета регистра
label_trim_trim_text = Обрезать текст
label_trim_case_sensitivity = Чувствительность к регистру
# Normalize name
label_normalize_name =
    Everything - renames the name to one that contains only the
                              characters `a-z`, `0-9`, `-`, ` `, `.`.
                              e.g. `żółć CZERona.Txt` -> `zolc-czerwona.txt`
    
    Partial - works exactly same as option above, but allows
                      to use spaces ` ` and big letters `A-Z`
                      e.g. ` Źrebię Krokietowe.Rar ` -> `Zrebie Krokietowe.Rar`
check_button_normalize_everything = Все
check_button_normalize_partial = Частично
# RuleType
rule_type_custom = Свой
rule_type_case_size = Размер заявки
rule_type_purge = Purge
rule_type_add_text = Добавить текст
rule_type_trim = Обрезать
rule_type_replace = Заменить
rule_type_add_number = Добавить номер
rule_type_normalize = Нормализовать
# RulePlace
rule_place_none = Н/Д
rule_place_extension = Только расширение
rule_place_name = Только имя
rule_place_extension_name = Расширение и имя
rule_place_before_extension = Перед расширением
rule_place_after_extension = После расширения
rule_place_before_name = Перед именем
rule_place_after_name = После имени
rule_place_from_name_start = С начала
rule_place_from_name_end_reverse = С конца имени до начала
rule_place_from_extension_start = С начала расширения
rule_place_from_extension_end_reverse = От окончания расширения до начала
# Rule Description
rule_description_full_normalize = Полная нормализация
rule_description_partial_normalize = Частичная нормализация
rule_description_zeros = и заполняйте { $zeros } нулями,
rule_description_step = Начиная с { $start } с шагом { $step }{ $zeros }
rule_description_lowercase = Строчные буквы
rule_description_uppercase = Прописные
rule_description_text = текст
rule_description_added_text = Добавлен текст:
rule_description_start = старт
rule_description_end_of_name = конец имени
rule_description_extension = расширение
rule_description_end_of_extension = конец расширения
rule_description_trimming = Обрезка "{ $trim_text }" от { $where_remove }
rule_description_custom_rule = Другое правило: { $custom_rule }
rule_description_replace = Замена { $additional_regex_text } "{ $text_to_find }" на "{ $text_to_replace }"
# Notebooks
notebook_tab_custom = Свой
notebook_tab_case_size = Верхний/нижний регистр
notebook_tab_purge = Purge
notebook_tab_add_number = Добавить номер
notebook_tab_add_text = Добавить текст
notebook_tab_replace = Заменить
notebook_tab_trim = Обрезать
notebook_tab_normalize = Нормализовать имя
# Renaming dialog
renaming_question = Вы уверены, что хотите переименовать { $number_of_renamed_files } файлов?
renaming_destination_file_exists = Файл назначения уже существует.
renaming_renamed_files = Правильно переименовано { $properly_renamed } файлов
renaming_ignored_files = Игнорировать { $ignored } файлов, потому что имя до и после изменения одинаковое.
renaming_failed_files = Не удалось переименовать { $failed_vector } файлов
renaming_list_of_failed_to_rename = Список неудачных переименований
renaming_error = ошибка
renaming_some_records_not_updated = Некоторые записи не обновляются, вы можете сделать это, нажав на кнопку Update Names.\nВы уверены, что хотите продолжить без обновления имени?
renaming_missing_files = Отсутствующие файлы
renaming_require_missing_files = Вы должны использовать хотя бы 1 файл
renaming_missing_rules = Пропущенные правила
renaming_require_missing_rules = Вы должны использовать по крайней мере 1 правило
