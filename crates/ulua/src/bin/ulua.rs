//! `ulua` — the installable command-line entry point (`cargo install ulua`).
//!
//! Thin wrapper over `ulua-repl-cli`'s `main` (the Luau script runner / REPL,
//! a faithful port of the upstream `luau` CLI). Gated behind the `cli` feature
//! so library-only users of the umbrella crate don't build the CLI stack.

fn main() {
  ulua_repl_cli::main();
}
