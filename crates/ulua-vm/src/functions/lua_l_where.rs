//! Source: `VM/src/laux.cpp:71-83` (hand-ported)

use crate::{
  functions::{
    cstr_cow, currentline::currentline, getluaproto::get_lua_proto, lua_o_chunkid::lua_o_chunkid,
    lua_o_pushfstring::lua_o_pushfstring, lua_pushlstring::lua_pushlstring_bytes,
    lua_rawcheckstack::lua_rawcheckstack,
  },
  macros::{getstr::getstr, is_lua::isLua, lua_idsize::LUA_IDSIZE},
  records::lua_state::LuaState,
};

/// 调用序契约（正确性，非内存安全——`l` 的存活与独占已由 `&mut LuaState` 承载，r16-v43
/// 收形、wave-6d 降为安全 `pub(crate) fn`）：体内沿 `l.ci..l.base_ci` 帧链的裸指针游走
/// （`ci.sub(1)`、`isLua!`、`(*proto).source` 读、`currentline(&*ci)`）与 `lua_o_chunkid`
/// 缓冲裸窗仍属真实裸操作，屏障按 r16-v21 判例以窄块保留。其余前提：`level` 沿
/// `l.ci..l.base_ci` 帧链上跳（遇 base_ci 提前压空串返回），落点 `ci` 若 `isLua!` 则
/// `get_lua_proto` 非空、其 `(*proto).source` 存活（读 `len` 且 `getstr` 覆盖串体，写入
/// `chunkbuf[LUA_IDSIZE]`）；`currentline(ci)` 复用同帧。每处 `lua_pushlstring_bytes`/
/// `lua_o_pushfstring` 前先 `lua_rawcheckstack(l,1)` 保证 `top` 后留 ≥1 槽；可触发 GC。
/// cpp VM/src/laux.cpp:72
pub(crate) fn lua_l_where(l: &mut LuaState, level: i32) {
  let mut ci = l.ci;
  // 保留计数重复：level 是沿调用信息链上跳的帧数，每轮先与 base_ci 边界比较再 ci.sub(1)
  // 取上一层指针，循环变量不参与取数，跳动本身没有可切片化的数组
  for _ in 0..level {
    if ci == l.base_ci {
      lua_rawcheckstack(l, 1);
      // SAFETY: `l` 存活独占由接收者引用承载（契约）；`lua_pushlstring_bytes` 借引用形写
      // 刚由 `lua_rawcheckstack(l, 1)` 预留的 top 槽，前置序与收口前逐位一致。
      unsafe { lua_pushlstring_bytes(l, b"") };
      return;
    }
    // SAFETY: 契约保证 `base_ci <= ci` 且二者同处一个 CallInfo 数组，`sub(1)` 上跳一层
    // 不越出该分配；比较侧 `ci == l.base_ci` 只读指针值，本身即安全。
    ci = unsafe { ci.sub(1) };
  }

  // SAFETY: `ci` 为帧链落点、指向存活 CallInfo（契约），`isLua!` 对 `(*ci).func` 的裸读
  // 在帧-对象存活界内。
  if unsafe { isLua!(ci) } {
    let mut chunkbuf = [0; LUA_IDSIZE as usize];
    // SAFETY: `isLua!` 为真 → `get_lua_proto(ci)` 非空且 `(*proto).source` 存活（契约）；
    // `getstr(source)` 仅取存活 TString 柔性数组成员地址；`chunkbuf` 写窗即本地
    // `[0; LUA_IDSIZE]`；`(*source).len` 同读界内；`currentline(&*ci)` 同帧纯读，
    // 借出引用止于 tuple 求值当句。左至右求值序与收口前（chunkid 后 line）逐位一致。
    let (chunkid, line) = unsafe {
      let proto = get_lua_proto(ci);
      let source = (*proto).source;
      (
        lua_o_chunkid(
          chunkbuf.as_mut_ptr(),
          chunkbuf.len(),
          getstr(source),
          (*source).len as usize,
        ),
        currentline(&*ci),
      )
    };
    if line > 0 {
      // SAFETY: `chunkid` 为 `lua_o_chunkid` 落点（`chunkbuf` 内 NUL 结尾串或源名偏移串），
      // 调用期间存活且 NUL 终止，`cstr_cow` 的 NUL 扫描必界内终止。
      let chunk = unsafe { cstr_cow(chunkid) };
      lua_o_pushfstring(l, format_args!("{}:{}: ", chunk, line));
      return;
    }
  }

  lua_rawcheckstack(l, 1);
  // SAFETY: 同前一处——`l` 存活独占由接收者引用承载，写入槽由 `lua_rawcheckstack(l, 1)`
  // 预留。
  unsafe { lua_pushlstring_bytes(l, b"") };
}
