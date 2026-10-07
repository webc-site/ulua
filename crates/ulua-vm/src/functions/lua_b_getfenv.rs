use crate::{
  functions::{
    getfunc::getfunc, lua_getfenv::lua_getfenv, lua_iscfunction::lua_iscfunction,
    lua_setsafeenv::lua_setsafeenv,
  },
  macros::{lua_globalsindex::LUA_GLOBALSINDEX, lua_lib_fn::lua_lib_fn},
  records::lua_state::LuaState,
};

/// # Safety
/// `l` 的存活与独占已由 `&mut LuaState` 承载（r16-v29 收形）；`getfunc`/`lua_setsafeenv` 仍收裸形，
/// 转手各经一次 `l.as_mut_ptr()` 就地重建（借用窗止于当句），屏障按 r16-v21 判例保留；其余前提即
/// C++ 参考实现之栈序约定（索引 1 槽函数值存活可读）。
pub(crate) unsafe fn lua_b_getfenv(l: &mut LuaState) -> i32 {
  unsafe {
    getfunc(l.as_mut_ptr(), 1);
    if lua_iscfunction(l, -1) != 0 {
      l.push_value(LUA_GLOBALSINDEX);
    } else {
      lua_getfenv(l, -1);
    }
    lua_setsafeenv(l.as_mut_ptr(), -1, 0);
    1
  }
}

lua_lib_fn!(pub(crate) fn lua_b_getfenv @ref, lua_b_getfenv_arm);
