;; From helix 25.07.1, runtime/queries/elixir/indents.scm (MPL-2.0,
;; https://github.com/helix-editor/helix), changed here: a `do` block's `else`,
;; `rescue`, `catch` and `after` start a line a level out and add none of
;; their own, the block's being theirs.
;; This Source Code Form is subject to the terms of the Mozilla Public
;; License, v. 2.0: LICENSES/MPL-2.0.txt, or https://mozilla.org/MPL/2.0/.
[
  (anonymous_function)
  (do_block)
  (stab_clause)
] @indent

[
  "end"
  "else"
  "rescue"
  "catch"
  "after"
] @outdent
