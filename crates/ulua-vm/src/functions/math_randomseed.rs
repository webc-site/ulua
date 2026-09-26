use crate::{
  functions::{lua_l_checkinteger::lua_l_checkinteger, pcg_32_seed::pcg_32_seed},
  macros::lua_lib_fn::lua_lib_fn,
  records::lua_state::LuaState,
};

/// # Safety
/// `l` 须为存活 LuaState 并处于受保护帧：栈 1 号位经 `lua_l_checkinteger` 取整数种子（非整数抛错）；
/// `(*l).global` 指向有效 global_State，其 `rngstate` 字段可写（`pcg_32_seed` 就地重置种子，不分配/不 GC）。
/// cpp/VM/src/lmathlib.cpp:278 math_randomseed。
pub(crate) unsafe fn math_randomseed(l: *mut LuaState) -> i32 {
  unsafe {
    let seed = lua_l_checkinteger(l, 1) as u64;

    let state_ptr = (*l).global;
    let rng_state = &mut (*state_ptr).rngstate;
    pcg_32_seed(rng_state, seed);

    0
  }
}

lua_lib_fn!(pub(crate) fn math_randomseed, math_randomseed_arm);
