use crate::{
  functions::index_2_addr::index_2_addr, macros::lightuserdatatag::lightuserdatatag,
  records::lua_state::LuaState, type_aliases::stk_id::StkId,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_lightuserdatatag(l: *mut LuaState, idx: i32) -> i32 {
  unsafe {
    let o: StkId = index_2_addr(l, idx);

    if (*o).is_lightuserdata() {
      lightuserdatatag!(o)
    } else {
      -1
    }
  }
}
