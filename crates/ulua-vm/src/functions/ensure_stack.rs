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
/// 调用序契约（正确性，非内存安全——`l` 的存活/独占前提已由 `&mut` 接收者类型承载，r12-w6
/// 收形降为安全 `fn`）：`l` 的 `ci` 指向本线程的合法帧。体内唯一裸读为 `(*l.ci).top`——
/// LuaState 不变量保护的自有当前帧字段；槽距读数收编既有边界原语 `slot_distance`
/// （其本体即被替代式 `offset_from` 的同址镜像），`lua_checkstack` 为引用形安全门面。
fn try_reserve_stack(l: &mut LuaState, size: i32) -> bool {
  if size <= 0 {
    return true;
  }
  // SAFETY: `l.ci` 指向当前帧（契约），其 `top` 与 `l.top` 同属一个栈数组，
  // `slot_distance` 只做同分配内的槽距相对比较；i32 折形无截差（槽距受栈上限约束），
  // 与被替代式 `size as isize <= ci->top.offset_from(top)` 谓词逐位等价、短路次序不变。
  LuaState::slot_distance(l.top, unsafe { (*l.ci).top }) >= size || lua_checkstack(l, size) != 0
}

/// 在 `l` 上推送并抛出 "stack overflow"，对应 cpp lapi.cpp:64
/// `luaO_pushfstring(L, "stack overflow")` 与其后的 `lua_error(L)`；发散。
///
/// 调用序契约（正确性，非内存安全——`l` 的存活/独占由 `&mut` 承载，r12-w6 收形降为安全
/// `fn`）：`l` 处于可抛错帧；窄块内 `lua_o_pushfstring` 仍收 `*mut` 形参，`as` 重建自
/// 独占借用、借用窗止于当句（r16-v21 判例），压 msg 与抛错的次序与 cpp 逐指令一致。
fn report_stack_overflow(l: &mut LuaState) -> ! {
  // SAFETY: 契约保证 `l` 可推送错误消息并可抛错；`lua_error` 引用形安全门面直传借用。
  unsafe {
    lua_o_pushfstring(l, format_args!("stack overflow"));
  }
  lua_error(l)
}

/// 栈槽不足时先扩容；扩不出来就在 `error_l`（xmove 场景下是 `from`）上抛
/// "stack overflow"。缺了这一步，`api_incr_top`/结果槽写入会直接越过
/// `ci->top` 写到栈数组之外。
///
/// 调用序契约（正确性，非内存安全——两参均为 `&mut` 独占借用，r12-w6 收形降为安全 `fn`）：
/// `l` 与 `error_l` 各借用一个存活已初始化的 `LuaState`，二者同属一个 `global_State`；
/// 借用期内不得有其它别名同时可变访问这两个状态。xmove 场景下两者须为不同线程
/// （`ensure_stack` 用于同线程）。扩容只触碰 `l`，抛错只在扩不出来时触碰 `error_l`
/// 且随即发散，两侧的可变使用窗口不重叠。
pub(crate) fn ensure_stack_impl(l: &mut LuaState, error_l: &mut LuaState, size: i32) {
  if !try_reserve_stack(l, size) {
    report_stack_overflow(error_l);
  }
}

/// cpp `ensure_stack(L, size)`，即在当前线程上报错的 `ensure_stack_impl`。
///
/// 调用序契约（正确性，非内存安全）同 [`ensure_stack_impl`]，只是报错线程就是 `l`；
/// `try_reserve_stack` 与 `report_stack_overflow` 依次独占借用 `l`，后者发散故不与前者交叠。
pub(crate) fn ensure_stack(l: &mut LuaState, size: i32) {
  if !try_reserve_stack(l, size) {
    report_stack_overflow(l);
  }
}
