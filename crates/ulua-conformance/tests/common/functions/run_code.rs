use core::{
  ffi::{c_char, c_int},
  ptr::null_mut,
  slice::from_raw_parts,
};

use ulua_compiler::functions::luau_compile::luau_compile;
use ulua_vm::{
  functions::{lua_pcall::lua_pcall, luau_load::luau_load},
  macros::lua_multret::LUA_MULTRET,
  records::lua_state::LuaState,
};

use crate::common::functions::c_alloc::c_free;

/// 编译-加载-调用 `source` 并把 MULTRET 结果留在栈顶的收口（内部单次边界）。
///
/// 前置条件（由用例保证，与直接调 C API 同语义）：`l` 须指向存活 `LuaState`（用例里由
/// `new_state()` 取得后 `state.as_ptr()` 布线），并在本函数全程（含 `lua_pcall` 执行脚本
/// 期间）保持存活；`l` 的栈顶须留够结果余量（末句按脚本返回个数压栈）。`source` 按 UTF-8
/// 字节整段交给 `luau_compile`，编译缓冲在本函数内释放。cpp `tests/DirectFieldAccess.test.cpp:50` `runCode`
pub fn run_code(l: *mut LuaState, source: &str) -> c_int {
  // Safety: `l` 全程存活（见函数级契约）；`source` 是调用方持有的合法 UTF-8 字节区间，
  // 选项传 null 表示取编译端缺省；`luau_compile` 返回非空产物并写出 `bytecode_size`。
  let mut bytecode_size = 0usize;
  let bytecode = unsafe {
    luau_compile(
      source.as_ptr() as *const c_char,
      source.len(),
      // FFI: c-API 要求 NULL
      null_mut(),
      &mut bytecode_size,
    )
  };

  // cpp 用 `std::string bytecode` 承载 RAII；本端口 `luau_compile` 返回裸缓冲，
  // 故与上游 `runCode` 的控制流一致：load 之后统一释放一次再分支。
  // Safety: 裸缓冲自 `bytecode` 起 `bytecode_size` 字节连续可读（data()/size() 同形态）。
  let bytes = unsafe { from_raw_parts(bytecode.cast::<u8>(), bytecode_size) };
  // Safety: `l` 存活；"test" 是字面量 chunk 名，`bytes` 为上一步物化的合法切片。
  let load_result = unsafe { luau_load(l, "test", bytes, 0) };
  // Safety: `bytecode` 是 luau_compile 交回、尚未释放的裸缓冲。
  unsafe { c_free(bytecode.cast()) };

  if load_result != 0 {
    return -1;
  }

  // Safety: `l` 存活且栈顶为刚加载的 chunk；LUA_MULTRET 按脚本返回个数压栈（函数级契约
  // 已要求调用方留够结果余量）。
  unsafe { lua_pcall(l, 0, LUA_MULTRET, 0) }
}
