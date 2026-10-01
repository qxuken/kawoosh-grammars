;; From helix 25.07.1, runtime/queries/fortran/indents.scm (MPL-2.0,
;; https://github.com/helix-editor/helix), changed here: the loop is `do_loop` in
;; the grammar pinned here; a `case`'s statements are a level in, and
;; `contains` starts a line a level out.
;; This Source Code Form is subject to the terms of the Mozilla Public
;; License, v. 2.0: LICENSES/MPL-2.0.txt, or https://mozilla.org/MPL/2.0/.
[
  (module)
  (program)
  (subroutine)
  (function)
  ; (interface)
  (if_statement)
  (do_loop)
  (where_statement)
  (derived_type_definition)
  (enum)
  (case_statement)
] @indent

[
  (end_module_statement)
  (end_program_statement)
  (end_subroutine_statement)
  (end_function_statement)
  ; (end_interface_statement)
  (end_if_statement)
  (end_do_loop_statement)
  (else_clause)
  (elseif_clause)
  (end_type_statement)
  (end_enum_statement)
  (end_where_statement)
  (contains_statement)
] @outdent

