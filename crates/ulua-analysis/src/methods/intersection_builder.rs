//! `intersection_builder` 方法汇总：原先按 cpp 符号逐方法拆分的同前缀小文件合并至此，行为逐字保留。

use crate::{
  functions::{begin_type::begin_intersection_type, follow_type, get_type},
  records::{
    arena_handle::Handle, builtin_types::BuiltinTypes, intersection_builder::IntersectionBuilder,
    intersection_type::IntersectionType, never_type::NeverType, type_arena::TypeArena,
    type_ids::TypeIds, unknown_type::UnknownType,
  },
  type_aliases::type_id::TypeId,
};

impl IntersectionBuilder {
  pub fn add(&mut self, ty: TypeId) {
    let ty = follow_type::follow(ty);

    if get_type::get::<NeverType>(ty).is_some() {
      self.is_bottom = true;
      return;
    }

    if get_type::get::<UnknownType>(ty).is_some() {
      return;
    }

    if let Some(itv) = get_type::get::<IntersectionType>(ty) {
      // C++ `for (auto part : itv)` — IntersectionTypeIterator 防环展平并 follow
      // Bound,裸遍历 parts 会漏掉嵌套 intersection。
      for part in begin_intersection_type(itv) {
        self.parts.insert_type_id(part);
      }
    } else {
      self.parts.insert_type_id(ty);
    }
  }
}

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

impl IntersectionBuilder {
  pub fn new(arena: Handle<TypeArena>, builtin_types: Handle<BuiltinTypes>) -> Self {
    Self {
      arena,
      builtin_types,
      parts: TypeIds::new(),
      is_bottom: false,
    }
  }
}

impl IntersectionBuilder {
  pub fn reserve(&mut self, size: usize) {
    self.parts.reserve(size);
  }
}

impl IntersectionBuilder {
  pub fn size(&self) -> usize {
    self.parts.size()
  }
}
