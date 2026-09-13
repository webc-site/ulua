use ulua_vm::{
  functions::{lua_isuserdata::lua_isuserdata, lua_touserdatatagged::lua_touserdatatagged},
  records::lua_state,
};

use crate::type_aliases::lua_state::LuaState;
pub fn is_type_user_data(l: *mut LuaState, idx: i32) -> bool {
  // kTypeUserdataTag is a constant used for Luau Type Function userdata.
  const K_TYPE_USERDATA_TAG: i32 = 42;

  unsafe {
    if lua_isuserdata(l as *mut lua_state::LuaState, idx) == 0 {
      return false;
    }

    let result = lua_touserdatatagged(l as *mut lua_state::LuaState, idx, K_TYPE_USERDATA_TAG);

    !result.is_null()
  }
}
