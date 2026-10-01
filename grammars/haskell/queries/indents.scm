;; Written here, in helix's dialect (helix has none for haskell): what
;; a layout opens is a level in from the line that opened it.
[
  (data_type)
  (newtype)
  (class)
  (instance)
  (function)
  (bind)
  (do)
  (case)
  (lambda_case)
  (alternative)
  (let_in)
  (record)
  (list)
  (tuple)
  (parens)
  (exports)
  (import_list)
] @indent

[
  ")"
  "]"
  "}"
] @outdent

; A `let`'s later bindings line up under its first.
(local_binds
  . (_) @anchor
  (#set! "scope" "tail")) @align
