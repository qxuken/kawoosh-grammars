; The grammar's own names each language by its capture (`@css`,
; `@javascript`), nvim's old spelling. The same, in tree-sitter's, with
; a `<script>` and a `<style>` read by their `lang`: javascript and css
; unless it names another (`ts`, `scss`). An expression in `{ }` reads
; as typescript, javascript's superset, as vue's do: a component in ts
; writes ts there too.

((style_element
  (start_tag) @_tag
  (raw_text) @injection.content)
  (#not-match? @_tag "\\slang\\s*=")
  (#set! injection.language "css"))

((style_element
  (start_tag
    (attribute
      (attribute_name) @_lang
      [(attribute_value) @_value
       (quoted_attribute_value (attribute_value) @_value)]))
  (raw_text) @injection.content)
  (#eq? @_lang "lang")
  (#any-of? @_value "css" "postcss")
  (#set! injection.language "css"))

((style_element
  (start_tag
    (attribute
      (attribute_name) @_lang
      [(attribute_value) @_value
       (quoted_attribute_value (attribute_value) @_value)]))
  (raw_text) @injection.content)
  (#eq? @_lang "lang")
  (#eq? @_value "scss")
  (#set! injection.language "scss"))

((script_element
  (start_tag) @_tag
  (raw_text) @injection.content)
  (#not-match? @_tag "\\slang\\s*=")
  (#set! injection.language "javascript"))

((script_element
  (start_tag
    (attribute
      (attribute_name) @_lang
      [(attribute_value) @_value
       (quoted_attribute_value (attribute_value) @_value)]))
  (raw_text) @injection.content)
  (#eq? @_lang "lang")
  (#any-of? @_value "js" "javascript")
  (#set! injection.language "javascript"))

((script_element
  (start_tag
    (attribute
      (attribute_name) @_lang
      [(attribute_value) @_value
       (quoted_attribute_value (attribute_value) @_value)]))
  (raw_text) @injection.content)
  (#eq? @_lang "lang")
  (#any-of? @_value "ts" "typescript")
  (#set! injection.language "typescript"))

((raw_text_expr) @injection.content
  (#set! injection.language "typescript"))
