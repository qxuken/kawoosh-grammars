;; From helix 25.07.1, runtime/queries/odin/indents.scm (MPL-2.0,
;; https://github.com/helix-editor/helix).
;; This Source Code Form is subject to the terms of the Mozilla Public
;; License, v. 2.0: LICENSES/MPL-2.0.txt, or https://mozilla.org/MPL/2.0/.
[
  (block)
  (enum_declaration)
  (union_declaration)
  (struct_declaration)
  (struct)
  (parameters)
  (tuple_type)
  (struct_type)
  (call_expression)
  (switch_case)
] @indent

[
 ")"
 "]"
] @outdent

; Have to do all closing brackets separately because the one for switch statements shouldn't end.
(block "}" @outdent)
(enum_declaration "}" @outdent)
(union_declaration "}" @outdent)
(struct_declaration "}" @outdent)
(struct "}" @outdent)
(struct_type "}" @outdent)
