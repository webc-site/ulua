use crate::{
  enums::lua_type::LuaType,
  functions::lua_newuserdatatagged::lua_newuserdatatagged,
  macros::{lua_lib_fn::lua_lib_fn, utag_proxy::UTAG_PROXY},
  records::lua_state::LuaState,
};

/// # Safety
/// `l` 的存活与独占已由 `&mut LuaState` 承载（r16-v29 收形）；`lua_newuserdatatagged` 仍收裸形，
/// 转手经一次 `l.as_mut_ptr()` 就地重建（借用窗止于当句），屏障按 r16-v21 判例保留；`l` 仍须处于
/// 受保护帧：栈 1 号位须为 nil/boolean/无值（`type_of` 配合 `arg_expected` 校验，否则抛错），
/// 随后 `lua_newuserdatatagged`/`new_table`/`set_metatable` 分配对象并可 GC。
/// cpp/VM/src/lbaselib.cpp:412 luaB_newproxy。
pub(crate) unsafe fn lua_b_newproxy(l: &mut LuaState) -> i32 {
  unsafe {
    let t = l.type_of(1);
    l.arg_expected(
      matches!(t, LuaType::Nil | LuaType::Boolean | LuaType::None),
      1,
      "nil or boolean",
    );

    let needsmt = l.to_boolean(1);

    lua_newuserdatatagged(l.as_mut_ptr(), 0, UTAG_PROXY);

    if needsmt {
      l.new_table();
      l.set_metatable(-2);
    }

    1
  }
}

lua_lib_fn!(pub(crate) fn lua_b_newproxy @ref, lua_b_newproxy_arm);
