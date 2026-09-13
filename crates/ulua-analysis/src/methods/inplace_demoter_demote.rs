use crate::{
  functions::get_mutable_level::get_mutable_level, records::inplace_demoter::InplaceDemoter,
  type_aliases::type_id::TypeId,
};

impl InplaceDemoter {
  pub fn demote(&mut self, ty: TypeId) -> bool {
    let level = { get_mutable_level(ty) };
    if !level.is_null() && unsafe { (*level).subsumes_strict(&self.new_level) } {
      unsafe {
        *level = self.new_level;
      }
      return true;
    }
    false
  }
}
