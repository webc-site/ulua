use crate::{
  enums::tms::TMS,
  macros::gfasttm::gfasttm,
  records::{lua_state::LuaState, lua_t_value::TValue, lua_table::LuaTable},
};

/// cpp `ltm.h:45` `#define fasttm(l, et, e) gfasttm(l->global, et, e)` 对应。
///
/// # Safety
///
/// `l` 必须指向存活的 `LuaState`；`et` 非空时必须指向有效的 `LuaTable`。
#[inline(always)]
pub unsafe fn fasttm(l: *mut LuaState, et: *mut LuaTable, e: TMS) -> *const TValue {
  // Safety: 契约保证 `l` 存活，读其 `global` 字段；`et` 按 gfasttm 契约原样转发
  unsafe { gfasttm((*l).global, et, e) }
}
