use crate::{macros::lua_callinfo_native::LUA_CALLINFO_NATIVE, records::lua_state::LuaState};

/// # Safety
/// `l` 须为存活 LuaState 且 `base_ci <= ci`、CallInfo 数组连续（`ci.offset_from(base_ci)` 给帧深度，`offset(-level)` 定位帧）；
/// `level` 非负且 `< ci-base_ci`（越界时安全返回 0），命中的 `(*ci).flags` 可读。不抛错/不分配/不触碰 GC。
/// cpp/VM/src/ldebug.cpp:459 luaG_isnative。
pub unsafe fn lua_g_isnative(l: *mut LuaState, level: i32) -> i32 {
  unsafe {
    if (level as u32) >= ((*l).ci.offset_from((*l).base_ci) as u32) {
      return 0;
    }

    let ci = (*l).ci.offset(-level as isize);
    if ((*ci).flags & LUA_CALLINFO_NATIVE as u32) != 0 {
      1
    } else {
      0
    }
  }
}
