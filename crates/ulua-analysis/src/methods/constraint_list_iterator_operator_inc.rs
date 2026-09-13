use crate::records::iterator::Iterator;

impl Iterator {
  #[inline]
  pub fn operator_inc(&mut self) -> &mut Iterator {
    let cl = unsafe { self.cl.as_ref() };
    if self.index < cl.order.len() {
      self.index += 1;
      self.advance_until_present_or_end();
    }
    self
  }
}
