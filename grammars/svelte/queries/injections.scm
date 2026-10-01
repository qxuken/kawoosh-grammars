; The grammar's own names each language by its capture (`@css`,
; `@javascript`), nvim's old spelling. The same, in tree-sitter's.
((style_element
  (raw_text) @injection.content)
  (#set! injection.language "css"))

((script_element
  (raw_text) @injection.content)
  (#set! injection.language "javascript"))

((raw_text_expr) @injection.content
  (#set! injection.language "javascript"))
