//! The builder: every grammar under `grammars/` fetched at its
//! revision, built for every target with zig, its queries made whole,
//! checked with this machine's library, and written as one archive —
//! and the manifest that names them all, which is what kawoosh reads.
//!
//!   kawoosh-grammars build [NAME…] [--out DIR] [--work DIR] [--any-zig]
//!   kawoosh-grammars check [NAME…] [--work DIR] [--any-zig]
//!   kawoosh-grammars list
//!
//! `build` with names builds those (and fetches what they inherit); the
//! manifest then lists only them, so a release is a build with none.
//! `check` is the build's checks with this machine's library alone and
//! nothing written: what adding or moving a grammar is tried with.

mod archive;
mod check;
mod compile;
mod queries;
mod source;
mod spec;

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::time::Instant;

use serde::Serialize;

use compile::{TARGETS, Zig};
use spec::Spec;

/// `manifest.json`: what there is to install and how to know a file is
/// one of theirs. A base URL holds it beside the archives it names.
#[derive(Serialize)]
struct Manifest {
    /// Of this file's shape; a reader that knows a lower one stops.
    format: u32,
    /// Every archive holds `lib/TARGET.EXT` for each.
    targets: Vec<&'static str>,
    grammars: BTreeMap<String, Row>,
}

#[derive(Serialize)]
struct Row {
    extensions: Vec<String>,
    filenames: Vec<String>,
    shebangs: Vec<String>,
    aliases: Vec<String>,
    /// The comment tokens, as `grammar.toml` says them; left out where
    /// the language has none.
    #[serde(skip_serializing_if = "Option::is_none")]
    comment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    comment_block: Option<Vec<String>>,
    /// How its files indent, likewise: `"tab"` or `"space"`, and the
    /// width.
    #[serde(skip_serializing_if = "Option::is_none")]
    indent_style: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    indent_size: Option<u32>,
    repo: String,
    rev: String,
    path: String,
    license: String,
    symbol: String,
    abi: usize,
    /// The query files the archive holds under `queries/`.
    queries: Vec<String>,
    archive: String,
    size: u64,
    blake3: String,
}

struct Args {
    names: Vec<String>,
    out: PathBuf,
    work: PathBuf,
    any_zig: bool,
    /// `check`: this machine's library, the checks, and no archive.
    check: bool,
}

fn main() -> ExitCode {
    let mut argv = std::env::args().skip(1);
    let result = match argv.next().as_deref() {
        Some("build") => parse(argv, false).and_then(build),
        Some("check") => parse(argv, true).and_then(build),
        Some("list") => list(),
        _ => {
            eprintln!("usage: kawoosh-grammars build [NAME…] [--out DIR] [--work DIR] [--any-zig]");
            eprintln!("       kawoosh-grammars check [NAME…] [--work DIR] [--any-zig]");
            eprintln!("       kawoosh-grammars list");
            return ExitCode::from(2);
        }
    };
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("{e}");
            ExitCode::FAILURE
        }
    }
}

fn parse(mut argv: impl Iterator<Item = String>, check: bool) -> Result<Args, String> {
    let mut args = Args {
        names: Vec::new(),
        out: PathBuf::from("dist"),
        work: PathBuf::from("work"),
        any_zig: false,
        check,
    };
    while let Some(a) = argv.next() {
        let mut value = |flag: &str| {
            argv.next()
                .ok_or_else(|| format!("{flag} takes a directory"))
        };
        match a.as_str() {
            "--out" => args.out = PathBuf::from(value("--out")?),
            "--work" => args.work = PathBuf::from(value("--work")?),
            "--any-zig" => args.any_zig = true,
            flag if flag.starts_with('-') => return Err(format!("no flag {flag}")),
            _ => args.names.push(a),
        }
    }
    Ok(args)
}

