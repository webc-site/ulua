use core::ptr::null_mut;

use ulua_vm::records::{lua_execution_callbacks::lua_ExecutionCallbacks, lua_state::lua_State};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn on_close_state(l: *mut lua_State) {
  unsafe {
    if l.is_null() {
      return;
    }

    let global = (*l).global;
    if !global.is_null() {
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
}

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
#[unsafe(export_name = "ulua_on_close_state")]
pub unsafe extern "C-unwind" fn on_close_state_export(l: *mut lua_State) {
  unsafe { on_close_state(l) };
}
