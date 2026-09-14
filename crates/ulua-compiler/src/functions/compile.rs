use alloc::string::String;
use core::ffi::CStr;
use std::panic::{AssertUnwindSafe, catch_unwind};

use ulua_ast::records::{
  allocator::Allocator, ast_name_table::AstNameTable, parse_options::ParseOptions,
  parse_result::ParseResult, parser::Parser,
};
use ulua_bytecode::records::{
  bytecode_builder::BytecodeBuilder, bytecode_encoder::BytecodeEncoder,
};
use ulua_common::macros::luau_timetrace_scope::LUAU_TIMETRACE_SCOPE;

use crate::{
  functions::compile_or_throw_compiler::compile_or_throw_bytecode_builder_parse_result_ast_name_table_compile_options,
  records::{compile_error::CompileError, compile_options::CompileOptions},
};

/// 对应 cpp `compile(source, options, parseOptions, BytecodeEncoder* encoder)`。
/// 编码器按值接收以静态分发; 传 [`NoopEncoder`] 即 cpp 的空变换编码器。
pub fn compile(
  source: &str,
  options: &CompileOptions,
  parse_options: &ParseOptions,
  encoder: impl BytecodeEncoder + 'static,
) -> String {
  LUAU_TIMETRACE_SCOPE!("compile", "Compiler");

  // Box 钉堆：AstNameTable/Parser 内部存 `*mut Allocator`（捕获宿主地址），
  // 宿主一旦移动即悬垂（同 tests.rs string_table! 先例）。堆地址永不移动。
  let mut allocator = Box::new(Allocator::new());
  let mut names = AstNameTable::new(&mut allocator);
  let result: ParseResult = Parser::parse(
    source,
    source.len(),
    &mut names,
    &mut allocator,
    parse_options.clone(),
  );

  if !result.errors.is_empty() {
    let parse_error = &result.errors[0];
    let error = format!(
      ":{}: {}",
      parse_error.get_location().begin.line + 1,
      parse_error.what()
    );
    return BytecodeBuilder::get_error(&error);
  }

  match catch_unwind(AssertUnwindSafe(|| {
    let mut bcb = BytecodeBuilder::new(Some(Box::new(encoder) as Box<dyn BytecodeEncoder>));
    compile_or_throw_bytecode_builder_parse_result_ast_name_table_compile_options(
      &mut bcb, &result, &mut names, options,
    );
    bcb.get_bytecode().clone()
  })) {
    Ok(bytecode) => bytecode,
    Err(payload) => {
      // C++ `catch (CompileError& e)`: the panic payload IS the thrown
      // CompileError; recover its location/message rather than raising anew.
      let compile_error = payload
        .downcast::<CompileError>()
        .expect("compile() caught a non-CompileError panic");
      let compile_error_location = compile_error.get_location();
      let error = format!(
        ":{}: {}",
        compile_error_location.begin.line + 1,
        unsafe { CStr::from_ptr(compile_error.what()) }.to_string_lossy()
      );
      BytecodeBuilder::get_error(&error)
    }
  }
}
