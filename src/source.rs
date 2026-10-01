//! A grammar's source at its revision, fetched with git into the work
//! directory and kept there: a build that finds the checkout at the
//! revision fetches nothing.

use std::path::{Path, PathBuf};
use std::process::Command;

use crate::spec::Spec;

/// Runs `cmd`, answering its stdout, or what it said on stderr.
pub fn run(cmd: &mut Command) -> Result<String, String> {
    let shown = format!("{cmd:?}");
    let out = cmd.output().map_err(|e| format!("{shown}: {e}"))?;
    if !out.status.success() {
        let err = String::from_utf8_lossy(&out.stderr);
        return Err(format!("{shown}: {}", err.trim()));
    }
    Ok(String::from_utf8_lossy(&out.stdout).into_owned())
}

fn git(dir: &Path) -> Command {
    let mut c = Command::new("git");
    c.arg("-C").arg(dir);
    c
}

/// The checkout of `spec`'s repository at its revision, under
/// `work/src`. One commit is fetched, by its hash, with no history.
pub fn fetch(work: &Path, spec: &Spec) -> Result<PathBuf, String> {
    let rev = &spec.source.rev;
    let dir = work
        .join("src")
        .join(format!("{}-{}", spec.name, &rev[..12]));
    let at = run(git(&dir).args(["rev-parse", "HEAD"])).ok();
    if at.as_deref().map(str::trim) == Some(rev) {
        return Ok(dir);
    }
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).map_err(|e| format!("{}: {e}", dir.display()))?;
    run(git(&dir).args(["init", "-q"]))?;
    run(git(&dir).args(["remote", "add", "origin", &spec.source.repo]))?;
    run(git(&dir).args(["fetch", "-q", "--depth", "1", "origin", rev]))?;
    run(git(&dir).args([
        "-c",
        "advice.detachedHead=false",
        "checkout",
        "-q",
        "FETCH_HEAD",
    ]))?;
    Ok(dir)
}

/// The licence texts of a checkout: files named `LICENSE…`, `LICENCE…`
/// or `COPYING…` in the grammar's directory, else at the root. None is
/// an error: a library is not handed on without its terms.
pub fn licenses(checkout: &Path, path: &str) -> Result<Vec<(String, Vec<u8>)>, String> {
    for dir in [checkout.join(path), checkout.to_path_buf()] {
        let mut found = Vec::new();
        for entry in std::fs::read_dir(&dir).map_err(|e| format!("{}: {e}", dir.display()))? {
            let p = entry.map_err(|e| e.to_string())?.path();
            let name = p.file_name().unwrap().to_string_lossy().into_owned();
            let upper = name.to_ascii_uppercase();
            let is = ["LICENSE", "LICENCE", "COPYING"]
                .iter()
                .any(|w| upper.starts_with(w));
            if is && p.is_file() {
                let data = std::fs::read(&p).map_err(|e| format!("{}: {e}", p.display()))?;
                found.push((name, data));
            }
        }
        if !found.is_empty() {
            found.sort();
            return Ok(found);
        }
    }
    Err("no LICENSE, LICENCE or COPYING file in the checkout".into())
}
