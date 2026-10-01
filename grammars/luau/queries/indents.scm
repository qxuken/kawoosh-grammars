;; From helix 25.07.1, runtime/queries/lua/indents.scm (MPL-2.0,
;; https://github.com/helix-editor/helix) — lua's, luau's own there being
;; for another grammar — changed here: an `elseif` and an `else` sit at
;; their `if`'s level, their bodies a level in.
;; This Source Code Form is subject to the terms of the Mozilla Public
;; License, v. 2.0: LICENSES/MPL-2.0.txt, or https://mozilla.org/MPL/2.0/.
[
  (function_definition)
  (function_declaration)
  (method_index_expression)
  (field)
  (if_statement)
  (for_statement)
  (repeat_statement)
  (while_statement)
  (table_constructor)
  (arguments)
  (do_statement)
  (object_type)
] @indent

[
  "end"
  "until"
  "}"
  ")"
] @outdent

[
  (elseif_statement)
  (else_statement)
] @indent
(elseif_statement "elseif" @outdent)
(else_statement "else" @outdent)
