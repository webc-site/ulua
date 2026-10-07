//! Diagnostic helper: read fuzzer bytes on stdin, print the Luau source that
//! `generate_typed` decodes them into. Lets us turn a `typeck_typed` crash
//! reproducer (raw driver bytes) back into source to feed C++ luau-analyze.
//! Shared stdin body lives in [`ulua_fuzz::dump_source`].
//!   cargo run --release --no-default-features --bin dump_typed < repro.bin
fn main() {
  ulua_fuzz::dump_source(ulua_fuzz::generate_typed);
}
