//! Source: `VM/src/lmathlib.cpp:473-541` (hand-ported)

use crate::{
  functions::{
    lua_encodepointer::lua_encodepointer, lua_l_register::lua_l_register,
    lua_pushnumber::lua_pushnumber, lua_setfield::lua_setfield, math_abs::math_abs,
    math_acos::math_acos, math_asin::math_asin, math_atan::math_atan, math_atan_2::math_atan2,
    math_ceil::math_ceil, math_clamp::math_clamp_arm, math_cos::math_cos, math_cosh::math_cosh,
    math_deg::math_deg, math_exp::math_exp, math_floor::math_floor, math_fmod::math_fmod,
    math_frexp::math_frexp_arm, math_isfinite::math_isfinite, math_isinf::math_isinf,
    math_isnan::math_isnan, math_ldexp::math_ldexp_arm, math_lerp::math_lerp_arm,
    math_log::math_log_arm, math_log_10::math_log_10, math_map::math_map_arm, math_max::math_max,
    math_min::math_min, math_modf::math_modf_arm, math_noise::math_noise_arm, math_pow::math_pow,
    math_rad::math_rad, math_random::math_random_arm, math_randomseed::math_randomseed_arm,
    math_round::math_round, math_sign::math_sign, math_sin::math_sin, math_sinh::math_sinh,
    math_sqrt::math_sqrt, math_tan::math_tan, math_tanh::math_tanh, pcg_32_seed::pcg_32_seed,
  },
  macros::{
    luau_e::LUAU_E, luau_nan::LUAU_NAN, luau_phi::LUAU_PHI, luau_pi::LUAU_PI,
    luau_sqrt_2::LUAU_SQRT2, luau_tau::LUAU_TAU,
  },
  records::{lua_l_reg::LuaLReg, lua_state::LuaState},
};

static MATH_FUNCS: [LuaLReg; 37] = [
  LuaLReg::new(b"abs", math_abs),
  LuaLReg::new(b"acos", math_acos),
  LuaLReg::new(b"asin", math_asin),
  LuaLReg::new(b"atan2", math_atan2),
  LuaLReg::new(b"atan", math_atan),
  LuaLReg::new(b"ceil", math_ceil),
  LuaLReg::new(b"cosh", math_cosh),
  LuaLReg::new(b"cos", math_cos),
  LuaLReg::new(b"deg", math_deg),
  LuaLReg::new(b"exp", math_exp),
  LuaLReg::new(b"floor", math_floor),
  LuaLReg::new(b"fmod", math_fmod),
  LuaLReg::new(b"frexp", math_frexp_arm),
  LuaLReg::new(b"ldexp", math_ldexp_arm),
  LuaLReg::new(b"log10", math_log_10),
  LuaLReg::new(b"log", math_log_arm),
  LuaLReg::new(b"max", math_max),
  LuaLReg::new(b"min", math_min),
  LuaLReg::new(b"modf", math_modf_arm),
  LuaLReg::new(b"pow", math_pow),
  LuaLReg::new(b"rad", math_rad),
  LuaLReg::new(b"random", math_random_arm),
  LuaLReg::new(b"randomseed", math_randomseed_arm),
  LuaLReg::new(b"sinh", math_sinh),
  LuaLReg::new(b"sin", math_sin),
  LuaLReg::new(b"sqrt", math_sqrt),
  LuaLReg::new(b"tanh", math_tanh),
  LuaLReg::new(b"tan", math_tan),
  LuaLReg::new(b"noise", math_noise_arm),
  LuaLReg::new(b"clamp", math_clamp_arm),
  LuaLReg::new(b"sign", math_sign),
  LuaLReg::new(b"round", math_round),
  LuaLReg::new(b"map", math_map_arm),
  LuaLReg::new(b"lerp", math_lerp_arm),
  LuaLReg::new(b"isnan", math_isnan),
  LuaLReg::new(b"isinf", math_isinf),
  LuaLReg::new(b"isfinite", math_isfinite),
];

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe extern "C-unwind" fn luaopen_math(l: *mut LuaState) -> i32 {
  unsafe {
    let mut seed = lua_encodepointer(l, l as usize) as u64;
    seed ^= 0;
    pcg_32_seed(&mut (*(*l).global).rngstate, seed);

    lua_l_register(l, c"math".as_ptr(), &MATH_FUNCS);

    lua_pushnumber(l, LUAU_PI);
    lua_setfield(l, -2, c"pi".as_ptr());
    lua_pushnumber(l, f64::INFINITY);
    lua_setfield(l, -2, c"huge".as_ptr());
    lua_pushnumber(l, LUAU_NAN);
    lua_setfield(l, -2, c"nan".as_ptr());
    lua_pushnumber(l, LUAU_E);
    lua_setfield(l, -2, c"e".as_ptr());
    lua_pushnumber(l, LUAU_PHI);
    lua_setfield(l, -2, c"phi".as_ptr());
    lua_pushnumber(l, LUAU_SQRT2);
    lua_setfield(l, -2, c"sqrt2".as_ptr());
    lua_pushnumber(l, LUAU_TAU);
    lua_setfield(l, -2, c"tau".as_ptr());

    1
  }
}
