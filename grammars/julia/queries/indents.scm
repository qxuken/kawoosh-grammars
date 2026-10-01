;; From helix 25.07.1, runtime/queries/julia/indents.scm (MPL-2.0,
;; https://github.com/helix-editor/helix), changed here: the grammar pinned here has
;; no `parameter_list`; `catch`, `finally`, `else` and `elseif` start a line
;; a level out.
;; This Source Code Form is subject to the terms of the Mozilla Public
;; License, v. 2.0: LICENSES/MPL-2.0.txt, or https://mozilla.org/MPL/2.0/.
[
  (struct_definition)
  (macro_definition)
  (function_definition)
  (compound_statement)
  (if_statement)
  (try_statement)
  (for_statement)
  (while_statement)
  (let_statement)
  (quote_statement)
  (do_clause)
  (assignment)
  (for_binding)
  (call_expression)
  (parenthesized_expression)
  (tuple_expression)
  (comprehension_expression)
  (matrix_expression)
  (vector_expression)
] @indent

[
  "end"
  ")"
  "]"
  "}"
  "catch"
  "finally"
  "else"
  "elseif"
] @outdent

(argument_list
  . (_) @anchor
  (#set! "scope" "tail")) @align

(curly_expression
  . (_) @anchor
  (#set! "scope" "tail")) @align
