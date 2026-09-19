use crate::{
  enums::lua_gc_op::LuaGcOp,
  functions::{lua_gc::lua_gc, lua_pushinteger::lua_pushinteger},
  type_aliases::lua_state::lua_State,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
#[cfg_attr(feature = "capi", unsafe(export_name = "ulua_lua_b_gcinfo"))]
pub(crate) unsafe extern "C-unwind" fn lua_b_gcinfo(l: *mut lua_State) -> i32 {
  unsafe {
    lua_pushinteger(l, lua_gc(l, LuaGcOp::Count as i32, 0));
    1
  }
}
