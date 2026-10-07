use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::{c_slice_mut, lua_d_call::lua_d_call},
  macros::{
    lua_d_checkstack::luaD_checkstack, restorestack::restorestack, savestack::savestack,
    setobj_2_s::setobj_2_s,
  },
  records::lua_state::LuaState,
  type_aliases::{stk_id::StkId, t_value::TValue},
};

/// # Safety
/// `l` 必须指向存活 `lua_State`，操作数 `*const TValue`/`StkId` 指针可读/可写且对齐，TM 调用协议（res 槽、栈余量）满足。
pub(crate) unsafe fn call_t_mres(
  l: *mut LuaState,
  mut res: StkId,
  f: *const TValue,
  p1: *const TValue,
  p2: *const TValue,
) -> StkId {
  // SAFETY: 契约保证 `l` 存活、tm 为可调用值、res 可写且 p1/p2（或 p2 为 null 时单参）满足元方法调用约定
  unsafe {
    let result = savestack!(l, res);

    // r12-w7a2 收编：cpp `luaT_callTMres` 同形首步即 `StkId top = L->top;` 单读——
    // 界断言与三槽写窗共用这一次预绑定（其间无任何栈操作），原两处场域重读
    // 收为等价窗读；断言求值点与界判语义原位不动
    let top: StkId = (*l).top;
    LUAU_ASSERT!(top.add(3) < (*l).stack.add((*l).stacksize as usize));

    // 栈窗口 top..top+3：上方断言保证界内，三槽写收为 c_slice_mut 视图，
    // 写仍逐序落在 checkstack 之前
    let args = c_slice_mut(top, 3);
    setobj_2_s!(l, &raw mut args[0], f);
    setobj_2_s!(l, &raw mut args[1], p1);
    setobj_2_s!(l, &raw mut args[2], p2);

    luaD_checkstack!(l, 3);
    (*l).advance_top(3); // 收编：裸场域抬顶经 advance_top 原语（同址现读同宽偏移）

    // 保留（恢复点后现读）：checkstack 可搬栈，被调函数槽必须现读场域——
    // cpp 同形最小读面 `luaD_call(L, L->top - 3, 1)`，无可收编的重复重读
    lua_d_call(l, (*l).top.offset(-3), 1);

    res = restorestack!(l, result);
    (*l).rewind_top(1); // 收编：裸场域回落经 rewind_top 原语
    setobj_2_s!(l, res, (*l).top);

    res
  }
}
