use alloc::vec::Vec;
use core::ffi::c_char;
use std::{
  ffi::CString,
  panic::{AssertUnwindSafe, catch_unwind, resume_unwind},
};

use ulua_ast::records::{
  allocator::Allocator, ast_name_table::AstNameTable, parse_options::ParseOptions, parser::Parser,
};
use ulua_bytecode::records::bytecode_builder::BytecodeBuilder;
use ulua_cli_lib::{
  functions::{
    read_file::read_file, report_compile_error::report_compile_error,
    report_parse_error::report_parse_error,
  },
  records::lua_state_guard::LuaStateGuard,
};
use ulua_code_gen::{
  functions::summarize_bytecode::summarize_bytecode,
  records::function_bytecode_summary::FunctionBytecodeSummary,
};
use ulua_compiler::{
  functions::compile_or_throw_compiler::compile_or_throw_bytecode_builder_parse_result_ast_name_table_compile_options,
  records::compile_error::CompileError,
};
use ulua_vm::functions::{lua_l_newstate::lua_l_newstate, luau_load::luau_load};

use crate::functions::copts::copts;

pub fn analyze_file(
  name: &str,
  nesting_limit: u32,
  summaries: &mut Vec<FunctionBytecodeSummary>,
) -> bool {
  // CString 校验内嵌 NUL 并提供 NUL 结尾 chunkname 供 luau_load；
  // 文件路径直接用 &str 交给 std::fs，不再传带尾 NUL 的 C 串
  let Ok(name_c) = CString::new(name) else {
    eprintln!("Error opening {}", name);
    return false;
  };
  let Some(source) = read_file(name) else {
    eprintln!("Error opening {}", name);
    return false;
  };

  // Box 钉堆：AstNameTable/Parser 捕获宿主地址，宿主移动即悬垂。
  let mut allocator = Box::new(Allocator::new());
  let mut names = AstNameTable::new(&mut allocator);
  let parse_options = ParseOptions::default();
  let parse_result = Parser::parse(
    source.as_str(),
    source.len(),
    &mut names,
    &mut allocator,
    parse_options,
  );

  if !parse_result.errors.is_empty() {
    for error in &parse_result.errors {
      report_parse_error(name, error);
    }

    return false;
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

  if let Err(payload) = compile_result {
    if let Some(error) = payload.downcast_ref::<CompileError>() {
      report_compile_error(name, error);
      return false;
    }

    resume_unwind(payload);
  }

  let bytecode = bcb.get_bytecode();
  let global_state = LuaStateGuard(lua_l_newstate());
  let l = global_state.0;

  if unsafe {
    luau_load(
      l,
      name_c.as_ptr(),
      bytecode.as_ptr() as *const c_char,
      bytecode.len(),
      0,
    )
  } == 0
  {
    *summaries = unsafe { summarize_bytecode(l, -1, nesting_limit) };
    true
  } else {
    eprintln!("Error loading bytecode {}", name);
    false
  }
}
