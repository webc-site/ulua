use crate::{
  enums::tms::TMS,
  macros::gfasttm::gfasttm,
  records::{lua_state::lua_State, lua_t_value::TValue, lua_table::LuaTable},
};

/// cpp `ltm.h:45` `#define fasttm(l, et, e) gfasttm(l->global, et, e)` 对应。
///
/// # Safety
///
/// `l` 必须指向存活的 `lua_State`；`et` 非空时必须指向有效的 `LuaTable`。
#[inline(always)]
pub unsafe fn fasttm(l: *mut lua_State, et: *mut LuaTable, e: TMS) -> *const TValue {
  unsafe { gfasttm((*l).global, et, e) }
}
