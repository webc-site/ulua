use crate::{
  enums::lua_type::LuaType,
  functions::lua_newuserdatatagged::lua_newuserdatatagged,
  macros::{lua_lib_fn::lua_lib_fn, utag_proxy::UTAG_PROXY},
  records::lua_state::LuaState,
};

/// # Safety
/// `l` 须为存活 LuaState 并处于受保护帧：栈 1 号位须为 nil/boolean/无值（`lua_type` 配合 `arg_expected` 校验，
/// 否则抛错），随后 `lua_newuserdatatagged`/`lua_newtable`/`lua_setmetatable` 分配对象并可 GC。
/// cpp/VM/src/lbaselib.cpp:412 luaB_newproxy。
pub(crate) unsafe fn lua_b_newproxy(l: *mut LuaState) -> i32 {
  unsafe {
    let t = (*l).type_of(1);
    (*l).arg_expected(
      matches!(t, LuaType::Nil | LuaType::Boolean | LuaType::None),
      1,
      "nil or boolean",
    );

    let needsmt = (*l).to_boolean(1);

    lua_newuserdatatagged(l, 0, UTAG_PROXY);

    if needsmt {
      (*l).new_table();
      (*l).set_metatable(-2);
    }

    1
  }
}

lua_lib_fn!(pub(crate) fn lua_b_newproxy, lua_b_newproxy_arm);
