use alloc::boxed::Box;
use core::{
  ptr::{from_mut, null_mut},
  sync::atomic::Ordering,
};

use ulua_vm::records::{
  lua_execution_callbacks::{FORN_HEAT_ARMED, lua_ExecutionCallbacks},
  lua_state::LuaState,
};

use crate::{
  enums::options::CodeGenContextKind, functions::get_code_gen_context::get_code_gen_context,
  records::base_code_gen_context::BaseCodeGenContext,
};

/// 状态关闭钩子（ecb.close 槽位实现）：注销 code-gen 上下文并把 global 的 ecb 回调表复位为零值。
///
/// # Safety
/// `extern "C-unwind"` FFI 边界：`l` 为 null 或其 `global` 为 null 时按「无可注销上下文」早返回；
/// 否则 `l` 指向存活 `LuaState`、其 `global` 指向存活 `global_State`。`get_code_gen_context` 依其
/// `# Safety` 返回串行窗口内独占借用；Standalone 上下文由 `luau_codegen_create` 的 `Box::into_raw`
/// 泄漏登记，本回调在 state 关闭时至多触发一次，紧随其后 ecb 整体清零、不再有其它持有者。
pub unsafe extern "C-unwind" fn on_close_state(l: *mut LuaState) {
  if l.is_null() {
    return;
  }

  // Safety: `l` 已判空，指向存活 LuaState，其 `global` 或为 null 或指向存活 global_State。
  let global = unsafe { (*l).global };
  if global.is_null() {
    return;
  }

  // oracle: cpp CodeGenContext.cpp:371-375 `onCloseState` = `getCodeGenContext(L)->onCloseState()`
  //   + `L->global->ecb = lua_ExecutionCallbacks{}`。
  // onCloseState 为虚调用：Standalone 上下文在此销毁自身，Shared 为 no-op。
  // Rust 侧以 kind 判别静态分支——原先经 fn 槽 shim 的「取指针→下行转型→&mut 自毁」
  // 链条已删除，这里只保留唯一一处所有权回收边界。
  // Safety: 见函数头——先在只读判定后结束借用，再以 `from_mut` 显式转手裸指针做配对回收
  // （Shared 由创建者持有，不得在此回收）。
  if let Some(ctx) = unsafe { get_code_gen_context(l) }
    && ctx.kind == CodeGenContextKind::Standalone
  {
    let raw: *mut BaseCodeGenContext = from_mut(ctx);
    drop(unsafe { Box::from_raw(raw) });
  }

  // Safety: `global` 已判空，覆写活结构的 ecb 字段为默认值（与 cpp 语义一致）。
  unsafe { (*global).ecb = lua_ExecutionCallbacks::default() };
  // T3 单载荷门：进程级武装槽随 state 关闭解除——计数单元（注册表 Box）随
  // forn_traces 整体 drop，槽内悬垂指针必须先行清除。
  FORN_HEAT_ARMED.store(null_mut(), Ordering::Relaxed);
}
