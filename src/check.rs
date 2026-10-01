//! What a grammar must pass to be released, tried with this machine's
//! library as kawoosh will load it: the symbol is there, the ABI is one
//! kawoosh's tree-sitter reads, every query compiles against the
//! grammar, and the sample parses with no error. A revision moved past
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

/// Checks the grammar, answering its ABI.
pub fn check(lib: &Path, symbol: &str, queries: &Set, sample: &Path) -> Result<usize, String> {
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
        tree_sitter::Query::new(&language, text).map_err(|e| {
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
    }
    let text = std::fs::read_to_string(sample).map_err(|e| format!("{}: {e}", sample.display()))?;
    let mut parser = tree_sitter::Parser::new();
    parser.set_language(&language).map_err(|e| e.to_string())?;
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
    Ok(abi)
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
