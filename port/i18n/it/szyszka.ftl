# Upper buttons
upper_start_renaming_button = Inizia A Rinominare
upper_add_files_button = Aggiungi File
upper_add_folders_button = Aggiungi Cartelle
upper_remove_selection_button = Rimuovi Selezione
upper_update_names_button = Aggiorna Nomi
upper_results_one_up_button = Uno Su
upper_results_one_down_button = Uno Giù
upper_select_popup_button = Seleziona
# Bottom Buttons
bottom_rule_add_button = Aggiungi Regola
bottom_rule_edit_button = Modifica Regola
bottom_rule_remove_button = Rimuovi Regola
bottom_rule_one_up_button = Uno Su
bottom_rule_one_down_button = Uno Giù
bottom_rule_save_rules_button = Salva Regole
bottom_rule_load_rules_button = Carica Regole
# Edit names
edit_names_used_in_rules = Nomi usati nelle regole: { $rules }
edit_names_choose_name = Scegli il nome delle regole (se esiste, la sovrascriverà)
# Tree View Rules
tree_view_upper_column_type = Tipo
tree_view_upper_column_current_name = Nome Corrente
tree_view_upper_column_future_name = Nome Futuro
tree_view_upper_column_path = Percorso
# Tree View Results
tree_view_bottom_tool_type = Tipo Di Strumento
tree_view_bottom_usage_name = Nome Utilizzo
tree_view_bottom_description = Descrizione
# Settings
settings_language_label = Lingua
settings_open_rules = Apri file impostazioni regole
settings_open_cache_custom_texts = Apri file cache personalizzato
settings_open_config_dir = Apri cache dir
check_button_dark_theme = Icone scure
# Other in main window
bottom_rule_label_rules = Regole
upper_files_folders_label = File/Cartelle
upper_files_folders_label_update = File/Cartelle({ $files_number }) - ##### UPDATE REQUISITI #####
upper_files_folders_label_up_to_date = File/Cartelle({ $files_number }) - aggiornato
# Select popover
button_select_all = Seleziona Tutto
button_select_reverse = Selezione Inversa
button_select_custom = Seleziona Personalizzato
button_unselect_custom = Deseleziona Personalizzato
button_select_changed = Seleziona Modificato
button_unselect_changed = Deseleziona Modificato
# Un/Select custom
select_custom_example = Uso: */folder-nr*/* o name-version-*.txt
select_custom_path = Percorso
select_custom_current_path = Percorso Corrente
select_custom_future_path = Percorso Futuro
select_custom_path_current_name = Percorso + Nome Corrente
select_custom_path_future_name = Percorso + Nome Futuro
select_custom_directory_file = Cartella/File
select_custom_select_directory = Seleziona Cartella
select_custom_unselect_directory = Deseleziona Cartella
# General
dialog_button_ok = Ok
dialog_button_cancel = Annulla
# Dialogs
dialog_name_files_to_include = File da includere
dialog_name_folders_to_include = Cartelle da includere
dialog_scan_inside = Scansiona all'interno
dialog_ignore_folders = Ignora cartelle
dialog_confirm_renaming = Conferma rinominazione
dialog_outdated_results = Risultati obsoleti
dialog_results_of_renaming = Risultati della rinominazione
dialog_save_rule = Salva Regola
dialog_select_custom = Seleziona Personalizzato
dialog_unselect_custom = Deseleziona Personalizzato

# Rule Window


## Common

label_usage_type = Tipo Di Utilizzo:
label_example = ESEMPIO
label_example_text_before = Prima:
label_example_text_after = Dopo:
button_rule_window_add = Aggiungi Regola

## Custom

label_custom_instruction =
    $(NAME) - stampa il nome del file
    $(EXT) - l'estensione di stampa
    $(MODIF) - stampa la data di modifica del file
    $(CREAT) - stampa la creazione del file
    $(CURR) - stampa il nome del file corrente con l'estensione
    $(PARENT) - stampa il nome della cartella padre
    $(N)/$(K) - stampa i numeri (gli argomenti sono opzionali)
    $(N:3:4:5) stampa i numeri da 3, con passo 4
            e li riempie con zeri a 5 posizioni.
    	<unk> K invece solo la posizione nella lista, utilizza anche l'elemento di posizione nella cartella.
menu_button_load_custom_rule = Selettore regola personalizzata
button_save_custom_rule = Salva regola personalizzata

## Upper/Lower Case

check_button_letters_type_uppercase = Maiuscolo
check_button_letters_type_lowercase = Minuscolo
check_button_letters_usage_name = Solo Nome
check_button_letters_usage_extension = Solo Estensione
check_button_letters_usage_both = Entrambi
label_letters_tool_type = Tipo Strumento:
# Purge
label_purge_tool_type = Tipo Strumento:
check_button_purge_name = Solo Nome
check_button_purge_extension = Solo Estensione
check_button_purge_both = Entrambi
# Add number
label_add_number_place = Luogo per inserire il numero:
label_add_number_settings = Impostazioni numero:
check_button_add_number_before_name = Prima Del Nome
check_button_add_number_after_name = Dopo Il Nome
label_number_start_number = Numero iniziale
label_number_step = Passo
label_number_fill_zeros = Riempi con zeri
# Add text
check_button_add_text_before_name = Prima Del Nome
check_button_add_text_after_name = Dopo Il Nome
label_add_text = Testo da aggiungere:
# Replace
check_button_replace_name = Solo Nome
check_button_replace_extension = Solo Estensione
check_button_replace_both = Entrambi
check_button_replace_case_sensitive = Distingui Maiuscole
check_button_replace_case_insensitive = Distingui Maiuscole
check_button_replace_regex = Usa regex
check_button_replace_replace_all = Sostituisci tutte le occorrenze
label_replace_replacing_strings = Sostituzione Stringe:
label_replace_text_to_find = Testo da trovare
label_replace_text_to_replace = Testo sostituito
label_replace_captures = Catture
label_replace_captured_captures = Catture catturate
label_replace_captures_number = ({ $capture_number } captures)
label_replace_no_captures = Nessuna cattura
label_replace_invalid_regex = REGEX INVALIDO
# Trim
check_button_trim_name_start = Inizio Nome
check_button_trim_name_end = Fine Nome
check_button_trim_extension_start = Inizio Estensione
check_button_trim_extension_end = Fine Estensione
check_button_trim_case_sensitive = Distingui Maiuscole
check_button_trim_case_insensitive = Distingui Maiuscole
label_trim_trim_text = Taglia testo
label_trim_case_sensitivity = Sensibilità Caso
# Normalize name
label_normalize_name =
    Everything - renames the name to one that contains only the
                              characters `a-z`, `0-9`, `-`, ` `, `.`.
                              e.g. `żółć CZERona.Txt` -> `zolc-czerwona.txt`
    
    Partial - works exactly same as option above, but allows
                      to use spaces ` ` and big letters `A-Z`
                      e.g. ` Źrebię Krokietowe.Rar ` -> `Zrebie Krokietowe.Rar`
check_button_normalize_everything = Tutto
check_button_normalize_partial = Parziale
# RuleType
rule_type_custom = Personalizzato
rule_type_case_size = Dimensione Caso
rule_type_purge = Purge
rule_type_add_text = Aggiungi Testo
rule_type_trim = Taglia
rule_type_replace = Sostituisci
rule_type_add_number = Aggiungi Numero
rule_type_normalize = Normalizza
# RulePlace
rule_place_none = N/A
rule_place_extension = Solo Estensione
rule_place_name = Solo Nome
rule_place_extension_name = Estensione e nome
rule_place_before_extension = Prima Dell'Estensione
rule_place_after_extension = Dopo L'Estensione
rule_place_before_name = Prima Del Nome
rule_place_after_name = Dopo Il Nome
rule_place_from_name_start = Da Inizio
rule_place_from_name_end_reverse = Dalla fine del nome all'inizio
rule_place_from_extension_start = Dall'Inizio Dell'Estensione
rule_place_from_extension_end_reverse = Dalla fine dell'estensione all'avvio
# Rule Description
rule_description_full_normalize = Normalizzazione completa
rule_description_partial_normalize = Normalizzazione parziale
rule_description_zeros = e riempiendo con { $zeros } zeri,
rule_description_step = A partire da { $start } con il passo { $step }{ $zeros }
rule_description_lowercase = Minuscolo
rule_description_uppercase = Maiuscolo
rule_description_text = testo
rule_description_added_text = Testo aggiunto:
rule_description_start = inizio
rule_description_end_of_name = fine del nome
rule_description_extension = estensione
rule_description_end_of_extension = fine dell'estensione
rule_description_trimming = Rifilatura "{ $trim_text }" da { $where_remove }
rule_description_custom_rule = Regola personalizzata: { $custom_rule }
rule_description_replace = Sostituzione { $additional_regex_text } "{ $text_to_find }" con "{ $text_to_replace }"
# Notebooks
notebook_tab_custom = Personalizzato
notebook_tab_case_size = Casi Superiori/Minori
notebook_tab_purge = Purge
notebook_tab_add_number = Aggiungi Numero
notebook_tab_add_text = Aggiungi Testo
notebook_tab_replace = Sostituisci
notebook_tab_trim = Taglia
notebook_tab_normalize = Nome Normalizza
# Renaming dialog
renaming_question = Sei sicuro di voler rinominare { $number_of_renamed_files } file?
renaming_destination_file_exists = Il file di destinazione esiste già.
renaming_renamed_files = Rinominato correttamente { $properly_renamed } file
renaming_ignored_files = Ignorato { $ignored } file, perché il nome prima e dopo la modifica è lo stesso.
renaming_failed_files = Impossibile rinominare { $failed_vector } file
renaming_list_of_failed_to_rename = Elenco di tutti i rinomini falliti
renaming_error = errore
renaming_some_records_not_updated = Alcuni record non sono aggiornati, puoi farlo facendo clic sul pulsante Aggiorna nomi.\nSei sicuro di voler procedere senza aggiornare i nomi?
renaming_missing_files = File Mancanti
renaming_require_missing_files = È necessario utilizzare almeno 1 file
renaming_missing_rules = Regole Mancanti
renaming_require_missing_rules = Devi usare almeno 1 regola
