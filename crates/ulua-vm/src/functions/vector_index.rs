use core::slice::from_raw_parts;

use crate::{
  functions::{
    lua_l_checklstring::lua_l_checklstring, lua_l_checkvector::lua_l_checkvector,
    lua_pushnumber::lua_pushnumber,
  },
  macros::{lua_l_error::luaL_error, lua_vector_size::LUA_VECTOR_SIZE},
  records::lua_state::LuaState,
};

/// # Safety
/// `l` 须为存活 `LuaState` 且处于受保护帧：`luaL_checkvector(l,1)` 取 vector（`v` 指向 `LUA_VECTOR_SIZE`
/// 个元素，仅按 `ic<LUA_VECTOR_SIZE` 读 v[0..=3]）；`luaL_checklstring(l,2,&len)` 取单字符分量名（`name`
/// 覆盖 len 字节）；非法名经 `luaL_error` 抛错回退。`lua_pushnumber` 需 `(*l).top` 后 ≥1 空槽；可触发 GC。
/// cpp VM/src/lveclib.cpp:256
pub(crate) unsafe extern "C-unwind" fn vector_index(l: *mut LuaState) -> i32 {
  unsafe {
    let v = lua_l_checkvector(l, 1);
    let mut len = 0usize;
    let name = lua_l_checklstring(l, 2, &mut len);

    if len == 1 {
      let ic = (*name as i32 | 0x20) - 'x' as i32;

      const W_OFFSET: i32 = -1; // 'w' - 'x'
      let ic = if ic == W_OFFSET { 3 } else { ic as usize };

      if ic < LUA_VECTOR_SIZE as usize {
        lua_pushnumber(l, (*v.add(ic)) as f64);
        return 1;
      }
    }

    let name_bytes = from_raw_parts(name as *const u8, len);
    let name = String::from_utf8_lossy(name_bytes);
    luaL_error!(l, "attempt to index vector with '{}'", name)
  }
}
