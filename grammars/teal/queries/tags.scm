; Teal's outline, written here: functions and methods, the types a file
; declares (records, interfaces, enums, aliases) and what is in them,
; then its variables.
(function_statement name: (identifier) @name) @definition.function
(function_statement name: (table_function_name method: (identifier) @name)) @definition.method
(function_statement name: (table_function_name entry: (identifier) @name)) @definition.function
(macroexp_statement name: (identifier) @name) @definition.macro

(record_declaration name: (identifier) @name) @definition.struct
(interface_declaration name: (identifier) @name) @definition.interface
(enum_declaration name: (identifier) @name) @definition.enum
(type_declaration name: (identifier) @name value: (anon_record)) @definition.struct
(type_declaration name: (identifier) @name value: (anon_interface)) @definition.interface
(type_declaration name: (identifier) @name) @definition.type
(typedef name: (identifier) @name) @definition.type
(field key: (identifier) @name) @definition.field

; Variables, after every pattern that names a definition better.
(program (var_declaration (var_declarators . (var name: (identifier) @name))) @definition.variable)
