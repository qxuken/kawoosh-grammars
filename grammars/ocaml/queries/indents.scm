;; From helix 25.07.1, runtime/queries/ocaml/indents.scm (MPL-2.0,
;; https://github.com/helix-editor/helix), changed here: `end`, `)` and `]` start a
;; line a level out, as `}` does.
;; This Source Code Form is subject to the terms of the Mozilla Public
;; License, v. 2.0: LICENSES/MPL-2.0.txt, or https://mozilla.org/MPL/2.0/.
[
  (let_binding)
  (type_binding)
  (structure)
  (signature)
  (record_declaration)
  (function_expression)
  (match_case)
] @indent

[
  "}"
  "]"
  ")"
  "end"
] @outdent
