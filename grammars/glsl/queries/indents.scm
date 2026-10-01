;; From helix 25.07.1, runtime/queries/glsl/indents.scm (MPL-2.0,
;; https://github.com/helix-editor/helix).
;; This Source Code Form is subject to the terms of the Mozilla Public
;; License, v. 2.0: LICENSES/MPL-2.0.txt, or https://mozilla.org/MPL/2.0/.
[
  (init_declarator)
  (compound_statement)
  (preproc_arg)
  (field_declaration_list)
  (case_statement)
  (conditional_expression)
  (enumerator_list)
  (struct_specifier)
  (compound_literal_expression)
] @indent

[
  "#define"
  "#ifdef"
  "#endif"
  "{"
  "}"
] @outdent
