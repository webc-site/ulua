use crate::{
  records::{intersection_builder::IntersectionBuilder, intersection_type::IntersectionType},
  type_aliases::type_id::TypeId,
};

impl IntersectionBuilder {
  pub fn build(&mut self) -> TypeId {
    if self.is_bottom {
      return unsafe { (*self.builtin_types).never_type };
    }

    if self.parts.size() == 0 {
      return unsafe { (*self.builtin_types).unknown_type };
    }

    if self.parts.size() == 1 {
      return self.parts.front();
    }

    unsafe {
      (*self.arena).add_type(IntersectionType {
        parts: self.parts.take(),
      })
    }
  }
}
