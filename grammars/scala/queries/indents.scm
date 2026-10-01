;; From helix 25.07.1, runtime/queries/scala/indents.scm (MPL-2.0,
;; https://github.com/helix-editor/helix).
;; This Source Code Form is subject to the terms of the Mozilla Public
;; License, v. 2.0: LICENSES/MPL-2.0.txt, or https://mozilla.org/MPL/2.0/.
[
  (block)
  (arguments)
  (parameter)
  (class_definition)
  (trait_definition)
  (object_definition)
  (function_definition)
  (val_definition)
  (import_declaration)
  (while_expression)
  (do_while_expression)
  (for_expression)
  (try_expression)
  (match_expression)
] @indent

[
  "}"
  "]"
  ")"
] @outdent
