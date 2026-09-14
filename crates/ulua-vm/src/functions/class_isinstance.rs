use core::ffi::c_int;

use crate::{
  enums::lua_type::LuaType,
  functions::{
    lua_a_toobject::luaA_toobject, lua_l_checkany::lua_l_checkany,
    lua_l_checktype::lua_l_checktype, lua_pushboolean::lua_pushboolean,
  },
  macros::{classvalue::classvalue, objectvalue::objectvalue, ttisobject::ttisobject},
  records::{luau_class::LuauClass, luau_object::LuauObject},
  type_aliases::{lua_state::lua_State, t_value::TValue},
};

#[unsafe(export_name = "ulua_class_isinstance")]
pub(crate) unsafe extern "C-unwind" fn class_isinstance(l: *mut lua_State) -> c_int {
  unsafe {
    lua_l_checkany(l, 1);
    lua_l_checktype(l, 2, LuaType::Class as c_int);

    let inst: *const TValue = luaA_toobject(l, 1);
    let obj: *const TValue = luaA_toobject(l, 2);

    // classvalue! returns &mut ManuallyDrop<LuauClass>. We cast to raw pointer for comparison.
    let lclass = classvalue!(obj) as *mut _ as *mut LuauClass;

    if !ttisobject!(inst) {
      lua_pushboolean(l, 0);
      return 1;
    }

    let obj_ptr = objectvalue!(inst) as *mut _ as *mut LuauObject;
    let mut obj_class = (*obj_ptr).lclass;

    while !obj_class.is_null() {
      if obj_class == lclass {
        lua_pushboolean(l, 1);
        return 1;
      }
      obj_class = (*obj_class).super_;
    }

    lua_pushboolean(l, 0);
    1
  }
}
