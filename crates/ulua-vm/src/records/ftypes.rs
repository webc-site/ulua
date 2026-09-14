use core::{
  ffi::{c_char, c_double, c_float},
  fmt::{Debug, Formatter, Result},
  mem::size_of,
};
#[repr(C)]
#[derive(Copy, Clone)]
pub union Ftypes {
  pub f: c_float,
  pub d: c_double,
  pub n: c_double,
  pub buff: [c_char; 5 * size_of::<c_double>()],
}

impl Debug for Ftypes {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    f.debug_struct("Ftypes").finish_non_exhaustive()
  }
}
