use crate::{
  functions::stringresizeprotected::stringresizeprotected,
  macros::lua_minstrtabsize::LUA_MINSTRTABSIZE, type_aliases::lua_state::lua_State,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub(crate) unsafe fn shrinkbuffers(l: *mut lua_State) {
  unsafe {
    let g = (*l).global;
    if (*g).strt.nuse < ((*g).strt.size / 4) as u32 && (*g).strt.size > LUA_MINSTRTABSIZE * 2 {
      stringresizeprotected(l, (*g).strt.size / 2);
    }
  }
}
