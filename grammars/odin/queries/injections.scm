; The grammar's own is `(comment) @comment`, nvim's old spelling: a
; capture named for the language. This is the same in tree-sitter's.
((comment) @injection.content
  (#set! injection.language "comment"))
