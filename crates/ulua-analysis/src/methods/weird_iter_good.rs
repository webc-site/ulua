use crate::records::weird_iter::WeirdIter;

impl WeirdIter {
  pub fn weird_iter_good(&self) -> bool {
    !self.pack.is_null() && self.index < unsafe { (*self.pack).head.len() }
  }
}
