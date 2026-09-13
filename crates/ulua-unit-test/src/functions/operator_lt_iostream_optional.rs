use alloc::string::String;
use core::fmt::Write;

pub fn operator_lt_ostream_nullopt_t(lhs: &mut String) -> &mut String {
  let _ = write!(lhs, "none");
  lhs
}
