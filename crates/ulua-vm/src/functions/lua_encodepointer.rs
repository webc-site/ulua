use crate::records::lua_state::LuaState;

/// 指针混淆编码（`lua_encodepointer`）。`l` 以引用传入（存活由类型保证）；其 `global`
/// 存活且 `ptrenckey[..PTRENCKEY_LEN]` 已初始化为 `lua_State` 结构不变量（读
/// `PTRENCKEY_LEN` 个 u64 密钥做混淆乘加）；`p` 仅为待混淆的数值，本函数不解引用 `p`、
/// 无副作用、不分配、不抛错。
/// cpp VM/src/lapi.cpp:1831
pub fn lua_encodepointer(l: &LuaState, p: usize) -> usize {
  let p = p as u64;
  // SAFETY: `g` 为存活 LuaState 挂接的 global_State（结构不变量），块内只读
  // ptrenckey 密钥数组。
  let ptrenckey = unsafe { (*l.global).ptrenckey };

  let result = (ptrenckey[0].wrapping_mul(p).wrapping_add(ptrenckey[2]))
    ^ (ptrenckey[1].wrapping_mul(p).wrapping_add(ptrenckey[3]));

  result as usize
}
