use crate::{
  functions::{index_2_addr::index_2_addr, lua_v_tonumber::lua_v_tonumber},
  records::lua_state::LuaState,
  type_aliases::t_value::TValue,
};

/// `lua_isnumber` 核心（cpp `VM/src/lapi.cpp:368`）。`l` 以引用传入（存活由类型
/// 保证）；`idx` 为任意（伪）索引，越界经硬化的 `index_2_addr` 返回只读哨兵槽。
/// 非数字时 `lua_v_tonumber(o,&mut n)` 仅本地试转不写回栈、可安全丢弃。
/// 只读判定，不分配、不抛错。cpp VM/src/lapi.cpp:368
pub fn lua_isnumber(l: &LuaState, idx: i32) -> i32 {
  let o = index_2_addr(l, idx);
  // SAFETY:o 为栈上有效 TValue 或只读哨兵槽；`lua_v_tonumber` 仅读 `*o`、
  // 失败时写回本地零值 TValue。
  unsafe {
    if (*o).is_number() {
      1
    } else {
      let mut n = TValue::default();
      lua_v_tonumber(&*o, &mut n).is_some() as i32
    }
  }
}
