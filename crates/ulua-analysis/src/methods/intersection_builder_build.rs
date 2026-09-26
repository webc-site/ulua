use crate::{
  records::{intersection_builder::IntersectionBuilder, intersection_type::IntersectionType},
  type_aliases::type_id::TypeId,
};

impl IntersectionBuilder {
  pub fn build(&mut self) -> TypeId {
    let builtin_types = self.builtin_types.get();

    if self.is_bottom {
      return builtin_types.never_type;
    }

    if self.parts.size() == 0 {
      return builtin_types.unknown_type;
    }

    if self.parts.size() == 1 {
      return self.parts.front();
    }

    self.arena.get_mut().add_type(IntersectionType {
      parts: self.parts.take(),
    })
  }
}
