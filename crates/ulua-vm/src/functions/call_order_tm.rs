use crate::{
  enums::tms::TMS,
  functions::{
    call_t_mres::call_t_mres, lua_g_ordererror::lua_g_ordererror,
    lua_o_rawequal_obj::lua_o_rawequal_obj, lua_t_gettmbyobj::lua_t_gettmbyobj,
  },
  macros::l_isfalse::l_isfalse,
  records::lua_state::LuaState,
  type_aliases::t_value::TValue,
};

/// # Safety
/// 调用方须保证：`l` 为存活且处于可调用状态的 `lua_State`；`p1`/`p2` 指向栈帧内可读、对齐的 TValue 槽位；
/// `(*l).top` 起已为 TM 调用结果预留栈槽（`call_t_mres` 写 top 并回读，越界即破坏栈不变式）。
/// 对应 cpp lvmutils.cpp:379 `call_orderTM`，其同样假设操作数与 res 槽位于当前有效栈窗口。
pub unsafe fn call_order_tm(
  l: *mut LuaState,
  p1: *const TValue,
  p2: *const TValue,
  event: TMS,
  error: bool,
) -> i32 {
  // Safety: 契约保证 `l` 为存活调用帧、p1/p2 为可读对齐 TValue、res 可写；比较 TM 调用协议与栈余量由该帧保证
  unsafe {
    let tm1 = lua_t_gettmbyobj(l, p1, event);

    if (*tm1).is_nil() {
      if error {
        lua_g_ordererror(l, p1, p2, event);
      }
      return -1;
    }

    let tm2 = lua_t_gettmbyobj(l, p2, event);
    if lua_o_rawequal_obj(tm1, tm2) == 0 {
      if error {
        lua_g_ordererror(l, p1, p2, event);
      }
      return -1;
    }

    call_t_mres(l, (*l).top, tm1, p1, p2);
    if l_isfalse!((*l).top) { 0 } else { 1 }
  }
}
