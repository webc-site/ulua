use core::ptr::eq;

use ulua_vm::records::lua_state::lua_State;

use crate::functions::{get_code_gen_context::get_code_gen_context, on_enter::on_enter};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
///
/// C++ 直接比较函数指针：`ecb.enter == onEnter`。
#[inline]
pub unsafe fn is_native_execution_enabled(l: *mut lua_State) -> bool {
  unsafe {
    if l.is_null() || get_code_gen_context(l).is_null() {
      return false;
    }

    let global = (*l).global;
    if global.is_null() {
      return false;
    }

    // 函数指针无唯一地址保证，按地址比较（C++ 直接比较 onEnter 指针）
    if let Some(enter_fn) = (*global).ecb.enter {
      eq(enter_fn as *const (), on_enter as *const ())
    } else {
      false
    }
  }
}
