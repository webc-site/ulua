use crate::{
  functions::{lua_l_checkinteger::lua_l_checkinteger, pcg_32_seed::pcg_32_seed},
  type_aliases::lua_state::lua_State,
};

pub(crate) unsafe extern "C-unwind" fn math_randomseed(l: *mut lua_State) -> i32 {
  unsafe {
    let seed = lua_l_checkinteger(l, 1) as u64;

    let state_ptr = (*l).global;
    let rng_state = &mut (*state_ptr).rngstate;
    pcg_32_seed(rng_state, seed);

    0
  }
}
