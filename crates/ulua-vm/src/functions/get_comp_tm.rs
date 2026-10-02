use core::ptr::null;

use crate::{
  enums::tms::TMS,
  functions::lua_o_rawequal_obj::lua_o_rawequal_obj,
  macros::fasttm::fasttm,
  records::{lua_state::LuaState, lua_table::LuaTable},
  type_aliases::t_value::TValue,
};

/// # Safety
///
/// `l` 必须指向存活 `lua_State`，操作数 `*const TValue`/`StkId` 指针可读/可写且对齐，TM 调用协议（res 槽、栈余量）满足。
/// `mt1`/`mt2` 为可空只读裸指针（元表缺席即 NULL，与 `fasttm` 的可空入约同形）：非空时须指向存活 `LuaTable`；
/// 本函数仅读二者判定同表与取元方法，不写表。
pub(crate) unsafe fn get_comp_tm(
  l: *mut LuaState,
  mt1: *const LuaTable,
  mt2: *const LuaTable,
  event: TMS,
) -> *const TValue {
  // SAFETY: 契约保证 `l` 的 global 比较函数表存活且成员 comp 字段为已注册的非空函数指针，取回后调用约定匹配操作数类型
  unsafe {
    let tm1 = fasttm(l, mt1, event);

    if tm1.is_null() {
      return null();
    }

    if mt1 == mt2 {
      return tm1;
    }

    let tm2 = fasttm(l, mt2, event);
    if tm2.is_null() {
      return null();
    }

    if lua_o_rawequal_obj(&*tm1, &*tm2) != 0 {
      return tm1;
    }

    null()
  }
}
