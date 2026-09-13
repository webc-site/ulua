use ulua_vm::records::lua_state::lua_State;

use crate::functions::{
  get_code_gen_context::get_code_gen_context, on_enter::on_enter,
  on_enter_disabled::on_enter_disabled,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
#[inline]
pub unsafe fn set_native_execution_enabled(l: *mut lua_State, enabled: bool) {
  unsafe {
    if get_code_gen_context(l).is_null() {
      return;
    }

    // on_enter / on_enter_disabled 的 ABI 与 ecb.enter 槽位一致，直接装入
    let global = (*l).global;
    (*global).ecb.enter = if enabled {
      Some(on_enter)
    } else {
      Some(on_enter_disabled)
    };
  }
}
