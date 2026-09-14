//! Shared helpers for the ulua end-to-end integration tests.
//!
//! Included via `mod common;` from each `tests/*.rs` file. Not all helpers are
//! used by every test file, so individual unused items are tolerated.

/// Locate one of the six shipping CLI binaries and wrap it in an
/// `assert_cmd::Command`.
///
/// `assert_cmd::Command::cargo_bin` only works for binaries belonging to the
/// *current* crate (it reads `CARGO_BIN_EXE_<name>`, which cargo sets only for
/// the package under test). Our binaries live in sibling workspace crates, so
/// we resolve their path from `CARGO_MANIFEST_DIR` (the `ulua-e2e` crate dir)
/// up to the workspace root, then into `target/<profile>/<name>`. This is also
/// robust to the project's `build-dir` relocation (final binaries stay under
/// `./target`, only intermediates move).
use std::env::consts::EXE_SUFFIX;
use std::{
  env::{current_exe, var_os},
  fs::File,
  io::Write,
  path::PathBuf,
};

use assert_cmd::Command;
use tempfile::TempDir;
pub fn bin(name: &str) -> Command {
  let mut candidates: Vec<PathBuf> = Vec::new();

  // Honor the platform executable suffix: the bins are `ulua-analyze` on Unix
  // but `ulua-analyze.exe` on Windows, so a bare `name` join would never
  // `.exists()` there and every spawn test would fail to locate the binary.
  let file_name = format!("{name}{}", EXE_SUFFIX);

  // Most robust source: this test executable's own location. cargo / nextest
  // run integration tests from `<target>/<profile>/deps/<test-exe>` (or build dir),
  // so walking up to the active profile directory (`debug` or `release`) finds
  // the workspace `[[bin]]` outputs even when CARGO_TARGET_DIR is set globally.
  if let Ok(exe) = current_exe() {
    let mut cur = exe.parent();
    while let Some(dir) = cur {
      if dir
        .file_name()
        .is_some_and(|name| name == "debug" || name == "release")
      {
        candidates.push(dir.join(&file_name));
        break;
      }
      cur = dir.parent();
    }
  }

  // Fallbacks: an explicit `CARGO_TARGET_DIR`, then `<workspace root>/target`.
  let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
  // crates/ulua-e2e -> crates -> <workspace root>
  let workspace_target = manifest_dir
    .parent()
    .and_then(|p| p.parent())
    .map(|root| root.join("target"));
  let target_roots: Vec<PathBuf> = var_os("CARGO_TARGET_DIR")
    .map(PathBuf::from)
    .into_iter()
    .chain(workspace_target)
    .collect();
  for target in &target_roots {
    candidates.push(target.join("debug").join(&file_name));
    candidates.push(target.join("release").join(&file_name));
  }

  let path = candidates.iter().find(|p| p.exists()).unwrap_or_else(|| {
    panic!(
      "could not locate binary {name}; looked in {:?}. \
             Build the workspace bins first (cargo build --workspace --bins).",
      candidates
    )
  });
  Command::new(path)
}

/// Create a fresh temp dir and write `source` into `<dir>/<name>`, returning the
/// dir (which must be kept alive for the file to persist) and the full path.
pub fn write_script(name: &str, source: &str) -> (TempDir, PathBuf) {
  let dir = tempfile::tempdir().expect("create tempdir");
  let path = dir.path().join(name);
  let mut f = File::create(&path).expect("create script file");
  f.write_all(source.as_bytes()).expect("write script");
  f.flush().expect("flush script");
  (dir, path)
}
