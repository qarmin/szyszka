# Upper buttons
upper_start_renaming_button = Börja döpa om
upper_add_files_button = Lägg till filer
upper_add_folders_button = Lägg till mappar
upper_remove_selection_button = Ta bort markering
upper_update_names_button = Uppdatera namn
upper_results_one_up_button = En Upp
upper_results_one_down_button = En Ner
upper_select_popup_button = Välj
# Bottom Buttons
bottom_rule_add_button = Lägg till regel
bottom_rule_edit_button = Redigera regel
bottom_rule_remove_button = Ta bort regel
bottom_rule_one_up_button = En Upp
bottom_rule_one_down_button = En Ner
bottom_rule_save_rules_button = Spara regler
bottom_rule_load_rules_button = Ladda regler
# Edit names
edit_names_used_in_rules = Namn som används i regler: { $rules }
edit_names_choose_name = Välj namn på regler (om det finns, kommer att åsidosätta det)
# Tree View Rules
tree_view_upper_column_type = Typ
tree_view_upper_column_current_name = Nuvarande namn
tree_view_upper_column_future_name = Framtida namn
tree_view_upper_column_path = Sökväg
# Tree View Results
tree_view_bottom_tool_type = Typ av verktyg
tree_view_bottom_usage_name = Användarnamn
tree_view_bottom_description = Beskrivning
# Settings
settings_language_label = Språk
settings_open_rules = Öppna regelinställningar fil
settings_open_cache_custom_texts = Öppna anpassad cachefil
settings_open_config_dir = Öppna cache dir
check_button_dark_theme = Mörka ikoner
# Other in main window
bottom_rule_label_rules = Regler
upper_files_folders_label = Filer/Mappar
upper_files_folders_label_update = Filer/Mappar({ $files_number }) - ##### UPPDATERA KRÄVS #####
upper_files_folders_label_up_to_date = Filer/Mappar({ $files_number }) - aktuella
# Select popover
button_select_all = Markera alla
button_select_reverse = Omvänd markering
button_select_custom = Välj anpassad
button_unselect_custom = Avmarkera anpassad
button_select_changed = Välj Ändrad
button_unselect_changed = Avmarkera ändrad
# Un/Select custom
select_custom_example = Användning: */folder-nr*/* eller namn-version-*.txt
select_custom_path = Sökväg
select_custom_current_path = Nuvarande sökväg
select_custom_future_path = Framtida sökväg
select_custom_path_current_name = Sökväg + aktuellt namn
select_custom_path_future_name = Sökväg + framtida namn
select_custom_directory_file = Katalog/Fil
select_custom_select_directory = Välj katalog
select_custom_unselect_directory = Avmarkera katalog
# General
dialog_button_ok = OK
dialog_button_cancel = Avbryt
# Dialogs
dialog_name_files_to_include = Filer att inkludera
dialog_name_folders_to_include = Mappar att inkludera
dialog_scan_inside = Skanna inuti
dialog_ignore_folders = Ignorera mappar
dialog_confirm_renaming = Bekräfta namnbyte
dialog_outdated_results = Föråldrade resultat
dialog_results_of_renaming = Resultat av att byta namn
dialog_save_rule = Spara regel
dialog_select_custom = Välj anpassad
dialog_unselect_custom = Avmarkera anpassad

# Rule Window


## Common

label_usage_type = Typ av användning:
label_example = EXEMPEL
label_example_text_before = Före:
label_example_text_after = Efter:
button_rule_window_add = Regel Lägg till

## Custom

label_custom_instruction =
    $(NAME) - utskrifter filnamn
    $(EXT) - utskrifter tillägg
    $(MODIF) - utskrifter fil ändringsdatum
    $(CREAT) - utskrifter fil skapande
    $(CURR) - skriver ut nuvarande filnamn med tillägg
    $(PARENT) - utskrifter förälder mappnamn
    $(N)/$(K) - utskrifter siffror (argument är frivilliga)
    $(N:3:4:5) utskrifter nummer från 3, med steg 4
            och fyller dem med nollor till 5 positioner.
    	<unk> K istället bara position i listan, använder även positionsobjekt i mappen.
menu_button_load_custom_rule = Anpassad regel väljare
button_save_custom_rule = Spara anpassad regel

## Upper/Lower Case

check_button_letters_type_uppercase = Versaler
check_button_letters_type_lowercase = Gemener
check_button_letters_usage_name = Endast namn
check_button_letters_usage_extension = Endast tillägg
check_button_letters_usage_both = Båda
label_letters_tool_type = Typ av verktyg:
# Purge
label_purge_tool_type = Typ av verktyg:
check_button_purge_name = Endast namn
check_button_purge_extension = Endast tillägg
check_button_purge_both = Båda
# Add number
label_add_number_place = Plats att sätta nummer:
label_add_number_settings = Nummerinställningar:
check_button_add_number_before_name = Före namn
check_button_add_number_after_name = Efter namn
label_number_start_number = Starta nummer
label_number_step = Steg
label_number_fill_zeros = Fyll med nollor
# Add text
check_button_add_text_before_name = Före namn
check_button_add_text_after_name = Efter namn
label_add_text = Text att tillägga:
# Replace
check_button_replace_name = Endast namn
check_button_replace_extension = Endast tillägg
check_button_replace_both = Båda
check_button_replace_case_sensitive = Ärendekänslig
check_button_replace_case_insensitive = Ärendet okänslig
check_button_replace_regex = Använd regex
check_button_replace_replace_all = Ersätt alla förekomster
label_replace_replacing_strings = Ersätter strängar:
label_replace_text_to_find = Text att hitta
label_replace_text_to_replace = Ersatt text
label_replace_captures = Fånga
label_replace_captured_captures = Fångade fångar
label_replace_captures_number = ({ $capture_number } captures)
label_replace_no_captures = Inga bilder
label_replace_invalid_regex = INVALID REGEX
# Trim
check_button_trim_name_start = Start för namn
check_button_trim_name_end = Namn Slut
check_button_trim_extension_start = Starta tillägg
check_button_trim_extension_end = Tillägg Slut
check_button_trim_case_sensitive = Ärendekänslig
check_button_trim_case_insensitive = Ärendet okänslig
label_trim_trim_text = Trimma text
label_trim_case_sensitivity = Ärendets känslighet
# Normalize name
label_normalize_name =
    Everything - renames the name to one that contains only the
                              characters `a-z`, `0-9`, `-`, ` `, `.`.
                              e.g. `żółć CZERona.Txt` -> `zolc-czerwona.txt`
    
    Partial - works exactly same as option above, but allows
                      to use spaces ` ` and big letters `A-Z`
                      e.g. ` Źrebię Krokietowe.Rar ` -> `Zrebie Krokietowe.Rar`
check_button_normalize_everything = Allt
check_button_normalize_partial = Delvis
# RuleType
rule_type_custom = Anpassad
rule_type_case_size = Storlek på ärende
rule_type_purge = Purge
rule_type_add_text = Lägg till text
rule_type_trim = Beskär
rule_type_replace = Ersätt
rule_type_add_number = Lägg till nummer
rule_type_normalize = Normalisera
# RulePlace
rule_place_none = N/A
rule_place_extension = Endast tillägg
rule_place_name = Endast namn
rule_place_extension_name = Tillägg och namn
rule_place_before_extension = Före tillägg
rule_place_after_extension = Efter tillägg
rule_place_before_name = Före namn
rule_place_after_name = Efter namn
rule_place_from_name_start = Från början
rule_place_from_name_end_reverse = Från namnslut till start
rule_place_from_extension_start = Från tilläggsstart
rule_place_from_extension_end_reverse = Från förlängningsslut till start
# Rule Description
rule_description_full_normalize = Helt normalisera
rule_description_partial_normalize = Delvis normalisera
rule_description_zeros = och fyllning med { $zeros } nollor,
rule_description_step = Börjar med { $start } med steg { $step }{ $zeros }
rule_description_lowercase = Gemener
rule_description_uppercase = Versaler
rule_description_text = text
rule_description_added_text = Lade till text:
rule_description_start = start
rule_description_end_of_name = slutet av namnet
rule_description_extension = tillägg
rule_description_end_of_extension = slut på förlängning
rule_description_trimming = Beskär "{ $trim_text }" från { $where_remove }
rule_description_custom_rule = Anpassad regel: { $custom_rule }
rule_description_replace = Ersätter { $additional_regex_text } "{ $text_to_find }" med "{ $text_to_replace }"
# Notebooks
notebook_tab_custom = Anpassad
notebook_tab_case_size = Övre/Lägre Fall
notebook_tab_purge = Purge
notebook_tab_add_number = Lägg till nummer
notebook_tab_add_text = Lägg till text
notebook_tab_replace = Ersätt
notebook_tab_trim = Beskär
notebook_tab_normalize = Normalisera namn
# Renaming dialog
renaming_question = Är du säker på att du vill byta namn på { $number_of_renamed_files } filer?
renaming_destination_file_exists = Målfilen finns redan.
renaming_renamed_files = Korrekt omdöpt till { $properly_renamed } filer
renaming_ignored_files = Ignorerade { $ignored } filer, eftersom namnet före och efter ändringen är samma.
renaming_failed_files = Det gick inte att byta namn på { $failed_vector } filer
renaming_list_of_failed_to_rename = Lista över alla misslyckade namnbyte
renaming_error = fel
renaming_some_records_not_updated = Vissa poster är inte uppdaterade, du kan göra det genom att klicka på knappen Uppdateringsnamn.\nÄr du säker på att du vill fortsätta utan att uppdatera namn?
renaming_missing_files = Saknade filer
renaming_require_missing_files = Du måste använda minst 1 fil
renaming_missing_rules = Saknade regler
renaming_require_missing_rules = Du måste använda minst 1 regel
