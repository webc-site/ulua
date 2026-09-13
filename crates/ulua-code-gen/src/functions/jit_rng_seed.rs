pub fn jit_rng_seed(ptr: usize) -> u64 {
  let mut state: u64 = 0;
  state = state.wrapping_mul(6364136223846793005).wrapping_add(105);
  state = state.wrapping_add(ptr as u64);
  state = state.wrapping_mul(6364136223846793005).wrapping_add(105);
  state
}
