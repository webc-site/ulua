use core::ffi::c_void;

use crate::{
  functions::{
    index_2_addr::index_2_addr, lua_g_readonlyerror::check_writable, lua_h_setp::lua_h_setp,
  },
  macros::{
    api_check::api_check, api_checknelems::api_checknelems, lua_c_barriert::luaC_barriert,
    setobj_2_t::setobj2t,
  },
  records::lua_state::LuaState,
  type_aliases::stk_id::StkId,
};

/// `lua_rawsetptagged` 核心（cpp `lapi.cpp:1055`）。r16-v4b 收口：接收者 `&mut LuaState`
/// 形保持，unsafe 自整块体体内内移到真实裸触点、逐点注记。r16-v4c 步骤 3（v4b 留形
/// 点回头）：`pub unsafe fn` → `pub fn`——被调方 [`lua_h_setp`] 已在 v4c 步骤 2 safe
/// 化（其内部窗自带逐点 SAFETY 注记），裸参 `p` 此后只入 safe 被调实参位（位模式键，
/// 全链无解引用），`clippy::not_unsafe_ptr_arg_deref` 触发条件消亡；`p: *mut c_void`
/// 保持按值裸形不切片化。体内真实裸触点（槽解引用、`check_writable`/取槽/写值/屏障/
/// rehash 族）收口为 v4b 同款单窄 unsafe 窗不动。
///
/// # Safety
/// 调用序契约（正确性，非内存安全；safe fn 文档断言，由调用方承载）：`l` 为存活且独占
/// 驱动的 `LuaState`（引用形由类型承载）；`l.top` 之下留有 1 个值元素（`api_checknelems
/// 1`）作为待写入值；`idx` 经 `index_2_addr` 解析出的槽须为 table（`api_check is_table`）
/// 且非只读（否则 `lua_g_readonlyerror` 抛错）；`p` 为用作 light-C 指针键的裸地址（仅作
/// 位模式键入表、不被 GC 追踪，本端全程不解引用），`tag` 为其标签整数；`setobj2t`/屏障
/// 可触发 GC，须受保护帧。
pub fn lua_rawsetptagged(l: &mut LuaState, idx: i32, p: *mut c_void, tag: i32) {
  // SAFETY: 契约保证 `l` 存活且独占驱动（类型承载）、`idx` 解析出的槽可读且为表、
  // 顶下 1 值元素可读；`check_writable` 按 cpp C-ABI 契约消费 `l`/表裸形（`&mut`
  // 隐式转裸实参系本仓既有形制，同 `lua_rawset` 先例），`lua_h_setp` 已 safe 化、其
  // l/t 裸形纯转发（写点收口在其内部窗，见 callee 文档）；`val` 经 `top_slot` 读数
  // 原语在写点前现取，`setobj2t`/屏障/rehash 均不改写栈顶字段（同 `lua_rawset` 论
  // 证）；块内 `p` 仅作键位模式入 safe 被调实参位，无解引用。
  unsafe {
    api_checknelems!(l, 1);
    let o: StkId = index_2_addr(l, idx);
    api_check!(l, (*o).is_table());
    check_writable(l, (*o).as_table_ptr());
    let val = l.top_slot(-1);
    setobj2t!(l, lua_h_setp(l, (*o).as_table_ptr(), p, tag), val);
    luaC_barriert!(l, (*o).as_table_ptr(), val);
    // 弹栈一格：`rewind_top(1)` 提交原语镜像原顶回退一格的落值形
    l.rewind_top(1);
  }
}
