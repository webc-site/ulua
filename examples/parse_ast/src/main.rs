//! 把 Luau 源码解析为 AST 并检查解析错误，直接使用 `ulua::ast` 层
//! （不编译、不执行）。
//!
//!     cargo run -p ulua-example-parse-ast

use std::error::Error;

use ulua::ast::records::{
  allocator::Allocator, ast_name_table::AstNameTable, parse_options::ParseOptions,
  parse_result::ParseResult, parser::Parser,
};

/// 语法正确的脚本：解析不应产出诊断。
const GOOD: &str = "local function add(a, b)\n    return a + b\nend\nreturn add(2, 3)";
/// 语法错误的脚本：解析器应给出带位置的诊断。
const BAD: &str = "local x = \nreturn x +";
/// 诊断行号是 0-based，打印时转为 1-based。
const FIRST_LINE: u32 = 1;

/// 解析一次 `source`。`allocator`/`names` 由调用方持有并贯穿整个程序：
/// 二者（以及解析产物）捕获宿主地址，被移动即悬垂，故 allocator 先 `Box` 钉堆。
fn parse(source: &str, names: &mut AstNameTable, allocator: &mut Allocator) -> ParseResult {
  Parser::parse(source, names, allocator, ParseOptions::default())
}

fn main() -> Result<(), Box<dyn Error>> {
  let mut allocator = Box::new(Allocator::new());
  let mut names = AstNameTable::new(&mut allocator);

  let good = parse(GOOD, &mut names, &mut allocator);
  if let Some(first) = good.errors.first() {
    // 预期之外的诊断即示例失败：交给 `main` 的 Result 出口报告。
    return Err(
      format!(
        "unexpected parse error at line {}: {}",
        first.get_location().begin.line + FIRST_LINE,
        first.what()
      )
      .into(),
    );
  }
  println!("parsed OK ({} bytes of source)", GOOD.len());

  // 错误恢复：解析器报告带源码位置的诊断。
  let bad = parse(BAD, &mut names, &mut allocator);
  println!("\nmalformed input produced {} error(s):", bad.errors.len());
  for e in &bad.errors {
    let loc = e.get_location();
    println!("  line {}: {}", loc.begin.line + FIRST_LINE, e.what());
  }
  Ok(())
}
