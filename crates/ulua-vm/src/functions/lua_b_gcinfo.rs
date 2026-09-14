use crate::{
  enums::lua_gc_op::LuaGcOp,
  functions::{lua_gc::lua_gc, lua_pushinteger::lua_pushinteger},
  type_aliases::lua_state::lua_State,
};

#[unsafe(export_name = "ulua_lua_b_gcinfo")]
pub(crate) unsafe extern "C-unwind" fn lua_b_gcinfo(l: *mut lua_State) -> i32 {
  unsafe {
    lua_pushinteger(l, lua_gc(l, LuaGcOp::Count as i32, 0));
    1
  }
}
