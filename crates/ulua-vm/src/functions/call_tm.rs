use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::{c_slice_mut, lua_d_call::lua_d_call},
  macros::{lua_d_checkstack::luaD_checkstack, setobj_2_s::setobj_2_s},
  records::lua_state::LuaState,
  type_aliases::t_value::TValue,
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
  // Safety: 契约保证 `l` 为存活调用帧、tm 为存活闭包值、p1/p2 可读；luaD_call 重入所需栈空间由调用方预留
  unsafe {
    LUAU_ASSERT!((*l).top.offset(4) < (*l).stack.add((*l).stacksize as usize));

    // 栈窗口 top..top+4：上方断言保证界内，四槽写收为 c_slice_mut 视图，
    // 取代对 `(*l).top` 的五连裸指针重读/偏移；写仍逐序落在 checkstack 之前
    let args = c_slice_mut((*l).top, 4);
    setobj_2_s!(l, &raw mut args[0], f);
    setobj_2_s!(l, &raw mut args[1], p1);
    setobj_2_s!(l, &raw mut args[2], p2);
    setobj_2_s!(l, &raw mut args[3], p3);

    luaD_checkstack!(l, 4);
    (*l).top = (*l).top.add(4);

    lua_d_call(l, (*l).top.offset(-4), 0);
  }
}
