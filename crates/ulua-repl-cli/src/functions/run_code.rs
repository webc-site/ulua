//! cpp `Repl.cpp:239` 的 `runCode` 薄壳：全部加载/执行/打印/错误组装逻辑已
//! 下沉到 [`ulua_vm::functions::run_loaded_chunk`]（含 `_PRETTYPRINT` 为 nil 时
//! 回退 print 的 REPL 差异，传 `pretty_print_fallback = true`），此处仅保留
//! 编译入口与 REPL 形态的返回约定。

use alloc::string::String;

use ulua_vm::{
  functions::{lua_checkstack::lua_checkstack, run_loaded_chunk::run_loaded_chunk},
  macros::lua_minstack::LUA_MINSTACK,
  records::lua_state::LuaState,
};

use crate::functions::compile_source::compile_source;

/// 运行 `source`：成功返回 `None`，失败返回 `Some(错误文本)`。
///
/// cpp `Repl.cpp:239` 的 `runCode` 用空串当成功哨兵；这里用 `Option` 表达同一
/// 语义，避免调用方把「错误文本恰为空」误判成成功。
///
/// # Safety
///
/// `l` must be a valid, active pointer to a `LuaState`.
pub unsafe fn run_code(l: *mut LuaState, source: &str) -> Option<String> {
  // Safety: l 是 REPL 循环全程有效的主线程状态（fn /// # Safety），checkstack
  // 预留后续 VM 调用所需槽位。
  unsafe { lua_checkstack(l, LUA_MINSTACK) };

  let bytecode = compile_source(source);

  // Safety: run_loaded_chunk 的 `# Safety` 契约（活跃状态机、调用前栈平衡）由
  // 上方 checkstack 与 REPL 主循环的栈平衡保证。
  unsafe { run_loaded_chunk(l, &bytecode, true) }.err()
}
