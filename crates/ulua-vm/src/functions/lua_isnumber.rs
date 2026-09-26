use crate::{
  functions::{index_2_addr::index_2_addr, lua_v_tonumber::lua_v_tonumber},
  records::lua_state::LuaState,
  type_aliases::t_value::TValue,
};

/// # Safety
/// `l` 须为存活 `LuaState` 且 `idx` 为合法（伪）索引，使 `index_2_addr` 返回落在栈/上值区间的可读 `TValue*`；
/// 非数字时 `lua_v_tonumber(o,&mut n)` 仅本地试转不写回栈、可安全丢弃。只读判定，不分配、不抛错。
/// cpp VM/src/lapi.cpp:368
pub unsafe fn lua_isnumber(l: *mut LuaState, idx: i32) -> i32 {
  // Safety:index_2_addr 依赖 C API 契约 —— l 有效且 idx 为合法（伪）索引。
  let o = unsafe { index_2_addr(l, idx) };
  // Safety:o 指向栈上有效 TValue；tonumber_ 失败时写回零值 TValue。
  unsafe {
    if (*o).is_number() {
      1
    } else {
      let mut n = TValue::default();
      lua_v_tonumber(o, &mut n).is_some() as i32
    }
  }
}
