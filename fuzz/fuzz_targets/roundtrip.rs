// Metamorphic oracle: the CST-preserving pretty-printer must be IDEMPOTENT.
// Formatting already-formatted code is a no-op, so `format(format(src))` must
// equal `format(src)`. A divergence means the parser → CST → printer round-trip
// loses or rewrites information (a real bug even when nothing crashes); a panic
// in the printer is a crash finding. This exercises a large surface — the lexer,
// the parser, the CST node map, and the printer — that the "type-check / compile"
// targets don't directly stress.

use ulua_ast::records::pretty_print_result::PrettyPrintResult;

#[cfg(not(feature = "afl-runtime"))]
include!("standalone.rs");

use ulua_ast::{
  functions::pretty_print_pretty_printer::pretty_print_string_view_parse_options_bool_bool as format_source,
  records::parse_options::ParseOptions,
};

fn format_once(src: &str) -> PrettyPrintResult {
  // ignore_parse_errors = false: a parse failure yields an empty `code` with a
  // populated `parse_error`, which we treat as "not formattable" and skip.
  format_source(
    src,
    ParseOptions::default(),
    /* with_types */ false,
    false,
  )
}

fn exercise_input(data: &[u8]) {
  let src = ulua_fuzz::generate(data);

  let first = format_once(&src);
  // Only parseable programs can round-trip; skip the deliberately-corrupted ones.
  if !first.parse_error.is_empty() {
    return;
  }

  let second = format_once(&first.code);
  // Re-formatting valid formatted output must itself parse cleanly...
  assert!(
    second.parse_error.is_empty(),
    "re-formatting formatted output failed to parse: {}\n--- formatted ---\n{}",
    second.parse_error,
    first.code
  );
  // ...and be a fixpoint.
  assert_eq!(
    first.code, second.code,
    "pretty-printer is not idempotent\n--- source ---\n{src}\n--- format(src) ---\n{}\n--- format(format(src)) ---\n{}",
    first.code, second.code
  );
}

ulua_fuzz::fuzz_main!(exercise_input);
