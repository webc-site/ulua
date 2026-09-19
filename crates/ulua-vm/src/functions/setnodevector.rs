use core::{ptr::addr_of_mut, slice::from_raw_parts_mut};

use crate::{
  enums::lua_type::LuaType,
  functions::runerror::runerror,
  macros::{
    ceillog_2::ceillog2, dummynode::dummynode, lua_m_newarray::luaM_newarray,
    setnilvalue::setnilvalue,
  },
  records::{lua_node::LuaNode, lua_table::LuaTable},
  type_aliases::lua_state::lua_State,
};

const MAXBITS: i32 = 26;

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub(crate) unsafe fn setnodevector(l: *mut lua_State, t: *mut LuaTable, mut size: i32) {
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

      // 切片化批量初始化，替代逐下标指针推进
      for n in from_raw_parts_mut((*t).node, size as usize) {
        n.key.set_next(0);
        n.key.value = Default::default();
        n.key.extra = [0];
        n.key.set_tt(LuaType::Nil as i32);
        setnilvalue!(addr_of_mut!(n.val));
      }
    }

    (*t).lsizenode = lsize as u8;
    (*t).nodemask8 = ((1 << lsize) - 1) as u8;
    (*t).union.lastfree = size;
  }
}
