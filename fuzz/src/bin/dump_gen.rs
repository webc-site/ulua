//! Diagnostic helper: read fuzzer bytes on stdin, print the Luau source that
//! `generate` decodes them into. See [`ulua_fuzz::dump_source`].
//!   cargo run --release --no-default-features --bin dump_gen < repro.bin
fn main() {
  ulua_fuzz::dump_source(ulua_fuzz::generate);
}
