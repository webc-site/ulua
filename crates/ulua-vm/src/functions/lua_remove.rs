use core::ptr::copy;

use crate::{
  functions::index_2_addr::index_2_addr, macros::api_check::api_check,
  records::lua_state::LuaState, type_aliases::stk_id::StkId,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_remove(l: *mut LuaState, idx: i32) {
  unsafe {
    let p: StkId = index_2_addr(l, idx);
    api_check!(l, !p.is_null());
    let count = (*l).top.offset_from(p) - 1;
    if count > 0 {
      copy(p.add(1), p, count as usize);
    }
    (*l).top = (*l).top.sub(1);
  }
}
