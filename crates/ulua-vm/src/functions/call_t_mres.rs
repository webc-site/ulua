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
  // Safety: 契约保证 `l` 存活、tm 为可调用值、res 可写且 p1/p2（或 p2 为 null 时单参）满足元方法调用约定
  unsafe {
    let result = savestack!(l, res);

    LUAU_ASSERT!((*l).top.offset(3) < (*l).stack.add((*l).stacksize as usize));

    // 栈窗口 top..top+3：上方断言保证界内，三槽写收为 c_slice_mut 视图，
    // 取代对 `(*l).top` 的四连裸指针重读/偏移；写仍逐序落在 checkstack 之前
    let args = c_slice_mut((*l).top, 3);
    setobj_2_s!(l, &raw mut args[0], f);
    setobj_2_s!(l, &raw mut args[1], p1);
    setobj_2_s!(l, &raw mut args[2], p2);

    luaD_checkstack!(l, 3);
    (*l).top = (*l).top.add(3);

    lua_d_call(l, (*l).top.offset(-3), 1);

    res = restorestack!(l, result);
    (*l).top = (*l).top.offset(-1);
    setobj_2_s!(l, res, (*l).top);

    res
  }
}
