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
/// DELIBERATE DEVIATION（review.md §2/§9.3）：`Repl.h` 导出入口，编译产物经
/// `run_loaded_chunk`（ulua-vm c-API）在状态句柄上执行；成功/失败以 `Option<String>`
/// 表达（cpp 空串哨兵的 Rust 化）。句柄收编为借用 `&mut LuaState`——存活/独占前提由
/// 类型承载，故入口降级为安全 `fn`，`# Safety` 契约上移到各调用方唯一的裸指针物化点
/// （`run_repl_impl` 的 helper 句柄、`ulua-cli-test` 夹具的 `lua_l_newstate` 结果）；
/// 唯一的真实不安全只剩 `run_loaded_chunk` 一处 c-API 边界调用，就地 `// Safety:` 论证。
///
/// 调用序契约（由 `&mut` 承载存活）：`l` 为活跃、已 openlibs/sandbox 的状态机，调用前
/// 栈平衡。
pub fn run_code(l: &mut LuaState, source: &str) -> Option<String> {
  lua_checkstack(l, LUA_MINSTACK);

  let bytecode = compile_source(source);

  // Safety: `run_loaded_chunk` 为 ulua-vm c-API 边界，其 `# Safety` 契约（活跃状态机、
  // 调用前栈平衡）由上方 checkstack 与调用方的存活/栈平衡前提保证；`l` 的借用经隐式
  // 重借折成裸指针，窗止于当句调用。
  unsafe { run_loaded_chunk(l, &bytecode, true) }.err()
}
