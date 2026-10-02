use core::ffi::c_void;

use crate::{
  functions::{
    ensure_stack::ensure_stack, index_2_addr::index_2_addr, lapi_barrier::lua_c_threadbarrier_lapi,
    lua_h_getp::lua_h_getp,
  },
  macros::{
    api_check::api_check, api_incr_top::api_incr_top, setobj_2_s::setobj_2_s, ttype::ttype,
  },
  records::lua_state::LuaState,
  type_aliases::stk_id::StkId,
};

/// `lua_rawgetptagged` 核心（cpp lapi.cpp:886）。r16-v4b 收口：接收者 `&mut LuaState`
/// 形保持，unsafe 自整块体体内内移到真实裸触点、逐点注记；`# Safety` 段改调用序契约
/// 形制。签名留 `unsafe fn` 的裁决依据：裸参 `p` 直接流入既有 `unsafe fn` 镜像点
/// `lua_h_getp`，命中 `clippy::not_unsafe_ptr_arg_deref`（该 lint 把「裸参入其它函数」
/// 一律视作可能解引用；safe 化需改被调方签名或引入全仓首例 `#[allow]`，均系禁扩面项，
/// 按纪律留形申报移交，判语同 `ulua-repl-cli` 「c-API 边界本体留 unsafe 形」先例）。
/// # Safety
/// 调用序契约（正确性，非内存安全）：`l` 为存活且独占驱动的 `LuaState`（引用形由类型
/// 承载）；`idx` 为当前帧可解析的合法索引且该槽必须是表（cpp lapi.cpp:891
/// `api_check!(ttype_is_table(gct(ptr)))`，debug 断言；release 非表由调用方违约当场
/// 错读）；`p/tag` 仅作 lightuserdata 键的位模式比较与入 TValue 载荷，本端全程不解
/// 引用（可空/可为任意整数编码值）；取回引用与压栈槽 `top.sub(1)` 处的值配套对应，
/// 返回的 ttype 即该槽 tag。
pub unsafe fn lua_rawgetptagged(l: &mut LuaState, idx: i32, p: *mut c_void, tag: i32) -> i32 {
  // SAFETY: 契约保证 `l` 存活且独占驱动（类型承载）；thread 屏障与栈扩容为 C-ABI
  // 镜像裸触点，扩容先行其后才可派生栈槽地址（「扩容先行、借用后派生」纪律）；
  // `p` 在本块仅作透明转发值，不解引用。
  unsafe {
    lua_c_threadbarrier_lapi(l);
    ensure_stack(l, 1);
  }

  let t: StkId = index_2_addr(l, idx);
  // SAFETY: 契约保证 `idx` 解析出的槽（含 `LUA_O_NILOBJECT` 哨兵）在使用点可读且为
  // 表；`lua_h_getp` 只读表取回存活槽引用，`setobj_2_s` 写顶后保留槽（扩容已先行），
  // `api_incr_top`/`ttype` 均在栈数组界内；块内 `p` 始终仅作键位模式/载荷，无解引用。
  unsafe {
    api_check!(l, (*t).is_table());

    setobj_2_s!(l, l.top, lua_h_getp((*t).as_table_ptr(), p, tag));
    api_incr_top!(l);

    ttype!(l.top.sub(1)) as i32
  }
}
