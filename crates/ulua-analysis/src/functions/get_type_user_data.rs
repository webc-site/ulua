use ulua_vm::functions::{
  lua_l_typeerror_l::lua_l_typeerror_l, lua_touserdatatagged::lua_touserdatatagged,
};
// kTypeUserdataTag is a constant used for Luau Type Function userdata.
use ulua_vm::records::lua_state;

use crate::type_aliases::{lua_state::LuaState, type_function_type_id::TypeFunctionTypeId};
const K_TYPE_USERDATA_TAG: i32 = 42;

pub fn get_type_user_data(l: *mut LuaState, idx: i32) -> TypeFunctionTypeId {
  // Safety: `l` 由 Lua VM 运行时约定传入并全程存活，`l as *mut lua_state::LuaState` 为同址重解释。
  // `lua_touserdatatagged` 在 tag 匹配时返回 VM 为该 userdata 持有的数据指针、否则返回 null；
  // `*typ` 仅在 `!typ.is_null()` 守卫后读取，指向 VM 分配且在本次调用内存活的 TypeFunctionTypeId。
  // 不匹配时以 `lua_l_typeerror_l`（返回 `!`）抛类型错误、不返回。单线程串行，无并发别名。
  unsafe {
    let typ = lua_touserdatatagged(l as *mut lua_state::LuaState, idx, K_TYPE_USERDATA_TAG)
      as *mut TypeFunctionTypeId;

    if !typ.is_null() {
      return *typ;
    }

    lua_l_typeerror_l(l as *mut lua_state::LuaState, idx, "type");
  }
}
