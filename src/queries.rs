//! A grammar's queries as its archive holds them: the checkout's own,
//! the repository's laid over them file by file — in the checkout's
//! place, or after it when the file says `; extends` — and in front of
//! each the same file of every grammar it inherits. What is shipped is
//! whole: nobody downstream resolves an `; inherits:` or an
//! `; extends`.

use std::collections::BTreeMap;
use std::path::Path;

use crate::spec::Spec;

/// A grammar's query files: the name (`highlights.scm`), the text.
pub type Set = BTreeMap<String, String>;

/// What is taken from a checkout. `indents.scm` is not: kawoosh reads
/// helix's dialect and a grammar's repository carries nvim's, so
/// indents come from this repository alone.
const UPSTREAM: [&str; 3] = ["highlights.scm", "injections.scm", "tags.scm"];

/// A grammar's queries before inheritance: the checkout's (beside the
/// grammar, else under the root's `queries/` in a directory of the
/// grammar's path — xml's are in `queries/xml` — or of its name, as
/// nvim lays them out — vue's are in `queries/vue` — else at the root;
/// the first of these holding a query file), with in front of each
/// file what it says it inherits from a directory of queries beside
/// its own in the checkout that is no grammar (vue's `html_tags`),
/// then every `.scm` of the repository's
/// `queries/` — which replaces the checkout's of that name, or, saying
/// `; extends` in the comments at its top, goes after it: a reader
/// gives a node two patterns match to the later, so a few patterns
/// added change a few captures and the rest stay the grammar's.
pub fn own(checkout: &Path, spec: &Spec) -> Result<Set, String> {
    let mut set = Set::new();
    let dirs = [
        checkout.join(&spec.source.path).join("queries"),
        checkout.join("queries").join(&spec.source.path),
        checkout.join("queries").join(&spec.name),
        checkout.join("queries"),
    ];
    let holds = |d: &Path| UPSTREAM.iter().any(|n| d.join(n).is_file());
    if let Some(dir) = dirs.iter().find(|d| holds(d)) {
        for name in UPSTREAM {
            let file = dir.join(name);
            if file.is_file() && !spec.skip.iter().any(|s| s == name) {
                let text = read(&file)?;
                set.insert(name.to_string(), with_siblings(&text, name, dir, spec)?);
            }
        }
    }
    let over = spec.dir.join("queries");
    if over.is_dir() {
        for entry in std::fs::read_dir(&over).map_err(|e| format!("{}: {e}", over.display()))? {
            let path = entry.map_err(|e| e.to_string())?.path();
            if path.extension().is_some_and(|e| e == "scm") {
                let name = path.file_name().unwrap().to_string_lossy().into_owned();
                let ours = read(&path)?;
                match set.get_mut(&name) {
                    Some(theirs) if extends(&ours) => {
                        if !theirs.ends_with('\n') {
                            theirs.push('\n');
                        }
                        theirs.push_str(&ours);
                    }
                    _ => {
                        set.insert(name, ours);
                    }
                }
            }
        }
    }
    let upstream = dirs.iter().find(|d| holds(d)).map(|d| d.as_path());
    for (file, text) in &set {
        for base in said_inherits(text) {
            if !spec.inherits.contains(&base) && !sibling(upstream, &base) {
                return Err(format!(
                    "{file} says `; inherits: {base}`: name it in grammar.toml's [queries] inherits"
                ));
            }
        }
    }
    Ok(set)
}

/// `text`, file `name` of the checkout's directory `dir`, with the same
/// file of each directory beside `dir` it says it inherits in front —
/// a set of queries several grammars of one repository share that is
/// no grammar itself (vue's `html_tags`), theirs first in turn. One it
/// says that is no such directory is left to `[queries] inherits`.
fn with_siblings(text: &str, name: &str, dir: &Path, spec: &Spec) -> Result<String, String> {
    fn go(
        text: &str,
        name: &str,
        dir: &Path,
        spec: &Spec,
        path: &mut Vec<String>,
    ) -> Result<String, String> {
        let mut out = String::new();
        for base in said_inherits(text) {
            let sibling = dir.parent().map(|p| p.join(&base));
            let Some(sibling) = sibling.filter(|s| s.is_dir() && !spec.inherits.contains(&base))
            else {
                continue;
            };
            if path.contains(&base) {
                return Err(format!("{name}: queries inherit in a circle at `{base}`"));
            }
            let file = sibling.join(name);
            if file.is_file() {
                path.push(base.clone());
                out.push_str(&go(&read(&file)?, name, &sibling, spec, path)?);
                path.pop();
                if !out.ends_with('\n') {
                    out.push('\n');
                }
            }
        }
        out.push_str(text);
        Ok(out)
    }
    go(text, name, dir, spec, &mut Vec::new())
}

/// Whether a query file says `; inherits:` a directory beside `dir` in
/// the checkout — resolved there ([`with_siblings`]), not a grammar.
fn sibling(dir: Option<&Path>, base: &str) -> bool {
    dir.and_then(Path::parent)
        .is_some_and(|p| p.join(base).is_dir())
}

