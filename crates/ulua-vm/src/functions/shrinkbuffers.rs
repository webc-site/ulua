use crate::{
  functions::stringresizeprotected::stringresizeprotected,
  macros::lua_minstrtabsize::LUA_MINSTRTABSIZE, type_aliases::lua_state::lua_State,
};

pub(crate) unsafe fn shrinkbuffers(l: *mut lua_State) {
  unsafe {
    let g = (*l).global;
    if (*g).strt.nuse < ((*g).strt.size / 4) as u32 && (*g).strt.size > LUA_MINSTRTABSIZE * 2 {
      stringresizeprotected(l, (*g).strt.size / 2);
    }
  }
}
