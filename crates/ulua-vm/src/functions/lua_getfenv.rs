use core::ptr::eq;

use crate::{
  enums::lua_type::LuaType,
  functions::{
    ensure_stack::ensure_stack, index_2_addr::index_2_addr, lapi_barrier::lua_c_threadbarrier_lapi,
  },
  macros::{
    api_check::api_check, api_incr_top::api_incr_top, lua_o_nilobject::LUA_O_NILOBJECT,
    sethvalue::sethvalue, setnilvalue::setnilvalue, ttype::ttype,
  },
  records::lua_state::LuaState,
  type_aliases::stk_id::StkId,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_getfenv(l: *mut LuaState, idx: i32) {
  unsafe {
    lua_c_threadbarrier_lapi(l);
    ensure_stack(l, 1);

    let o: StkId = index_2_addr(l, idx);
    api_check!(l, !eq(o, LUA_O_NILOBJECT));

    match ttype!(o) {
      x if x == LuaType::Function as u32 => {
        sethvalue!(l, (*l).top, (*o).as_closure().env);
      }
      x if x == LuaType::Thread as u32 => {
        sethvalue!(l, (*l).top, (*o).as_thread().gt);
      }
      _ => {
        setnilvalue!((*l).top);
      }
    }

    api_incr_top!(l);
  }
}
