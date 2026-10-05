; Written here: an element's content a level in, a tag's attributes on
; their own lines a level in, the closing tag and a closing `>` or `/>`
; on its own line back out.
[
  (element)
  (STag)
  (EmptyElemTag)
  (doctypedecl)
] @indent

[
  (ETag)
  ">"
  "/>"
  "]"
] @outdent
