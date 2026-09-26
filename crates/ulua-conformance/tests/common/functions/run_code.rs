use core::ffi::c_int;

use ulua_ast::records::parse_options::ParseOptions;
use ulua_bytecode::records::bytecode_encoder::NoopEncoder;
use ulua_compiler::{
  functions::compile::compile, records::compile_options::CompileOptions,
};
use ulua_vm::{
  functions::{lua_pcall::lua_pcall, luau_load::luau_load},
  macros::lua_multret::LUA_MULTRET,
  records::lua_state::LuaState,
};

/// 编译-加载-调用 `source` 并把 MULTRET 结果留在栈顶的收口。
///
/// 前置条件（由用例保证，与直接调 C API 同语义）：`l` 须指向存活 `LuaState`（用例里由
/// `new_state()` 取得后 `state.as_ptr()` 布线），并在本函数全程（含 `lua_pcall` 执行脚本
/// 期间）保持存活；`l` 的栈顶须留够结果余量（末句按脚本返回个数压栈）。`source`
/// 交给 `compile`。cpp `tests/DirectFieldAccess.test.cpp:50` `runCode`
pub fn run_code(l: *mut LuaState, source: &str) -> c_int {
  let bytecode = compile(
    source.as_bytes(),
    &CompileOptions::default(),
    &ParseOptions::default(),
    NoopEncoder,
  );

  // Safety: `l` 存活；"test" 是字面量 chunk 名，`bytecode` 为安全生成的字节码切片。
  let load_result = unsafe { luau_load(l, "test", &bytecode, 0) };
  if load_result != 0 {
    return -1;
  }

  // Safety: `l` 存活且栈顶为刚加载的 chunk；LUA_MULTRET 按脚本返回个数压栈（函数级契约
  // 已要求调用方留够结果余量）。
  unsafe { lua_pcall(l, 0, LUA_MULTRET, 0) }
}
