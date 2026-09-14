//! Node: `cxx:Function:Luau.VM:VM/src/lmathlib.cpp:517:luaopen_math`
//! Source: `VM/src/lmathlib.cpp:473-541` (hand-ported)

use core::{ffi::c_int, ptr::null};

use crate::{
  functions::{
    lua_encodepointer::lua_encodepointer, lua_l_register::lua_l_register,
    lua_pushnumber::lua_pushnumber, lua_setfield::lua_setfield, math_abs::math_abs,
    math_acos::math_acos, math_asin::math_asin, math_atan::math_atan, math_atan_2::math_atan2,
    math_ceil::math_ceil, math_clamp::math_clamp, math_cos::math_cos, math_cosh::math_cosh,
    math_deg::math_deg, math_exp::math_exp, math_floor::math_floor, math_fmod::math_fmod,
    math_frexp::math_frexp, math_isfinite::math_isfinite, math_isinf::math_isinf,
    math_isnan::math_isnan, math_ldexp::math_ldexp, math_lerp::math_lerp, math_log::math_log,
    math_log_10::math_log_10, math_map::math_map, math_max::math_max, math_min::math_min,
    math_modf::math_modf, math_noise::math_noise, math_pow::math_pow, math_rad::math_rad,
    math_random::math_random, math_randomseed::math_randomseed, math_round::math_round,
    math_sign::math_sign, math_sin::math_sin, math_sinh::math_sinh, math_sqrt::math_sqrt,
    math_tan::math_tan, math_tanh::math_tanh, pcg_32_seed::pcg_32_seed,
  },
  macros::{
    luau_e::LUAU_E, luau_nan::LUAU_NAN, luau_phi::LUAU_PHI, luau_pi::LUAU_PI,
    luau_sqrt_2::LUAU_SQRT2, luau_tau::LUAU_TAU,
  },
  records::lua_l_reg::LuaLReg,
  type_aliases::lua_state::lua_State,
};

struct MathFuncs([LuaLReg; 38]);
unsafe impl Sync for MathFuncs {}

static MATH_FUNCS: MathFuncs = MathFuncs([
  LuaLReg {
    name: c"abs".as_ptr(),
    func: Some(math_abs),
  },
  LuaLReg {
    name: c"acos".as_ptr(),
    func: Some(math_acos),
  },
  LuaLReg {
    name: c"asin".as_ptr(),
    func: Some(math_asin),
  },
  LuaLReg {
    name: c"atan2".as_ptr(),
    func: Some(math_atan2),
  },
  LuaLReg {
    name: c"atan".as_ptr(),
    func: Some(math_atan),
  },
  LuaLReg {
    name: c"ceil".as_ptr(),
    func: Some(math_ceil),
  },
  LuaLReg {
    name: c"cosh".as_ptr(),
    func: Some(math_cosh),
  },
  LuaLReg {
    name: c"cos".as_ptr(),
    func: Some(math_cos),
  },
  LuaLReg {
    name: c"deg".as_ptr(),
    func: Some(math_deg),
  },
  LuaLReg {
    name: c"exp".as_ptr(),
    func: Some(math_exp),
  },
  LuaLReg {
    name: c"floor".as_ptr(),
    func: Some(math_floor),
  },
  LuaLReg {
    name: c"fmod".as_ptr(),
    func: Some(math_fmod),
  },
  LuaLReg {
    name: c"frexp".as_ptr(),
    func: Some(math_frexp),
  },
  LuaLReg {
    name: c"ldexp".as_ptr(),
    func: Some(math_ldexp),
  },
  LuaLReg {
    name: c"log10".as_ptr(),
    func: Some(math_log_10),
  },
  LuaLReg {
    name: c"log".as_ptr(),
    func: Some(math_log),
  },
  LuaLReg {
    name: c"max".as_ptr(),
    func: Some(math_max),
  },
  LuaLReg {
    name: c"min".as_ptr(),
    func: Some(math_min),
  },
  LuaLReg {
    name: c"modf".as_ptr(),
    func: Some(math_modf),
  },
  LuaLReg {
    name: c"pow".as_ptr(),
    func: Some(math_pow),
  },
  LuaLReg {
    name: c"rad".as_ptr(),
    func: Some(math_rad),
  },
  LuaLReg {
    name: c"random".as_ptr(),
    func: Some(math_random),
  },
  LuaLReg {
    name: c"randomseed".as_ptr(),
    func: Some(math_randomseed),
  },
  LuaLReg {
    name: c"sinh".as_ptr(),
    func: Some(math_sinh),
  },
  LuaLReg {
    name: c"sin".as_ptr(),
    func: Some(math_sin),
  },
  LuaLReg {
    name: c"sqrt".as_ptr(),
    func: Some(math_sqrt),
  },
  LuaLReg {
    name: c"tanh".as_ptr(),
    func: Some(math_tanh),
  },
  LuaLReg {
    name: c"tan".as_ptr(),
    func: Some(math_tan),
  },
  LuaLReg {
    name: c"noise".as_ptr(),
    func: Some(math_noise),
  },
  LuaLReg {
    name: c"clamp".as_ptr(),
    func: Some(math_clamp),
  },
  LuaLReg {
    name: c"sign".as_ptr(),
    func: Some(math_sign),
  },
  LuaLReg {
    name: c"round".as_ptr(),
    func: Some(math_round),
  },
  LuaLReg {
    name: c"map".as_ptr(),
    func: Some(math_map),
  },
  LuaLReg {
    name: c"lerp".as_ptr(),
    func: Some(math_lerp),
  },
  LuaLReg {
    name: c"isnan".as_ptr(),
    func: Some(math_isnan),
  },
  LuaLReg {
    name: c"isinf".as_ptr(),
    func: Some(math_isinf),
  },
  LuaLReg {
    name: c"isfinite".as_ptr(),
    func: Some(math_isfinite),
  },
  LuaLReg {
    name: null(),
    func: None,
  },
]);

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe extern "C-unwind" fn luaopen_math(l: *mut lua_State) -> c_int {
  unsafe {
    let mut seed = lua_encodepointer(l, l as usize) as u64;
    seed ^= 0;
    pcg_32_seed(&mut (*(*l).global).rngstate, seed);

    lua_l_register(l, c"math".as_ptr(), MATH_FUNCS.0.as_ptr());

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
