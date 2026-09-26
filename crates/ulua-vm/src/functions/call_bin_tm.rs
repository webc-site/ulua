use crate::{
  enums::tms::TMS,
  functions::{call_t_mres::call_t_mres, lua_t_gettmbyobj::lua_t_gettmbyobj},
  records::lua_state::LuaState,
  type_aliases::{stk_id::StkId, t_value::TValue},
};

/// # Safety
///
/// `l` 必须指向存活 `lua_State`，操作数 `*const TValue`/`StkId` 指针可读/可写且对齐，TM 调用协议（res 槽、栈余量）满足。
pub(crate) unsafe fn call_bin_tm(
  l: *mut LuaState,
  p1: *const TValue,
  p2: *const TValue,
  res: StkId,
  event: TMS,
) -> i32 {
  // Safety: 契约保证 `l` 为存活调用帧、p1/p2 为可读对齐 TValue、res 为可写栈槽；TM 查找与 call_tm 协议在该帧栈界内
  unsafe {
    let mut tm = lua_t_gettmbyobj(l, p1, event); // try first operand
    if (*tm).is_nil() {
      tm = lua_t_gettmbyobj(l, p2, event); // try second operand
    }
    if (*tm).is_nil() {
      return 0;
    }
    call_t_mres(l, res, tm, p1, p2);
    1
  }
}
