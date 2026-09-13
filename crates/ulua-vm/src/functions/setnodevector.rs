use core::ffi::{c_char, c_int};

use crate::{
  enums::{lua_status::LuaStatus, lua_type::LuaType},
  functions::{lua_d_throw_ldo::luaD_throw, lua_g_pusherror::lua_g_pusherror},
  macros::{
    ceillog_2::ceillog2, dummynode::dummynode, lua_m_newarray::luaM_newarray,
    setnilvalue::setnilvalue,
  },
  records::{lua_node::LuaNode, lua_table::LuaTable},
  type_aliases::lua_state::lua_State,
};

const MAXBITS: i32 = 26;

unsafe fn runerror(l: *mut lua_State, msg: *const c_char) -> ! {
  unsafe {
    lua_g_pusherror(l, msg);
    luaD_throw(l, LuaStatus::ErrRun as c_int);
  }
}

pub(crate) unsafe fn setnodevector(l: *mut lua_State, t: *mut LuaTable, mut size: c_int) {
  unsafe {
    let lsize: i32;

    if size == 0 {
      (*t).node = dummynode as *mut LuaNode;
      lsize = 0;
    } else {
      lsize = ceillog2(size as u32);
      if lsize > MAXBITS {
        runerror(l, c"table overflow".as_ptr());
      }

      size = 1 << lsize;
      (*t).node = luaM_newarray!(l, size as usize, LuaNode, (*t).memcat);

      let mut i = 0;
      while i < size {
        let n = (*t).node.add(i as usize);
        (*n).key.set_next(0);
        (*n).key.value = Default::default();
        (*n).key.extra = [0];
        (*n).key.set_tt(LuaType::Nil as i32);
        setnilvalue!(core::ptr::addr_of_mut!((*n).val));
        i += 1;
      }
    }

    (*t).lsizenode = lsize as u8;
    (*t).nodemask8 = ((1 << lsize) - 1) as u8;
    (*t).union.lastfree = size;
  }
}
