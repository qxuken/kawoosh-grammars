;; From helix 25.07.1, runtime/queries/java/indents.scm (MPL-2.0,
;; https://github.com/helix-editor/helix).
;; This Source Code Form is subject to the terms of the Mozilla Public
;; License, v. 2.0: LICENSES/MPL-2.0.txt, or https://mozilla.org/MPL/2.0/.
[
  (class_body)
  (enum_body)
  (interface_body)
  (constructor_body)
  (annotation_type_body)
  (module_body)
  (block)
  (switch_block)
  (array_initializer)
  (argument_list)
  (formal_parameters)
  (annotation_argument_list)
  (element_value_array_initializer)
] @indent

[
  "}"
  ")"
  "]"
] @outdent

; Single statement after if/while/for without brackets
(if_statement
  consequence: (_) @indent
  (#not-kind-eq? @indent "block")
  (#set! "scope" "all"))
(while_statement
  body: (_) @indent
  (#not-kind-eq? @indent "block")
  (#set! "scope" "all"))
(for_statement
  (_) @indent
  (#not-kind-eq? @indent "block")
  (#set! "scope" "all"))
