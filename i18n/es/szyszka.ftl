# Upper buttons
upper_start_renaming_button = Iniciar renombrado
upper_add_files_button = Añadir Archivos
upper_add_folders_button = Añadir Carpetas
upper_remove_selection_button = Eliminar selección
upper_update_names_button = Actualizar nombres
upper_results_one_up_button = Uno arriba
upper_results_one_down_button = Un Abajo
upper_select_popup_button = Seleccionar
# Bottom Buttons
bottom_rule_add_button = Añadir Regla
bottom_rule_edit_button = Editar regla
bottom_rule_remove_button = Eliminar regla
bottom_rule_one_up_button = Uno arriba
bottom_rule_one_down_button = Un Abajo
bottom_rule_save_rules_button = Guardar Reglas
bottom_rule_load_rules_button = Cargar Reglas
# Edit names
edit_names_used_in_rules = Nombres usados en las reglas: { $rules }
edit_names_choose_name = Elija el nombre de las reglas (si existe, lo anulará)
# Tree View Rules
tree_view_upper_column_type = Tipo
tree_view_upper_column_current_name = Nombre actual
tree_view_upper_column_future_name = Nombre futuro
tree_view_upper_column_path = Ruta
# Tree View Results
tree_view_bottom_tool_type = Tipo de herramienta
tree_view_bottom_usage_name = Nombre de uso
tree_view_bottom_description = Descripción
# Settings
settings_language_label = Idioma
settings_open_rules = Abrir archivo de configuración de reglas
settings_open_cache_custom_texts = Abrir archivo de caché personalizado
settings_open_config_dir = Abrir directorio de caché
# Other in main window
bottom_rule_label_rules = Reglas
upper_files_folders_label = Archivos/Carpetas
upper_files_folders_label_update = Archivos({ $files_number }) - ##### REQUIERDO ACTUALIZADO #####
upper_files_folders_label_up_to_date = Archivos({ $files_number }) - actualizado
# Select popover
button_select_all = Seleccionar todo
button_select_reverse = Invertir selección
button_select_custom = Seleccionar Personalizado
button_unselect_custom = Deseleccionar Personalizado
button_select_changed = Seleccionar cambiado
button_unselect_changed = Deseleccionar cambiado
# Un/Select custom
select_custom_example = Uso: */folder-nr*/* o name-version-*.txt
select_custom_path = Ruta
select_custom_current_path = Ruta actual
select_custom_future_path = Ruta futura
select_custom_path_current_name = Ruta + Nombre Actual
select_custom_path_future_name = Ruta + nombre futuro
select_custom_directory_file = Directorio/Archivo
select_custom_select_directory = Seleccionar directorio
select_custom_unselect_directory = Deseleccionar directorio
# General
dialog_button_ok = Ok
dialog_button_cancel = Cancelar
# Dialogs
dialog_name_files_to_include = Archivos a incluir
dialog_name_folders_to_include = Carpetas a incluir
dialog_scan_inside = Escanear dentro
dialog_ignore_folders = Ignorar carpetas
dialog_confirm_renaming = Confirmar renombrado
dialog_outdated_results = Resultados obsoletos
dialog_results_of_renaming = Resultados del renombrado
dialog_save_rule = Guardar regla
dialog_select_custom = Seleccionar Personalizado
dialog_unselect_custom = Deseleccionar Personalizado

# Rule Window


## Common

label_usage_type = Tipo de uso:
label_example = EXAMPLE
label_example_text_before = Before:
label_example_text_after = Después:
button_rule_window_add = Regla Añadir

## Custom

label_custom_instruction =
    $(NAME) - print file name
    $(EXT) - print extension
    $(MODIF) - print file modification date
    $(CREAT) - print file creation
    $(CURR) - print file name with extension
    $(PARENT) - prints parent folder name
    $(N)/$(K) - prints numbers(arguments are optional)
    $(N:3:4:5) print numbers from 3, con el paso 4
            y los llena con cero a 5 posiciones.
    	K en su lugar sólo la posición en la lista, también utiliza el elemento de posición en la carpeta.
menu_button_load_custom_rule = Selector de reglas personalizado
button_save_custom_rule = Guardar regla personalizada

## Upper/Lower Case

check_button_letters_type_uppercase = Mayúsculas
check_button_letters_type_lowercase = Minúsculas
check_button_letters_usage_name = Sólo Nombre
check_button_letters_usage_extension = Sólo extensión
check_button_letters_usage_both = Ambos
label_letters_tool_type = Tipo de herramienta:
# Purge
label_purge_tool_type = Tipo de herramienta:
check_button_purge_name = Sólo Nombre
check_button_purge_extension = Sólo extensión
check_button_purge_both = Ambos
# Add number
label_add_number_place = Colocar para poner número:
label_add_number_settings = Ajustes del número:
check_button_add_number_before_name = Antes de Nombre
check_button_add_number_after_name = Después del nombre
label_number_start_number = Número inicial
label_number_step = Paso
label_number_fill_zeros = Rellenar con ceros
# Add text
check_button_add_text_before_name = Antes de Nombre
check_button_add_text_after_name = Después del nombre
label_add_text = Texto a añadir:
# Replace
check_button_replace_name = Sólo Nombre
check_button_replace_extension = Sólo extensión
check_button_replace_both = Ambos
check_button_replace_case_sensitive = Sensitivo mayúsculas
check_button_replace_case_insensitive = Insensible a mayúsculas
check_button_replace_regex = Usar regex
check_button_replace_replace_all = Reemplazar todas las ocurrencias
label_replace_replacing_strings = Reemplazando cadenas:
label_replace_text_to_find = Texto a encontrar
label_replace_text_to_replace = Texto reemplazado
label_replace_captures = Capturas
label_replace_captured_captures = Capturar capturas
label_replace_captures_number = ({ $capture_number } captures)
label_replace_no_captures = No hay capturas
label_replace_invalid_regex = REGEX INVÁLIDO
# Trim
check_button_trim_name_start = Nombre Inicio
check_button_trim_name_end = Fin del nombre
check_button_trim_extension_start = Inicio de extensión
check_button_trim_extension_end = Fin de extensión
check_button_trim_case_sensitive = Sensitivo mayúsculas
check_button_trim_case_insensitive = Insensible a mayúsculas
label_trim_trim_text = Recortar texto
label_trim_case_sensitivity = Sensibilidad de Caso
# Normalize name
label_normalize_name =
    Everything - renames the name to one that contains only the
                              characters `a-z`, `0-9`, `-`, ` `, `.`.
                              e.g. `żółć CZERona.Txt` -> `zolc-czerwona.txt`
    
    Partial - works exactly same as option above, but allows
                      to use spaces ` ` and big letters `A-Z`
                      e.g. ` Źrebię Krokietowe.Rar ` -> `Zrebie Krokietowe.Rar`
check_button_normalize_everything = Todo
check_button_normalize_partial = Parcial
# RuleType
rule_type_custom = Personalizado
rule_type_case_size = Tamaño del caso
rule_type_purge = Purge
rule_type_add_text = Añadir texto
rule_type_trim = Recortar
rule_type_replace = Reemplazar
rule_type_add_number = Añadir número
rule_type_normalize = Normalizar
# RulePlace
rule_place_none = N/A
rule_place_extension = Sólo extensión
rule_place_name = Sólo Nombre
rule_place_extension_name = Extensión y nombre
rule_place_before_extension = Antes de la extensión
rule_place_after_extension = Después de la extensión
rule_place_before_name = Antes de Nombre
rule_place_after_name = Después del nombre
rule_place_from_name_start = Desde el inicio
rule_place_from_name_end_reverse = De nombre final al inicio
rule_place_from_extension_start = Desde inicio de extensión
rule_place_from_extension_end_reverse = De la extensión final al inicio
# Rule Description
rule_description_full_normalize = normalización completa
rule_description_partial_normalize = Normalización parcial
rule_description_zeros = y rellenando con { $zeros } ceros,
rule_description_step = Comenzando con { $start } con el paso { $step }{ $zeros }
rule_description_lowercase = Minúsculas
rule_description_uppercase = Mayúsculas
rule_description_text = texto
rule_description_added_text = Texto añadido:
rule_description_start = empezar
rule_description_end_of_name = fin del nombre
rule_description_extension = extensión
rule_description_end_of_extension = fin de la extensión
rule_description_trimming = Recortando "{ $trim_text }" de { $where_remove }
rule_description_custom_rule = Regla personalizada: { $custom_rule }
rule_description_replace = Reemplazando { $additional_regex_text } "{ $text_to_find }" por "{ $text_to_replace }"
# Notebooks
notebook_tab_custom = Personalizado
notebook_tab_case_size = Casos superior/inferior
notebook_tab_purge = Purge
notebook_tab_add_number = Añadir número
notebook_tab_add_text = Añadir texto
notebook_tab_replace = Reemplazar
notebook_tab_trim = Recortar
notebook_tab_normalize = Normalizar nombre
# Renaming dialog
renaming_question = ¿Estás seguro de que quieres renombrar { $number_of_renamed_files } archivos?
renaming_destination_file_exists = El archivo de destino ya existe.
renaming_renamed_files = Archivos renombrados correctamente { $properly_renamed }
renaming_ignored_files = Ignorados { $ignored } archivos, porque el nombre antes y después del cambio son los mismos.
renaming_failed_files = Error al renombrar { $failed_vector } archivos
renaming_list_of_failed_to_rename = Lista de todos los renombrados fallidos
renaming_error = error
renaming_some_records_not_updated = Algunos registros no se actualizan, puede hacerlo haciendo clic en el botón Actualizar nombres.\n¿Está seguro que desea continuar sin actualizar nombres?
renaming_missing_files = Falta archivos
renaming_require_missing_files = Necesitas usar al menos 1 archivo
renaming_missing_rules = Reglas faltantes
renaming_require_missing_rules = Necesitas usar al menos 1 regla
