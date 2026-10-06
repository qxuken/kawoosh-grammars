# kawoosh-grammars

Tree-sitter grammars for kawoosh, built ahead: the list of them, the
queries they are read with, and the builder that turns each into one
file an editor can fetch and load without a compiler.

A release is a folder of

- `manifest.json` — every grammar: the files that are its language
  (`extensions`, `filenames`, `shebangs`, `aliases`), where it came from
  (`repo`, `rev`, `path`, `license`), its `abi` and `symbol`, and its
  archive's name, `size` and `blake3`;
- `NAME.sqlar` — one archive a grammar.

## An archive

A [sqlite archive](https://sqlite.org/sqlar.html): the table
`sqlar(name, mode, mtime, sz, data)`, a blob deflated with zlib when
that is smaller, and a table `meta(key, value)` saying what it was
built from.

```
lib/x86_64-windows.dll    lib/aarch64-windows.dll
lib/x86_64-linux.so       lib/aarch64-linux.so      (glibc 2.17 and later)
lib/x86_64-macos.dylib    lib/aarch64-macos.dylib
queries/highlights.scm    queries/injections.scm    queries/tags.scm    queries/indents.scm
LICENSE
```

Each library exports `tree_sitter_NAME` (the manifest's `symbol`), as
`tree-sitter build` would make it. The queries are whole: what a
grammar inherits is already in front of its own. `sqlite3` reads one:

```bash
sqlite3 zig.sqlar -At
```

```bash
sqlite3 zig.sqlar -Ax lib/x86_64-linux.so queries/highlights.scm
```

## A grammar

```
grammars/
  zig/
    grammar.toml
    sample.zig        # parsed by the check: no ERROR node
    queries/          # only what replaces or adds to the grammar's own
      indents.scm
```

```toml
# grammars/zig/grammar.toml — the directory's name is the language's
extensions = ["zig", "zon"]   # lowercase, without the dot
filenames = []                # whole file names: "Dockerfile"
shebangs = []                 # interpreters of a `#!` line: "ruby"
aliases = []                  # other spellings: "rb"
# symbol = "tree_sitter_zig"  # when it is not tree_sitter_NAME
comment = "//"                # the line comment token, without its space
# comment_block = ["/*", "*/"]  # the block pair; either left out where there is none

[source]
repo = "https://github.com/tree-sitter-grammars/tree-sitter-zig"
rev = "6479aa13f32f701c383083d8b28360ebd682fb7d"   # a whole commit
# path = "."                  # where src/parser.c is: "tsx" in typescript's repository
license = "MIT"               # SPDX; the text is copied from the checkout

# [queries]
# inherits = ["c"]            # grammars here whose queries go in front
# skip = ["injections.scm"]   # query files of the checkout not taken
```

Queries: `highlights.scm`, `injections.scm` and `tags.scm` are taken
from the grammar's own `queries/` at the revision — beside the
grammar, or under the root's in a directory of the grammar's `path`
(xml's are in `queries/xml`), or at the root. A file in this
repository's `queries/` replaces the one of its name — or goes after
it, when the comments it opens with say `; extends`: a reader gives a
node two patterns match to the later one, so a few patterns added
change a few captures and the rest stay the grammar's.

```scheme
; grammars/zig/queries/highlights.scm
; extends
((identifier) @constant
  (#match? @constant "^[A-Z][A-Z0-9_]*$"))
```

`indents.scm` is only ever this repository's: kawoosh reads helix's
dialect — `@indent`, `@outdent`, `@align` with `@anchor`, `@extend`,
the predicates `#not-kind-eq?`, `#same-line?`, `#not-same-line?`,
`#one-line?`, `#not-one-line?`, and `#set! "scope" "all"|"tail"` — and
a grammar's repository carries nvim's. The check refuses one in another
dialect, since kawoosh would refuse the grammar whole. Most are helix's
own, changed where the grammar pinned here or its language's usual
style asked, and each says so at its top; a few are written here. One
goes in when kawoosh's indenter, given the query, leaves every line of
the grammar's sample where it is — tried from kawoosh
(`KAWOOSH_GRAMMARS_REPO=. cargo nextest run -p kawoosh --test
grammars`, after a build here), since the indenter is its own.

To add one: the directory, the `grammar.toml`, a `sample.*` that uses
the language broadly, and `cargo run -- check NAME`, which builds this
machine's library alone, runs the checks and writes nothing. To move
one: a new `rev`, and the same. A sample is written here, not copied
from the grammar's repository: it is this repository's, under its
licence.

A query file of the checkout that does not hold — an `injections.scm`
in nvim's old spelling, a `tags.scm` with no `@name` — is replaced by
one in `queries/`, or left out with `skip`. A grammar's known limits
are said in a comment at the top of its `grammar.toml`.

A grammar is refused when its checkout has no `src/parser.c` (it
commits no generated parser) or its scanner is C++.

## Building

Needs git, a Rust toolchain, and zig at the version in `.zig-version`
(`$ZIG` names another command, `python3 -m ziglang` for one).

```bash
cargo run --release -- build
```

writes `dist/`. For each grammar the builder

1. fetches the source at `rev` into `work/src` (one commit, kept);
2. builds the six libraries with `zig cc`;
3. makes the queries whole;
4. checks, with this machine's library: the symbol is there, the ABI
   is one tree-sitter 0.27 reads (13 to 15), every query compiles
   against the grammar and has the capture its kind is read by
   (`@injection.content` in `injections.scm`, `@name` in `tags.scm`),
   the sample parses with no ERROR or MISSING node;
5. writes the archive, reads this machine's library back out of it,
   and adds the manifest's row.

One grammar failing fails the build and writes no manifest.
`cargo run -- build zig ruby` builds those alone, `cargo run -- check`
is steps 1 to 4 for this machine alone, and `cargo run -- list` says
what is here.

The same sources and the same zig give the same bytes on one machine.
Two machines are not promised to agree, so a host that builds its own
release serves its own manifest: take a manifest and the archives it
names from the same place.

## Releases

A tag `rN` on main builds and publishes on both hosts:

- `https://drydock9.qxuken.dev/qxuken/kawoosh-grammars/releases/download/latest/`
- `https://github.com/qxuken/kawoosh-grammars/releases/latest/download/`

## Licence

The builder and what is written here are MIT (`LICENSE`). Each grammar
is under its own licence, named in its `grammar.toml` and carried in
its archive. An `indents.scm` that says it is from helix is under the
Mozilla Public License 2.0 (`LICENSES/MPL-2.0.txt`), as helix is; the
file says so, and goes into its grammar's archive as it is.
