//! Source: `VM/src/laux.cpp:71-83` (hand-ported)

use crate::{
  functions::{
    cstr_cow, currentline::currentline, getluaproto::get_lua_proto, lua_o_chunkid::lua_o_chunkid,
    lua_o_pushfstring::lua_o_pushfstring, lua_pushlstring::lua_pushlstring,
    lua_rawcheckstack::lua_rawcheckstack,
  },
  macros::{getstr::getstr, is_lua::isLua, lua_idsize::LUA_IDSIZE},
  records::lua_state::LuaState,
};

/// # Safety
/// `l` 须为存活 `LuaState`：`level` 沿 `(*l).ci..(*l).base_ci` 帧链上跳（遇 base_ci 提前压空串返回），
/// 落点 `ci` 若 `isLua!` 则 `get_lua_proto` 非空、其 `(*proto).source` 存活（读 `len` 且 `getstr` 覆盖串体，写入
/// `chunkbuf[LUA_IDSIZE]`）；`currentline(l,ci)` 复用同帧。每处 `lua_pushlstring`/`lua_o_pushfstring` 前先
/// `lua_rawcheckstack(l,1)` 保证 `(*l).top` 后留 ≥1 槽；可触发 GC。
/// cpp VM/src/laux.cpp:72
pub(crate) unsafe fn lua_l_where(l: *mut LuaState, level: i32) {
  unsafe {
    let mut ci = (*l).ci;
    // 保留计数重复：level 是沿调用信息链上跳的帧数，每轮先与 base_ci 边界比较再 ci.sub(1)
    // 取上一层指针，循环变量不参与取数，跳动本身没有可切片化的数组
    for _ in 0..level {
      if ci == (*l).base_ci {
        lua_rawcheckstack(l, 1);
        lua_pushlstring(l, c"".as_ptr(), 0);
        return;
      }
      ci = ci.sub(1);
    }

    if isLua!(ci) {
      let proto = get_lua_proto(ci);
      let source = (*proto).source;
      let mut chunkbuf = [0; LUA_IDSIZE as usize];
      let chunkid = lua_o_chunkid(
        chunkbuf.as_mut_ptr(),
        chunkbuf.len(),
        getstr(source),
        (*source).len as usize,
      );
      let line = currentline(l, ci);
      if line > 0 {
        let chunk = cstr_cow(chunkid);
        lua_o_pushfstring(l, format_args!("{}:{}: ", chunk, line));
        return;
      }
    }

    lua_rawcheckstack(l, 1);
    lua_pushlstring(l, c"".as_ptr(), 0);
  }
}
