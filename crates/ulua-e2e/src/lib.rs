//! `ulua-e2e` — end-to-end / integration test suite for the ulua project.
//!
//! This crate carries no production code: it exists purely as the home for the
//! integration tests under `tests/`, which drive the six shipping CLI binaries
//! (`ulua`, `ulua-analyze`, `ulua-ast`, `ulua-compile`, `ulua-bytecode`,
//! `ulua-reduce`) and the `ulua` umbrella library API as a real product —
//! exercising IO, feature flags, and hostile edge cases.
//!
//! The shared helpers below are used by several of the integration test files.

/// Names of the six CLI binaries under test, as `assert_cmd::cargo_bin` expects.
pub const BINARIES: [&str; 6] = [
  "ulua",
  "ulua-analyze",
  "ulua-ast",
  "ulua-compile",
  "ulua-bytecode",
  "ulua-reduce",
];
