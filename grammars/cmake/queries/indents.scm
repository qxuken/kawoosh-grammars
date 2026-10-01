;; From helix 25.07.1, runtime/queries/cmake/indents.scm (MPL-2.0,
;; https://github.com/helix-editor/helix), changed here: the commands that close or
;; divide a block start a line a level out.
;; This Source Code Form is subject to the terms of the Mozilla Public
;; License, v. 2.0: LICENSES/MPL-2.0.txt, or https://mozilla.org/MPL/2.0/.
[
  (if_condition)
  (foreach_loop)
  (while_loop)
  (function_def)
  (macro_def)
  (normal_command)
] @indent

[
  ")"
  (elseif_command)
  (else_command)
  (endif_command)
  (endforeach_command)
  (endwhile_command)
  (endfunction_command)
  (endmacro_command)
] @outdent
