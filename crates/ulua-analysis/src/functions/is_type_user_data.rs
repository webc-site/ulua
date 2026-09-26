use ulua_vm::{
  functions::{lua_isuserdata::lua_isuserdata, lua_touserdatatagged::lua_touserdatatagged},
  records::lua_state,
};

use crate::type_aliases::lua_state::LuaState;
pub fn is_type_user_data(l: *mut LuaState, idx: i32) -> bool {
  // kTypeUserdataTag is a constant used for Luau Type Function userdata.
  const K_TYPE_USERDATA_TAG: i32 = 42;

  // Safety: `l` 由 Lua VM 运行时约定传入并全程存活，`l as *mut lua_state::LuaState` 为同址重解释
  // （不改变地址）。`lua_isuserdata` 返回整数、`lua_touserdatatagged` 返回 tag 匹配时的数据指针或
  // null，二者均按 VM 约定以 `l`+`idx` 索引栈；此处仅比较返回整数、对指针做 `is_null()` 判定，
  // 不解引用任何指针。单线程串行，无别名冲突。
  unsafe {
    if lua_isuserdata(l as *mut lua_state::LuaState, idx) == 0 {
      return false;
    }

    let result = lua_touserdatatagged(l as *mut lua_state::LuaState, idx, K_TYPE_USERDATA_TAG);

    !result.is_null()
  }
}
