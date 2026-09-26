use crate::{
  enums::lua_type::LuaType,
  functions::{
    lua_h_getnum::lua_h_getnum, lua_l_argerror_l::lua_l_argerror_l, lua_l_checkany::lua_l_checkany,
    lua_l_checktype::lua_l_checktype, lua_l_optinteger::lua_l_optinteger,
    lua_pushinteger::lua_pushinteger, lua_pushnil::lua_pushnil,
  },
  macros::equalobj::equalobj,
  records::lua_state::LuaState,
  type_aliases::{stk_id::StkId, t_value::TValue},
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe extern "C-unwind" fn tfind(l: *mut LuaState) -> i32 {
  unsafe {
    lua_l_checktype(l, 1, LuaType::Table as i32);
    lua_l_checkany(l, 2);
    let init = lua_l_optinteger(l, 3, 1);
    if init < 1 {
      // The dependency card for lua_l_argerror_l shows it takes &str.
      lua_l_argerror_l(l, 3, "index out of range");
    }

    let t = (*(*l).base).as_table_ptr();

    let mut i = init;
    loop {
      let e: *const TValue = lua_h_getnum(t, i);
      if (*e).is_nil() {
        break;
      }

      let v: StkId = (*l).base.offset(1);

      if equalobj!(l, v, e) {
        lua_pushinteger(l, i);
        return 1;
      }
      // C++ does `i++` unconditionally; if the table has an element at INT_MAX
      // that doesn't match, the increment is signed-overflow UB (upstream
      // ltablib.cpp:533). There is no valid index past INT_MAX, so stop cleanly.
      if i == i32::MAX {
        break;
      }
      i += 1;
    }

    lua_pushnil(l);
    1
  }
}
