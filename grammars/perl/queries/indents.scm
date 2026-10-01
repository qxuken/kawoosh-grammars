;; From helix 25.07.1, runtime/queries/perl/indents.scm (MPL-2.0,
;; https://github.com/helix-editor/helix).
;; This Source Code Form is subject to the terms of the Mozilla Public
;; License, v. 2.0: LICENSES/MPL-2.0.txt, or https://mozilla.org/MPL/2.0/.
[
  (block)
  (conditional_statement)
  (loop_statement)
  (cstyle_for_statement)
  (for_statement)
  (elsif)
  (array_element_expression)
  (hash_element_expression)
  (coderef_call_expression)
  (anonymous_slice_expression)
  (slice_expression)
  (keyval_expression)
  (anonymous_array_expression)
  (anonymous_hash_expression)
  (stub_expression)
  (func0op_call_expression)
  (func1op_call_expression)
  (map_grep_expression)
  (function_call_expression)
  (method_call_expression)
  (attribute)
] @indent

[
  "}"
  "]"
  ")"
] @outdent
