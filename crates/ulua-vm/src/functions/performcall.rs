use core::ptr::addr_of_mut;

use crate::{
  functions::{
    lua_c_barrierback::lua_c_barrierback, luau_execute::luau_execute, luau_precall::luau_precall,
  },
  macros::{
    isblack::isblack, lua_callinfo_return::LUA_CALLINFO_RETURN, pcrlua::PCRLUA,
    scheduled_reentry::SCHEDULED_REENTRY,
  },
  records::gc_object::GCObject,
  type_aliases::{lua_state::lua_State, stk_id::StkId},
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub(crate) unsafe fn performcall(
  l: *mut lua_State,
  func: StkId,
  nresults: i32,
  preparereentry: bool,
) {
  unsafe {
    if luau_precall(l, func, nresults) == PCRLUA {
      (*(*l).ci).flags |= LUA_CALLINFO_RETURN as u32;

      let oldactive = (*l).isactive;
      (*l).isactive = true;

      let o = l as *mut GCObject;
      if isblack!(o) {
        lua_c_barrierback(l, o, addr_of_mut!((*l).gclist));
      }

      if preparereentry {
        (*l).status = SCHEDULED_REENTRY as u8;
      } else {
        luau_execute(l);
      }

      if !oldactive {
        (*l).isactive = false;
      }
    }
  }
}
