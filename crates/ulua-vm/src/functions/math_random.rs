//! Source: `VM/src/lmathlib.cpp:234`
//!
//! `math.random` — 0 args: a double in [0,1) from two PCG32 draws via `ldexp`
//! (here `* 2^-64`); 1 arg `u`: integer in [1,u]; 2 args `l,u`: integer in
//! [l,u]. Bounds use the high 32 bits of a 64-bit multiply (Lemire-style) to
//! avoid modulo bias. Argument checks mirror the C++ exactly.

use crate::{
  functions::pcg_32_random::pcg_32_random,
  macros::{lua_l_error::luaL_error, lua_lib_fn::lua_lib_fn},
  records::lua_state::LuaState,
};

/// 调用序契约（正确性，非内存安全——`l` 的存活/独占前提已由 `&mut` 接收者类型承载；r16-v40 收形后
/// 读栈/校验/写回全经安全门面，rng 写点收口 `gs_mut` 门面一句一借，仅默认分支报错路径按
/// `fieldargs`/`check_div_args_64` 判例保留一处 `l.as_mut_ptr()` 窄重建 `unsafe` 块，故本体降为安全 `fn`）：
/// `l` 须处于可抛错的受保护帧，按 `get_top` 实参个数分支：0 个抽两轮 `pcg_32_random` 压 [0,1) 浮点；
/// 1 个经 `check_integer(1)` 取上限并 `arg_check(1 <= u, 1, "interval is empty")`（非整数/空区间抛错发散）；
/// 2 个经 `check_integer(1)`/`check_integer(2)` 依次 `arg_check` "interval is empty" 与
/// "interval is too large"；其余个数 `luaL_error` "wrong number of arguments" 发散。结果各写一槽，返回 1。
/// cpp/VM/src/lmathlib.cpp:234 math_random。
pub(crate) fn math_random(l: &mut LuaState) -> i32 {
  match l.get_top() {
    0 => {
      let rl = pcg_32_random(&mut l.gs_mut().rngstate);
      let rh = pcg_32_random(&mut l.gs_mut().rngstate);
      let bits = (rl as u64) | ((rh as u64) << 32);
      let rd = (bits as f64) * 2.0f64.powi(-64);
      l.push_number(rd);
    }
    1 => {
      let u = l.check_integer(1);
      l.arg_check(1 <= u, 1, "interval is empty");

      let x = (u as u64).wrapping_mul(pcg_32_random(&mut l.gs_mut().rngstate) as u64);
      let r = (1 + (x >> 32)) as i32;
      l.push_integer(r);
    }
    2 => {
      let low = l.check_integer(1);
      let u = l.check_integer(2);
      l.arg_check(low <= u, 2, "interval is empty");

      let ul = (u as u32).wrapping_sub(low as u32);
      l.arg_check(ul < u32::MAX, 2, "interval is too large");
      let x = (ul as u64 + 1).wrapping_mul(pcg_32_random(&mut l.gs_mut().rngstate) as u64);
      let r = (low as i64 + (x >> 32) as i64) as i32;
      l.push_integer(r);
    }
    _ => {
      // SAFETY: `l` 为借用形式的存活调用帧（&mut 保证有效且独占），as_mut_ptr 由该借用重取裸指针，luaL_error 抛错不返回。
      unsafe { luaL_error!(l.as_mut_ptr(), "wrong number of arguments") };
    }
  }
  1
}

lua_lib_fn!(pub(crate) fn math_random @ref, math_random_arm);
