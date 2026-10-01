;; From helix 25.07.1, runtime/queries/zig/indents.scm (MPL-2.0,
;; https://github.com/helix-editor/helix), changed here: a struct's, an enum's, a
;; union's and an error set's members are a level in.
;; This Source Code Form is subject to the terms of the Mozilla Public
;; License, v. 2.0: LICENSES/MPL-2.0.txt, or https://mozilla.org/MPL/2.0/.
[
  (block)
  (switch_expression)
  (initializer_list)
  (struct_declaration)
  (enum_declaration)
  (union_declaration)
  (opaque_declaration)
  (error_set_declaration)
] @indent

[
  "}"
  "]"
  ")"
] @outdent
