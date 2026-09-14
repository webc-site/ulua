use core::{ffi::c_int, ptr::null_mut};

use crate::{
  functions::index_2_addr::index2addr,
  macros::ttisthread::ttisthread,
  records::lua_state::lua_State as LuaStateRecord,
  type_aliases::{lua_state::lua_State, stk_id::StkId},
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_tothread(l: *mut lua_State, idx: c_int) -> *mut lua_State {
  unsafe {
    let o: StkId = index2addr(l, idx);

    if !ttisthread!(o) {
      null_mut()
    } else {
      core::ptr::addr_of_mut!((*(*o).value.gc).th) as *mut LuaStateRecord
    }
  }
}
