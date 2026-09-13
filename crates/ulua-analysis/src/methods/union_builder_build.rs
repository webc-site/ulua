use crate::{
  records::{union_builder::UnionBuilder, union_type::UnionType},
  type_aliases::type_id::TypeId,
};

impl UnionBuilder {
  pub fn build(&mut self) -> TypeId {
    if self.is_top {
      return unsafe { (*self.builtin_types).unknown_type };
    }

    if self.options.size() == 0 {
      return unsafe { (*self.builtin_types).never_type };
    }

    if self.options.size() == 1 {
      return self.options.front();
    }

    let options_vec = self.options.take();
    let union_type = UnionType {
      options: options_vec,
    };
    unsafe { (*self.arena).add_type(union_type) }
  }
}
