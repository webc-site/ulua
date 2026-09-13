//! Node: `cxx:Function:Luau.VM:VM/src/lbaselib.cpp:467:luaopen_base`
//! Source: `VM/src/lbaselib.cpp:438-489` (hand-ported)

use core::ptr::null;

use crate::{
  functions::{
    auxopen::auxopen, lua_b_assert::lua_b_assert, lua_b_error::lua_b_error,
    lua_b_gcinfo::lua_b_gcinfo, lua_b_getfenv::lua_b_getfenv,
    lua_b_getmetatable::lua_b_getmetatable, lua_b_inext::lua_b_inext, lua_b_ipairs::lua_b_ipairs,
    lua_b_newproxy::lua_b_newproxy, lua_b_next::lua_b_next, lua_b_pairs::lua_b_pairs,
    lua_b_pcallcont::lua_b_pcallcont, lua_b_pcally::lua_b_pcally, lua_b_print::lua_b_print,
    lua_b_rawequal::lua_b_rawequal, lua_b_rawget::lua_b_rawget, lua_b_rawlen::lua_b_rawlen,
    lua_b_rawset::lua_b_rawset, lua_b_select::lua_b_select, lua_b_setfenv::lua_b_setfenv,
    lua_b_setmetatable::lua_b_setmetatable, lua_b_tonumber::lua_b_tonumber,
    lua_b_tostring::lua_b_tostring, lua_b_type::lua_b_type, lua_b_typeof::lua_b_typeof,
    lua_b_xpcallcont::lua_b_xpcallcont, lua_b_xpcally::lua_b_xpcally,
    lua_l_register::lua_l_register, lua_pushcclosurek::lua_pushcclosurek,
    lua_pushlstring::lua_pushlstring, lua_pushvalue::lua_pushvalue, lua_setfield::lua_setfield,
  },
  macros::{lua_globalsindex::LUA_GLOBALSINDEX, lua_setglobal::lua_setglobal},
  records::lua_l_reg::LuaLReg,
  type_aliases::lua_state::lua_State,
};

struct BaseFuncs([LuaLReg; 20]);
unsafe impl Sync for BaseFuncs {}

static BASE_FUNCS: BaseFuncs = BaseFuncs([
  LuaLReg {
    name: c"assert".as_ptr(),
    func: Some(lua_b_assert),
  },
  LuaLReg {
    name: c"error".as_ptr(),
    func: Some(lua_b_error),
  },
  LuaLReg {
    name: c"gcinfo".as_ptr(),
    func: Some(lua_b_gcinfo),
  },
  LuaLReg {
    name: c"getfenv".as_ptr(),
    func: Some(lua_b_getfenv),
  },
  LuaLReg {
    name: c"getmetatable".as_ptr(),
    func: Some(lua_b_getmetatable),
  },
  LuaLReg {
    name: c"next".as_ptr(),
    func: Some(lua_b_next),
  },
  LuaLReg {
    name: c"newproxy".as_ptr(),
    func: Some(lua_b_newproxy),
  },
  LuaLReg {
    name: c"print".as_ptr(),
    func: Some(lua_b_print),
  },
  LuaLReg {
    name: c"rawequal".as_ptr(),
    func: Some(lua_b_rawequal),
  },
  LuaLReg {
    name: c"rawget".as_ptr(),
    func: Some(lua_b_rawget),
  },
  LuaLReg {
    name: c"rawset".as_ptr(),
    func: Some(lua_b_rawset),
  },
  LuaLReg {
    name: c"rawlen".as_ptr(),
    func: Some(lua_b_rawlen),
  },
  LuaLReg {
    name: c"select".as_ptr(),
    func: Some(lua_b_select),
  },
  LuaLReg {
    name: c"setfenv".as_ptr(),
    func: Some(lua_b_setfenv),
  },
  LuaLReg {
    name: c"setmetatable".as_ptr(),
    func: Some(lua_b_setmetatable),
  },
  LuaLReg {
    name: c"tonumber".as_ptr(),
    func: Some(lua_b_tonumber),
  },
  LuaLReg {
    name: c"tostring".as_ptr(),
    func: Some(lua_b_tostring),
  },
  LuaLReg {
    name: c"type".as_ptr(),
    func: Some(lua_b_type),
  },
  LuaLReg {
    name: c"typeof".as_ptr(),
    func: Some(lua_b_typeof),
  },
  LuaLReg {
    name: null(),
    func: None,
  },
]);

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe extern "C-unwind" fn luaopen_base(l: *mut lua_State) -> i32 {
  unsafe {
    lua_pushvalue(l, LUA_GLOBALSINDEX);
    lua_setglobal(l, c"_G".as_ptr());

    lua_l_register(l, c"_G".as_ptr(), BASE_FUNCS.0.as_ptr());
    lua_pushlstring(l, c"Luau".as_ptr(), 4);
    lua_setglobal(l, c"_VERSION".as_ptr());

    auxopen(l, c"ipairs".as_ptr(), Some(lua_b_ipairs), Some(lua_b_inext));
    auxopen(l, c"pairs".as_ptr(), Some(lua_b_pairs), Some(lua_b_next));

    lua_pushcclosurek(
      l,
      Some(lua_b_pcally),
      c"pcall".as_ptr(),
      0,
      Some(lua_b_pcallcont),
    );
    lua_setfield(l, -2, c"pcall".as_ptr());

    lua_pushcclosurek(
      l,
      Some(lua_b_xpcally),
      c"xpcall".as_ptr(),
      0,
      Some(lua_b_xpcallcont),
    );
    lua_setfield(l, -2, c"xpcall".as_ptr());

    1
  }
}
