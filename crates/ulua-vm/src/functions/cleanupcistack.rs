use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  macros::clvalue::clvalue,
  records::{call_info::CallInfo, closure::Closure, lua_state::lua_State},
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub(crate) unsafe fn cleanupcistack(l: *mut lua_State) {
  unsafe {
    let mut lastci: *mut CallInfo = (*l).ci;
    while lastci != (*l).base_ci {
      let func = (*lastci).func;
      let closure = clvalue!(func) as *const _ as *mut Closure;
      LUAU_ASSERT!((*closure).usage > 0);
      (*closure).usage -= 1;
      lastci = lastci.offset(-1);
    }
  }
}
