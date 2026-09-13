use core::ptr::null;

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::records::{
  internal_compiler_error::InternalCompilerError, type_pack::TypePack,
  type_pack_iterator::TypePackIterator,
};
impl TypePackIterator {
  pub fn operator_inc(&mut self) {
    LUAU_ASSERT!(!self.tp.is_null());

    self.current_index += 1;
    while !self.tp.is_null() && self.current_index >= unsafe { (*self.tp).head.len() } {
      self.current_type_pack = if let Some(tail) = unsafe { (*self.tp).tail } {
        unsafe { (*self.log).follow_type_pack_id(tail) }
      } else {
        null()
      };

      self.tp = if !self.current_type_pack.is_null() {
        unsafe { (*self.log).txn_log_get_mutable::<TypePack, _>(self.current_type_pack) }
      } else {
        null()
      };

      if !self.tp.is_null() {
        // Step twice on each iteration to detect cycles
        self.tail_cycle_check = if let Some(tail) = unsafe { (*self.tp).tail } {
          unsafe { (*self.log).follow_type_pack_id(tail) }
        } else {
          null()
        };

        if self.current_type_pack == self.tail_cycle_check {
          panic!(
            "{}",
            InternalCompilerError::internal_compiler_error_string_string(
              "TypePackIterator detected a type pack cycle".to_string(),
              "".to_string(),
            )
            .message
          );
        }
      }

      self.current_index = 0;
    }
  }
}
