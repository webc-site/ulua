use core::{
  ffi::{c_int, c_void},
  ptr::null,
};

use crate::{
  enums::lua_type::LuaType,
  functions::index_2_addr::index2addr,
  macros::{gcvalue::gcvalue, iscollectable::iscollectable, ttype::ttype, uvalue::uvalue},
  type_aliases::{lua_state::lua_State, stk_id::StkId},
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_topointer(l: *mut lua_State, idx: c_int) -> *const c_void {
  unsafe {
    let o: StkId = index2addr(l, idx);
    let tt = ttype!(o);

    if tt == LuaType::UserData as i32 {
      uvalue!(o).data.as_ptr() as *const c_void
    } else if tt == LuaType::LightUserData as i32 {
      // pvalue(o) is defined as check_exp(ttislightuserdata(o), (o)->value.p)
      // In Rust, the pvalue macro is currently a stub or constant,
      // so we access the union field directly to match the C++ logic.
      (*o).value.p as *const c_void
    } else {
      if iscollectable!(o) {
        gcvalue!(o) as *const c_void
      } else {
        null()
      }
    }
  }
}
