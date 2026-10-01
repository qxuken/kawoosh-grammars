;; From helix 25.07.1, runtime/queries/protobuf/indents.scm (MPL-2.0,
;; https://github.com/helix-editor/helix), changed here: the grammar pinned here has
;; a body for a message and an enum alone, so a oneof, a service and an
;; rpc are indented as they are.
;; This Source Code Form is subject to the terms of the Mozilla Public
;; License, v. 2.0: LICENSES/MPL-2.0.txt, or https://mozilla.org/MPL/2.0/.
[
  (message_body)
  (enum_body)
  (oneof)
  (service)
  (rpc)
  (block_lit)
] @indent

"}" @outdent
