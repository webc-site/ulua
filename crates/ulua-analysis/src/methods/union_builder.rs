//! `union_builder` 方法汇总：原先按 cpp 符号逐方法拆分的同前缀小文件合并至此，行为逐字保留。

use crate::{
  functions::{begin_type::begin_union_type, follow_type, get_type},
  records::{
    arena_handle::Handle, builtin_types::BuiltinTypes, never_type::NeverType,
    type_arena::TypeArena, type_ids::TypeIds, union_builder::UnionBuilder, union_type::UnionType,
    unknown_type::UnknownType,
  },
  type_aliases::type_id::TypeId,
};

impl UnionBuilder {
  pub fn add(&mut self, ty: TypeId) {
    let ty = follow_type::follow(ty);

    if get_type::get::<NeverType>(ty).is_some() || self.is_top {
      return;
    }

    if get_type::get::<UnknownType>(ty).is_some() {
      self.is_top = true;
      return;
    }

    if let Some(utv) = get_type::get::<UnionType>(ty) {
      // C++ `for (auto option : utv)`——UnionTypeIterator 展平嵌套 union 并
      // follow,裸遍历 options 会漏掉嵌套成员。
      for option in begin_union_type(utv) {
        self.options.insert_type_id(option);
      }
    } else {
      self.options.insert_type_id(ty);
    }
  }
}

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

impl UnionBuilder {
  pub fn reserve(&mut self, size: usize) {
    self.options.reserve(size);
  }
}

impl UnionBuilder {
  pub fn size(&self) -> usize {
    self.options.size()
  }
}

impl UnionBuilder {
  pub fn new(arena: Handle<TypeArena>, builtin_types: Handle<BuiltinTypes>) -> Self {
    Self {
      arena,
      builtin_types,
      options: TypeIds::new(),
      is_top: false,
    }
  }
}
