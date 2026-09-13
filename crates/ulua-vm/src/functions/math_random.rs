//! Node: `cxx:Function:Luau.VM:VM/src/lmathlib.cpp:234:math_random`
//!
//! `math.random` — 0 args: a double in [0,1) from two PCG32 draws via `ldexp`
//! (here `* 2^-64`); 1 arg `u`: integer in [1,u]; 2 args `l,u`: integer in
//! [l,u]. Bounds use the high 32 bits of a 64-bit multiply (Lemire-style) to
//! avoid modulo bias. Argument checks mirror the C++ exactly.

use crate::{
  functions::{
    lua_gettop::lua_gettop, lua_l_checkinteger::lua_l_checkinteger,
    lua_pushinteger::lua_pushinteger, lua_pushnumber::lua_pushnumber, pcg_32_random::pcg_32_random,
  },
  macros::{lua_l_argcheck::luaL_argcheck, lua_l_error::luaL_error},
  type_aliases::lua_state::lua_State,
};

pub(crate) unsafe extern "C-unwind" fn math_random(l: *mut lua_State) -> i32 {
  unsafe {
    let g = (*l).global;
    match lua_gettop(l) {
      0 => {
        let rl = pcg_32_random(&mut (*g).rngstate);
        let rh = pcg_32_random(&mut (*g).rngstate);
        let bits = (rl as u64) | ((rh as u64) << 32);
        let rd = (bits as f64) * 2.0f64.powi(-64);
        lua_pushnumber(l, rd);
      }
      1 => {
        let u = lua_l_checkinteger(l, 1);
        luaL_argcheck!(l, 1 <= u, 1, "interval is empty");

        let x = (u as u64).wrapping_mul(pcg_32_random(&mut (*g).rngstate) as u64);
        let r = (1 + (x >> 32)) as i32;
        lua_pushinteger(l, r);
      }
      2 => {
        let low = lua_l_checkinteger(l, 1);
        let u = lua_l_checkinteger(l, 2);
        luaL_argcheck!(l, low <= u, 2, "interval is empty");

        let ul = (u as u32).wrapping_sub(low as u32);
        luaL_argcheck!(l, ul < u32::MAX, 2, "interval is too large");
        let x = (ul as u64 + 1).wrapping_mul(pcg_32_random(&mut (*g).rngstate) as u64);
        let r = (low as i64 + (x >> 32) as i64) as i32;
        lua_pushinteger(l, r);
      }
      _ => {
        luaL_error!(l, "wrong number of arguments");
      }
    }
    1
  }
}
