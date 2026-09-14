use crate::records::symbol::Symbol;

impl Symbol {
  #[inline]
  pub fn operator_eq_symbol(&self, rhs: &Self) -> bool {
    if !self.local.is_null() {
      self.local == rhs.local
    } else if !self.global.value.is_null() {
      !rhs.global.value.is_null() && unsafe { self.global.operator_eq_raw(rhs.global.value) }
    } else {
      rhs.local.is_null() && rhs.global.value.is_null()
    }
  }
}
