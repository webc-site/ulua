use core::{ffi::c_int, ops::Deref, ptr::null};

use crate::{
  functions::{
    int_64_add::int64_add, int_64_arshift::int64_arshift, int_64_band::int64_band,
    int_64_bnot::int64_bnot, int_64_bor::int64_bor, int_64_bswap::int64_bswap,
    int_64_btest::int64_btest, int_64_bxor::int64_bxor, int_64_clamp::int64_clamp,
    int_64_countlz::int64_countlz, int_64_countrz::int64_countrz, int_64_create::int64_create,
    int_64_div::int64_div, int_64_extract::int64_extract, int_64_fromstring::int64_fromstring,
    int_64_ge::int64_ge, int_64_gt::int64_gt, int_64_idiv::int64_idiv, int_64_le::int64_le,
    int_64_lrotate::int64_lrotate, int_64_lshift::int64_lshift, int_64_lt::int64_lt,
    int_64_max::int64_max, int_64_min::int64_min, int_64_mod::int64_mod, int_64_mul::int64_mul,
    int_64_neg::int64_neg, int_64_rem::int64_rem, int_64_replace::int64_replace,
    int_64_rrotate::int64_rrotate, int_64_rshift::int64_rshift, int_64_sub::int64_sub,
    int_64_tonumber::int64_tonumber, int_64_udiv::int64_udiv, int_64_uge::int64_uge,
    int_64_ugt::int64_ugt, int_64_ule::int64_ule, int_64_ult::int64_ult, int_64_urem::int64_urem,
    lua_l_register::lua_l_register, lua_pushinteger_64::lua_pushinteger_64,
    lua_setfield::lua_setfield,
  },
  records::lua_l_reg::LuaLReg,
  type_aliases::lua_state::lua_State,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub(crate) unsafe extern "C-unwind" fn luaopen_integer(l: *mut lua_State) -> c_int {
  unsafe {
    // Register the integer library functions
    // Note: int64lib is defined in lintlib.cpp; in this translation context we use the local static.
    lua_l_register(l, c"int64".as_ptr(), INT64LIB.as_ptr());

    // Push LLONG_MAX and set it as "maxsigned"
    lua_pushinteger_64(l, i64::MAX);
    lua_setfield(l, -2, c"maxsigned".as_ptr());

    // Push LLONG_MIN and set it as "minsigned"
    lua_pushinteger_64(l, i64::MIN);
    lua_setfield(l, -2, c"minsigned".as_ptr());

    1
  }
}

// cpp lintlib.cpp `int64lib` 注册表（39 项 + 哨兵，顺序与 C++ 一致）。
struct Int64Lib([LuaLReg; 40]);
unsafe impl Sync for Int64Lib {}

impl Deref for Int64Lib {
  type Target = [LuaLReg; 40];

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

static INT64LIB: Int64Lib = Int64Lib([
  LuaLReg {
    name: c"create".as_ptr(),
    func: Some(int64_create),
  },
  LuaLReg {
    name: c"tonumber".as_ptr(),
    func: Some(int64_tonumber),
  },
  LuaLReg {
    name: c"neg".as_ptr(),
    func: Some(int64_neg),
  },
  LuaLReg {
    name: c"add".as_ptr(),
    func: Some(int64_add),
  },
  LuaLReg {
    name: c"sub".as_ptr(),
    func: Some(int64_sub),
  },
  LuaLReg {
    name: c"mul".as_ptr(),
    func: Some(int64_mul),
  },
  LuaLReg {
    name: c"div".as_ptr(),
    func: Some(int64_div),
  },
  LuaLReg {
    name: c"min".as_ptr(),
    func: Some(int64_min),
  },
  LuaLReg {
    name: c"max".as_ptr(),
    func: Some(int64_max),
  },
  LuaLReg {
    name: c"rem".as_ptr(),
    func: Some(int64_rem),
  },
  LuaLReg {
    name: c"idiv".as_ptr(),
    func: Some(int64_idiv),
  },
  LuaLReg {
    name: c"udiv".as_ptr(),
    func: Some(int64_udiv),
  },
  LuaLReg {
    name: c"urem".as_ptr(),
    func: Some(int64_urem),
  },
  LuaLReg {
    name: c"mod".as_ptr(),
    func: Some(int64_mod),
  },
  LuaLReg {
    name: c"clamp".as_ptr(),
    func: Some(int64_clamp),
  },
  LuaLReg {
    name: c"band".as_ptr(),
    func: Some(int64_band),
  },
  LuaLReg {
    name: c"bor".as_ptr(),
    func: Some(int64_bor),
  },
  LuaLReg {
    name: c"bnot".as_ptr(),
    func: Some(int64_bnot),
  },
  LuaLReg {
    name: c"bxor".as_ptr(),
    func: Some(int64_bxor),
  },
  LuaLReg {
    name: c"lt".as_ptr(),
    func: Some(int64_lt),
  },
  LuaLReg {
    name: c"le".as_ptr(),
    func: Some(int64_le),
  },
  LuaLReg {
    name: c"ult".as_ptr(),
    func: Some(int64_ult),
  },
  LuaLReg {
    name: c"ule".as_ptr(),
    func: Some(int64_ule),
  },
  LuaLReg {
    name: c"gt".as_ptr(),
    func: Some(int64_gt),
  },
  LuaLReg {
    name: c"ge".as_ptr(),
    func: Some(int64_ge),
  },
  LuaLReg {
    name: c"ugt".as_ptr(),
    func: Some(int64_ugt),
  },
  LuaLReg {
    name: c"uge".as_ptr(),
    func: Some(int64_uge),
  },
  LuaLReg {
    name: c"lshift".as_ptr(),
    func: Some(int64_lshift),
  },
  LuaLReg {
    name: c"rshift".as_ptr(),
    func: Some(int64_rshift),
  },
  LuaLReg {
    name: c"arshift".as_ptr(),
    func: Some(int64_arshift),
  },
  LuaLReg {
    name: c"lrotate".as_ptr(),
    func: Some(int64_lrotate),
  },
  LuaLReg {
    name: c"rrotate".as_ptr(),
    func: Some(int64_rrotate),
  },
  LuaLReg {
    name: c"extract".as_ptr(),
    func: Some(int64_extract),
  },
  LuaLReg {
    name: c"replace".as_ptr(),
    func: Some(int64_replace),
  },
  LuaLReg {
    name: c"btest".as_ptr(),
    func: Some(int64_btest),
  },
  LuaLReg {
    name: c"countrz".as_ptr(),
    func: Some(int64_countrz),
  },
  LuaLReg {
    name: c"countlz".as_ptr(),
    func: Some(int64_countlz),
  },
  LuaLReg {
    name: c"bswap".as_ptr(),
    func: Some(int64_bswap),
  },
  LuaLReg {
    name: c"fromstring".as_ptr(),
    func: Some(int64_fromstring),
  },
  LuaLReg {
    name: null(),
    func: None,
  },
]);
