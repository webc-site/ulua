use crate::{
  enums::lua_type::LuaType,
  functions::{
    getfunc::getfunc, lua_iscfunction::lua_iscfunction, lua_pushthread::lua_pushthread,
    lua_setfenv::lua_setfenv, lua_setsafeenv::lua_setsafeenv,
  },
  macros::{lua_l_error::luaL_error, lua_lib_fn::lua_lib_fn},
  records::lua_state::LuaState,
};

/// # Safety
/// `l` 的存活与独占已由 `&mut LuaState` 承载（r16-v29 收形）；`getfunc`/`lua_setsafeenv`/
/// `luaL_error!`（终端 `lua_l_error_l`）仍收裸形，转手各经一次 `l.as_mut_ptr()` 就地重建
/// （借用窗止于当句），屏障按 r16-v21 判例保留；其余前提即 C++ 参考实现之栈序约定
/// （索引 1/2 槽存活可读，抛错路径须处于受保护帧）。
pub(crate) unsafe fn lua_b_setfenv(l: &mut LuaState) -> i32 {
  unsafe {
    l.check_type(2, LuaType::Table);
    getfunc(l.as_mut_ptr(), 0);
    l.push_value(2);
    lua_setsafeenv(l.as_mut_ptr(), -1, 0);
    if l.is_number(1) && l.to_number(1).unwrap_or(0.0) == 0.0 {
      lua_pushthread(l);
      l.insert(-2);
      lua_setfenv(l, -2);
      return 0;
    } else if lua_iscfunction(l, -2) != 0 || lua_setfenv(l, -2) == 0 {
      luaL_error!(
        l.as_mut_ptr(),
        "'setfenv' cannot change environment of given object"
      );
    }
    1
  }
}

lua_lib_fn!(pub(crate) fn lua_b_setfenv @ref, lua_b_setfenv_arm);
