#[inline]
pub fn jit_rng_random(state: &mut u64) -> u32 {
  let oldstate = *state;
  *state = oldstate
    .wrapping_mul(6364136223846793005)
    .wrapping_add((105 | 1) as u64);
  let xorshifted = (((oldstate >> 18) ^ oldstate) >> 27) as u32;
  let rot = (oldstate >> 59) as u32;
  let rot_neg = (-(rot as i32)) & 31;
  (xorshifted >> rot) | (xorshifted << (rot_neg as u32))
}

pub fn jit_rng_seed(ptr: usize) -> u64 {
  let mut state: u64 = 0;
  state = state.wrapping_mul(6364136223846793005).wrapping_add(105);
  state = state.wrapping_add(ptr as u64);
  state = state.wrapping_mul(6364136223846793005).wrapping_add(105);
  state
}
