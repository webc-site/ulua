use core::ptr::addr_of_mut;

use crate::{
  enums::lua_type::LuaType,
  functions::maybesetaboundary::maybesetaboundary,
  macros::{setnilvalue::setnilvalue, sizenode::sizenode},
  records::lua_table::LuaTable,
};

/// # Safety
/// `tt` 须指向存活 `LuaTable` 且其 `array/node` 指针与 `sizearray`、`sizenode` 记录的分配
/// 尺寸一致（cpp ltable.cpp:1411）：清空窗按该元数据为界读写，`is_hash_dummy()` 判为
/// 哨兵时跳过节点区写入。
pub(crate) unsafe fn lua_h_clear(tt: *mut LuaTable) {
  unsafe {
    // cpp ltable.cpp:1411-1436 luaH_clear：数组段切 array_window_mut 共享窗清零——
    // null 数组（sizearray 为 0）与零长均归空窗，免原「显式判空 + 手工
    // from_raw_parts_mut」两段守卫（窗形内部 c_slice_mut 已容 null 配 0 长）。
    for slot in (*tt).array_window_mut() {
      setnilvalue!(slot);
    }

    maybesetaboundary(tt, 0);

    // 哨兵判据收口 is_hash_dummy（空哈希唯一判据，勿以窗长==0 判空）：
    // lastfree 复位与节点区清零按 cpp 只落实向量；窗内逐格清序与原
    // `from_raw_parts_mut(node, sizenode)` 顺次逐位一致。
    if !(*tt).is_hash_dummy() {
      let size = sizenode!(tt);
      (*tt).union.lastfree = size;

      for n in (*tt).node_window_mut() {
        n.key.value = Default::default();
        n.key.extra = [0];
        n.key.set_tt(LuaType::Nil as i32);
        setnilvalue!(addr_of_mut!(n.val));
        n.key.set_next(0);
      }
    }

    (*tt).tmcache.set(!0u8);
  }
}
