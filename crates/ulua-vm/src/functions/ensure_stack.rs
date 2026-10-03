//! cpp `ensure_stack_impl` / `ensure_stack`（`VM/src/lapi.cpp:57-66`）。

use crate::{
  functions::{
    lua_checkstack::lua_checkstack, lua_error::lua_error, lua_o_pushfstring::lua_o_pushfstring,
  },
  records::lua_state::LuaState,
};

/// 栈余量检查与扩容：不足 `size` 槽时尝试扩，返回 `true` 表示栈已容得下 `size`。
///
/// `size <= 0` 恒不触发（invariant `top <= ci->top`），这里显式短路，以免对
/// `top` 做负向偏移。比较也走偏移量而非指针相加，指针相加本身就要求结果
/// 仍落在同一分配内。
///
/// # Safety
/// `l` 须借用一个存活已初始化的 `LuaState`，其 `ci` 指向本线程的合法帧。
unsafe fn try_reserve_stack(l: &mut LuaState, size: i32) -> bool {
  if size <= 0 {
    return true;
  }
  // SAFETY: `l` 存活，`l.ci` 指向当前帧，其 `top` 与 `l.top` 同属一个栈数组，
  // `offset_from` 只做同分配内的槽距相对比较。
  unsafe { size as isize <= (*l.ci).top.offset_from(l.top) || lua_checkstack(l, size) != 0 }
}

/// 在 `l` 上推送并抛出 "stack overflow"，对应 cpp lapi.cpp:64
/// `luaO_pushfstring(L, "stack overflow")` 与其后的 `lua_error(L)`；发散。
///
/// # Safety
/// `l` 须借用一个存活已初始化、处于可抛错帧的 `LuaState`。
unsafe fn report_stack_overflow(l: &mut LuaState) -> ! {
  // SAFETY: 契约保证 `l` 可推送错误消息并可抛错。
  unsafe {
    lua_o_pushfstring(l, format_args!("stack overflow"));
    lua_error(l)
  }
}

/// 栈槽不足时先扩容；扩不出来就在 `error_l`（xmove 场景下是 `from`）上抛
/// "stack overflow"。缺了这一步，`api_incr_top`/结果槽写入会直接越过
/// `ci->top` 写到栈数组之外。
///
/// # Safety
/// `l` 与 `error_l` 各借用一个存活已初始化的 `LuaState`，二者同属一个
/// `global_State`；借用期内不得有其它别名同时可变访问这两个状态。xmove 场景下
/// 两者须为不同线程（`ensure_stack` 用于同线程）。
pub(crate) unsafe fn ensure_stack_impl(l: &mut LuaState, error_l: &mut LuaState, size: i32) {
  // SAFETY: 契约保证两侧存活；扩容只触碰 `l`，抛错只在扩不出来时触碰 `error_l`
  // 且随即发散，两侧的可变使用窗口不重叠。
  unsafe {
    if !try_reserve_stack(l, size) {
      report_stack_overflow(error_l);
    }
  }
}

/// cpp `ensure_stack(L, size)`，即在当前线程上报错的 `ensure_stack_impl`。
///
/// # Safety
/// 同 [`ensure_stack_impl`]，只是报错线程就是 `l`。
pub(crate) unsafe fn ensure_stack(l: &mut LuaState, size: i32) {
  // SAFETY: 契约保证 `l` 存活；`try_reserve_stack` 与 `report_stack_overflow`
  // 依次独占借用 `l`，后者发散故不与前者交叠。
  unsafe {
    if !try_reserve_stack(l, size) {
      report_stack_overflow(l);
    }
  }
}
