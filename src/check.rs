//! What a grammar must pass to be released, tried with this machine's
//! library as kawoosh will load it: the symbol is there, the ABI is one
//! kawoosh's tree-sitter reads, every query compiles against the
//! grammar and has the capture its kind is read by, and the sample
//! parses with no error. A revision moved past
//! its queries fails here, not in an editor.

use std::path::Path;

use tree_sitter_language::LanguageFn;

use crate::queries::Set;

/// Opens `lib` and takes the language off `symbol`. The library stays
/// open for the life of the process: the language points into it.
fn language(lib: &Path, symbol: &str) -> Result<tree_sitter::Language, String> {
    // SAFETY: a grammar library has no initialisers of its own, and the
    // symbol is the `LanguageFn` shape every grammar exports.
    let library =
        unsafe { libloading::Library::new(lib) }.map_err(|e| format!("{}: {e}", lib.display()))?;
    let raw: unsafe extern "C" fn() -> *const () = unsafe {
        *library
            .get::<unsafe extern "C" fn() -> *const ()>(symbol.as_bytes())
            .map_err(|_| format!("the library exports no `{symbol}`"))?
    };
    std::mem::forget(library);
    Ok(unsafe { LanguageFn::from_raw(raw) }.into())
}

/// Checks the grammar and its queries, answering the language for
/// [`sample`] to parse with. Its ABI is `abi_version()`.
pub fn grammar(lib: &Path, symbol: &str, queries: &Set) -> Result<tree_sitter::Language, String> {
    let language = language(lib, symbol)?;
    let abi = language.abi_version();
    let (min, max) = (
        tree_sitter::MIN_COMPATIBLE_LANGUAGE_VERSION,
        tree_sitter::LANGUAGE_VERSION,
    );
    if !(min..=max).contains(&abi) {
        return Err(format!(
            "grammar ABI {abi}; tree-sitter {min}..={max} is what kawoosh reads"
        ));
    }
    if !queries.contains_key("highlights.scm") {
        return Err("no highlights.scm, in the checkout or here".into());
    }
    for (file, text) in queries {
        let query = tree_sitter::Query::new(&language, text).map_err(|e| {
            let what = match e.kind {
                tree_sitter::QueryErrorKind::NodeType => "the grammar has no node",
                tree_sitter::QueryErrorKind::Field => "the grammar has no field",
                tree_sitter::QueryErrorKind::Capture => "no capture",
                _ => "does not compile:",
            };
            format!(
                "{file}:{}:{}: {what} {}",
                e.row + 1,
                e.column + 1,
                e.message
            )
        })?;
        // The capture a reader finds the file's point by: tree-sitter's
        // own convention for each, and what kawoosh refuses a file
        // without.
        let needs = match file.as_str() {
            "injections.scm" => Some("injection.content"),
            "tags.scm" | "outline.scm" => Some("name"),
            _ => None,
        };
        if let Some(capture) = needs
            && !query.capture_names().contains(&capture)
        {
            return Err(format!(
                "{file}: no @{capture} capture; replace it with one that has, in queries/"
            ));
        }
        if file == "indents.scm" {
            indents(&query)?;
        }
    }
    Ok(language)
}

/// The predicates an indent query may use past tree-sitter's own, in
/// the dialect kawoosh reads: helix's.
const INDENT_PREDICATES: [&str; 5] = [
    "not-kind-eq?",
    "same-line?",
    "not-same-line?",
    "one-line?",
    "not-one-line?",
];

/// The captures of that dialect.
const INDENT_CAPTURES: [&str; 8] = [
    "indent",
    "indent.always",
    "outdent",
    "outdent.always",
    "align",
    "anchor",
    "extend",
    "extend.prevent-once",
];

/// An `indents.scm` is in the dialect its reader knows: kawoosh refuses
/// a grammar whole — its colours too — whose indent query uses a
/// predicate it does not have or a `scope` that is not `all` or
/// `tail`, so one is refused here first. A query with none of the
/// dialect's captures is nvim's (`@indent.begin`), which would compile
/// and do nothing.
fn indents(query: &tree_sitter::Query) -> Result<(), String> {
    for i in 0..query.pattern_count() {
        for p in query.general_predicates(i) {
            if !INDENT_PREDICATES.contains(&p.operator.as_ref()) {
                return Err(format!(
                    "indents.scm: no predicate #{} in the dialect kawoosh reads",
                    p.operator
                ));
            }
        }
        for p in query.property_settings(i) {
            if &*p.key == "scope" && !matches!(p.value.as_deref(), Some("all" | "tail")) {
                return Err(format!(
                    "indents.scm: scope {:?} is not all or tail",
                    p.value.as_deref().unwrap_or("")
                ));
            }
        }
    }
    if !query
        .capture_names()
        .iter()
        .any(|c| INDENT_CAPTURES.contains(c))
    {
        return Err(
            "indents.scm: none of @indent, @outdent, @align, @extend: not helix's dialect".into(),
        );
    }
    Ok(())
}

/// Parses the sample: no ERROR and no MISSING node in its tree.
pub fn sample(language: &tree_sitter::Language, sample: &Path) -> Result<(), String> {
    let text = std::fs::read_to_string(sample).map_err(|e| format!("{}: {e}", sample.display()))?;
    let mut parser = tree_sitter::Parser::new();
    parser.set_language(language).map_err(|e| e.to_string())?;
    let tree = parser
        .parse(&text, None)
        .ok_or("the sample did not parse")?;
    if let Some(node) = first_error(tree.root_node()) {
        let at = node.start_position();
        let what = if node.is_missing() {
            format!("a missing `{}`", node.kind())
        } else {
            "an ERROR".into()
        };
        let name = sample.file_name().unwrap().to_string_lossy();
        return Err(format!(
            "{name}:{}:{}: {what} in the sample's tree",
            at.row + 1,
            at.column + 1
        ));
    }
    Ok(())
}

/// The first ERROR or MISSING node under `node`, in the text's order.
fn first_error(node: tree_sitter::Node) -> Option<tree_sitter::Node> {
    if !node.has_error() {
        return None;
    }
    if node.is_error() || node.is_missing() {
        return Some(node);
    }
    let mut cursor = node.walk();
    node.children(&mut cursor).find_map(first_error)
}
