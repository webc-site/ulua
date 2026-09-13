use crate::{
  functions::stringresizeprotected::stringresizeprotected,
  macros::lua_minstrtabsize::LUA_MINSTRTABSIZE, type_aliases::lua_state::lua_State,
};

pub(crate) unsafe fn shrinkbuffersfull(l: *mut lua_State) {
  unsafe {
    let g = (*l).global;
    let mut hashsize = (*g).strt.size;

    while (*g).strt.nuse < (hashsize / 4) as u32 && hashsize > LUA_MINSTRTABSIZE * 2 {
      hashsize /= 2;
    }

    if hashsize != (*g).strt.size {
      stringresizeprotected(l, hashsize);
    }
  }
}