/// Whether a query file says `; extends` — nvim's word for it — in the
/// comments it opens with, before its first pattern.
fn extends(text: &str) -> bool {
    text.lines()
        .map(str::trim)
        .take_while(|l| l.is_empty() || l.starts_with(';'))
        .any(|l| l.trim_start_matches(';').trim() == "extends")
}

fn read(path: &Path) -> Result<String, String> {
    std::fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))
}

/// The names an nvim-style `; inherits: a,b` comment gives.
fn said_inherits(text: &str) -> Vec<String> {
    text.lines()
        .filter_map(|l| l.trim().strip_prefix(';')?.trim().strip_prefix("inherits:"))
        .flat_map(|names| names.split(','))
        .map(|n| {
            n.trim()
                .trim_start_matches('(')
                .trim_end_matches(')')
                .to_string()
        })
        .filter(|n| !n.is_empty())
        .collect()
}

/// `name`'s queries with its inherited grammars' in front, theirs with
/// their own inherited first. A file only a base has is inherited
/// whole.
pub fn whole(
    name: &str,
    specs: &BTreeMap<String, Spec>,
    owns: &BTreeMap<String, Set>,
) -> Result<Set, String> {
    fn go(
        name: &str,
        specs: &BTreeMap<String, Spec>,
        owns: &BTreeMap<String, Set>,
        path: &mut Vec<String>,
    ) -> Result<Set, String> {
        if path.iter().any(|p| p == name) {
            path.push(name.to_string());
            return Err(format!("queries inherit in a circle: {}", path.join(" → ")));
        }
        path.push(name.to_string());
        let mut set = Set::new();
        for base in &specs[name].inherits {
            for (file, text) in go(base, specs, owns, path)? {
                let slot = set.entry(file).or_default();
                slot.push_str(&text);
                if !slot.ends_with('\n') {
                    slot.push('\n');
                }
            }
        }
        for (file, text) in &owns[name] {
            set.entry(file.clone()).or_default().push_str(text);
        }
        path.pop();
        Ok(set)
    }
    go(name, specs, owns, &mut Vec::new())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn spec(name: &str, dir: &Path, inherits: &[&str]) -> Spec {
        let rev = "0123456789abcdef0123456789abcdef01234567";
        let mut s = Spec::parse(
            name,
            dir,
            &format!("extensions = [\"{name}\"]\n[source]\nrepo = \"https://x\"\nrev = \"{rev}\"\nlicense = \"MIT\"\n"),
        )
        .unwrap();
        s.inherits = inherits.iter().map(|s| s.to_string()).collect();
        s
    }

    fn temp(name: &str) -> std::path::PathBuf {
        let dir =
            std::env::temp_dir().join(format!("kawoosh-grammars-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    fn put(path: &Path, text: &str) {
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(path, text).unwrap();
    }

    #[test]
    fn the_repository_s_queries_lie_over_the_checkout_s() {
        let t = temp("own");
        let (checkout, dir) = (t.join("checkout"), t.join("grammars/zig"));
        put(
            &checkout.join("queries/highlights.scm"),
            "upstream highlights",
        );
        put(
            &checkout.join("queries/injections.scm"),
            "upstream injections",
        );
        put(&checkout.join("queries/indents.scm"), "nvim's dialect");
        put(&checkout.join("queries/locals.scm"), "not read");
        put(&dir.join("queries/highlights.scm"), "ours");
        put(&dir.join("queries/indents.scm"), "helix's dialect");
        let mut skipping = spec("zig", &dir, &[]);
        skipping.skip = vec!["injections.scm".into()];
        assert!(
            !own(&checkout, &skipping)
                .unwrap()
                .contains_key("injections.scm")
        );
        let set = own(&checkout, &spec("zig", &dir, &[])).unwrap();
        assert_eq!(
            set,
            Set::from([
                ("highlights.scm".into(), "ours".into()),
                ("injections.scm".into(), "upstream injections".into()),
                ("indents.scm".into(), "helix's dialect".into()),
            ])
        );
        std::fs::remove_dir_all(t).unwrap();
    }

    #[test]
    fn a_file_that_says_extends_goes_after_the_checkout_s() {
        let t = temp("extends");
        let (checkout, dir) = (t.join("checkout"), t.join("grammars/zig"));
        put(
            &checkout.join("queries/highlights.scm"),
            "(identifier) @variable",
        );
        put(&checkout.join("queries/tags.scm"), "theirs");
        put(
            &dir.join("queries/highlights.scm"),
            ";; A fix or two.\n; extends\n\n(type_identifier) @type\n",
        );
        // Said after a pattern, it is a comment like any other.
        put(&dir.join("queries/tags.scm"), "(x) @name\n; extends\n");
        // With nothing of the checkout's to go after, it is the file.
        put(
            &dir.join("queries/indents.scm"),
            "; extends\n(block) @indent\n",
        );
        let set = own(&checkout, &spec("zig", &dir, &[])).unwrap();
        assert_eq!(
            set["highlights.scm"],
            "(identifier) @variable\n;; A fix or two.\n; extends\n\n(type_identifier) @type\n"
        );
        assert_eq!(set["tags.scm"], "(x) @name\n; extends\n");
        assert_eq!(set["indents.scm"], "; extends\n(block) @indent\n");
        std::fs::remove_dir_all(t).unwrap();
    }

    #[test]
    fn a_grammar_in_a_subdirectory_has_its_queries_there() {
        let t = temp("sub");
        let checkout = t.join("checkout");
        put(&checkout.join("queries/highlights.scm"), "the root's");
        put(&checkout.join("tsx/queries/highlights.scm"), "tsx's");
        let mut s = spec("tsx", &t.join("grammars/tsx"), &[]);
        s.source.path = "tsx".into();
        assert_eq!(own(&checkout, &s).unwrap()["highlights.scm"], "tsx's");
        std::fs::remove_dir_all(t).unwrap();
    }

    #[test]
    fn nvim_s_layout_and_a_shared_set_of_queries() {
        let t = temp("nvim");
        let checkout = t.join("checkout");
        put(&checkout.join("src/parser.c"), "");
        put(
            &checkout.join("queries/vue/highlights.scm"),
            "; inherits: html_tags\n(interpolation) @punctuation",
        );
        put(
            &checkout.join("queries/vue/injections.scm"),
            "(raw_text) @injection.content",
        );
        put(
            &checkout.join("queries/html_tags/highlights.scm"),
            "(tag_name) @tag",
        );
        let dir = t.join("grammars/vue");
        let set = own(&checkout, &spec("vue", &dir, &[])).unwrap();
        assert_eq!(
            set["highlights.scm"],
            "(tag_name) @tag\n; inherits: html_tags\n(interpolation) @punctuation"
        );
        assert_eq!(set["injections.scm"], "(raw_text) @injection.content");
        // A base that is neither a directory there nor a grammar named.
        put(
            &checkout.join("queries/vue/tags.scm"),
            "; inherits: go\n(x) @name",
        );
        assert!(
            own(&checkout, &spec("vue", &dir, &[]))
                .unwrap_err()
                .contains("inherits: go")
        );
        std::fs::remove_dir_all(t).unwrap();
    }

    #[test]
    fn a_grammar_in_a_subdirectory_may_keep_its_queries_under_the_root_s() {
        let t = temp("under");
        let checkout = t.join("checkout");
        put(&checkout.join("xml/src/parser.c"), "");
        put(&checkout.join("queries/xml/highlights.scm"), "xml's");
        put(&checkout.join("queries/dtd/highlights.scm"), "dtd's");
        let mut s = spec("xml", &t.join("grammars/xml"), &[]);
        s.source.path = "xml".into();
        assert_eq!(own(&checkout, &s).unwrap()["highlights.scm"], "xml's");
        std::fs::remove_dir_all(t).unwrap();
    }

    #[test]
    fn an_inherits_comment_must_be_said_in_the_toml() {
        let t = temp("says");
        let checkout = t.join("checkout");
        put(
            &checkout.join("queries/highlights.scm"),
            "; inherits: ecma,jsx\n(x) @y",
        );
        let dir = t.join("grammars/js");
        let err = own(&checkout, &spec("js", &dir, &["ecma"])).unwrap_err();
        assert!(err.contains("`; inherits: jsx`"), "{err}");
        assert!(own(&checkout, &spec("js", &dir, &["ecma", "jsx"])).is_ok());
        std::fs::remove_dir_all(t).unwrap();
    }

    #[test]
    fn inherited_queries_go_in_front_file_by_file() {
        let dir = Path::new("grammars");
        let specs = BTreeMap::from([
            ("ecma".to_string(), spec("ecma", dir, &[])),
            ("js".to_string(), spec("js", dir, &["ecma"])),
            ("ts".to_string(), spec("ts", dir, &["js"])),
        ]);
        let set = |pairs: &[(&str, &str)]| -> Set {
            pairs
                .iter()
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .collect()
        };
        let owns = BTreeMap::from([
            (
                "ecma".to_string(),
                set(&[
                    ("highlights.scm", "ecma"),
                    ("indents.scm", "ecma indents\n"),
                ]),
            ),
            ("js".to_string(), set(&[("highlights.scm", "js\n")])),
            (
                "ts".to_string(),
                set(&[("highlights.scm", "ts"), ("tags.scm", "ts tags")]),
            ),
        ]);
        let ts = whole("ts", &specs, &owns).unwrap();
        assert_eq!(ts["highlights.scm"], "ecma\njs\nts");
        assert_eq!(ts["indents.scm"], "ecma indents\n");
        assert_eq!(ts["tags.scm"], "ts tags");
        assert_eq!(whole("ecma", &specs, &owns).unwrap(), owns["ecma"]);
    }

    #[test]
    fn a_circle_of_inherits_is_said() {
        let dir = Path::new("grammars");
        let specs = BTreeMap::from([
            ("a".to_string(), spec("a", dir, &["b"])),
            ("b".to_string(), spec("b", dir, &["a"])),
        ]);
        let owns = BTreeMap::from([("a".to_string(), Set::new()), ("b".to_string(), Set::new())]);
        assert_eq!(
            whole("a", &specs, &owns).unwrap_err(),
            "queries inherit in a circle: a → b → a"
        );
    }
}
