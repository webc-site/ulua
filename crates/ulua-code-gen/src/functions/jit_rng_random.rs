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
