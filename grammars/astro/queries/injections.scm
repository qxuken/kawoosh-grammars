; The grammar's own, with what it leaves to nvim's html: a `<style>`
; read as css unless its `lang` names scss, and a `<script>` of json
; (`application/ld+json`, `importmap`) as json. Every other `<script>`
; is typescript, as Astro compiles it, and so is the frontmatter and
; every `{ }`.

(frontmatter
  (frontmatter_js_block) @injection.content
  (#set! injection.language "typescript"))

(attribute_interpolation
  (attribute_js_expr) @injection.content
  (#set! injection.language "typescript"))

(html_interpolation
  (permissible_text) @injection.content
  (#set! injection.language "typescript"))

((script_element
  (start_tag) @_tag
  (raw_text) @injection.content)
  (#not-match? @_tag "\\stype\\s*=\\s*[\"']?(application/(ld\\+)?json|importmap)[\"'\\s>]")
  (#set! injection.language "typescript"))

((script_element
  (start_tag
    (attribute
      (attribute_name) @_type
      [(attribute_value) @_value
       (quoted_attribute_value (attribute_value) @_value)]))
  (raw_text) @injection.content)
  (#eq? @_type "type")
  (#any-of? @_value "application/json" "application/ld+json" "importmap")
  (#set! injection.language "json"))

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
