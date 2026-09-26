use crate::{records::lua_state::LuaState, type_aliases::t_value::TValue};

pub type SortPredicate =
  Option<unsafe extern "C-unwind" fn(l: *mut LuaState, l: *const TValue, r: *const TValue) -> i32>;
