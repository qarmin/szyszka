# Upper buttons
upper_start_renaming_button = Začít přejmenovat
upper_add_files_button = Přidat soubory
upper_add_folders_button = Přidat složky
upper_remove_selection_button = Odstranit výběr
upper_update_names_button = Aktualizovat názvy
upper_results_one_up_button = Jeden nahoru
upper_results_one_down_button = Jeden Dolů
upper_select_popup_button = Vybrat
# Bottom Buttons
bottom_rule_add_button = Přidat pravidlo
bottom_rule_edit_button = Upravit pravidlo
bottom_rule_remove_button = Odstranit pravidlo
bottom_rule_one_up_button = Jeden nahoru
bottom_rule_one_down_button = Jeden Dolů
bottom_rule_save_rules_button = Uložit pravidla
bottom_rule_load_rules_button = Načíst pravidla
# Edit names
edit_names_used_in_rules = Jména použitá v pravidlech: { $rules }
edit_names_choose_name = Zvolte název pravidel (pokud existuje, přepíše je)
# Tree View Rules
tree_view_upper_column_type = Typ
tree_view_upper_column_current_name = Aktuální název
tree_view_upper_column_future_name = Budoucí název
tree_view_upper_column_path = Cesta
# Tree View Results
tree_view_bottom_tool_type = Typ nástroje
tree_view_bottom_usage_name = Název použití
tree_view_bottom_description = L 343, 22.12.2009, s. 1).
# Settings
settings_language_label = Jazyk
settings_open_rules = Otevřít soubor s nastavením pravidel
settings_open_cache_custom_texts = Otevřít vlastní soubor mezipaměti
settings_open_config_dir = Otevřít dir keše
check_button_dark_theme = Tmavé ikony
# Other in main window
bottom_rule_label_rules = Pravidla
upper_files_folders_label = Soubory/Složky
upper_files_folders_label_update = Soubory/Složky({ $files_number }) - ##### AKTUALIZOVAT POŽADOVANÉ #
upper_files_folders_label_up_to_date = Soubory/Složky({ $files_number }) - aktuální
# Select popover
button_select_all = Vybrat vše
button_select_reverse = Reverzní výběr
button_select_custom = Vybrat vlastní
button_unselect_custom = Zrušit výběr vlastních
button_select_changed = Vybrat změněné
button_unselect_changed = Zrušit výběr změněn
# Un/Select custom
select_custom_example = Použití: */folder-nr*/* nebo name-version-*.txt
select_custom_path = Cesta
select_custom_current_path = Aktuální cesta
select_custom_future_path = Budoucí cesta
select_custom_path_current_name = Cesta + aktuální název
select_custom_path_future_name = Cesta + budoucí jméno
select_custom_directory_file = Adresář/soubor
select_custom_select_directory = Vybrat adresář
select_custom_unselect_directory = Zrušit výběr adresáře
# General
dialog_button_ok = Ok
dialog_button_cancel = Zrušit
# Dialogs
dialog_name_files_to_include = Soubory, které chcete zahrnout
dialog_name_folders_to_include = Složky, které chcete zahrnout
dialog_scan_inside = Skenovat uvnitř
dialog_ignore_folders = Ignorovat složky
dialog_confirm_renaming = Potvrdit přejmenování
dialog_outdated_results = Zastaralé výsledky
dialog_results_of_renaming = Výsledky přejmenování
dialog_save_rule = Uložit pravidlo
dialog_select_custom = Vybrat vlastní
dialog_unselect_custom = Zrušit výběr vlastních

# Rule Window


## Common

label_usage_type = Typ použití:
label_example = PŘÍKLAD
label_example_text_before = Rychlo:
label_example_text_after = Poté
button_rule_window_add = Přidat pravidlo

## Custom

label_custom_instruction =
    $(NAME) - vytiskne název souboru
    $(EXT) - vytiskne příponu
    $(MODIF) - vytiskne datum úpravy souboru
    $(CREAT) - vytiskne vytvoření souboru
    $(CURR) - vytiskne aktuální název souboru s příponou
    $(PARENT) - vytiskne název nadřazené složky
    $(N)/$(K) - vytiskne čísla (argumenty jsou volitelné)
    $(N:3:4:5) vytiskne čísla od 3, s kroku 4
            a vyplní je nulami na 5 pozic.
    	K místo pouze pozice v seznamu, také používá položku pozice ve složce.
menu_button_load_custom_rule = Vlastní výběr pravidel
button_save_custom_rule = Uložit vlastní pravidlo

## Upper/Lower Case

check_button_letters_type_uppercase = Velká písmena
check_button_letters_type_lowercase = Malá písmena
check_button_letters_usage_name = Pouze jméno
check_button_letters_usage_extension = Pouze rozšíření
check_button_letters_usage_both = Obojí
label_letters_tool_type = Typ nástroje:
# Purge
label_purge_tool_type = Typ nástroje:
check_button_purge_name = Pouze jméno
check_button_purge_extension = Pouze rozšíření
check_button_purge_both = Obojí
# Add number
label_add_number_place = Místo určení číslo:
label_add_number_settings = Nastavení čísla:
check_button_add_number_before_name = Před názvem
check_button_add_number_after_name = Za jménem
label_number_start_number = Počáteční číslo
label_number_step = Krok
label_number_fill_zeros = Vyplnit nulami
# Add text
check_button_add_text_before_name = Před názvem
check_button_add_text_after_name = Za jménem
label_add_text = Text k přidání:
# Replace
check_button_replace_name = Pouze jméno
check_button_replace_extension = Pouze rozšíření
check_button_replace_both = Obojí
check_button_replace_case_sensitive = Citlivá písmena
check_button_replace_case_insensitive = Rozlišovat velikost písmen
check_button_replace_regex = Použít regex
check_button_replace_replace_all = Nahradit všechny výskyty
label_replace_replacing_strings = Nahrazující řetězce:
label_replace_text_to_find = Text k nalezení
label_replace_text_to_replace = Nahrazený text
label_replace_captures = Zachytávání
label_replace_captured_captures = Zajaté snímky
label_replace_captures_number = ({ $capture_number } captures)
label_replace_no_captures = Žádné snímky
label_replace_invalid_regex = NEPLATNÝ REGEX
# Trim
check_button_trim_name_start = Počátek jména
check_button_trim_name_end = Název končí
check_button_trim_extension_start = Začátek rozšíření
check_button_trim_extension_end = Konec rozšíření
check_button_trim_case_sensitive = Citlivá písmena
check_button_trim_case_insensitive = Rozlišovat velikost písmen
label_trim_trim_text = Oříznout text
label_trim_case_sensitivity = Citlivost případu
# Normalize name
label_normalize_name =
    Everything - renames the name to one that contains only the
                              characters `a-z`, `0-9`, `-`, ` `, `.`.
                              e.g. `żółć CZERona.Txt` -> `zolc-czerwona.txt`
    
    Partial - works exactly same as option above, but allows
                      to use spaces ` ` and big letters `A-Z`
                      e.g. ` Źrebię Krokietowe.Rar ` -> `Zrebie Krokietowe.Rar`
check_button_normalize_everything = Vše
check_button_normalize_partial = Částečné
# RuleType
rule_type_custom = Vlastní
rule_type_case_size = Velikost písmen
rule_type_purge = Purge
rule_type_add_text = Přidat text
rule_type_trim = Oříznout
rule_type_replace = Nahradit
rule_type_add_number = Přidat číslo
rule_type_normalize = Normalizovat
# RulePlace
rule_place_none = Nepřichází v úvahu
rule_place_extension = Pouze rozšíření
rule_place_name = Pouze jméno
rule_place_extension_name = Rozšíření a název
rule_place_before_extension = Před rozšířením
rule_place_after_extension = Po rozšíření
rule_place_before_name = Před názvem
rule_place_after_name = Za jménem
rule_place_from_name_start = Od začátku
rule_place_from_name_end_reverse = Od jména konce do začátku
rule_place_from_extension_start = Od začátku rozšíření
rule_place_from_extension_end_reverse = Od rozšíření do začátku
# Rule Description
rule_description_full_normalize = Plná normalizace
rule_description_partial_normalize = Částečná normalizace
rule_description_zeros = a doplňte { $zeros } nulami,
rule_description_step = Začíná s { $start } s krokem { $step }{ $zeros }
rule_description_lowercase = Malá písmena
rule_description_uppercase = Velká písmena
rule_description_text = text
rule_description_added_text = Přidaný text:
rule_description_start = Začít
rule_description_end_of_name = konec jména
rule_description_extension = Rozšíření
rule_description_end_of_extension = konec prodloužení
rule_description_trimming = Krytý "{ $trim_text }" od { $where_remove }
rule_description_custom_rule = Vlastní pravidlo: { $custom_rule }
rule_description_replace = Nahrazeno { $additional_regex_text } "{ $text_to_find }" " "{ $text_to_replace }"
# Notebooks
notebook_tab_custom = Vlastní
notebook_tab_case_size = Horní/dolní bedny
notebook_tab_purge = Purge
notebook_tab_add_number = Přidat číslo
notebook_tab_add_text = Přidat text
notebook_tab_replace = Nahradit
notebook_tab_trim = Oříznout
notebook_tab_normalize = Normalizovat název
# Renaming dialog
renaming_question = Jste si jisti, že chcete přejmenovat { $number_of_renamed_files } souborů?
renaming_destination_file_exists = Cílový soubor již existuje.
renaming_renamed_files = Správně přejmenované soubory { $properly_renamed }
renaming_ignored_files = Ignorováno { $ignored } souborů, protože jméno před a po změně je stejné.
renaming_failed_files = Nepodařilo se přejmenovat soubory { $failed_vector }
renaming_list_of_failed_to_rename = Seznam všech neúspěšných přejmenování
renaming_error = Chyba
renaming_some_records_not_updated = Některé záznamy nejsou aktualizovány, můžete tak učinit kliknutím na tlačítko Název aktualizace.\nJste si jisti, že chcete pokračovat bez aktualizace?
renaming_missing_files = Chybějící soubory
renaming_require_missing_files = Musíte použít alespoň 1 soubor
renaming_missing_rules = Chybějící pravidla
renaming_require_missing_rules = Musíte použít alespoň jedno pravidlo
