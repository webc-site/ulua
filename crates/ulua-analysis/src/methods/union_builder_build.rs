use crate::{
  records::{union_builder::UnionBuilder, union_type::UnionType},
  type_aliases::type_id::TypeId,
};

impl UnionBuilder {
  pub fn build(&mut self) -> TypeId {
    if self.is_top {
      return self.builtin_types.get().unknown_type;
    }

    if self.options.size() == 0 {
      return self.builtin_types.get().never_type;
    }

    if self.options.size() == 1 {
      return self.options.front();
    }

    let options_vec = self.options.take();
    let union_type = UnionType {
      options: options_vec,
    };
    self.arena.get_mut().add_type(union_type)
  }
}
