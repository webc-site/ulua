use core::{
  ptr::{addr_of_mut, eq},
  slice,
};

use crate::{
  enums::lua_type::LuaType,
  functions::maybesetaboundary::maybesetaboundary,
  macros::{dummynode::dummynode, setnilvalue::setnilvalue, sizenode::sizenode},
  records::lua_table::LuaTable,
};

/// # Safety
/// `tt` 须指向存活 `LuaTable` 且其 `array/node` 指针与 `sizearray`、`sizenode` 记录的分配
/// 尺寸一致（cpp ltable.cpp:1411）：清空切片按该元数据为界读写，`node` 为 dummynode 时跳过
/// 节点区写入。
pub unsafe fn lua_h_clear(tt: *mut LuaTable) {
  unsafe {
    // cpp ltable.cpp:1411-1436 luaH_clear：array 为空时 sizearray 为 0，
    // 需先判空再建切片（from_raw_parts 不接受 null，即使长度为 0）。
    if !(*tt).array.is_null() {
      let array = slice::from_raw_parts_mut((*tt).array, (*tt).sizearray as usize);
      for slot in array {
        setnilvalue!(slot);
      }
    }

    maybesetaboundary(tt, 0);

    if !eq((*tt).node, dummynode) {
      let size = sizenode!(tt);
      (*tt).union.lastfree = size;

      let nodes = slice::from_raw_parts_mut((*tt).node, size as usize);
      for n in nodes {
        n.key.value = Default::default();
        n.key.extra = [0];
        n.key.set_tt(LuaType::Nil as i32);
        setnilvalue!(addr_of_mut!(n.val));
        n.key.set_next(0);
      }
    }

    (*tt).tmcache = !0u8;
  }
}
