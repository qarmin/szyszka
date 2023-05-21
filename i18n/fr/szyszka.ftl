# Upper buttons
upper_start_renaming_button = Commencer à renommer
upper_add_files_button = Ajouter des fichiers
upper_add_folders_button = Ajouter des dossiers
upper_remove_selection_button = Supprimer la sélection
upper_update_names_button = Mettre à jour les noms
upper_results_one_up_button = Une fois de plus
upper_results_one_down_button = Un vers le bas
upper_select_popup_button = Sélectionner
# Bottom Buttons
bottom_rule_add_button = Ajouter une règle
bottom_rule_edit_button = Modifier la règle
bottom_rule_remove_button = Supprimer la règle
bottom_rule_one_up_button = Une fois de plus
bottom_rule_one_down_button = Un vers le bas
bottom_rule_save_rules_button = Enregistrer les règles
bottom_rule_load_rules_button = Règles de chargement
# Edit names
edit_names_used_in_rules = Noms utilisés dans les règles : { $rules }
edit_names_choose_name = Choisissez le nom des règles (si elles existent, les remplaceront)
# Tree View Rules
tree_view_upper_column_type = Type de texte
tree_view_upper_column_current_name = Nom actuel
tree_view_upper_column_future_name = Nom du futur
tree_view_upper_column_path = Chemin d'accès
# Tree View Results
tree_view_bottom_tool_type = Type d'outil
tree_view_bottom_usage_name = Nom de l'utilisation
tree_view_bottom_description = Libellé
# Settings
settings_language_label = Langue
settings_open_rules = Ouvrir le fichier de paramètres des règles
settings_open_cache_custom_texts = Ouvrir un fichier de cache personnalisé
settings_open_config_dir = Ouvrir le répertoire de cache
# Other in main window
bottom_rule_label_rules = Règles
upper_files_folders_label = Fichiers/Dossiers
upper_files_folders_label_update = Fichiers/Dossiers ({ $files_number }) - ##### MISE À JOUR REQUISE #####
upper_files_folders_label_up_to_date = Fichiers/Dossiers ({ $files_number }) - à jour
# Select popover
button_select_all = Tout sélectionner
button_select_reverse = Inverser la sélection
button_select_custom = Sélectionnez Personnalisé
button_unselect_custom = Désélectionner Personnalisé
button_select_changed = Sélectionner les modifications
button_unselect_changed = Désélectionner Changé
# Un/Select custom
select_custom_example = Utilisation: */folder-nr*/* ou name-version-*.txt
select_custom_path = Chemin d'accès
select_custom_current_path = Chemin actuel
select_custom_future_path = Chemin du futur
select_custom_path_current_name = Chemin d'accès + Nom actuel
select_custom_path_future_name = Chemin d'accès + Nom futur
select_custom_directory_file = Répertoire/Fichier
select_custom_select_directory = Sélectionner un répertoire
select_custom_unselect_directory = Désélectionner le répertoire
# General
dialog_button_ok = Ok
dialog_button_cancel = Abandonner
# Dialogs
dialog_name_files_to_include = Fichiers à inclure
dialog_name_folders_to_include = Dossiers à inclure
dialog_scan_inside = Scanner à l'intérieur
dialog_ignore_folders = Ignorer les dossiers
dialog_confirm_renaming = Confirmer le renommage
dialog_outdated_results = Résultats périmés
dialog_results_of_renaming = Résultats du renommage
dialog_save_rule = Enregistrer la règle
dialog_select_custom = Sélectionnez Personnalisé
dialog_unselect_custom = Désélectionner Personnalisé

# Rule Window


## Common

label_usage_type = Type d'utilisation :
label_example = EXEMPLE
label_example_text_before = Avant:
label_example_text_after = Après:
button_rule_window_add = Ajouter une règle

## Custom

label_custom_instruction =
    $(NAME) - affiche le nom du fichier
    $(EXT) - affiche l'extension
    $(MODIF) - affiche la date de modification du fichier
    $(CREAT) - affiche la création du fichier
    $(CURR) - affiche le nom du fichier actuel avec l'extension
    $(PARENT) - affiche le nom du dossier parent
    $(N)/$(K) - affiche les nombres (les arguments sont facultatifs)
    $(N:3:4:5) affiche les numéros à partir de 3, avec l'étape 4
            et les remplit avec des zéros à 5 positions.
    	K à la place seulement position dans la liste, utilise également l'élément de position dans le dossier.
menu_button_load_custom_rule = Sélecteur de règles personnalisé
button_save_custom_rule = Enregistrer la règle personnalisée

## Upper/Lower Case

check_button_letters_type_uppercase = Majuscule
check_button_letters_type_lowercase = minuscule
check_button_letters_usage_name = Seulement le nom
check_button_letters_usage_extension = Extension uniquement
check_button_letters_usage_both = Les deux
label_letters_tool_type = Type d'outil:
# Purge
label_purge_tool_type = Type d'outil:
check_button_purge_name = Seulement le nom
check_button_purge_extension = Extension uniquement
check_button_purge_both = Les deux
# Add number
label_add_number_place = Lieu pour mettre le numéro:
label_add_number_settings = Paramètres de numéro:
check_button_add_number_before_name = Avant le nom
check_button_add_number_after_name = Après le nom
label_number_start_number = Numéro de départ
label_number_step = Étape
label_number_fill_zeros = Remplir avec des zéros
# Add text
check_button_add_text_before_name = Avant le nom
check_button_add_text_after_name = Après le nom
label_add_text = Texte à ajouter :
# Replace
check_button_replace_name = Seulement le nom
check_button_replace_extension = Extension uniquement
check_button_replace_both = Les deux
check_button_replace_case_sensitive = Sensible à la casse
check_button_replace_case_insensitive = Cas Insensible
check_button_replace_regex = Utiliser regex
check_button_replace_replace_all = Remplacer toutes les occurrences
label_replace_replacing_strings = Remplacement des chaînes :
label_replace_text_to_find = Texte à trouver
label_replace_text_to_replace = Texte remplacé
label_replace_captures = Captures
label_replace_captured_captures = Captures capturées
label_replace_captures_number = ({ $capture_number } captures)
label_replace_no_captures = Aucune capture
label_replace_invalid_regex = REGEX NON VALIDE
# Trim
check_button_trim_name_start = Nom de début
check_button_trim_name_end = Nom de fin
check_button_trim_extension_start = Début de l'extension
check_button_trim_extension_end = Fin de l'extension
check_button_trim_case_sensitive = Sensible à la casse
check_button_trim_case_insensitive = Cas Insensible
label_trim_trim_text = Couper le texte
label_trim_case_sensitivity = Sensibilité à la casse
# Normalize name
label_normalize_name =
    Everything - renames the name to one that contains only the
                              characters `a-z`, `0-9`, `-`, ` `, `.`.
                              e.g. `żółć CZERona.Txt` -> `zolc-czerwona.txt`
    
    Partial - works exactly same as option above, but allows
                      to use spaces ` ` and big letters `A-Z`
                      e.g. ` Źrebię Krokietowe.Rar ` -> `Zrebie Krokietowe.Rar`
check_button_normalize_everything = Tout
check_button_normalize_partial = Partiellement
# RuleType
rule_type_custom = Personnalisé
rule_type_case_size = Taille du boîtier
rule_type_purge = Purge
rule_type_add_text = Ajouter un texte
rule_type_trim = Ajuster
rule_type_replace = Remplacer
rule_type_add_number = Ajouter un numéro
rule_type_normalize = Normaliser
# RulePlace
rule_place_none = N/A
rule_place_extension = Extension uniquement
rule_place_name = Seulement le nom
rule_place_extension_name = Extension et nom
rule_place_before_extension = Avant l'extension
rule_place_after_extension = Après l'extension
rule_place_before_name = Avant le nom
rule_place_after_name = Après le nom
rule_place_from_name_start = Depuis le début
rule_place_from_name_end_reverse = Du nom de fin au début
rule_place_from_extension_start = Depuis le début de l'extension
rule_place_from_extension_end_reverse = Depuis la fin de l'extension jusqu'au début
# Rule Description
rule_description_full_normalize = Normalisation complète
rule_description_partial_normalize = Normalisation partielle
rule_description_zeros = et remplir avec { $zeros } zéros,
rule_description_step = Commence par { $start } avec l'étape { $step }{ $zeros }
rule_description_lowercase = minuscule
rule_description_uppercase = Majuscule
rule_description_text = texte
rule_description_added_text = Texte ajouté :
rule_description_start = Début
rule_description_end_of_name = fin de nom
rule_description_extension = extension
rule_description_end_of_extension = fin d'extension
rule_description_trimming = Ajustement de "{ $trim_text }" de { $where_remove }
rule_description_custom_rule = Règle personnalisée : { $custom_rule }
rule_description_replace = Remplacement de { $additional_regex_text } "{ $text_to_find }" par "{ $text_to_replace }"
# Notebooks
notebook_tab_custom = Personnalisé
notebook_tab_case_size = Cas majuscules/minuscules
notebook_tab_purge = Purge
notebook_tab_add_number = Ajouter un numéro
notebook_tab_add_text = Ajouter un texte
notebook_tab_replace = Remplacer
notebook_tab_trim = Ajuster
notebook_tab_normalize = Normaliser le nom
# Renaming dialog
renaming_question = Êtes-vous sûr de vouloir renommer { $number_of_renamed_files } fichiers ?
renaming_destination_file_exists = Le fichier de destination existe déjà.
renaming_renamed_files = Fichiers correctement renommés { $properly_renamed }
renaming_ignored_files = Ignoré les fichiers { $ignored } , car le nom avant et après le changement est le même.
renaming_failed_files = Impossible de renommer { $failed_vector } fichiers
renaming_list_of_failed_to_rename = Liste de tous les renommages échoués
renaming_error = Erreur
renaming_some_records_not_updated = Certains enregistrements ne sont pas mis à jour, vous pouvez le faire en cliquant sur le bouton Mettre à jour les noms.\nÊtes-vous sûr de vouloir continuer sans mettre à jour les noms ?
renaming_missing_files = Fichiers manquants
renaming_require_missing_files = Vous devez utiliser au moins 1 fichier
renaming_missing_rules = Règles manquantes
renaming_require_missing_rules = Vous devez utiliser au moins 1 règle
