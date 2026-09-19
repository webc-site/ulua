use crate::type_aliases::lua_state::lua_State;

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
#[cfg_attr(feature = "capi", unsafe(export_name = "ulua_lua_encodepointer"))]
pub(crate) unsafe fn lua_encodepointer(l: *mut lua_State, p: usize) -> usize {
  unsafe {
    let g = (*l).global;
    let p = p as u64;
    let ptrenckey = (*g).ptrenckey;

    let result = (ptrenckey[0].wrapping_mul(p).wrapping_add(ptrenckey[2]))
      ^ (ptrenckey[1].wrapping_mul(p).wrapping_add(ptrenckey[3]));

    result as usize
  }
}
