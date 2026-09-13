use core::ffi::c_int;

use crate::type_aliases::lua_state::lua_State;
pub type LuaCFunction = Option<unsafe extern "C-unwind" fn(l: *mut lua_State) -> c_int>;

pub use LuaCFunction as LuaCfunction;
pub use LuaCFunction as lua_CFunction;
