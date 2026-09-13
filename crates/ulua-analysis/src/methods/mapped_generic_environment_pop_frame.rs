use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::records::mapped_generic_environment::MappedGenericEnvironment;

impl MappedGenericEnvironment {
  pub fn pop_frame(&mut self) {
    LUAU_ASSERT!(self.current_scope_index.is_some());
    if let Some(current_scope_index) = self.current_scope_index {
      let new_frame_index = self.frames[current_scope_index].parent_scope_index;
      self.current_scope_index = new_frame_index.unwrap_or(0_usize).into();
    }
  }
}
