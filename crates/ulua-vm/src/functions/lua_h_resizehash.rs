use crate::{
  functions::resize::resize,
  records::{lua_state::LuaState, lua_table::LuaTable},
};

/// # Safety
/// `l` 须存活（重分配经其分配器，可抛 ERR_MEM）；`t` 须为存活 `LuaTable` 且 `nhsize >= 0` 为期望的
/// 哈希部大小。`resize` 会重建 node 区并释放旧节点：调用方不得再持有 `t` 内旧 LuaNode/TValue 槽指针，
/// 否则悬垂写。cpp ltable.cpp:709。
pub(crate) unsafe fn lua_h_resizehash(l: *mut LuaState, t: *mut LuaTable, nhsize: i32) {
  // Safety: 契约保证 `t` 为存活 LuaTable 且 nhsize 为合法目标哈希大小，块内经 rehash 重建 node 区并搬运键值
  unsafe {
    resize(l, t, (*t).sizearray, nhsize);
  }
}
