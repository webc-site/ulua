use ulua_common::macros::luau_assert::LUAU_UNLIKELY;

use crate::{
  enums::{tms::TMS, value_view::ValueView},
  functions::{
    call_order_tm::call_order_tm, lua_g_ordererror::lua_g_ordererror, lua_v_strcmp::lua_v_strcmp,
  },
  macros::{luai_numlt::luai_numlt, ttype::ttype},
  records::lua_state::LuaState,
  type_aliases::t_value::TValue,
};

/// cpp `luaV_lessthan`（`VM/src/lvmutils.cpp:423`）：`<` 的 tag 分发。
///
/// §11 pass B（比较/算术簇）：tag 不等先报错（`lua_g_ordererror` 不返回），两侧同 tag 后
/// 数值/字符串两条快路径的 `ttisnumber! + nvalue!`、`ttisstring! + tsvalue!` 读链收敛为
/// [`ValueView`] 成对变体 match（变体即 tag，payload 由视图带出）。快路径结果以
/// `Option` 带出 match——`call_order_tm` 会调用任意函数（可 GC、可搬栈），栈槽借用不外溢。
///
/// # Safety
/// `l` 必须指向存活 `LuaState`，操作数 `*const TValue`/`StkId` 指针可读/可写且对齐，TM 调用协议（res 槽、栈余量）满足。
pub(crate) unsafe extern "C-unwind" fn lua_v_lessthan(
  l: *mut LuaState,
  lhs: *const TValue,
  rhs: *const TValue,
) -> i32 {
  // Safety: 契约保证 `l` 为存活调用帧、TValue/StkId 指针可读/可写且对齐，块内取值、TM 调用与报错路径均在该帧栈界内
  unsafe {
    if LUAU_UNLIKELY!(ttype!(lhs) != ttype!(rhs)) {
      lua_g_ordererror(l, lhs, rhs, TMS::TmLt);
    }

    let fast = match (ValueView::from_tvalue(&*lhs), ValueView::from_tvalue(&*rhs)) {
      (ValueView::Number(a), ValueView::Number(b)) => Some(luai_numlt(a, b) as i32),
      (ValueView::String(a), ValueView::String(b)) => Some((lua_v_strcmp(a, b) < 0) as i32),
      // 其余同 tag 对（含 Boolean/Table/Function 与 Proto/Upval 等内部 tag）走元方法
      _ => None,
    };

    if let Some(res) = fast {
      return res;
    }

    call_order_tm(l, lhs, rhs, TMS::TmLt, true)
  }
}

/// # Safety
/// `l` 必须指向存活 `LuaState`，操作数 `*const TValue`/`StkId` 指针可读/可写且对齐，TM 调用协议（res 槽、栈余量）满足。
pub unsafe extern "C-unwind" fn lua_v_lessthan_export(
  l: *mut LuaState,
  lhs: *const TValue,
  rhs: *const TValue,
) -> i32 {
  // Safety: 导出壳原样转发同契约 `lua_v_lessthan`；lhs/rhs 为存活可读 TValue 指针
  unsafe { lua_v_lessthan(l, lhs, rhs) }
}
