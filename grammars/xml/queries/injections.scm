; Written here: the grammar's repository has none. An SVG's `<style>`
; and `<script>`, as text or in a CDATA section.
((element
  (STag (Name) @_tag)
  (content
    [
      (CharData) @injection.content
      (CDSect (CData) @injection.content)
    ]))
  (#eq? @_tag "style")
  (#set! injection.language "css"))

((element
  (STag (Name) @_tag)
  (content
    [
      (CharData) @injection.content
      (CDSect (CData) @injection.content)
    ]))
  (#eq? @_tag "script")
  (#set! injection.language "javascript"))
