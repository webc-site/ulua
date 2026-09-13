use core::ffi::c_void;

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{enums::polarity::Polarity, records::free_type_searcher::FreeTypeSearcher};
impl FreeTypeSearcher {
  pub fn seen_with_current_polarity(&mut self, ty: *const c_void) -> bool {
    match self.polarity {
      Polarity::Positive => {
        if self.seen_positive.contains(&ty) {
          return true;
        }
        self.seen_positive.insert(ty);
        false
      }
      Polarity::Negative => {
        if self.seen_negative.contains(&ty) {
          return true;
        }
        self.seen_negative.insert(ty);
        false
      }
      Polarity::Mixed => {
        if self.seen_positive.contains(&ty) && self.seen_negative.contains(&ty) {
          return true;
        }
        self.seen_positive.insert(ty);
        self.seen_negative.insert(ty);
        false
      }
      _ => {
        LUAU_ASSERT!(false);
        false
      }
    }
  }
}
