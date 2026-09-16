use core::ptr::null_mut;

use ulua_vm::records::lua_state::lua_State;

use crate::records::base_code_gen_context::BaseCodeGenContext;

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
#[inline]
pub unsafe fn get_code_gen_context(l: *mut lua_State) -> *mut BaseCodeGenContext {
  // SAFETY: This mirrors the C++ implementation:
  // return static_cast<BaseCodeGenContext*>(l->global->ecb.context);
  // Caller must ensure `l` is a valid lua_State pointer with a valid `global`
  // and that `global->ecb.context` points to a BaseCodeGenContext (or is null).
  unsafe {
    if l.is_null() {
      return null_mut();
    }

    let global = (*l).global;
    if global.is_null() {
      return null_mut();
    }

    let ctx = (*global).ecb.context;
    ctx as *mut BaseCodeGenContext
  }
}
