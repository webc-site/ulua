use core::slice::from_raw_parts;

use crate::{
  functions::{lua_l_checklstring::lua_l_checklstring, lua_l_checkvector::lua_l_checkvector},
  macros::{lua_l_error::luaL_error, lua_lib_fn::lua_lib_fn, lua_vector_size::LUA_VECTOR_SIZE},
  records::lua_state::LuaState,
};

/// # Safety
/// `l` 须为存活 `LuaState` 且处于受保护帧：`luaL_checkvector(l,1)` 取 vector（`v` 指向 `LUA_VECTOR_SIZE`
/// 个元素，仅按 `ic<LUA_VECTOR_SIZE` 读 v[0..=3]）；`luaL_checklstring(l,2,&len)` 取单字符分量名（`name`
/// 覆盖 len 字节）；非法名经 `luaL_error` 抛错回退。`lua_pushnumber` 需 `(*l).top` 后 ≥1 空槽；可触发 GC。
/// cpp VM/src/lveclib.cpp:256
pub(crate) unsafe fn vector_index(l: *mut LuaState) -> i32 {
  unsafe {
    let v = lua_l_checkvector(&mut *l, 1);
    let mut len = 0usize;
    let name = lua_l_checklstring(&mut *l, 2, &mut len);

    if len == 1 {
      let ic = (*name as i32 | 0x20) - 'x' as i32;

      const W_OFFSET: i32 = -1; // 'w' - 'x'
      let ic = if ic == W_OFFSET { 3 } else { ic as usize };

      if ic < LUA_VECTOR_SIZE as usize {
        (*l).push_number((*v.add(ic)) as f64);
        return 1;
      }
    }

    let name_bytes = from_raw_parts(name as *const u8, len);
    let name = String::from_utf8_lossy(name_bytes);
    luaL_error!(l, "attempt to index vector with '{}'", name)
  }
}

lua_lib_fn!(pub(crate) fn vector_index, vector_index_arm);

// r7-tprod2 尾矿台账（本文件票面 1 枚：让 1）——:36 `String::from_utf8_lossy`
// 仅错误臂可达（len==1 快路径已早返；失配名进 luaL_error 即 unwind），合法
// UTF-8 走 Borrowed 零堆配，剥壳运行期省 0；文案须逐字节对齐 cpp lveclib.cpp:280
// `'%s'`，Cow 即下限形态，不动。
