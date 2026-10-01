use core::ptr::null;

use crate::{
  enums::value_view::ValueView, functions::index_2_addr::index_2_addr, records::lua_state::LuaState,
};

/// `lua_tovector` 核心（cpp `lapi.cpp:567`）。`l` 以引用传入（存活由类型保证）；
/// `idx` 为任意（伪）索引，越界经硬化的 `index_2_addr` 返回只读哨兵槽（落 default
/// 臂返回 NULL，与 cpp 非向量分支一致）。解析出的槽若非 vector 则返回 NULL，否则
/// 返回指向该栈槽内 `LUA_VECTOR_SIZE` 个 `f32` 分量的数组指针（指针随栈重分配失效）。
pub fn lua_tovector(l: &LuaState, idx: i32) -> *const f32 {
  let o = index_2_addr(l, idx);

  // SAFETY:o 为栈上有效 TValue 或只读哨兵槽，仅读值。
  match unsafe { ValueView::from_tvalue(&*o) } {
    ValueView::Vector(v) => v.as_ptr(),
    _ => null(),
  }
}
