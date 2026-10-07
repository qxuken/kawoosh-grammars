; tree-sitter-templ's queries/templ/injections.scm at the pinned revision
; (MIT, Copyright (c) 2023 Vincent Rischmann) without its `; inherits: go`.

((element_comment) @injection.content
  (#set! injection.language "comment"))

((script_block_text) @injection.content
  (#set! injection.language "javascript"))

((script_element_text) @injection.content
  (#set! injection.language "javascript"))

((style_element_text) @injection.content
  (#set! injection.language "css"))
