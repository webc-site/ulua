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
/// 形保持，unsafe 自整块体体内内移到真实裸触点、逐点注记。r16-v4c 步骤 3（v4b 留形
/// 点回头）：`pub unsafe fn` → `pub fn`——被调方 [`lua_h_getp`] 已在 v4c 步骤 1 safe
/// 化，裸参 `p` 此后只入 safe 被调实参位（位模式比较/入哈希/入 TValue 载荷，全链无
/// 解引用），`clippy::not_unsafe_ptr_arg_deref` 触发条件消亡；`p: *mut c_void` 保持
/// 按值裸形不切片化（本函数全程不解引用，切片化=语义扩面）。体内真实裸触点窗（C-ABI
/// 屏障/扩容原语、槽解引用/搬栈/读 tag）保持 v4b 两窄窗不动。
///
/// # Safety
/// 调用序契约（正确性，非内存安全；safe fn 文档断言，由调用方承载）：`l` 为存活且独占
/// 驱动的 `LuaState`（引用形由类型承载）；`idx` 为当前帧可解析的合法索引且该槽必须是
/// 表（cpp lapi.cpp:891 `api_check!(ttype_is_table(gct(ptr)))`，debug 断言；release 非
/// 表由调用方违约当场错读）；`p/tag` 仅作 lightuserdata 键的位模式比较与入 TValue
/// 载荷，本端全程不解引用（可空/可为任意整数编码值）；取回引用与压栈槽 `top.sub(1)`
/// 处的值配套对应，返回的 ttype 即该槽 tag。
pub fn lua_rawgetptagged(l: &mut LuaState, idx: i32, p: *mut c_void, tag: i32) -> i32 {
  // SAFETY: 契约保证 `l` 存活且独占驱动（类型承载）；thread 屏障与栈扩容为 C-ABI
  // 镜像裸触点，扩容先行其后才可派生栈槽地址（「扩容先行、借用后派生」纪律）；
  // `p` 在本块仅作透明转发值，不解引用。
  unsafe {
    lua_c_threadbarrier_lapi(l);
    ensure_stack(l, 1);
  }

  let t: StkId = index_2_addr(l, idx);
  // SAFETY: 契约保证 `idx` 解析出的槽（含 `LUA_O_NILOBJECT` 哨兵）在使用点可读且为
  // 表；`(*t).as_table()` 按表槽不变式物化共享借用（一句一借，借用窗止于本语句），
  // `lua_h_getp` 已 safe 化、只读表取回存活槽指针，`setobj_2_s` 写顶后保留槽（扩容已
  // 先行），`api_incr_top`/`ttype` 均在栈数组界内；块内 `p` 始终仅作键位模式/载荷入
  // safe 被调实参位，无解引用。
  unsafe {
    api_check!(l, (*t).is_table());

    setobj_2_s!(l, l.top, lua_h_getp((*t).as_table(), p, tag));
    api_incr_top!(l);

    ttype!(l.top.sub(1)) as i32
  }
}
