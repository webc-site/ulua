use crate::{
  macros::{api_incr_top::api_incr_top, setnilvalue::setnilvalue},
  type_aliases::lua_state::lua_State,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
#[unsafe(export_name = "ulua_lua_pushnil")]
pub unsafe fn lua_pushnil(l: *mut lua_State) {
  unsafe {
    setnilvalue!((*l).top);
    api_incr_top!(l);
  }
}
