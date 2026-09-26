use crate::{
  enums::lua_type::LuaType,
  functions::{
    lua_newuserdatatagged::lua_newuserdatatagged, lua_setmetatable::lua_setmetatable,
    lua_toboolean::lua_toboolean, lua_type::lua_type,
  },
  macros::{
    lua_l_argexpected::luaL_argexpected, lua_newtable::lua_newtable, utag_proxy::UTAG_PROXY,
  },
  records::lua_state::LuaState,
};

/// # Safety
/// `l` 须为存活 LuaState 并处于受保护帧：栈 1 号位须为 nil/boolean/无值（`lua_type` 配合 `luaL_argexpected` 校验，
/// 否则抛错），随后 `lua_newuserdatatagged`/`lua_newtable`/`lua_setmetatable` 分配对象并可 GC。
/// cpp/VM/src/lbaselib.cpp:412 luaB_newproxy。
pub(crate) unsafe extern "C-unwind" fn lua_b_newproxy(l: *mut LuaState) -> i32 {
  unsafe {
    let t = lua_type(l, 1);
    luaL_argexpected!(
      l,
      t == LuaType::Nil as i32 || t == LuaType::Boolean as i32 || t == LuaType::None as i32,
      1,
      "nil or boolean"
    );

    let needsmt = lua_toboolean(l, 1) != 0;

    lua_newuserdatatagged(l, 0, UTAG_PROXY);

    if needsmt {
      lua_newtable(l);
      lua_setmetatable(l, -2);
    }

    1
  }
}
