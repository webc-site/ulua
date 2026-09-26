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
  records::{lua_l_reg::LuaLReg, lua_state::LuaState},
};

/// # Safety
/// `l` 须为存活 LuaState 且栈顶之上留足空槽（`lua_l_register` push 库表、随后 push+`lua_setfield` 各占一次临时），
/// 须在可分配/GC 的受保护帧内调用；`INT64LIB` 是静态 null 终止数组，`.as_ptr()` 指向其首项。
/// cpp/VM/src/lintlib.cpp:603 luaopen_integer。
pub(crate) unsafe extern "C-unwind" fn luaopen_integer(l: *mut LuaState) -> i32 {
  unsafe {
    // Register the integer library functions
    // Note: int64lib is defined in lintlib.cpp; in this translation context we use the local static.
    lua_l_register(l, c"integer".as_ptr(), &INT64LIB);

    // Push LLONG_MAX and set it as "maxsigned"
    lua_pushinteger_64(l, i64::MAX);
    lua_setfield(l, -2, c"maxsigned".as_ptr());

    // Push LLONG_MIN and set it as "minsigned"
    lua_pushinteger_64(l, i64::MIN);
    lua_setfield(l, -2, c"minsigned".as_ptr());

    1
  }
}

// cpp lintlib.cpp `int64lib` 注册表（39 项，顺序与 C++ 一致）。
static INT64LIB: [LuaLReg; 39] = [
  LuaLReg::new(b"create", int64_create),
  LuaLReg::new(b"tonumber", int64_tonumber),
  LuaLReg::new(b"neg", int64_neg),
  LuaLReg::new(b"add", int64_add),
  LuaLReg::new(b"sub", int64_sub),
  LuaLReg::new(b"mul", int64_mul),
  LuaLReg::new(b"div", int64_div),
  LuaLReg::new(b"min", int64_min),
  LuaLReg::new(b"max", int64_max),
  LuaLReg::new(b"rem", int64_rem),
  LuaLReg::new(b"idiv", int64_idiv),
  LuaLReg::new(b"udiv", int64_udiv),
  LuaLReg::new(b"urem", int64_urem),
  LuaLReg::new(b"mod", int64_mod),
  LuaLReg::new(b"clamp", int64_clamp),
  LuaLReg::new(b"band", int64_band),
  LuaLReg::new(b"bor", int64_bor),
  LuaLReg::new(b"bnot", int64_bnot),
  LuaLReg::new(b"bxor", int64_bxor),
  LuaLReg::new(b"lt", int64_lt),
  LuaLReg::new(b"le", int64_le),
  LuaLReg::new(b"ult", int64_ult),
  LuaLReg::new(b"ule", int64_ule),
  LuaLReg::new(b"gt", int64_gt),
  LuaLReg::new(b"ge", int64_ge),
  LuaLReg::new(b"ugt", int64_ugt),
  LuaLReg::new(b"uge", int64_uge),
  LuaLReg::new(b"lshift", int64_lshift),
  LuaLReg::new(b"rshift", int64_rshift),
  LuaLReg::new(b"arshift", int64_arshift),
  LuaLReg::new(b"lrotate", int64_lrotate),
  LuaLReg::new(b"rrotate", int64_rrotate),
  LuaLReg::new(b"extract", int64_extract),
  LuaLReg::new(b"replace", int64_replace),
  LuaLReg::new(b"btest", int64_btest),
  LuaLReg::new(b"countrz", int64_countrz),
  LuaLReg::new(b"countlz", int64_countlz),
  LuaLReg::new(b"bswap", int64_bswap),
  LuaLReg::new(b"fromstring", int64_fromstring),
];
