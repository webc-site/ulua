use crate::{
  records::{call_info::CallInfo, lua_state::LuaState, up_val::UpVal},
  type_aliases::t_value::TValue,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub(crate) unsafe fn correctstack(l: *mut LuaState, oldstack: *mut TValue) {
  unsafe {
    let stack_bytes = (*l).stack as *mut u8;
    let oldstack_addr = oldstack as isize;

    (*l).top = stack_bytes.wrapping_offset((*l).top as isize - oldstack_addr) as *mut TValue;

    let mut up: *mut UpVal = (*l).openupval;
    while !up.is_null() {
      (*up).v = stack_bytes.wrapping_offset((*up).v as isize - oldstack_addr) as *mut TValue;
      up = (*up).u.open.threadnext;
    }

    let mut ci: *mut CallInfo = (*l).base_ci;
    while ci <= (*l).ci {
      (*ci).top = stack_bytes.wrapping_offset((*ci).top as isize - oldstack_addr) as *mut TValue;
      (*ci).base = stack_bytes.wrapping_offset((*ci).base as isize - oldstack_addr) as *mut TValue;
      (*ci).func = stack_bytes.wrapping_offset((*ci).func as isize - oldstack_addr) as *mut TValue;
      ci = ci.wrapping_add(1);
    }

    (*l).base = stack_bytes.wrapping_offset((*l).base as isize - oldstack_addr) as *mut TValue;
  }
}
