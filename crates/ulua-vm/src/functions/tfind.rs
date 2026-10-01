use crate::{
  enums::lua_type::LuaType,
  functions::{
    lua_h_getnum::lua_h_getnum, lua_l_argerror_l::lua_l_argerror_l,
    lua_l_optinteger::lua_l_optinteger,
  },
  macros::{equalobj::equalobj, lua_lib_fn::lua_lib_fn},
  records::lua_state::LuaState,
  type_aliases::{stk_id::StkId, t_value::TValue},
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn tfind(l: *mut LuaState) -> i32 {
  unsafe {
    (*l).check_type(1, LuaType::Table);
    (*l).check_any(2);
    let init = lua_l_optinteger(&mut *l, 3, 1);
    if init < 1 {
      // The dependency card for lua_l_argerror_l shows it takes &str.
      lua_l_argerror_l(l, 3, "index out of range");
    }

    let t = (*(*l).base).as_table_ptr();

    let mut i = init;
    loop {
      let e: *const TValue = lua_h_getnum(&*t, i);
      if (*e).is_nil() {
        break;
      }

      let v: StkId = (*l).base.offset(1);

      if equalobj!(l, v, e) {
        (*l).push_integer(i);
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

    (*l).push_nil();
    1
  }
}

lua_lib_fn!(pub fn tfind, tfind_arm);
