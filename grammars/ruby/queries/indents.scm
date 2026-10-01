;; From helix 25.07.1, runtime/queries/ruby/indents.scm (MPL-2.0,
;; https://github.com/helix-editor/helix), changed here: `rescue`, `ensure` and
;; `else` start a line a level out, as `when` does.
;; This Source Code Form is subject to the terms of the Mozilla Public
;; License, v. 2.0: LICENSES/MPL-2.0.txt, or https://mozilla.org/MPL/2.0/.
[
  (argument_list)
  (array)
  (begin)
  (block)
  (call)
  (class)
  (case)
  (elsif)
  (if)
  (hash)
  (method)
  (module)
  (singleton_class)
  (singleton_method)
] @indent

[
  ")"
  "}"
  "]"
  "end"
  "when"
  "rescue"
  "ensure"
  "else"
] @outdent
