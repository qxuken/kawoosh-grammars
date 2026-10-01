//! A grammar's queries as its archive holds them: the checkout's own,
//! the repository's laid over them file by file, and in front of each
//! the same file of every grammar it inherits — so what is shipped is
//! whole and nobody downstream resolves an `; inherits:`.

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
/// grammar, else at the root), then every `.scm` of the repository's
/// `queries/`, which replaces the checkout's of that name.
pub fn own(checkout: &Path, spec: &Spec) -> Result<Set, String> {
    let mut set = Set::new();
    let dirs = [
        checkout.join(&spec.source.path).join("queries"),
        checkout.join("queries"),
    ];
    if let Some(dir) = dirs.iter().find(|d| d.is_dir()) {
        for name in UPSTREAM {
            let file = dir.join(name);
            if file.is_file() && !spec.skip.iter().any(|s| s == name) {
                set.insert(name.to_string(), read(&file)?);
            }
        }
    }
    let over = spec.dir.join("queries");
    if over.is_dir() {
        for entry in std::fs::read_dir(&over).map_err(|e| format!("{}: {e}", over.display()))? {
            let path = entry.map_err(|e| e.to_string())?.path();
            if path.extension().is_some_and(|e| e == "scm") {
                let name = path.file_name().unwrap().to_string_lossy().into_owned();
                set.insert(name, read(&path)?);
            }
        }
    }
    for (file, text) in &set {
        for base in said_inherits(text) {
            if !spec.inherits.contains(&base) {
                return Err(format!(
                    "{file} says `; inherits: {base}`: name it in grammar.toml's [queries] inherits"
                ));
            }
        }
    }
    Ok(set)
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
