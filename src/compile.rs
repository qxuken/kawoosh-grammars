//! The libraries: a grammar's `parser.c`, and its `scanner.c` if it
//! has one, built for every target from whatever machine this runs on.
//! zig is the compiler because it is every target's at once, with each
//! target's libc in it.

use std::path::{Path, PathBuf};
use std::process::Command;

use crate::source::run;

/// The zig the releases are built with; `.zig-version` at the root.
pub const ZIG_VERSION: &str = include_str!("../.zig-version");

pub struct Target {
    /// The name in an archive (`lib/NAME.EXT`) and in the manifest.
    pub name: &'static str,
    /// What zig calls it. Linux names the oldest glibc the library
    /// loads under.
    pub zig: &'static str,
    pub ext: &'static str,
}

pub const TARGETS: [Target; 6] = [
    Target {
        name: "aarch64-linux",
        zig: "aarch64-linux-gnu.2.17",
        ext: "so",
    },
    Target {
        name: "aarch64-macos",
        zig: "aarch64-macos",
        ext: "dylib",
    },
    Target {
        name: "aarch64-windows",
        zig: "aarch64-windows-gnu",
        ext: "dll",
    },
    Target {
        name: "x86_64-linux",
        zig: "x86_64-linux-gnu.2.17",
        ext: "so",
    },
    Target {
        name: "x86_64-macos",
        zig: "x86_64-macos",
        ext: "dylib",
    },
    Target {
        name: "x86_64-windows",
        zig: "x86_64-windows-gnu",
        ext: "dll",
    },
];

/// The target this machine is, if it is one of them: whose library the
/// check loads.
pub fn host() -> Option<&'static Target> {
    let name = format!("{}-{}", std::env::consts::ARCH, std::env::consts::OS);
    TARGETS.iter().find(|t| t.name == name)
}

pub struct Zig {
    argv: Vec<String>,
    pub version: String,
}

impl Zig {
    /// `$ZIG` — a command line, so `python3 -m ziglang` will do — else
    /// `zig` on the `PATH`.
    pub fn find() -> Result<Zig, String> {
        let said = std::env::var("ZIG").unwrap_or_else(|_| "zig".into());
        let argv: Vec<String> = said.split_whitespace().map(str::to_string).collect();
        if argv.is_empty() {
            return Err("$ZIG is empty".into());
        }
        let mut zig = Zig {
            argv,
            version: String::new(),
        };
        zig.version = run(zig.command().arg("version"))
            .map_err(|e| {
                format!(
                    "no zig ({e}); install zig {} or set $ZIG",
                    ZIG_VERSION.trim()
                )
            })?
            .trim()
            .to_string();
        Ok(zig)
    }

    fn command(&self) -> Command {
        let mut c = Command::new(&self.argv[0]);
        c.args(&self.argv[1..]);
        c
    }

    /// Builds `sources` into the shared library `out` for `target`, as
    /// `tree-sitter build` would: C11, optimised, nothing but the
    /// grammar's own headers.
    pub fn build(&self, sources: &[PathBuf], target: &Target, out: &Path) -> Result<(), String> {
        let include = sources[0].parent().unwrap();
        if let Some(dir) = out.parent() {
            std::fs::create_dir_all(dir).map_err(|e| format!("{}: {e}", dir.display()))?;
        }
        let mut c = self.command();
        c.args([
            "cc", "-target", target.zig, "-shared", "-fPIC", "-O2", "-s", "-std=c11", "-I",
        ])
        .arg(include)
        .args(sources)
        .arg("-o")
        .arg(out);
        run(&mut c)
            .map(|_| ())
            .map_err(|e| format!("{}: {e}", target.name))
    }
}

/// What is compiled: `src/parser.c`, and `src/scanner.c` beside it. A
/// grammar that commits no `parser.c`, or whose scanner is C++, is
/// refused: it waits for its upstream.
pub fn sources(grammar: &Path) -> Result<Vec<PathBuf>, String> {
    let src = grammar.join("src");
    let parser = src.join("parser.c");
    if !parser.is_file() {
        return Err(format!(
            "no {} in the checkout: the grammar commits no generated parser",
            Path::new("src").join("parser.c").display()
        ));
    }
    for cxx in ["scanner.cc", "scanner.cpp", "scanner.cxx"] {
        if src.join(cxx).is_file() {
            return Err(format!("its scanner is C++ (src/{cxx})"));
        }
    }
    let mut sources = vec![parser];
    let scanner = src.join("scanner.c");
    if scanner.is_file() {
        sources.push(scanner);
    }
    Ok(sources)
}
