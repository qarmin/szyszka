# Upper buttons
upper_start_renaming_button = Começar a Renomear
upper_add_files_button = Adicionar arquivos
upper_add_folders_button = Adicionar Pastas
upper_remove_selection_button = Remover Seleção
upper_update_names_button = Atualizar Nomes
upper_results_one_up_button = Um a mais
upper_results_one_down_button = Uma Abaixo
upper_select_popup_button = Selecionar
# Bottom Buttons
bottom_rule_add_button = Adicionar regra
bottom_rule_edit_button = Editar regra
bottom_rule_remove_button = Remover regra
bottom_rule_one_up_button = Um a mais
bottom_rule_one_down_button = Uma Abaixo
bottom_rule_save_rules_button = Salvar regras
bottom_rule_load_rules_button = Carregar regras
# Edit names
edit_names_used_in_rules = Nomes usados nas regras: { $rules }
edit_names_choose_name = Escolha o nome das regras (se existir, irá substituí-lo)
# Tree View Rules
tree_view_upper_column_type = tipo
tree_view_upper_column_current_name = Nome Atual
tree_view_upper_column_future_name = Nome Futuro
tree_view_upper_column_path = Caminho
# Tree View Results
tree_view_bottom_tool_type = Tipo de Ferramenta
tree_view_bottom_usage_name = Nome do uso
tree_view_bottom_description = Descrição:
# Settings
settings_language_label = IDIOMA
settings_open_rules = Abrir arquivo de configurações das regras
settings_open_cache_custom_texts = Abrir arquivo de cache personalizado
settings_open_config_dir = Abrir cache dir
# Other in main window
bottom_rule_label_rules = Regras
upper_files_folders_label = Arquivos/Pastas
upper_files_folders_label_update = Arquivos/Pastas ({ $files_number }) - ##### ATUALIZAÇÃO NECESSÁRIA DE ATUALIZAÇÃO #####
upper_files_folders_label_up_to_date = Arquivos/Pastas ({ $files_number }) - atualizados
# Select popover
button_select_all = Selecionar Todos
button_select_reverse = Seleção inversa
button_select_custom = Selecionar personalizado
button_unselect_custom = Desmarcar Personalizado
button_select_changed = Selecionar Alterado
button_unselect_changed = Desmarcar Alterado
# Un/Select custom
select_custom_example = Uso: */folder-nr*/* ou nome-versão-*.txt
select_custom_path = Caminho
select_custom_current_path = Caminho Atual
select_custom_future_path = Caminho futuro
select_custom_path_current_name = Caminho + Nome Atual
select_custom_path_future_name = Caminho + Nome Futuro
select_custom_directory_file = Diretório/Arquivo
select_custom_select_directory = Selecione o diretório
select_custom_unselect_directory = Desmarcar diretório
# General
dialog_button_ok = OK
dialog_button_cancel = cancelar
# Dialogs
dialog_name_files_to_include = Arquivos para incluir
dialog_name_folders_to_include = Pastas para incluir
dialog_scan_inside = Digitalizar dentro
dialog_ignore_folders = Ignorar pastas
dialog_confirm_renaming = Confirmar renomeação
dialog_outdated_results = Resultados desatualizados
dialog_results_of_renaming = Resultados da renomeação
dialog_save_rule = Salvar Regra
dialog_select_custom = Selecionar personalizado
dialog_unselect_custom = Desmarcar Personalizado

# Rule Window


## Common

label_usage_type = Tipo de uso:
label_example = EXEMPLO
label_example_text_before = Antes:
label_example_text_after = Depois:
button_rule_window_add = Adicionar Regra

## Custom

label_custom_instruction =
    $(NAME) - imprime o nome do arquivo
    $(EXT) - imprime extensão
    $(MODIF) - data de modificação do arquivo
    $(CREAT) - imprime a data de modificação do arquivo
    $(CURR) - imprime o nome do arquivo atual com a extensão
    $(PARENT) - imprime o nome da pasta pai
    $(N)/$(K) - imprime números (argumentos são opcionais)
    $(N:3:4:5) imprime números a partir de 3, com o passo 4
            e preenche-os com zeros a 5 posições.
    	K em vez disso, apenas posiciona na lista, também usa a posição de item na pasta.
menu_button_load_custom_rule = Seletor de regra personalizado
button_save_custom_rule = Salvar Regra Customizada

## Upper/Lower Case

check_button_letters_type_uppercase = Maiúscula
check_button_letters_type_lowercase = Minúsculo
check_button_letters_usage_name = Nome Somente
check_button_letters_usage_extension = Somente a extensão
check_button_letters_usage_both = Ambos
label_letters_tool_type = Tipo de Ferramenta:
# Purge
label_purge_tool_type = Tipo de Ferramenta:
check_button_purge_name = Nome Somente
check_button_purge_extension = Somente a extensão
check_button_purge_both = Ambos
# Add number
label_add_number_place = Coloque o número:
label_add_number_settings = Configurações de Números:
check_button_add_number_before_name = Antes do Nome
check_button_add_number_after_name = Depois do Nome
label_number_start_number = Número inicial
label_number_step = Passo
label_number_fill_zeros = Preencher com zeros
# Add text
check_button_add_text_before_name = Antes do Nome
check_button_add_text_after_name = Depois do Nome
label_add_text = Texto a adicionar:
# Replace
check_button_replace_name = Nome Somente
check_button_replace_extension = Somente a extensão
check_button_replace_both = Ambos
check_button_replace_case_sensitive = Caso sensível
check_button_replace_case_insensitive = Insensível ao caso
check_button_replace_regex = Usar expressão regular
check_button_replace_replace_all = Substituir todas as ocorrências
label_replace_replacing_strings = Substituindo Strings:
label_replace_text_to_find = Texto para encontrar
label_replace_text_to_replace = Texto substituído
label_replace_captures = Capturas
label_replace_captured_captures = Capturas capturadas
label_replace_captures_number = ({ $capture_number } captures)
label_replace_no_captures = Sem capturas
label_replace_invalid_regex = REGEX INVÁLIDO
# Trim
check_button_trim_name_start = Início do Nome
check_button_trim_name_end = Nome Final
check_button_trim_extension_start = Início da extensão
check_button_trim_extension_end = Final da Extensão
check_button_trim_case_sensitive = Caso sensível
check_button_trim_case_insensitive = Insensível ao caso
label_trim_trim_text = Recortar texto
label_trim_case_sensitivity = Sensibilidade do Caso
# Normalize name
label_normalize_name =
    Everything - renames the name to one that contains only the
                              characters `a-z`, `0-9`, `-`, ` `, `.`.
                              e.g. `żółć CZERona.Txt` -> `zolc-czerwona.txt`
    
    Partial - works exactly same as option above, but allows
                      to use spaces ` ` and big letters `A-Z`
                      e.g. ` Źrebię Krokietowe.Rar ` -> `Zrebie Krokietowe.Rar`
check_button_normalize_everything = Tudo
check_button_normalize_partial = Parcial
# RuleType
rule_type_custom = Personalizado
rule_type_case_size = Tamanho do Caso
rule_type_purge = Purge
rule_type_add_text = Adicionar Texto
rule_type_trim = Recortar
rule_type_replace = Substituir
rule_type_add_number = Adicionar Número
rule_type_normalize = Normalizar
# RulePlace
rule_place_none = N/D
rule_place_extension = Somente a extensão
rule_place_name = Nome Somente
rule_place_extension_name = Extensão e Nome
rule_place_before_extension = Antes da Extensão
rule_place_after_extension = Depois da extensão
rule_place_before_name = Antes do Nome
rule_place_after_name = Depois do Nome
rule_place_from_name_start = do início
rule_place_from_name_end_reverse = De Nome Final a Início
rule_place_from_extension_start = Do início da extensão
rule_place_from_extension_end_reverse = Da extensão final ao início
# Rule Description
rule_description_full_normalize = Normalização completa
rule_description_partial_normalize = Normalização parcial
rule_description_zeros = e preencha com { $zeros } zeros,
rule_description_step = Começando com { $start } com o passo { $step }{ $zeros }
rule_description_lowercase = Minúsculo
rule_description_uppercase = Maiúscula
rule_description_text = Texto
rule_description_added_text = Texto adicionado:
rule_description_start = Início
rule_description_end_of_name = fim do nome
rule_description_extension = extensão
rule_description_end_of_extension = fim da extensão
rule_description_trimming = Recortar "{ $trim_text }" de { $where_remove }
rule_description_custom_rule = Regra personalizada: { $custom_rule }
rule_description_replace = Substituindo { $additional_regex_text } "{ $text_to_find }" com "{ $text_to_replace }"
# Notebooks
notebook_tab_custom = Personalizado
notebook_tab_case_size = Casos Superiores/Minúsculos
notebook_tab_purge = Purge
notebook_tab_add_number = Adicionar Número
notebook_tab_add_text = Adicionar Texto
notebook_tab_replace = Substituir
notebook_tab_trim = Recortar
notebook_tab_normalize = Normalizar Nome
# Renaming dialog
renaming_question = Tem certeza que deseja renomear { $number_of_renamed_files } arquivos?
renaming_destination_file_exists = O arquivo de destino já existe.
renaming_renamed_files = Renomeado adequadamente { $properly_renamed } arquivos
renaming_ignored_files = Ignorado { $ignored } arquivos, porque o nome antes e depois da alteração são os mesmos.
renaming_failed_files = Falha ao renomear { $failed_vector } arquivos
renaming_list_of_failed_to_rename = Lista de todos os nomes falhados
renaming_error = Erro
renaming_some_records_not_updated = Alguns registros não são atualizados, você pode fazer isso clicando no botão Atualizar Nomes\nTem certeza de que deseja prosseguir sem atualizar os nomes?
renaming_missing_files = Ficheiros em falta
renaming_require_missing_files = Você precisa usar pelo menos 1 arquivo
renaming_missing_rules = Faltando Regras
renaming_require_missing_rules = Você precisa usar pelo menos 1 regra
