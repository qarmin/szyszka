# Upper buttons
upper_start_renaming_button = بدء إعادة التسمية
upper_add_files_button = إضافة ملفات
upper_add_folders_button = إضافة مجلدات
upper_remove_selection_button = إزالة التحديد
upper_update_names_button = تحديث الأسماء
upper_results_one_up_button = رفع واحد
upper_results_one_down_button = أسفل واحد
upper_select_popup_button = حدد
# Bottom Buttons
bottom_rule_add_button = إضافة قاعدة
bottom_rule_edit_button = تعديل القاعدة
bottom_rule_remove_button = إزالة القاعدة
bottom_rule_one_up_button = رفع واحد
bottom_rule_one_down_button = أسفل واحد
bottom_rule_save_rules_button = حفظ القواعد
bottom_rule_load_rules_button = تحميل القواعد
# Edit names
edit_names_used_in_rules = الأسماء المستخدمة في القواعد: { $rules }
edit_names_choose_name = اختر اسم القواعد(إذا كان موجودا، سوف يتجاوزها)
# Tree View Rules
tree_view_upper_column_type = نوع
tree_view_upper_column_current_name = الاسم الحالي
tree_view_upper_column_future_name = اسم المستقبل
tree_view_upper_column_path = المسار
# Tree View Results
tree_view_bottom_tool_type = نوع الأداة
tree_view_bottom_usage_name = اسم الاستخدام
tree_view_bottom_description = الوصف
# Settings
settings_language_label = اللغة
settings_open_rules = فتح ملف إعدادات القواعد
settings_open_cache_custom_texts = فتح ملف ذاكرة التخزين المؤقت المخصص
settings_open_config_dir = فتح ذاكرة التخزين المؤقت
check_button_dark_theme = أيقونات داكنة
# Other in main window
bottom_rule_label_rules = القواعد
upper_files_folders_label = الملفات/المجلدات
upper_files_folders_label_update = الملفات/المجلدات({ $files_number }) - ###### تحديث مطلوب QUIRED #### #
upper_files_folders_label_up_to_date = الملفات/المجلدات ({ $files_number }) - محدثة
# Select popover
button_select_all = حدد الكل
button_select_reverse = الاختيار العكسي
button_select_custom = تحديد مخصص
button_unselect_custom = إلغاء تحديد مخصص
button_select_changed = حدد التغيير
button_unselect_changed = غير محدد
# Un/Select custom
select_custom_example = الاستخدام: */folder-nr*/* أو name-version *.txt
select_custom_path = المسار
select_custom_current_path = المسار الحالي
select_custom_future_path = المسار المستقبلي
select_custom_path_current_name = المسار + الاسم الحالي
select_custom_path_future_name = المسار + اسم المستقبل
select_custom_directory_file = الدليل/الملف
select_custom_select_directory = حدد الدليل
select_custom_unselect_directory = إلغاء تحديد الدليل
# General
dialog_button_ok = حسناً
dialog_button_cancel = إلغاء
# Dialogs
dialog_name_files_to_include = الملفات المراد تضمينها
dialog_name_folders_to_include = مجلدات لتضمينها
dialog_scan_inside = مسح داخلي
dialog_ignore_folders = تجاهل المجلدات
dialog_confirm_renaming = تأكيد إعادة التسمية
dialog_outdated_results = النتائج العتيقة
dialog_results_of_renaming = نتائج إعادة التسمية
dialog_save_rule = حفظ القاعدة
dialog_select_custom = تحديد مخصص
dialog_unselect_custom = إلغاء تحديد مخصص

# Rule Window


## Common

label_usage_type = نوع الاستخدام:
label_example = اكتمال
label_example_text_before = سابقا:
label_example_text_after = بعد:
button_rule_window_add = إضافة قاعدة

## Custom

label_custom_instruction =
    $(NAME) - اسم الملف
    $(EXT) - امتداد الطباعة
    $(MODIF) - تاريخ تعديل الملف
    $(CREAT) - الطباعة إنشاء الملف
    $(CURR) - يطبع اسم الملف الحالي مع امتداد
    $(PARENT) - اسم المجلد الأصلي
    $(N)/$(K) - أرقام الطباعة (الحجج اختيارية)
    $(N3:4:5) الطباعة أرقام من 3، مع الخطوة 4
            و تملئهم بأصفار إلى 5 مواقع.
    	<unk> K بدلاً من ذلك فقط في القائمة، يستخدم أيضًا عنصر الموضع في المجلد.
menu_button_load_custom_rule = اختيار قاعدة مخصصة
button_save_custom_rule = حفظ قاعدة مخصصة

## Upper/Lower Case

check_button_letters_type_uppercase = الحروف
check_button_letters_type_lowercase = أقل حروف
check_button_letters_usage_name = الاسم فقط
check_button_letters_usage_extension = ملحق فقط
check_button_letters_usage_both = كلاهما
label_letters_tool_type = نوع الأداة:
# Purge
label_purge_tool_type = نوع الأداة:
check_button_purge_name = الاسم فقط
check_button_purge_extension = ملحق فقط
check_button_purge_both = كلاهما
# Add number
label_add_number_place = مكان لوضع الرقم:
label_add_number_settings = إعدادات الرقم:
check_button_add_number_before_name = قبل الاسم
check_button_add_number_after_name = بعد الاسم
label_number_start_number = رقم البدء
label_number_step = خطوة
label_number_fill_zeros = ملء بالأصفار
# Add text
check_button_add_text_before_name = قبل الاسم
check_button_add_text_after_name = بعد الاسم
label_add_text = النص المراد إضافة:
# Replace
check_button_replace_name = الاسم فقط
check_button_replace_extension = ملحق فقط
check_button_replace_both = كلاهما
check_button_replace_case_sensitive = حالة حساسة
check_button_replace_case_insensitive = حالة غير حساسة
check_button_replace_regex = استخدام regex
check_button_replace_replace_all = استبدال جميع الحوادث
label_replace_replacing_strings = استبدال المقاطع
label_replace_text_to_find = النص للبحث عنه
label_replace_text_to_replace = استبدال النص
label_replace_captures = التقاط
label_replace_captured_captures = التقاط مأسور
label_replace_captures_number = ({ $capture_number } captures)
label_replace_no_captures = لا يوجد التقاط
label_replace_invalid_regex = INVALID REGEX
# Trim
check_button_trim_name_start = إسم البداية
check_button_trim_name_end = نهاية الاسم
check_button_trim_extension_start = بدء الإضافات
check_button_trim_extension_end = نهاية الملحق
check_button_trim_case_sensitive = حالة حساسة
check_button_trim_case_insensitive = حالة غير حساسة
label_trim_trim_text = تقليم النص
label_trim_case_sensitivity = حساسية الحالة
# Normalize name
label_normalize_name =
    Everything - renames the name to one that contains only the
                              characters `a-z`, `0-9`, `-`, ` `, `.`.
                              e.g. `żółć CZERona.Txt` -> `zolc-czerwona.txt`
    
    Partial - works exactly same as option above, but allows
                      to use spaces ` ` and big letters `A-Z`
                      e.g. ` Źrebię Krokietowe.Rar ` -> `Zrebie Krokietowe.Rar`
check_button_normalize_everything = كل شيء
check_button_normalize_partial = جزئي
# RuleType
rule_type_custom = مخصص
rule_type_case_size = حجم الحالة
rule_type_purge = Purge
rule_type_add_text = إضافة نص
rule_type_trim = تقليم
rule_type_replace = استبدل
rule_type_add_number = إضافة رقم
rule_type_normalize = تطبيع
# RulePlace
rule_place_none = لا
rule_place_extension = ملحق فقط
rule_place_name = الاسم فقط
rule_place_extension_name = التمديد والاسم
rule_place_before_extension = قبل التمديد
rule_place_after_extension = بعد التمديد
rule_place_before_name = قبل الاسم
rule_place_after_name = بعد الاسم
rule_place_from_name_start = من البداية
rule_place_from_name_end_reverse = من الاسم إلى البداية
rule_place_from_extension_start = من بداية الملحق
rule_place_from_extension_end_reverse = من نهاية الملحق إلى البدء
# Rule Description
rule_description_full_normalize = التطبيع الكامل
rule_description_partial_normalize = تطبيع جزئي
rule_description_zeros = وملء بـ { $zeros } صفر،
rule_description_step = يبدأ بـ { $start } بخطوة { $step }{ $zeros }
rule_description_lowercase = أقل حروف
rule_description_uppercase = الحروف
rule_description_text = نص
rule_description_added_text = النص المضاف:
rule_description_start = ابدأ
rule_description_end_of_name = نهاية الاسم
rule_description_extension = التمديد
rule_description_end_of_extension = نهاية التمديد
rule_description_trimming = تقليم "{ $trim_text }" من { $where_remove }
rule_description_custom_rule = قاعدة مخصصة: { $custom_rule }
rule_description_replace = استبدال { $additional_regex_text } "{ $text_to_find }" ب "{ $text_to_replace }"
# Notebooks
notebook_tab_custom = مخصص
notebook_tab_case_size = الحالات العليا/السفلى
notebook_tab_purge = Purge
notebook_tab_add_number = إضافة رقم
notebook_tab_add_text = إضافة نص
notebook_tab_replace = استبدل
notebook_tab_trim = تقليم
notebook_tab_normalize = تطبيع الاسم
# Renaming dialog
renaming_question = هل أنت متأكد من أنك تريد إعادة تسمية { $number_of_renamed_files } ملف؟
renaming_destination_file_exists = ملف الوجهة موجود مسبقاً.
renaming_renamed_files = تمت إعادة تسميته بشكل صحيح إلى { $properly_renamed } ملفات
renaming_ignored_files = تجاهل { $ignored } ملفات، لأن الاسم قبل التغيير وبعده هو نفسه.
renaming_failed_files = فشل في إعادة تسمية { $failed_vector } ملفات
renaming_list_of_failed_to_rename = قائمة بجميع عمليات إعادة التسمية الفاشلة
renaming_error = خطأ
renaming_some_records_not_updated = لم يتم تحديث بعض السجلات، يمكنك القيام بذلك من خلال النقر على زر تحديث الأسماء.\nهل أنت متأكد من أنك تريد المضي قدما دون تحديث الأسماء؟
renaming_missing_files = الملفات المفقودة
renaming_require_missing_files = تحتاج إلى استخدام ملف واحد على الأقل
renaming_missing_rules = القواعد مفقودة
renaming_require_missing_rules = تحتاج إلى استخدام قاعدة واحدة على الأقل
