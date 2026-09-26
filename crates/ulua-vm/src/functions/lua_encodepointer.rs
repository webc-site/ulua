use crate::records::lua_state::LuaState;

/// # Safety
/// `l` 须为存活 `LuaState` 且 `(*l).global` 存活，其 `ptrenckey[0..4]` 已由 `freetrystackcallback` 初始化（读 4 个
/// u64 密钥做混淆乘加）；`p` 仅为待混淆的数值，本函数不解引用 `p`、无副作用、不分配、不抛错。
/// cpp VM/src/lapi.cpp:1831
pub unsafe fn lua_encodepointer(l: *mut LuaState, p: usize) -> usize {
  unsafe {
    let g = (*l).global;
    let p = p as u64;
    let ptrenckey = (*g).ptrenckey;

    let result = (ptrenckey[0].wrapping_mul(p).wrapping_add(ptrenckey[2]))
      ^ (ptrenckey[1].wrapping_mul(p).wrapping_add(ptrenckey[3]));

    result as usize
  }
}
