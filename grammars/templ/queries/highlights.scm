; tree-sitter-go 0.25.0's queries/highlights.scm (MIT, Copyright (c) 2014
; Max Brunsfeld), then tree-sitter-templ's own at the pinned revision
; (MIT, Copyright (c) 2023 Vincent Rischmann) without its `; inherits: go`.

; Function calls

(call_expression
  function: (identifier) @function)

(call_expression
  function: (identifier) @function.builtin
  (#match? @function.builtin "^(append|cap|close|complex|copy|delete|imag|len|make|new|panic|print|println|real|recover)$"))

(call_expression
  function: (selector_expression
    field: (field_identifier) @function.method))

; Function definitions

(function_declaration
  name: (identifier) @function)

(method_declaration
  name: (field_identifier) @function.method)

; Identifiers

(type_identifier) @type
(field_identifier) @property
(identifier) @variable

; Operators

[
  "--"
  "-"
  "-="
  ":="
  "!"
  "!="
  "..."
  "*"
  "*"
  "*="
  "/"
  "/="
  "&"
  "&&"
  "&="
  "%"
  "%="
  "^"
  "^="
  "+"
  "++"
  "+="
  "<-"
  "<"
  "<<"
  "<<="
  "<="
  "="
  "=="
  ">"
  ">="
  ">>"
  ">>="
  "|"
  "|="
  "||"
  "~"
] @operator

; Keywords

[
  "break"
  "case"
  "chan"
  "const"
  "continue"
  "default"
  "defer"
  "else"
  "fallthrough"
  "for"
  "func"
  "go"
  "goto"
  "if"
  "import"
  "interface"
  "map"
  "package"
  "range"
  "return"
  "select"
  "struct"
  "switch"
  "type"
  "var"
] @keyword

; Literals

[
  (interpreted_string_literal)
  (raw_string_literal)
  (rune_literal)
] @string

(escape_sequence) @escape

[
  (int_literal)
  (float_literal)
  (imaginary_literal)
] @number

[
  (true)
  (false)
  (nil)
  (iota)
] @constant.builtin

(comment) @comment

; ---- templ
(component_declaration
  name: (component_identifier) @function)

[
  (tag_start)
  (tag_end)
  (self_closing_tag)
  (void_element)
  (style_tag_start)
  (style_tag_end)
  (self_closing_style_tag)
] @tag

(attribute
  name: (attribute_name) @tag.attribute)

(attribute
  value: (quoted_attribute_value) @string)

[
  (element_text)
  (style_element_text)
] @string.special

(css_identifier) @function

(css_property
  name: (css_property_name) @property)

(css_property
  value: (css_property_value) @string)

[
  (expression)
  (dynamic_class_attribute_value)
] @function.method

(component_import
  name: (component_identifier) @function)

(component_render) @function.call

(element_comment) @comment @spell

"@" @operator

[
  "templ"
  "css"
  "script"
] @keyword
