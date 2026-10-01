//! A grammar as the repository holds it: `grammars/NAME/grammar.toml`,
//! a sample beside it, and the query files it changes. The directory's
//! name is the language's; the file's words are the manifest's and
//! `kawoosh.language`'s.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use serde::Deserialize;

/// `grammar.toml`, as written.
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct File {
    #[serde(default)]
    extensions: Vec<String>,
    #[serde(default)]
    filenames: Vec<String>,
    #[serde(default)]
    shebangs: Vec<String>,
    #[serde(default)]
    aliases: Vec<String>,
    /// The library's symbol, when it is not `tree_sitter_NAME`.
    symbol: Option<String>,
    source: Source,
    #[serde(default)]
    queries: Queries,
}

/// Where the grammar's source is, and under what terms.
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Source {
    pub repo: String,
    /// A commit, whole: a branch or a tag moves.
    pub rev: String,
    /// The directory holding `src/parser.c`, from the checkout's root.
    #[serde(default = "dot")]
    pub path: String,
    /// SPDX. The text itself is copied from the checkout.
    pub license: String,
}

#[derive(Clone, Debug, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Queries {
    /// Grammars of this repository whose queries go in front of this
    /// one's, file by file.
    #[serde(default)]
    pub inherits: Vec<String>,
    /// Query files of the checkout that are not taken: one that does
    /// not compile, or says what this repository would rather not.
    #[serde(default)]
    pub skip: Vec<String>,
}

fn dot() -> String {
    ".".into()
}

#[derive(Clone, Debug)]
pub struct Spec {
    pub name: String,
    /// `grammars/NAME`.
    pub dir: PathBuf,
    pub extensions: Vec<String>,
    pub filenames: Vec<String>,
    pub shebangs: Vec<String>,
    pub aliases: Vec<String>,
    pub symbol: String,
    pub source: Source,
    pub inherits: Vec<String>,
    pub skip: Vec<String>,
}

impl Spec {
    pub fn parse(name: &str, dir: &Path, text: &str) -> Result<Spec, String> {
        let f: File = toml::from_str(text).map_err(|e| e.to_string())?;
        let spec = Spec {
            name: name.to_string(),
            dir: dir.to_path_buf(),
            extensions: f.extensions,
            filenames: f.filenames,
            shebangs: f.shebangs,
            aliases: f.aliases,
            symbol: f
                .symbol
                .unwrap_or_else(|| format!("tree_sitter_{}", name.replace('-', "_"))),
            source: f.source,
            inherits: f.queries.inherits,
            skip: f.queries.skip,
        };
        spec.validate()?;
        Ok(spec)
    }

    fn validate(&self) -> Result<(), String> {
        let word = |s: &str| {
            !s.is_empty()
                && s.bytes()
                    .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'_' || b == b'-')
        };
        if !word(&self.name) {
            return Err(format!(
                "the name `{}` is not lowercase letters, digits, `_` and `-`",
                self.name
            ));
        }
        let s = &self.source;
        if s.rev.len() != 40 || !s.rev.bytes().all(|b| b.is_ascii_hexdigit()) {
            return Err(format!("rev `{}` is not a whole commit (40 hex)", s.rev));
        }
        if !s.repo.starts_with("https://") {
            return Err(format!("repo `{}` is not an https URL", s.repo));
        }
        let path = Path::new(&s.path);
        if path.is_absolute() || path.components().any(|c| c.as_os_str() == "..") {
            return Err(format!("path `{}` leaves the checkout", s.path));
        }
        if s.license.trim().is_empty() {
            return Err("license is empty".into());
        }
        for e in &self.extensions {
            if e.starts_with('.') || *e != e.to_ascii_lowercase() {
                return Err(format!("extension `{e}`: lowercase, without the dot"));
            }
        }
        if self.extensions.is_empty() && self.filenames.is_empty() && self.shebangs.is_empty() {
            return Err("no extensions, filenames or shebangs: no file is this language".into());
        }
        Ok(())
    }

    /// The one `sample.*` beside the `grammar.toml`.
    pub fn sample(&self) -> Result<PathBuf, String> {
        let mut found: Vec<PathBuf> = std::fs::read_dir(&self.dir)
            .map_err(|e| format!("{}: {e}", self.dir.display()))?
            .filter_map(|e| e.ok().map(|e| e.path()))
            .filter(|p| p.file_stem().is_some_and(|s| s == "sample") && p.is_file())
            .collect();
        match found.len() {
            1 => Ok(found.remove(0)),
            0 => Err("no sample.* beside grammar.toml".into()),
            _ => Err("more than one sample.*".into()),
        }
    }
}

