use core::ptr::null_mut;

use ulua_vm::records::{lua_execution_callbacks::lua_ExecutionCallbacks, lua_state::lua_State};

use crate::functions::get_code_gen_context::get_code_gen_context;

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn on_close_state(l: *mut lua_State) {
  unsafe {
    if l.is_null() {
      return;
    }

    let global = (*l).global;
    if global.is_null() {
      return;
    }

    // cpp CodeGenContext.cpp:371-375:
    //   static void onCloseState(lua_State* L)
    //   {
    //       getCodeGenContext(L)->onCloseState();
    //       L->global->ecb = lua_ExecutionCallbacks{};
    //   }
    // onCloseState 为虚调用：Standalone 上下文在此 delete this。
    let ctx = get_code_gen_context(l);
    if !ctx.is_null()
      && let Some(on_close) = (*ctx).on_close_state_fn
    {
      on_close(ctx);
    }

    (*global).ecb = lua_ExecutionCallbacks {
      context: null_mut(),
      close: None,
      destroy: None,
      enter: None,
      disable: None,
      getmemorysize: None,
      gettypemapping: None,
      getcounterdata: None,
      inlinefunction: None,
    };
  }
}

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
#[unsafe(export_name = "ulua_on_close_state")]
pub unsafe extern "C-unwind" fn on_close_state_export(l: *mut lua_State) {
  unsafe { on_close_state(l) };
}
