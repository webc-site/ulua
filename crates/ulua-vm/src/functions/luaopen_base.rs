//! Source: `VM/src/lbaselib.cpp:438-489` (hand-ported)

use crate::{
  functions::{
    auxopen::auxopen, lua_b_assert::lua_b_assert_arm, lua_b_error::lua_b_error_arm,
    lua_b_gcinfo::lua_b_gcinfo_arm, lua_b_getfenv::lua_b_getfenv_arm,
    lua_b_getmetatable::lua_b_getmetatable_arm, lua_b_inext::lua_b_inext_arm,
    lua_b_ipairs::lua_b_ipairs_arm, lua_b_newproxy::lua_b_newproxy_arm, lua_b_next::lua_b_next_arm,
    lua_b_pairs::lua_b_pairs_arm, lua_b_pcallcont::lua_b_pcallcont_arm,
    lua_b_pcally::lua_b_pcally_arm, lua_b_print::lua_b_print_arm,
    lua_b_rawequal::lua_b_rawequal_arm, lua_b_rawget::lua_b_rawget_arm,
    lua_b_rawlen::lua_b_rawlen_arm, lua_b_rawset::lua_b_rawset_arm, lua_b_select::lua_b_select_arm,
    lua_b_setfenv::lua_b_setfenv_arm, lua_b_setmetatable::lua_b_setmetatable_arm,
    lua_b_tonumber::lua_b_tonumber_arm, lua_b_tostring::lua_b_tostring_arm,
    lua_b_type::lua_b_type_arm, lua_b_typeof::lua_b_typeof_arm,
    lua_b_xpcallcont::lua_b_xpcallcont_arm, lua_b_xpcally::lua_b_xpcally_arm,
    lua_l_register::lua_l_register, lua_pushcclosurek::lua_pushcclosurek,
    lua_pushlstring::lua_pushlstring, lua_pushvalue::lua_pushvalue, lua_setfield::lua_setfield,
  },
  macros::{lua_globalsindex::LUA_GLOBALSINDEX, lua_setglobal::lua_setglobal},
  records::{lua_l_reg::LuaLReg, lua_state::LuaState},
};

static BASE_FUNCS: [LuaLReg; 19] = [
  LuaLReg::new(b"assert", lua_b_assert_arm),
  LuaLReg::new(b"error", lua_b_error_arm),
  LuaLReg::new(b"gcinfo", lua_b_gcinfo_arm),
  LuaLReg::new(b"getfenv", lua_b_getfenv_arm),
  LuaLReg::new(b"getmetatable", lua_b_getmetatable_arm),
  LuaLReg::new(b"next", lua_b_next_arm),
  LuaLReg::new(b"newproxy", lua_b_newproxy_arm),
  LuaLReg::new(b"print", lua_b_print_arm),
  LuaLReg::new(b"rawequal", lua_b_rawequal_arm),
  LuaLReg::new(b"rawget", lua_b_rawget_arm),
  LuaLReg::new(b"rawset", lua_b_rawset_arm),
  LuaLReg::new(b"rawlen", lua_b_rawlen_arm),
  LuaLReg::new(b"select", lua_b_select_arm),
  LuaLReg::new(b"setfenv", lua_b_setfenv_arm),
  LuaLReg::new(b"setmetatable", lua_b_setmetatable_arm),
  LuaLReg::new(b"tonumber", lua_b_tonumber_arm),
  LuaLReg::new(b"tostring", lua_b_tostring_arm),
  LuaLReg::new(b"type", lua_b_type_arm),
  LuaLReg::new(b"typeof", lua_b_typeof_arm),
];

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe extern "C-unwind" fn luaopen_base(l: *mut LuaState) -> i32 {
  unsafe {
    lua_pushvalue(l, LUA_GLOBALSINDEX);
    lua_setglobal(l, c"_G".as_ptr());

    lua_l_register(l, c"_G".as_ptr(), &BASE_FUNCS);
    lua_pushlstring(l, c"Luau".as_ptr(), 4);
    lua_setglobal(l, c"_VERSION".as_ptr());

    auxopen(
      l,
      c"ipairs".as_ptr(),
      Some(lua_b_ipairs_arm),
      Some(lua_b_inext_arm),
    );
    auxopen(
      l,
      c"pairs".as_ptr(),
      Some(lua_b_pairs_arm),
      Some(lua_b_next_arm),
    );

    lua_pushcclosurek(
      l,
      Some(lua_b_pcally_arm),
      c"pcall".as_ptr(),
      0,
      Some(lua_b_pcallcont_arm),
    );
    lua_setfield(l, -2, c"pcall".as_ptr());

    lua_pushcclosurek(
      l,
      Some(lua_b_xpcally_arm),
      c"xpcall".as_ptr(),
      0,
      Some(lua_b_xpcallcont_arm),
    );
    lua_setfield(l, -2, c"xpcall".as_ptr());

    1
  }
}
