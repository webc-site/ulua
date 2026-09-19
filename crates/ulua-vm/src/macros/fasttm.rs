use crate::{
  macros::gfasttm::gfasttm,
  records::{lua_state::lua_State, lua_t_value::TValue, lua_table::LuaTable},
};

/// # Safety
///
/// `l` must be a valid pointer to a live `lua_State`.
#[inline(always)]
pub unsafe fn fasttm(l: *mut lua_State, et: *mut LuaTable, e: i32) -> *const TValue {
  unsafe { gfasttm((*l).global, et, e) }
}
