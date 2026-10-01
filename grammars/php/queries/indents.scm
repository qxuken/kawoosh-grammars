;; From helix 25.07.1, runtime/queries/php/indents.scm (MPL-2.0,
;; https://github.com/helix-editor/helix).
;; This Source Code Form is subject to the terms of the Mozilla Public
;; License, v. 2.0: LICENSES/MPL-2.0.txt, or https://mozilla.org/MPL/2.0/.
[
  (array_creation_expression)
  (arguments)
  (formal_parameters)
  (compound_statement)
  (declaration_list)
  (binary_expression)
  (return_statement)
  (expression_statement)
  (switch_block)
  (anonymous_function_use_clause)
] @indent

[
  "}"
  ")"
  "]"
] @outdent