/// Every grammar under `root/grammars`, by name. The cross-checks are
/// here: an inherited name is a grammar, and no two claim one file.
pub fn load_all(root: &Path) -> Result<BTreeMap<String, Spec>, String> {
    let dir = root.join("grammars");
    let mut specs = BTreeMap::new();
    let entries = std::fs::read_dir(&dir).map_err(|e| format!("{}: {e}", dir.display()))?;
    for entry in entries {
        let path = entry.map_err(|e| e.to_string())?.path();
        let file = path.join("grammar.toml");
        if !file.is_file() {
            continue;
        }
        let name = path.file_name().unwrap().to_string_lossy().into_owned();
        let text =
            std::fs::read_to_string(&file).map_err(|e| format!("{}: {e}", file.display()))?;
        let spec = Spec::parse(&name, &path, &text).map_err(|e| format!("{name}: {e}"))?;
        specs.insert(name, spec);
    }
    cross_check(&specs)?;
    Ok(specs)
}

fn cross_check(specs: &BTreeMap<String, Spec>) -> Result<(), String> {
    let mut claimed: BTreeMap<String, &str> = BTreeMap::new();
    for spec in specs.values() {
        for base in &spec.inherits {
            if !specs.contains_key(base) {
                return Err(format!(
                    "{}: inherits `{base}`, which is no grammar here",
                    spec.name
                ));
            }
        }
        for a in &spec.aliases {
            if specs.contains_key(a) {
                return Err(format!(
                    "{}: the alias `{a}` is a grammar's name",
                    spec.name
                ));
            }
        }
        let claims = (spec
            .extensions
            .iter()
            .map(|e| format!("the extension `{e}`")))
        .chain(
            spec.filenames
                .iter()
                .map(|f| format!("the file name `{f}`")),
        )
        .chain(
            spec.shebangs
                .iter()
                .map(|s| format!("the interpreter `{s}`")),
        )
        .chain(spec.aliases.iter().map(|a| format!("the alias `{a}`")));
        for what in claims {
            if let Some(other) = claimed.insert(what.clone(), &spec.name)
                && other != spec.name
            {
                return Err(format!("{} and {other} both claim {what}", spec.name));
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const REV: &str = "0123456789abcdef0123456789abcdef01234567";

    fn toml(extra: &str, source_extra: &str) -> String {
        format!(
            "extensions = [\"zig\"]\n{extra}\n[source]\nrepo = \"https://example.com/x\"\nrev = \"{REV}\"\nlicense = \"MIT\"\n{source_extra}"
        )
    }

    fn parse(name: &str, text: &str) -> Result<Spec, String> {
        Spec::parse(name, Path::new("grammars").join(name).as_path(), text)
    }

    #[test]
    fn a_spec_fills_in_what_is_not_said() {
        let s = parse("zig", &toml("", "")).unwrap();
        assert_eq!(s.symbol, "tree_sitter_zig");
        assert_eq!(s.source.path, ".");
        assert!(s.inherits.is_empty() && s.filenames.is_empty());
        let s = parse(
            "c-sharp",
            &toml("symbol = \"tree_sitter_c_sharp\"", "path = \"x\""),
        )
        .unwrap();
        assert_eq!(
            (s.symbol.as_str(), s.source.path.as_str()),
            ("tree_sitter_c_sharp", "x")
        );
        assert_eq!(
            parse("c-sharp", &toml("", "")).unwrap().symbol,
            "tree_sitter_c_sharp"
        );
    }

    #[test]
    fn a_spec_that_is_wrong_says_how() {
        let err = |name: &str, text: &str| parse(name, text).unwrap_err();
        assert!(err("Zig", &toml("", "")).contains("lowercase"));
        assert!(err("zig", &toml("", "").replace(REV, "main")).contains("whole commit"));
        assert!(err("zig", &toml("", "").replace("https://", "git@")).contains("https"));
        assert!(err("zig", &toml("", "path = \"../x\"")).contains("leaves the checkout"));
        assert!(
            err("zig", &toml("", "").replace("\"zig\"", "\".zig\"")).contains("without the dot")
        );
        assert!(err("zig", &toml("", "").replace("[\"zig\"]", "[]")).contains("no file is this"));
        // A word the file does not have is a typo, not an extension.
        assert!(err("zig", &toml("extentions = []", "")).contains("unknown field"));
    }

    #[test]
    fn two_grammars_do_not_claim_one_file() {
        let a = parse("a", &toml("", "")).unwrap();
        let mut b = parse("b", &toml("", "")).unwrap();
        let both = |a: &Spec, b: &Spec| {
            cross_check(&BTreeMap::from([
                ("a".to_string(), a.clone()),
                ("b".to_string(), b.clone()),
            ]))
        };
        assert!(
            both(&a, &b)
                .unwrap_err()
                .contains("both claim the extension `zig`")
        );
        b.extensions = vec!["zag".into()];
        assert_eq!(both(&a, &b), Ok(()));
        b.inherits = vec!["c".into()];
        assert!(both(&a, &b).unwrap_err().contains("inherits `c`"));
        b.inherits = vec!["a".into()];
        b.aliases = vec!["a".into()];
        assert!(both(&a, &b).unwrap_err().contains("is a grammar's name"));
    }
}
