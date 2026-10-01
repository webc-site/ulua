use crate::{
  enums::lua_type::LuaType,
  functions::{lua_rawseti::lua_rawseti, moveelements::moveelements},
  macros::{lua_l_error::luaL_error, lua_lib_fn::lua_lib_fn},
  records::lua_state::LuaState,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn tinsert(l: *mut LuaState) -> i32 {
  unsafe {
    (*l).check_type(1, LuaType::Table);
    let n = (*l).obj_len(1) as i32;
    let top = (*l).get_top();
    let pos: i32;

    match top {
      2 => {
        // called with only 2 arguments
        pos = n + 1; // insert new element at the end
      }
      3 => {
        // 2nd argument is the position
        pos = (*l).check_integer(2);

        // move up elements if necessary
        if 1 <= pos && pos <= n {
          moveelements(l, 1, 1, pos, n, pos + 1, false);
        }
      }
      _ => luaL_error!(l, "wrong number of arguments to 'insert'"),
    }

    lua_rawseti(&mut *l, 1, pos); // t[pos] = v
    0
  }
}

lua_lib_fn!(pub fn tinsert, tinsert_arm);
