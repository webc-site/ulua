use crate::{
  enums::lua_type::LuaType, macros::lua_lib_fn::lua_lib_fn, records::lua_state::LuaState,
};

/// # Safety
/// `l` 须为存活 `LuaState` 且栈 index 1 为 table、index 2 为 function（`luaL_checktype` 校验，
/// 不符即抛错）；循环内 `lua_next`/`lua_call` 会读写栈、可 GC 可抛错，须在受保护帧内调用。
/// cpp `ltablib.cpp:35`。
pub unsafe fn foreach(l: *mut LuaState) -> i32 {
  unsafe {
    (*l).check_type(1, LuaType::Table);
    (*l).check_type(2, LuaType::Function);
    (*l).push_nil(); // first key
    while (*l).next(1) {
      (*l).push_value(2); // function
      (*l).push_value(-3); // key
      (*l).push_value(-3); // value
      (*l).call(2, 1);
      if !(*l).is_nil(-1) {
        return 1;
      }
      (*l).pop(2); // remove value and result
    }
    0
  }
}

lua_lib_fn!(pub fn foreach, foreach_arm);
