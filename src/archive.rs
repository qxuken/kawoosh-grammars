//! A grammar's archive: sqlite's own archive table,
//! `sqlar(name, mode, mtime, sz, data)`, a blob deflated with zlib when
//! that is smaller, and a `meta(key, value)` table saying what it was
//! built from. `sqlite3 NAME.sqlar -At` lists one; kawoosh reads one
//! with the sqlite and the zlib it already has.
//!
//! Nothing in one depends on when or where it was written: the rows
//! are in their names' order and carry no time.

use std::io::{Read, Write};
use std::path::Path;

use rusqlite::Connection;

/// A regular file anyone may read.
const MODE: i64 = 0o100644;

pub fn write(
    path: &Path,
    files: &[(String, Vec<u8>)],
    meta: &[(&str, String)],
) -> Result<(), String> {
    let say = |e: rusqlite::Error| format!("{}: {e}", path.display());
    let _ = std::fs::remove_file(path);
    if let Some(dir) = path.parent() {
        std::fs::create_dir_all(dir).map_err(|e| format!("{}: {e}", dir.display()))?;
    }
    let mut db = Connection::open(path).map_err(say)?;
    db.execute_batch(
        "PRAGMA page_size = 1024;
         CREATE TABLE sqlar(name TEXT PRIMARY KEY, mode INT, mtime INT, sz INT, data BLOB);
         CREATE TABLE meta(key TEXT PRIMARY KEY, value TEXT NOT NULL);",
    )
    .map_err(say)?;
    let tx = db.transaction().map_err(say)?;
    let mut files: Vec<&(String, Vec<u8>)> = files.iter().collect();
    files.sort();
    for (name, data) in files {
        let mut z = flate2::write::ZlibEncoder::new(Vec::new(), flate2::Compression::best());
        z.write_all(data).map_err(|e| e.to_string())?;
        let deflated = z.finish().map_err(|e| e.to_string())?;
        let stored = if deflated.len() < data.len() {
            &deflated
        } else {
            data
        };
        tx.execute(
            "INSERT INTO sqlar(name, mode, mtime, sz, data) VALUES (?1, ?2, 0, ?3, ?4)",
            rusqlite::params![name, MODE, data.len() as i64, stored],
        )
        .map_err(say)?;
    }
    for (key, value) in meta {
        tx.execute(
            "INSERT INTO meta(key, value) VALUES (?1, ?2)",
            rusqlite::params![key, value],
        )
        .map_err(say)?;
    }
    tx.commit().map_err(say)?;
    db.close().map_err(|(_, e)| say(e))
}

/// The file `name` of the archive at `path`, as it was before it was
/// stored: a blob shorter than its `sz` is inflated.
pub fn read(path: &Path, name: &str) -> Result<Option<Vec<u8>>, String> {
    let say = |e: rusqlite::Error| format!("{}: {e}", path.display());
    let db = Connection::open_with_flags(path, rusqlite::OpenFlags::SQLITE_OPEN_READ_ONLY)
        .map_err(say)?;
    let row: Option<(i64, Vec<u8>)> = db
        .query_row("SELECT sz, data FROM sqlar WHERE name = ?1", [name], |r| {
            Ok((r.get(0)?, r.get(1)?))
        })
        .map(Some)
        .or_else(|e| match e {
            rusqlite::Error::QueryReturnedNoRows => Ok(None),
            e => Err(say(e)),
        })?;
    let Some((sz, data)) = row else {
        return Ok(None);
    };
    if data.len() as i64 == sz {
        return Ok(Some(data));
    }
    let mut out = Vec::with_capacity(sz as usize);
    flate2::read::ZlibDecoder::new(&data[..])
        .read_to_end(&mut out)
        .map_err(|e| format!("{}: {name}: {e}", path.display()))?;
    Ok(Some(out))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_archive_gives_back_what_went_in() {
        let path =
            std::env::temp_dir().join(format!("kawoosh-grammars-{}.sqlar", std::process::id()));
        let long = b"(identifier) @variable\n".repeat(200);
        let short = b"x".to_vec();
        let files = vec![
            ("queries/highlights.scm".to_string(), long.clone()),
            ("LICENSE".to_string(), short.clone()),
        ];
        write(
            &path,
            &files,
            &[("name", "zig".into()), ("abi", "15".into())],
        )
        .unwrap();

        assert_eq!(
            read(&path, "queries/highlights.scm").unwrap(),
            Some(long.clone())
        );
        assert_eq!(read(&path, "LICENSE").unwrap(), Some(short));
        assert_eq!(read(&path, "lib/none.so").unwrap(), None);

        let db = Connection::open(&path).unwrap();
        // The long one is stored deflated, the short one as it is: the
        // rule `sqlite3 -A` reads by.
        let stored = |name: &str| -> (i64, i64) {
            db.query_row(
                "SELECT sz, length(data) FROM sqlar WHERE name = ?1",
                [name],
                |r| Ok((r.get(0)?, r.get(1)?)),
            )
            .unwrap()
        };
        let (sz, len) = stored("queries/highlights.scm");
        assert!(sz == long.len() as i64 && len < sz);
        assert_eq!(stored("LICENSE"), (1, 1));
        let abi: String = db
            .query_row("SELECT value FROM meta WHERE key = 'abi'", [], |r| r.get(0))
            .unwrap();
        assert_eq!(abi, "15");
        drop(db);

        // Written again, it is the same bytes.
        let first = std::fs::read(&path).unwrap();
        let mut backwards = files.clone();
        backwards.reverse();
        write(
            &path,
            &backwards,
            &[("name", "zig".into()), ("abi", "15".into())],
        )
        .unwrap();
        assert_eq!(std::fs::read(&path).unwrap(), first);
        std::fs::remove_file(&path).unwrap();
    }
}
