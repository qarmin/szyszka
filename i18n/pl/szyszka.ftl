# Upper buttons
upper_start_renaming_button = Zmień nazwę
upper_add_files_button = Dodaj Pliki
upper_add_folders_button = Dodaj foldery
upper_remove_selection_button = Usuń zaznaczone
upper_update_names_button = Aktualizuj nazwy
upper_results_one_up_button = Jeden w górę
upper_results_one_down_button = Jedno w dół
upper_select_popup_button = Zaznacz
# Bottom Buttons
bottom_rule_add_button = Dodaj regułę
bottom_rule_edit_button = Edytuj regułę
bottom_rule_remove_button = Usuń regułę
bottom_rule_one_up_button = Jeden w górę
bottom_rule_one_down_button = Jedno w dół
bottom_rule_save_rules_button = Zapisz reguły
bottom_rule_load_rules_button = Załaduj reguły
# Tree View Rules
tree_view_upper_column_type = Rodzaj
tree_view_upper_column_current_name = Bierząca nazwa
tree_view_upper_column_future_name = Przyszła nazwa
tree_view_upper_column_path = Ścieżka
# Tree View Results
tree_view_bottom_tool_type = Typ
tree_view_bottom_usage_name = Typ użycia
tree_view_bottom_description = Opis
# Settings
settings_language_label = Język
settings_open_rules = Otwórz plik ustawień reguł
settings_open_cache_custom_texts = Otwórz plik ustawień niestandardowych reguł
settings_open_config_dir = Otwórz katalog konfiguracyjny
# Other in main window
bottom_rule_label_rules = Reguły
upper_files_folders_label = Pliki/Foldery
# Select popover
button_select_all = Zaznacz wszystko
button_select_reverse = Odwróć zaznaczenie
button_select_custom = Własne zaznaczanie
button_unselect_custom = Własne odznaczanie
button_select_changed = Wybierz zmienione
button_unselect_changed = Odznacz zmienione
# Un/Select custom
select_custom_example = Składnia: */folder-nr*/* lub name-version-*.txt
select_custom_path = Ścieżka
select_custom_current_path = Bieżąca ścieżka
select_custom_future_path = Przyszła ścieżka
select_custom_path_current_name = Ścieżka + bieżąca nazwa
select_custom_path_future_name = Ścieżka + Przyszła nazwa
select_custom_directory_file = Katalog/Plik
select_custom_select_directory = Wybierz katalog
select_custom_unselect_directory = Odznacz katalog
# General
dialog_button_ok = Ok
dialog_button_cancel = Anuluj
# Dialogs
dialog_name_files_to_include = Pliki do dodania
dialog_name_folders_to_include = Foldery do dodania
dialog_scan_inside = Skanuj wewnątrz
dialog_ignore_folders = Ignoruj foldery

# Rule Window


## Common

label_usage_type = Typ użycia:
label_example = PRZYKŁAD
label_example_text_before = Przed:
label_example_text_after = Po:
button_rule_window_add = Dodaj regułę

## Custom

label_custom_instruction =
    $(NAME) - dodaje nazwę pliku
    $(EXT) - dodaje rozszerzenie
    $(MODIF) - dodaje datę modyfikacji pliku
    $(CREAT) - dodaje datę utworzenie pliku
    $(CURR) - dodaje bieżącą nazwę pliku z rozszerzeniem
    $(PARENT) - dodaje nazwę folderu nadrzędnego
    $(N)/$(K) - dodaje numery (argumenty są opcjonalne)
    $(N:3:4:5) dodaje numery od 3, z krokiem 4
            i wypełnia je zerami do 5 pozycji.
    	K zamiast pozycji na liście, używa pozycji w folderze.
menu_button_load_custom_rule = Wybierz zapisane reguły
button_save_custom_rule = Zapisz własną regułę

## Upper/Lower Case

check_button_letters_type_uppercase = Wielkie litery
check_button_letters_type_lowercase = Małe litery
check_button_letters_usage_name = Tylko Nazwa
check_button_letters_usage_extension = Tylko rozszerzenie
check_button_letters_usage_both = Obydwa
label_letters_tool_type = Typ narzędzia:
# Purge
label_purge_tool_type = Typ narzędzia:
check_button_purge_name = Tylko Nazwa
check_button_purge_extension = Tylko rozszerzenie
check_button_purge_both = Obydwa
# Add number
label_add_number_place = Miejsce do wpisania numeru:
label_add_number_settings = Ustawienia liczb:
check_button_add_number_before_name = Przed nazwą
check_button_add_number_after_name = Po nazwie
label_number_start_number = Numer początkowy
label_number_step = Krok
label_number_fill_zeros = Wypełnij zerami
# Add text
check_button_add_text_before_name = Przed nazwą
check_button_add_text_after_name = Po nazwie
label_add_text = Tekst do dodania:
# Replace
check_button_replace_name = Tylko Nazwa
check_button_replace_extension = Tylko rozszerzenie
check_button_replace_both = Obydwa
check_button_replace_case_sensitive = Uwzględnij Wielkość Liter
check_button_replace_case_insensitive = Nieuwzględniaj wielkości liter
check_button_replace_regex = Używaj regex
check_button_replace_replace_all = Zastąp wszystkie wystąpienia
label_replace_replacing_strings = Zastępowanie ciągów:
label_replace_text_to_find = Tekst do znalezienia
label_replace_text_to_replace = Tekst do zastąpienia
# Trim
check_button_trim_name_start = Początek nazwy
check_button_trim_name_end = Koniec nazwy
check_button_trim_extension_start = Początek rozszerzenia
check_button_trim_extension_end = Koniec rozszerzenia
check_button_trim_case_sensitive = Uwzględnij Wielkość Liter
check_button_trim_case_insensitive = Nieuwzględniaj wielkości liter
label_trim_trim_text = Przytnij tekst
label_trim_case_sensitivity = Wrażliwość na litery
# Normalize name
label_normalize_name =
    Wszystko - zmienia nazwę na nazwę, która zawiera tylko
                              znaki `a-z`, `0-9`, `-`, ` `, `.`.
                              e.g. `żółć CZERona.Txt` -> `zolc-czerwona.txt`
    
    Częściowo - działa dokładnie tak samo jak powyższa opcja, ale pozwala
                      na użycie spacji ` ` i dużych liter `A-Z`
                      e.g. ` Źrebię Krokietowe.Rar ` -> `Zrebie Krokietowe.Rar`
check_button_normalize_everything = Wszystko
check_button_normalize_partial = Częściowo
# RuleType
rule_type_custom = Niestandardowy
rule_type_case_size = Rozmiar Liter
rule_type_purge = Wyczyść
rule_type_add_text = Dodaj tekst
rule_type_trim = Przytnij
rule_type_replace = Zamień
rule_type_add_number = Dodaj numer
rule_type_normalize = Normalizuj
# RulePlace
rule_place_none = Nd.
rule_place_extension = Tylko rozszerzenie
rule_place_name = Tylko Nazwa
rule_place_extension_name = Rozszerzenie i nazwa
rule_place_before_extension = Przed rozszerzeniem
rule_place_after_extension = Po rozszerzeniu
rule_place_before_name = Przed nazwą
rule_place_after_name = Po nazwie
rule_place_from_name_start = Od początku
rule_place_from_name_end_reverse = Od końca do początku
rule_place_from_extension_start = Od początku rozszerzenia
rule_place_from_extension_end_reverse = Od końca rozszerzenia do początku
# Rule Description
rule_description_full_normalize = Pełna normalizacja
rule_description_partial_normalize = Częściowa normalizacja
rule_description_zeros = i wypełnianie { $zeros } zerami,
rule_description_step = Zaczynając od { step } od kroku { start }{ zeros }
rule_description_lowercase = Małe litery
rule_description_uppercase = Wielkie litery
rule_description_text = tekst
rule_description_added_text = Dodany tekst:
rule_description_start = start
rule_description_end_of_name = koniec nazwy
rule_description_extension = rozszerzenie
rule_description_end_of_extension = koniec rozszerzenia
rule_description_trimming = Przycinanie "{ $trim_text }" od { $where_remove }
rule_description_custom_rule = Niestandardowa reguła: { $custom_rule }
rule_description_replace = Zastępuje { $additional_regex_text } "{ $text_to_find }" z "{ $text_to_replace }"
