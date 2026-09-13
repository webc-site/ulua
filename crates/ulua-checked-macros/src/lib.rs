//! Compile-time checked Luau source macros for `ulua`.
//!
//! This crate is intentionally separate from `ulua-rt-derive`: derive users
//! should not pay for Luau's static typechecker.

use proc_macro::TokenStream;

mod expand_file;
mod expand_inline;
mod file_input;
mod inline_input;
mod module_entry;
mod paths;
mod report;

/// Type-check an inline Luau source string at Rust compile time.
///
/// Supported forms:
///
/// ```ignore
/// ulua::ulua!("--!strict\nreturn 1")
///
/// ulua::ulua! {
///     source = "--!strict\nlocal M = require(\"@m\")\nreturn M.x",
///     module = "main",
///     modules = {
///         "@m" => "--!strict\nreturn { x = 1 }",
///     },
/// }
/// ```
#[proc_macro]
pub fn ulua(input: TokenStream) -> TokenStream {
  expand_inline::expand(input.into()).into()
}

/// Type-check a Luau source file at Rust compile time and expand to
/// `include_str!(...)`.
///
/// Supported forms:
///
/// ```ignore
/// ulua::ulua_file!("scripts/main.luau")
///
/// ulua::ulua_file! {
///     root = "scripts/main.luau",
///     module = "game/Main",
///     modules = {
///         "game/Math" => "scripts/math.luau",
///         "@config" => "scripts/config.luau",
///     },
/// }
/// ```
#[proc_macro]
pub fn ulua_file(input: TokenStream) -> TokenStream {
  expand_file::expand(input.into()).into()
}

/// Alias for [`ulua`].
#[proc_macro]
pub fn luau(input: TokenStream) -> TokenStream {
  ulua(input)
}

/// Alias for [`ulua_file`].
#[proc_macro]
pub fn luau_file(input: TokenStream) -> TokenStream {
  ulua_file(input)
}
