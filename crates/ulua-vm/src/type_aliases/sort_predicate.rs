use core::ffi::c_int;

use crate::type_aliases::{lua_state::lua_State, t_value::TValue};

pub type SortPredicate = Option<
  unsafe extern "C-unwind" fn(l: *mut lua_State, l: *const TValue, r: *const TValue) -> c_int,
>;