fn list() -> Result<(), String> {
    for spec in spec::load_all(Path::new("."))?.values() {
        println!(
            "{:16} {}  {}",
            spec.name,
            &spec.source.rev[..12],
            spec.source.repo
        );
    }
    Ok(())
}

fn build(args: Args) -> Result<(), String> {
    let specs = spec::load_all(Path::new("."))?;
    for name in &args.names {
        if !specs.contains_key(name) {
            return Err(format!("no grammar {name} under grammars/"));
        }
    }
    let chosen: Vec<&Spec> = specs
        .values()
        .filter(|s| args.names.is_empty() || args.names.contains(&s.name))
        .collect();

    let zig = Zig::find()?;
    let pinned = compile::ZIG_VERSION.trim();
    if zig.version != pinned && !args.any_zig {
        return Err(format!(
            "zig {} here, releases are built with {pinned} (.zig-version); --any-zig builds anyway",
            zig.version
        ));
    }
    let host = compile::host().ok_or_else(|| {
        format!(
            "{}-{} is none of the targets, so there is no library to check with",
            std::env::consts::ARCH,
            std::env::consts::OS
        )
    })?;

    // What is built, and what it inherits, fetched: a grammar's queries
    // are made of its bases'.
    let mut needed: Vec<&Spec> = Vec::new();
    let mut stack: Vec<&Spec> = chosen.clone();
    while let Some(spec) = stack.pop() {
        if !needed.iter().any(|s| s.name == spec.name) {
            needed.push(spec);
            stack.extend(spec.inherits.iter().map(|b| &specs[b]));
        }
    }
    let mut failed: Vec<String> = Vec::new();
    let mut checkouts: BTreeMap<String, PathBuf> = BTreeMap::new();
    let mut owns: BTreeMap<String, queries::Set> = BTreeMap::new();
    for spec in &needed {
        let fetched = source::fetch(&args.work, spec)
            .and_then(|checkout| Ok((queries::own(&checkout, spec)?, checkout)));
        match fetched {
            Ok((own, checkout)) => {
                owns.insert(spec.name.clone(), own);
                checkouts.insert(spec.name.clone(), checkout);
            }
            Err(e) => failed.push(format!("{}: {e}", spec.name)),
        }
    }
    if !failed.is_empty() {
        return Err(failed.join("\n"));
    }

    if !args.check {
        std::fs::create_dir_all(&args.out).map_err(|e| format!("{}: {e}", args.out.display()))?;
    }
    let mut grammars = BTreeMap::new();
    for spec in &chosen {
        let started = Instant::now();
        let row = queries::whole(&spec.name, &specs, &owns)
            .and_then(|queries| one(spec, &checkouts[&spec.name], &queries, &zig, host, &args));
        match row {
            Ok(None) => println!(
                "{:16} ok  {:.1}s",
                spec.name,
                started.elapsed().as_secs_f64()
            ),
            Ok(Some(row)) => {
                println!(
                    "{:16} {:>5} KiB  abi {}  {:.1}s",
                    spec.name,
                    row.size / 1024,
                    row.abi,
                    started.elapsed().as_secs_f64()
                );
                grammars.insert(spec.name.clone(), row);
            }
            Err(e) => {
                println!("{:16} failed", spec.name);
                failed.push(format!("{}: {e}", spec.name));
            }
        }
    }
    if !failed.is_empty() {
        return Err(failed.join("\n"));
    }
    if args.check {
        return Ok(());
    }

    let manifest = Manifest {
        format: 1,
        targets: TARGETS.iter().map(|t| t.name).collect(),
        grammars,
    };
    let path = args.out.join("manifest.json");
    let mut text = serde_json::to_string_pretty(&manifest).map_err(|e| e.to_string())?;
    text.push('\n');
    std::fs::write(&path, text).map_err(|e| format!("{}: {e}", path.display()))?;
    println!(
        "{} grammars in {}",
        manifest.grammars.len(),
        args.out.display()
    );
    Ok(())
}

