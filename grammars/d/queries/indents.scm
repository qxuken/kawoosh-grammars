;; From helix 25.07.1, runtime/queries/d/indents.scm (MPL-2.0,
;; https://github.com/helix-editor/helix), changed here: a statement that is an `if`'s
;; or a loop's whole body, with no braces, is a level in on its own line.
;; This Source Code Form is subject to the terms of the Mozilla Public
;; License, v. 2.0: LICENSES/MPL-2.0.txt, or https://mozilla.org/MPL/2.0/.
[
  (parameters)
  (template_parameters)
  (expression_statement)
  (aggregate_body)
  (function_body)
  (block_statement)
  (case_statement)
] @indent

((scope_statement
  (_) @_body) @indent
  (#not-kind-eq? @_body "block_statement")
  (#set! "scope" "all"))

[
  (case)
  (default)
  "}"
  "]"
] @outdent
