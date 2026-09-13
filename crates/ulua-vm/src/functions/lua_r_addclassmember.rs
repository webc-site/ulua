use ulua_common::LUAU_ASSERT;

use crate::{
  enums::{lua_type::LuaType, tms::TMS},
  functions::{
    lua_h_getstr::lua_h_getstr, lua_h_new::lua_h_new, lua_h_setstr::lua_h_setstr,
    lua_s_newlstr::lua_s_newlstr,
  },
  macros::{
    lua_c_barrier::luaC_barrier, lua_c_objbarrier::luaC_objbarrier, nvalue::nvalue,
    setobj_2_class::setobj2class, setobj_2_t::setobj2t, ttisfunction::ttisfunction,
    ttisnumber::ttisnumber,
  },
  records::{
    global_state::global_State, lua_state::lua_State, luau_class::LuauClass, t_string::tstring,
  },
  type_aliases::t_value::TValue,
};

pub(crate) unsafe fn lua_r_addclassmember(
  l: *mut lua_State,
  classobject: *mut LuauClass,
  name: *mut tstring,
  value: *mut TValue,
) {
  unsafe {
    LUAU_ASSERT!(!(*classobject).staticmembers.is_null());

    let offset = lua_h_getstr((*classobject).memberstooffset, name);
    LUAU_ASSERT!(ttisnumber!(offset));
    let offsetint = nvalue!(offset) as i32;
    LUAU_ASSERT!(
      offsetint >= (*classobject).numberofinstancemembers
        && offsetint < (*classobject).numberofallmembers
    );
    LUAU_ASSERT!(ttisfunction!(value) && (*(*value).value.gc).gch.tt == LuaType::Function as u8);
    setobj2class!(
      l,
      (*classobject)
        .staticmembers
        .add((offsetint - (*classobject).numberofinstancemembers) as usize),
      value
    );
    luaC_barrier!(l, classobject, value);

    (*classobject).hasuserinitinchain |=
      name == lua_s_newlstr(l, c"__init".as_ptr() as *const _, 6);

    // Only metamethods in the parser's allowlist are supported (see ALLOWED_METAMETHODS in Parser.cpp)
    let is_metamethod = name == lua_s_newlstr(l, b"__tostring\0" as *const _ as *const _, 10);
    let mut is_metamethod = is_metamethod;
    let g: *mut global_State = (*l).global;
    // 任一 tmname 命中即视为元方法（C++ 循环 + break 的等价写法）
    is_metamethod = is_metamethod
      || (*g)
        .tmname
        .iter()
        .take(TMS::TmN as usize)
        .any(|&tmname| name == tmname);

    if is_metamethod {
      if (*classobject).instancemetatable.is_null() {
        (*classobject).instancemetatable = lua_h_new(l, 0, 1);
        luaC_objbarrier!(l, classobject, (*classobject).instancemetatable);
      }
      let dest = lua_h_setstr(l, (*classobject).instancemetatable, name);
      setobj2t!(l, dest, value);
      luaC_barrier!(l, (*classobject).instancemetatable, value);
    }
  }
}
