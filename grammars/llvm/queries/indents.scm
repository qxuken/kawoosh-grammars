;; From helix 25.07.1, runtime/queries/llvm/indents.scm (MPL-2.0,
;; https://github.com/helix-editor/helix), changed here: a label starts a line a
;; level out, at its function's.
;; This Source Code Form is subject to the terms of the Mozilla Public
;; License, v. 2.0: LICENSES/MPL-2.0.txt, or https://mozilla.org/MPL/2.0/.
[
  (function_body)
  (instruction)
] @indent

[
  "}"
  (label)
] @outdent
