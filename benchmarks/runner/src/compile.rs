//! 编译吞吐组引擎（`--group=compile`）：ulua parse 与 parse+compile 到字节码。

use std::panic::{AssertUnwindSafe, catch_unwind};

use ulua_ast::records::{
  allocator::Allocator, ast_name_table::AstNameTable, parse_options::ParseOptions, parser::Parser,
};
use ulua_bytecode::records::bytecode_builder::BytecodeBuilder;
use ulua_common::{
  functions::is_default_enabled_flag::is_default_enabled_flag, records::f_value::FValue,
};
use ulua_compiler::{
  functions::compile_or_throw_compiler::compile_or_throw_bytecode_builder_string_compile_options_parse_options,
  records::compile_options::CompileOptions,
};

/// 与 ulua-compile / ulua-analyze CLI 启动一致：把默认开启的 Luau FFlag
/// 归一为 true（默认关闭的不动），避免 fflag 状态影响编译/分析路径。
/// 在分组主流程（compile/analysis）开始前调用一次。
pub(crate) fn apply_luau_flags_default() {
  FValue::<bool>::set_all_unless(true, |name| !is_default_enabled_flag(name));
}

/// 纯解析（ulua-ast `Parser::parse`，含 Luau 类型语法），不产出字节码。
///
/// `Allocator` 经 `Box` 钉堆（`AstNameTable`/`Parser` 捕获宿主地址，移动即悬垂），
/// 与 `ulua_compiler::parse_pinned` 内部同一契约，此处直接复用公开 API 组合。
pub(crate) fn run_parse(src: &str) -> Result<Option<String>, String> {
  let mut allocator = Box::new(Allocator::new());
  let mut names = AstNameTable::new(&mut allocator);
  let result = Parser::parse(src, &mut names, &mut allocator, ParseOptions::default());
  if !result.errors.is_empty() {
    return Err(format!("解析失败: {} 处语法错误", result.errors.len()));
  }
  Ok(None)
}

/// parse + compile 到字节码（ulua-compiler 公开入口，与 luau-compile 管线同源）。
///
/// 入口在语法错误时 `panic_any(ParseErrors)`（cpp `throw` 的保真直译），
/// 与 CLI 一样以 `catch_unwind` 收口为失败值。
pub(crate) fn run_compile(src: &str) -> Result<Option<String>, String> {
  let outcome = catch_unwind(AssertUnwindSafe(|| {
    let mut bcb = BytecodeBuilder::new(None);
    compile_or_throw_bytecode_builder_string_compile_options_parse_options(
      &mut bcb,
      src,
      &CompileOptions::default(),
      &ParseOptions::default(),
    );
  }));
  match outcome {
    Ok(()) => Ok(None),
    Err(_) => Err("编译失败（panic: ParseErrors/CompileError）".to_owned()),
  }
}
