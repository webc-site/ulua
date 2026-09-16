use crate::{
  enums::lua_type::LuaType,
  functions::{
    lua_gettop::lua_gettop, lua_l_checktype::lua_l_checktype,
    lua_pcallyieldable::lua_pcallyieldable, lua_pushvalue::lua_pushvalue, lua_replace::lua_replace,
  },
  macros::lua_multret::LUA_MULTRET,
  type_aliases::lua_state::lua_State,
};

pub(crate) unsafe extern "C-unwind" fn lua_b_xpcally(l: *mut lua_State) -> i32 {
  unsafe {
    lua_l_checktype(l, 2, LuaType::Function as i32);

    // swap function & error function
    lua_pushvalue(l, 1);
    lua_pushvalue(l, 2);
    lua_replace(l, 1);
    lua_replace(l, 2);
    // at this point the stack looks like err, f, args

    lua_pcallyieldable(l, lua_gettop(l) - 2, LUA_MULTRET, 1)
  }
}
