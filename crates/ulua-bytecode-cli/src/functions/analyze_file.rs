use alloc::vec::Vec;
use std::panic::{AssertUnwindSafe, catch_unwind};

use ulua_ast::records::{
  allocator::Allocator, ast_name_table::AstNameTable, parse_options::ParseOptions, parser::Parser,
};
use ulua_bytecode::records::bytecode_builder::BytecodeBuilder;
use ulua_cli_lib::functions::{
  copts::copts, read_file::read_file, report_compile_panic::report_compile_panic,
  report_open_error::report_open_error, report_parse_errors::report_parse_errors,
};
use ulua_code_gen::{
  functions::summarize_bytecode::summarize_bytecode,
  records::function_bytecode_summary::FunctionBytecodeSummary,
};
use ulua_compiler::functions::compile_or_throw_compiler::compile_or_throw_bytecode_builder_parse_result_ast_name_table_compile_options;
use ulua_vm::{
  functions::{lua_l_newstate::lua_l_newstate, luau_load::luau_load},
  records::lua_state_guard::LuaStateGuard,
};

/// cpp `analyzeFile` (`CLI/src/Bytecode.cpp:114-159`)：编译源码 → 加载字节码 → 汇总。
/// cpp 的 `bool` + 引用出参在 Rust 侧收敛为 `Option<Vec<_>>`：成功即返回汇总，
/// 任一环节失败返回 `None`，不再有「失败时出参处于半成品状态」的问题。
pub fn analyze_file(name: &str, nesting_limit: u32) -> Option<Vec<FunctionBytecodeSummary>> {
  // chunkname 直接以 `&str` 交给 luau_load（cpp 侧为 `name.c_str()`，长度按
  // strlen 取，内部 NUL 的截断规则在 luau_load 里还原）；
  // 文件路径同样用 &str 交给 std::fs，不再传带尾 NUL 的 C 串
  let Some(source) = read_file(name) else {
    report_open_error(name);
    return None;
  };

  // Box 钉堆：AstNameTable/Parser 捕获宿主地址，宿主移动即悬垂。
  let mut allocator = Box::new(Allocator::new());
  let mut names = AstNameTable::new(&mut allocator);
  let parse_options = ParseOptions::default();
  let parse_result = Parser::parse(source.as_str(), &mut names, &mut allocator, parse_options);

  if report_parse_errors(name, &parse_result.errors) {
    return None;
  }

  let mut bcb = BytecodeBuilder::new(None);
  let options = copts();
  let compile_result = catch_unwind(AssertUnwindSafe(|| {
    compile_or_throw_bytecode_builder_parse_result_ast_name_table_compile_options(
      &mut bcb,
      &parse_result,
      &mut names,
      &options,
    );
  }));

  // CompileError 负载上报后折算 None；其余负载原样续抛（门面单点收口）
  report_compile_panic(name, compile_result)?;

  let bytecode = bcb.get_bytecode();
  let global_state = LuaStateGuard(lua_l_newstate());
  let l = global_state.0;
  if l.is_null() {
    eprintln!("Error initializing Lua state");
    return None;
  }

  // Safety: `l` 非空由上方判空保证（存活期至 guard Drop，覆盖整次调用）；
  // `name`/`bytecode` 为调用帧存活的合法借用，`luau_load` 在调用内读完；
  // -1 仅在 load 成功后指向压入的原型，`summarize_bytecode` 只读该槽。
  if unsafe { luau_load(l, name, bytecode, 0) == 0 } {
    // Safety: `l` 存活同上；-1 为 `luau_load` 成功压入的原型槽，`summarize_bytecode`
    // 只读该槽并遍历其常量表，不写栈外状态。
    Some(unsafe { summarize_bytecode(l, -1, nesting_limit) })
  } else {
    eprintln!("Error loading bytecode {name}");
    None
  }
}
