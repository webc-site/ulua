use crate::{macros::lua_callinfo_native::LUA_CALLINFO_NATIVE, records::lua_state::LuaState};

/// `lua_g_isnative`（cpp/VM/src/ldebug.cpp:459 luaG_isnative）。r16-v4b 引用形前移取
/// `&LuaState`（纯帧槽读数，不抛错/不分配/不触碰 GC；`l` 存活由类型承载），
/// `unsafe fn` 消亡转 safe fn，体内 ci 裸算术留存全部内移到真实裸触点 unsafe 块。
/// 调用序契约（正确性，非内存安全）：`base_ci <= ci` 且 CallInfo 数组连续
/// （`ci.offset_from(base_ci)` 给帧深度，`offset(-level)` 定位帧）；`level` 非负且
/// `< ci-base_ci`（越界时先短路安全返回 0，offset 只在判界后执行），命中的
/// `(*ci).flags` 可读。
pub fn lua_g_isnative(l: &LuaState, level: i32) -> i32 {
  // SAFETY: 契约保证 `l` 存活（类型承载）、`ci`/`base_ci` 为同数组内合法帧指针，
  // `offset_from` 仅作帧深度读数、`offset(-level)` 在判界短路后才执行、所得帧
  // `flags` 字段可读；块内无写点无重入。
  unsafe {
    if (level as u32) >= (l.ci.offset_from(l.base_ci) as u32) {
      return 0;
    }

    let ci = l.ci.offset(-level as isize);
    if ((*ci).flags & LUA_CALLINFO_NATIVE as u32) != 0 {
      1
    } else {
      0
    }
  }
}
