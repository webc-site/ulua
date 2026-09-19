//! 把 Luau 源码解析为 AST 并检查解析错误，直接使用 `ulua::ast` 层
//! （不编译、不执行）。
//!
//!     cargo run -p ulua-example-parse-ast

use ulua::ast::records::{
  allocator::Allocator, ast_name_table::AstNameTable, parse_options::ParseOptions,
  parse_result::ParseResult, parser::Parser,
};

fn parse(source: &str, names: &mut AstNameTable, allocator: &mut Allocator) -> ParseResult {
  Parser::parse(source, names, allocator, ParseOptions::default())
}

fn main() {
  let mut allocator = Box::new(Allocator::new());
  let mut names = AstNameTable::new(&mut allocator);
  let good = "local function add(a, b)\n    return a + b\nend\nreturn add(2, 3)";
  let result = parse(good, &mut names, &mut allocator);
  if result.errors.is_empty() {
    println!("parsed OK ({} bytes of source)", good.len());
  } else {
    for e in &result.errors {
      println!("unexpected parse error: {}", e.what());
    }
  }

  // 错误恢复：解析器报告带源码位置的诊断。
  let bad = "local x = \nreturn x +";
  let result = parse(bad, &mut names, &mut allocator);
  println!(
    "\nmalformed input produced {} error(s):",
    result.errors.len()
  );
  for e in &result.errors {
    let loc = e.get_location();
    println!("  line {}: {}", loc.begin.line + 1, e.what());
  }
}
