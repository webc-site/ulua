use core::ptr::null_mut;

use ulua_vm::records::{lua_execution_callbacks::lua_ExecutionCallbacks, lua_state::LuaState};

use crate::functions::get_code_gen_context::get_code_gen_context;

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
  // onCloseState 为虚调用：Standalone 上下文在此 delete this。
  // Safety: 经判空后 ctx 指向活上下文对象，on_close_state_fn 是注册进 ecb 的 C ABI
  // 回调且以 ctx 为唯一实参调用，满足其契约。
  let ctx = unsafe { get_code_gen_context(l) };
  if !ctx.is_null() {
    let on_close = unsafe { (*ctx).on_close_state_fn };
    if let Some(on_close) = on_close {
      unsafe { on_close(ctx) };
    }
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
