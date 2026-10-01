;; From helix 25.07.1, runtime/queries/nix/indents.scm (MPL-2.0,
;; https://github.com/helix-editor/helix), changed here: a `let`'s bindings are a
;; level in and its `in` and body are not; a binding's later lines are a
;; level in; the closing `''` of a string is a level out.
;; This Source Code Form is subject to the terms of the Mozilla Public
;; License, v. 2.0: LICENSES/MPL-2.0.txt, or https://mozilla.org/MPL/2.0/.
[
  (indented_string_expression)
  (string_expression)

  ; these are all direct parents of (binding_set)
  (attrset_expression)
  (let_attrset_expression)
  (rec_attrset_expression)

  (binding)
  (list_expression)
  (parenthesized_expression)
] @indent

; A `let`'s bindings are a level in; its `in` and what follows are not.
(let_expression) @indent
(let_expression "in" @outdent)
(let_expression body: (_) @outdent)

(if_expression [ "if" "then" "else" ] @align)

[
  "}"
  "]"
  ")"
  "''"
] @outdent
