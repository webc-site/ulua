use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::{c_slice_mut, lua_d_call::lua_d_call},
  macros::{lua_d_checkstack::luaD_checkstack, setobj_2_s::setobj_2_s},
  records::lua_state::LuaState,
  type_aliases::{stk_id::StkId, t_value::TValue},
};

/// # Safety
/// `l` 必须指向存活 `lua_State`，操作数 `*const TValue`/`StkId` 指针可读/可写且对齐，TM 调用协议（res 槽、栈余量）满足。
pub(crate) unsafe fn call_tm(
  l: *mut LuaState,
  f: *const TValue,
  p1: *const TValue,
  p2: *const TValue,
  p3: *const TValue,
) {
  // SAFETY: 契约保证 `l` 为存活调用帧、tm 为存活闭包值、p1/p2 可读；luaD_call 重入所需栈空间由调用方预留
  unsafe {
    // r12-w7a2 收编：界断言与四槽写窗共用一次 `(*l).top` 预绑定（其间无栈操作，
    // 等价窗读；cpp 同函数亦以单一 `StkId top` 局部驱动写窗），断言求值点原位不动
    let top: StkId = (*l).top;
    LUAU_ASSERT!(top.add(4) < (*l).stack.add((*l).stacksize as usize));

    // 栈窗口 top..top+4：上方断言保证界内，四槽写收为 c_slice_mut 视图；
    // 写仍逐序落在 checkstack 之前
    let args = c_slice_mut(top, 4);
    setobj_2_s!(l, &raw mut args[0], f);
    setobj_2_s!(l, &raw mut args[1], p1);
    setobj_2_s!(l, &raw mut args[2], p2);
    setobj_2_s!(l, &raw mut args[3], p3);

    luaD_checkstack!(l, 4);
    (*l).advance_top(4); // 收编：裸场域抬顶经 advance_top 原语

    // 保留（恢复点后现读）：checkstack 可搬栈，被调函数槽须现读场域——
    // cpp 同形最小读面，无可收编的重复重读
    lua_d_call(l, (*l).top.offset(-4), 0);
  }
}
