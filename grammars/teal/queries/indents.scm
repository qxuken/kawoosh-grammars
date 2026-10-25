; Teal's indents, written here after kawoosh's lua query (helix
; 25.07.1's, MPL-2.0, https://github.com/helix-editor/helix): Lua's
; blocks, and a record's, an interface's and an enum's.
[
  (function_statement)
  (anon_function)
  (macroexp_statement)
  (if_statement)
  (generic_for_statement)
  (numeric_for_statement)
  (repeat_statement)
  (while_statement)
  (do_statement)
  (record_declaration)
  (interface_declaration)
  (enum_declaration)
  (anon_record)
  (anon_interface)
  (table_constructor)
  (arguments)
  (table_entry)
] @indent

[
  "end"
  "until"
  "}"
  ")"
] @outdent

; An `elseif` / `else` sits at its `if`'s level, its body a level in.
[
  (elseif_block)
  (else_block)
] @indent
(elseif_block "elseif" @outdent)
(else_block "else" @outdent)
