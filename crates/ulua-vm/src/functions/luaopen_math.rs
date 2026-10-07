//! Source: `VM/src/lmathlib.cpp:473-541` (hand-ported)

use crate::{
  functions::{
    lua_encodepointer::lua_encodepointer, lua_l_register::lua_l_register_bytes, math_abs::math_abs,
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
    lua_lib_fn::lua_lib_fn, luau_e::LUAU_E, luau_nan::LUAU_NAN, luau_phi::LUAU_PHI,
    luau_pi::LUAU_PI, luau_sqrt_2::LUAU_SQRT2, luau_tau::LUAU_TAU,
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

/// 调用序契约（正确性，非内存安全——`l` 的存活/独占前提已由 `&mut` 接收者类型承载；本票收形后
/// 注册所经 `lua_l_register_bytes` 已降为安全 `fn`（r12-w6d，裸 C 函数指针与 `lua_s_new`
/// 转手屏障下沉被调内部窄块），体内无残留窄块，其余皆安全门面直调，故本体为安全 `fn`）：`l` 须为可分配、可抛错
/// 的受保护帧且栈顶之上留足空槽（注册表 push 库表并作为返回值）。
/// 种子落笔序逐点定性（w6d 口径保留面）：push_number/set_field_bytes 七对皆
/// records/lua_state/{stack,table}.rs 既有门面收编形态，零翻案；经 `gs_mut().rngstate` 的 pcg
/// 种子落笔自 r16-b3 起走安全门面一句一借、借用不跨调用；`lua_encodepointer`（引用形取参、
/// 指针身份种子读数，cpp 同形）为自由函数安全调用点。cpp/VM/src/lmathlib.cpp:517 luaopen_math。
pub fn luaopen_math(l: &mut LuaState) -> i32 {
  // DELIBERATE DEVIATION: cpp lmathlib.cpp:519-521 种子再 XOR time(NULL)/clock()；
  // Rust 侧保持确定性种子（wasm/一致性测试可重现），`^= 0` 即该豁免锚点，勿删。
  let mut seed = lua_encodepointer(l, l as *const LuaState as usize) as u64;
  seed ^= 0;
  pcg_32_seed(&mut l.gs_mut().rngstate, seed);

  // `MATH_FUNCS` 为本文件同卫生域生成的合法 `unsafe extern "C-unwind"` 臂静态表，名字为
  // 不含尾部 `\0` 的静态字节切片，满足 `lua_l_register_bytes` 切片契约
  lua_l_register_bytes(l, Some(b"math"), &MATH_FUNCS);

  l.push_number(LUAU_PI);
  l.set_field_bytes(-2, b"pi");
  l.push_number(f64::INFINITY);
  l.set_field_bytes(-2, b"huge");
  l.push_number(LUAU_NAN);
  l.set_field_bytes(-2, b"nan");
  l.push_number(LUAU_E);
  l.set_field_bytes(-2, b"e");
  l.push_number(LUAU_PHI);
  l.set_field_bytes(-2, b"phi");
  l.push_number(LUAU_SQRT2);
  l.set_field_bytes(-2, b"sqrt2");
  l.push_number(LUAU_TAU);
  l.set_field_bytes(-2, b"tau");

  1
}

lua_lib_fn!(pub fn luaopen_math @ref, luaopen_math_arm);
