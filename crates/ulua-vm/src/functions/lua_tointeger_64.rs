use crate::{
  functions::index_2_addr::index2addr,
  macros::{lvalue::lvalue, ttisinteger::ttisinteger},
  type_aliases::lua_state::lua_State,
};

pub(crate) unsafe fn lua_tointeger_64(l: *mut lua_State, idx: i32, isinteger: *mut i32) -> i64 {
  unsafe {
    let o = index2addr(l, idx);
    if ttisinteger!(o) {
      if !isinteger.is_null() {
        *isinteger = 1;
      }
      lvalue!(o)
    } else {
      if !isinteger.is_null() {
        *isinteger = 0;
      }
      0
    }
  }
}
