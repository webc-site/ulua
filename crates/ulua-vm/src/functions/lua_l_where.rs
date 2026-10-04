//! Source: `VM/src/laux.cpp:71-83` (hand-ported)

use alloc::string::String;

use crate::{
  functions::{
    currentline::currentline,
    getluaproto::get_lua_proto,
    lua_o_chunkid::{chunkid_slice, lua_o_chunkid_ref},
    lua_o_pushfstring::lua_o_pushfstring,
    lua_pushlstring::lua_pushlstring_bytes,
    lua_rawcheckstack::lua_rawcheckstack,
    tstr_bytes::{cut_at_nul, tstr_bytes},
  },
  macros::{is_lua::isLua, lua_idsize::LUA_IDSIZE},
  records::lua_state::LuaState,
};

/// 调用序契约（正确性，非内存安全——`l` 的存活与独占已由 `&mut LuaState` 承载，r16-v43
/// 收形、wave-6d 降为安全 `pub(crate) fn`）：体内沿 `l.ci..l.base_ci` 帧链的裸指针游走
/// （`ci.sub(1)`、`isLua!`、`(*proto).source` 读、`currentline(&*ci)`）与 `tstr_bytes`
/// 对驻留 TString 的串体折窗仍属真实裸操作，屏障按 r16-v21 判例以窄块保留；chunkid
/// 核心已收为栈上切片缓冲（`lua_o_chunkid_ref`/`chunkid_slice`），无裸窗。其余前提：
/// `level` 沿 `l.ci..l.base_ci` 帧链上跳（遇 base_ci 提前压空串返回），落点 `ci` 若
/// `isLua!` 则 `get_lua_proto` 非空、其 `(*proto).source` 存活（读 `len` 且串体覆盖
/// payload，写入本地 `chunkbuf[LUA_IDSIZE]`）；`currentline(ci)` 复用同帧。每处
/// `lua_pushlstring_bytes`/`lua_o_pushfstring` 前先 `lua_rawcheckstack(l,1)` 保证 `top`
/// 后留 ≥1 槽；可触发 GC。
/// cpp VM/src/laux.cpp:72
pub(crate) fn lua_l_where(l: &mut LuaState, level: i32) {
  let mut ci = l.ci;
  // 保留计数重复：level 是沿调用信息链上跳的帧数，每轮先与 base_ci 边界比较再 ci.sub(1)
  // 取上一层指针，循环变量不参与取数，跳动本身没有可切片化的数组
  for _ in 0..level {
    if ci == l.base_ci {
      lua_rawcheckstack(l, 1);
      lua_pushlstring_bytes(l, b"");
      return;
    }
    // SAFETY: 契约保证 `base_ci <= ci` 且二者同处一个 CallInfo 数组，`sub(1)` 上跳一层
    // 不越出该分配；比较侧 `ci == l.base_ci` 只读指针值，本身即安全。
    ci = unsafe { ci.sub(1) };
  }

  // SAFETY: `ci` 为帧链落点、指向存活 CallInfo（契约），`isLua!` 对 `(*ci).func` 的裸读
  // 在帧-对象存活界内。
  if unsafe { isLua!(ci) } {
    // §10：chunkid 切片核心就地写栈上 `[u8; LUA_IDSIZE]`，观察面按旧 C 串
    // 扫描读（首 NUL）等值截断，无指针垫片
    let mut chunkbuf = [0u8; LUA_IDSIZE as usize];
    // SAFETY: `isLua!` 为真 → `get_lua_proto(ci)` 非空且 `(*proto).source` 存活（契约）；
    // `tstr_bytes` 单点折出恰覆盖 payload 的串体窗（`(*source).len` 读界内）；
    // `currentline(&*ci)` 同帧纯读，借出窗止于本调用后续读点。左至右求值序与收口前
    // （chunkid 后 line）逐位一致。
    let (src, line) = unsafe {
      let proto = get_lua_proto(ci);
      (tstr_bytes((*proto).source), currentline(&*ci))
    };
    let site = lua_o_chunkid_ref(&mut chunkbuf, src);
    if line > 0 {
      let chunk = String::from_utf8_lossy(cut_at_nul(chunkid_slice(&chunkbuf, src, site)));
      lua_o_pushfstring(l, format_args!("{}:{}: ", chunk, line));
      return;
    }
  }

  lua_rawcheckstack(l, 1);
  lua_pushlstring_bytes(l, b"");
}
