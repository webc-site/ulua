//! Parse Luau source to an AST and inspect parse errors, using the `ulua::ast`
//! layer directly (no compilation or execution).
//!
//!     cargo run -p ulua-example-parse-ast

use ulua::ast::records::{
  allocator::Allocator, ast_name_table::AstNameTable, parse_options::ParseOptions,
  parse_result::ParseResult, parser::Parser,
};

fn parse(source: &str) -> ParseResult {
  // The arena allocator owns every AST node; the name table interns identifiers.
  let mut allocator = Allocator::new();
  let mut names = AstNameTable::new(&mut allocator);
  Parser::parse(
    source,
    source.len(),
    &mut names,
    &mut allocator,
    ParseOptions::default(),
  )
}

fn main() {
  let good = "local function add(a, b)\n    return a + b\nend\nreturn add(2, 3)";
  let result = parse(good);
  if result.errors.is_empty() {
    println!("parsed OK ({} bytes of source)", good.len());
  } else {
    for e in &result.errors {
      println!("unexpected parse error: {}", e.what());
    }
  }

  // Error recovery: the parser reports diagnostics with source locations.
  let bad = "local x = \nreturn x +";
  let result = parse(bad);
  println!(
    "\nmalformed input produced {} error(s):",
    result.errors.len()
  );
  for e in &result.errors {
    let loc = e.get_location();
    println!("  line {}: {}", loc.begin.line + 1, e.what());
  }
}
