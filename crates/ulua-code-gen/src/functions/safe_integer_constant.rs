pub fn safe_integer_constant(value: f64) -> bool {
  // Within 32 bits, note that we allow both max unsigned number as well as a negative counterpart
  // Doubles are actually ok within even larger bounds (but not exactly 2^53), but we use the function in 32 bit optimizations
  if value < -4294967295.0 || value > 4294967295.0 {
    return false;
  }

  (value as i64 as f64) == value
}
