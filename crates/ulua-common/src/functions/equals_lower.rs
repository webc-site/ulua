mod _inner {
  pub fn equals_lower(lhs: &[u8], rhs: &[u8]) -> bool {
    lhs.eq_ignore_ascii_case(rhs)
  }
}

pub use _inner::{equals_lower, equals_lower as equalsLower};
