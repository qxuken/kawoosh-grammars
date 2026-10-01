;; From helix 25.07.1, runtime/queries/kotlin/indents.scm (MPL-2.0,
;; https://github.com/helix-editor/helix).
;; This Source Code Form is subject to the terms of the Mozilla Public
;; License, v. 2.0: LICENSES/MPL-2.0.txt, or https://mozilla.org/MPL/2.0/.
[
  (class_body)
  (enum_class_body)
  (lambda_literal)

  ; _block is hidden in the grammar, so list all public wrappers explicitly.
  (function_body)
  (anonymous_initializer)
  (control_structure_body)
  (secondary_constructor)
  (try_expression)
  (catch_block)
  (finally_block)

  (property_declaration)
  (assignment)

  (when_expression)
  (call_expression)
  (if_expression)

  ; Binary expressions
  (multiplicative_expression)
  (additive_expression)
  (range_expression)
  (infix_expression)
  (elvis_expression)
  (check_expression)
  (comparison_expression)
  (equality_expression)
  (comparison_expression)
  (equality_expression)
  (conjunction_expression)
  (disjunction_expression)

  (call_suffix)
  (function_value_parameters)
] @indent

[
  "}"
  ")"
  "]"
] @outdent
