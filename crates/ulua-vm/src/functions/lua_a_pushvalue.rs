use crate::{
  functions::ensure_stack::ensure_stack, macros::api_incr_top::api_incr_top,
  records::lua_state::lua_State, type_aliases::t_value::TValue,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
#[cfg_attr(feature = "capi", unsafe(export_name = "ulua_luaA_pushvalue"))]
pub unsafe fn lua_a_pushvalue(l: *mut lua_State, o: *const TValue) {
  unsafe {
    ensure_stack(l, 1);
    *(*l).top = *o;
    api_incr_top!(l);
  }
}

pub use lua_a_pushvalue as luaA_pushvalue;
