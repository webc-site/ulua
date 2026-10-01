use core::ffi::c_char;

use crate::{
  functions::cstr_bytes,
  macros::{
    api_check::api_check, fixedbit::FIXEDBIT, l_setbit::l_setbit, lua_lutag_limit::LUA_LUTAG_LIMIT,
    lua_s_new::lua_s_new,
  },
  records::lua_state::LuaState,
};

/// # Safety
/// `l` 须为存活 LuaState 且 `(*l).global.lightuserdataname[..LUA_LUTAG_LIMIT]` 可寻址（`api_check` 断言 `tag` 在界内，
/// release 由调用方保证）；`name` 须为 NUL 结尾 C 串（`lua_s_new` intern 后写入并 `l_setbit` 置 FIXEDBIT 免回收）；
/// 目标槽须原本为空（不支持重命名）。cpp/VM/src/lapi.cpp:2097 lua_setlightuserdataname。
pub unsafe fn lua_setlightuserdataname(l: *mut LuaState, tag: i32, name: *const c_char) {
  unsafe {
    api_check!(l, (tag as u32) < LUA_LUTAG_LIMIT as u32);
    // renaming not supported
    api_check!(l, (*(*l).global).lightuserdataname[tag as usize].is_null());

    if (*(*l).global).lightuserdataname[tag as usize].is_null() {
      // 入参为 NUL 结尾 C 串，经 cstr_bytes 扫首个 NUL 得字节切片（保持原 lua_s_new 的 strlen 语义）
      let ts = lua_s_new(l, cstr_bytes(name));
      (*(*l).global).lightuserdataname[tag as usize] = ts;
      l_setbit!((*ts).hdr.marked, FIXEDBIT); // never collect these names
    }
  }
}
