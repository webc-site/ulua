use core::ptr::null;

use crate::{
  enums::value_view::ValueView, functions::index_2_addr::index_2_addr,
  records::lua_state::LuaState, type_aliases::stk_id::StkId,
};

/// # Safety
/// `l` 须为存活 `LuaState` 且 `idx` 为合法（伪）索引；解析出的槽若非 vector 则返回 NULL，否则返回
/// 指向该栈槽内 `LUA_VECTOR_SIZE` 个 `f32` 分量的数组指针（指针随栈重分配失效）。cpp `lapi.cpp:567`。
pub unsafe fn lua_tovector(l: *mut LuaState, idx: i32) -> *const f32 {
  unsafe {
    let o: StkId = index_2_addr(l, idx);

    match ValueView::from_tvalue(&*o) {
      ValueView::Vector(v) => v.as_ptr(),
      _ => null(),
    }
  }
}