/// One grammar: its libraries built, the host's checked, the archive
/// written, and its row of the manifest — or, for `check`, the host's
/// library alone, checked, and no row.
fn one(
    spec: &Spec,
    checkout: &Path,
    queries: &queries::Set,
    zig: &Zig,
    host: &compile::Target,
    args: &Args,
) -> Result<Option<Row>, String> {
    let sources = compile::sources(&checkout.join(&spec.source.path))?;
    let licenses = source::licenses(checkout, &spec.source.path)?;
    let targets: Vec<&compile::Target> = TARGETS
        .iter()
        .filter(|t| !args.check || t.name == host.name)
        .collect();

    let dir = args.work.join("build").join(&spec.name);
    let _ = std::fs::remove_dir_all(&dir);
    let lib_at = |t: &compile::Target| dir.join(t.name).join(format!("parser.{}", t.ext));
    let built: Vec<Result<(), String>> = std::thread::scope(|scope| {
        let handles: Vec<_> = targets
            .iter()
            .map(|t| scope.spawn(|| zig.build(&sources, t, &lib_at(t))))
            .collect();
        handles
            .into_iter()
            .map(|h| h.join().expect("a build thread panicked"))
            .collect()
    });
    let errors: Vec<String> = built.into_iter().filter_map(Result::err).collect();
    if !errors.is_empty() {
        return Err(errors.join("\n"));
    }

    // The queries first: a grammar with no sample yet is still told
    // whether its queries hold.
    let language = check::grammar(&lib_at(host), &spec.symbol, queries)?;
    let abi = language.abi_version();
    check::sample(&language, &spec.sample()?)?;
    if args.check {
        return Ok(None);
    }

    let mut files: Vec<(String, Vec<u8>)> = Vec::new();
    for t in &TARGETS {
        let path = lib_at(t);
        let data = std::fs::read(&path).map_err(|e| format!("{}: {e}", path.display()))?;
        files.push((format!("lib/{}.{}", t.name, t.ext), data));
    }
    for (file, text) in queries {
        files.push((format!("queries/{file}"), text.clone().into_bytes()));
    }
    files.extend(licenses);

    let archive = format!("{}.sqlar", spec.name);
    let path = args.out.join(&archive);
    let meta = [
        ("name", spec.name.clone()),
        ("repo", spec.source.repo.clone()),
        ("rev", spec.source.rev.clone()),
        ("path", spec.source.path.clone()),
        ("license", spec.source.license.clone()),
        ("symbol", spec.symbol.clone()),
        ("abi", abi.to_string()),
        ("zig", zig.version.clone()),
    ];
    archive::write(&path, &files, &meta)?;
    // What was checked is what is in the archive.
    let (host_name, host_lib) = &files[TARGETS.iter().position(|t| t.name == host.name).unwrap()];
    if archive::read(&path, host_name)?.as_ref() != Some(host_lib) {
        return Err(format!(
            "{archive}: {host_name} read back is not what was built"
        ));
    }
    let bytes = std::fs::read(&path).map_err(|e| format!("{}: {e}", path.display()))?;

    Ok(Some(Row {
        extensions: spec.extensions.clone(),
        filenames: spec.filenames.clone(),
        shebangs: spec.shebangs.clone(),
        aliases: spec.aliases.clone(),
        comment: spec.comment.clone(),
        comment_block: spec.comment_block.clone(),
        indent_style: spec.indent_style.clone(),
        indent_size: spec.indent_size,
        repo: spec.source.repo.clone(),
        rev: spec.source.rev.clone(),
        path: spec.source.path.clone(),
        license: spec.source.license.clone(),
        symbol: spec.symbol.clone(),
        abi,
        queries: queries.keys().cloned().collect(),
        archive,
        size: bytes.len() as u64,
        blake3: blake3::hash(&bytes).to_hex().to_string(),
    }))
}
