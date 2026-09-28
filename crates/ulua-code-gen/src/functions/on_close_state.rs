use alloc::boxed::Box;
use core::ptr::{from_mut, null_mut};

use ulua_vm::records::{lua_execution_callbacks::lua_ExecutionCallbacks, lua_state::LuaState};

use crate::{
  enums::options::CodeGenContextKind, functions::get_code_gen_context::get_code_gen_context,
  records::base_code_gen_context::BaseCodeGenContext,
};

/// 与 cpp `lua_ExecutionCallbacks{}` 聚合初始化逐字段一致的零值 ecb。
fn zero_execution_callbacks() -> lua_ExecutionCallbacks {
  lua_ExecutionCallbacks {
    context: null_mut(),
    close: None,
    destroy: None,
    enter: None,
    disable: None,
    getmemorysize: None,
    gettypemapping: None,
    getcounterdata: None,
    inlinefunction: None,
  }
}

/// 状态关闭钩子：注销 code-gen 上下文并把 global 的 ecb 回调表复位为零值。
///
/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn on_close_state(l: *mut LuaState) {
  if l.is_null() {
    return;
  }

  // Safety: `l` 已判空，指向存活 LuaState，其 `global` 或为 null 或指向存活 global_State。
  let global = unsafe { (*l).global };
  if global.is_null() {
    return;
  }

  // cpp CodeGenContext.cpp:371-375:
  //   static void onCloseState(LuaState* L)
  //   {
  //       getCodeGenContext(L)->onCloseState();
  //       L->global->ecb = lua_ExecutionCallbacks{};
  //   }
  // onCloseState 为虚调用：Standalone 上下文在此销毁自身，Shared 为 no-op。
  // Rust 侧以 kind 判别静态分支——原先经 fn 槽 shim 的「取指针→下行转型→&mut 自毁」
  // 链条已删除，这里只保留唯一一处所有权回收边界。
  // Safety: `get_code_gen_context` 依其 `# Safety` 返回串行窗口内独占借用；Standalone
  // 上下文由 luau_codegen_create 的 `Box::into_raw` 泄漏登记，本回调在 state 关闭时至多
  // 触发一次，紧随其后 ecb 整体清零，不再有其它持有者。先在只读判定后结束借用，再以
  // `from_mut` 显式转手裸指针做配对回收（Shared 由创建者持有，不得在此回收）。
  if let Some(ctx) = unsafe { get_code_gen_context(l) }
    && ctx.kind == CodeGenContextKind::Standalone
  {
    let raw: *mut BaseCodeGenContext = from_mut(ctx);
    drop(unsafe { Box::from_raw(raw) });
  }

  // Safety: `global` 已判空，覆写活结构的 ecb 字段为默认值（与 cpp 语义一致）。
  unsafe { (*global).ecb = zero_execution_callbacks() };
}

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe extern "C-unwind" fn on_close_state_export(l: *mut LuaState) {
  // Safety: 导出 C ABI 入口原样转发 l 给同契约 unsafe fn on_close_state; 该内部函数自带 l/global
  // 判空早返回, 故传空亦安全, 满足被调前置条件。
  unsafe { on_close_state(l) };
}
