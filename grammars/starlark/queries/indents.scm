;; From helix 25.07.1: runtime/queries/starlark/indents.scm inherits
;; python's, which this is (MPL-2.0, https://github.com/helix-editor/helix),
;; less what starlark has not: a class, a `try`, a `with`, a `while`, a
;; `match`, a `raise`, an `import from`.
;; This Source Code Form is subject to the terms of the Mozilla Public
;; License, v. 2.0: LICENSES/MPL-2.0.txt, or https://mozilla.org/MPL/2.0/.
[
  (list)
  (tuple)
  (dictionary)
  (set)

  (if_statement)
  (for_statement)
  (while_statement)
  (with_statement)
  (match_statement)
  (case_clause)

  (parenthesized_expression)
  (list_comprehension)
  (set_comprehension)
  (dictionary_comprehension)

  (tuple_pattern)
  (list_pattern)
  (argument_list)
  (parameters)
  (binary_operator)

  (function_definition)
] @indent

(ERROR
  .
  "def") @indent @extend
(ERROR
  (block) @indent @extend
  (#set! "scope" "all"))

[
  (if_statement)
  (for_statement)
  (while_statement)
  (with_statement)
  (match_statement)
  (case_clause)

  (function_definition)
] @extend

[
  (return_statement)
  (break_statement)
  (continue_statement)
  (pass_statement)
] @extend.prevent-once

[
  ")"
  "]"
  "}"
] @outdent
(elif_clause
  "elif" @outdent)
(else_clause
  "else" @outdent)

(parameters
  .
  (identifier) @anchor
  (#set! "scope" "tail")) @align
(argument_list
  .
  (_) @anchor
  (#set! "scope" "tail")) @align

