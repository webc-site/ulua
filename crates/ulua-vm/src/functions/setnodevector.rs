use core::{ptr::addr_of_mut, slice::from_raw_parts_mut};

use crate::{
  enums::lua_type::LuaType,
  functions::runerror::runerror,
  macros::{
    ceillog_2::ceillog2, dummynode::dummynode, err_table_overflow::ERR_TABLE_OVERFLOW,
    lua_m_newarray::luaM_newarray, setnilvalue::setnilvalue,
  },
  records::{lua_node::LuaNode, lua_state::LuaState, lua_table::LuaTable},
};

const MAXBITS: i32 = 26;

/// # Safety
/// `l` 须为存活 LuaState 并处于可分配/可抛错的受保护帧（`luaM_newarray` 分配哈希桶、`lsize > MAXBITS` 时经 `runerror` 抛 "table overflow"）；
/// `t` 须指向存活 LuaTable（就地写 `node`/`lsizenode`/`nodemask8`/`lastfree`），`size >= 0` 为期望桶数（0 时置 `dummynode`）。
/// cpp/VM/src/ltable.cpp:524 setnodevector。
pub(crate) unsafe fn setnodevector(l: *mut LuaState, t: *mut LuaTable, mut size: i32) {
  unsafe {
    let lsize: i32;

    if size == 0 {
      (*t).node = dummynode as *mut LuaNode;
      lsize = 0;
    } else {
      lsize = ceillog2(size as u32);
      if lsize > MAXBITS {
        runerror(l, ERR_TABLE_OVERFLOW.as_ptr().cast());
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
