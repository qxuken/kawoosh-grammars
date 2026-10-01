;; From helix 25.07.1, runtime/queries/fish/indents.scm (MPL-2.0,
;; https://github.com/helix-editor/helix), changed here: a `case`'s commands are a
;; level in from it, and `else` starts a line a level out.
;; This Source Code Form is subject to the terms of the Mozilla Public
;; License, v. 2.0: LICENSES/MPL-2.0.txt, or https://mozilla.org/MPL/2.0/.
[
  (function_definition)
  (while_statement)
  (for_statement)
  (if_statement)
  (begin_statement)
  (switch_statement)
  (case_clause)
] @indent

[
  "end"
  "else"
] @outdent
