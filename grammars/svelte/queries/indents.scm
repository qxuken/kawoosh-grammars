;; From helix 25.07.1, runtime/queries/svelte/indents.scm (MPL-2.0,
;; https://github.com/helix-editor/helix).
;; This Source Code Form is subject to the terms of the Mozilla Public
;; License, v. 2.0: LICENSES/MPL-2.0.txt, or https://mozilla.org/MPL/2.0/.
[
  (element)
  (if_statement)
  (each_statement)
  (await_statement)
  (script_element)
  (style_element)
] @indent

[
  (end_tag)
  (else_statement)
  (if_end_expr)
  (each_end_expr)
  (await_end_expr)
  ">"
  "/>"
] @outdent