use alloc::string::String;
use core::ffi::c_int;

use ulua_ast::records::parse_options::ParseOptions;
use ulua_bytecode::records::bytecode_builder::BytecodeBuilder;
use ulua_code_gen::{
  functions::summarize_bytecode::summarize_bytecode,
  records::function_bytecode_summary::FunctionBytecodeSummary,
};
use ulua_compiler::{
  functions::compile_or_throw_compiler::compile_or_throw_bytecode_builder_string_compile_options_parse_options,
  records::compile_options::CompileOptions,
};
use ulua_vm::records::lua_state::LuaState;

use crate::common::functions::{new_state::new_state, safe_api::load_bytes};

pub fn analyze_file(
  source: &str,
  nesting_limit: u32,
  opt_level: u32,
) -> Vec<FunctionBytecodeSummary> {
  let mut bytecode_builder = BytecodeBuilder::new(None);

  // 其余字段（回调与字符串表指针）一律取 `CompileOptions::default()` 的 `None`/`null`，
  // 不再用 `zeroed()` 逐个覆写：类型换成 `Option<fn..>` 时 `None` 才是合法初值。
  let options = CompileOptions {
    optimization_level: opt_level as c_int,
    debug_level: 1,
    type_info_level: 1,
    ..Default::default()
  };

  compile_or_throw_bytecode_builder_string_compile_options_parse_options(
    &mut bytecode_builder,
    &String::from(source),
    &options,
    &ParseOptions::default(),
  );

  let bytecode = bytecode_builder.get_bytecode();

  // `StateRef` 负责 `lua_close`：`load_bytes` 失败 panic 时也不会泄漏整个 LuaState。
  let global_state = new_state();
  let l: *mut LuaState = global_state.as_ptr();

  let result = load_bytes(l, "source", bytecode, 0);
  assert_eq!(result, 0, "analyze_file fixture failed to load bytecode");

  // Safety: `summarize_bytecode` 只读栈顶刚加载的 proto 与其字节码缓冲。
  unsafe { summarize_bytecode(l, -1, nesting_limit) }
}
