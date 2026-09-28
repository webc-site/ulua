//! `type_cacher` 方法汇总：原先按 cpp 符号逐方法拆分的同前缀小文件合并至此，行为逐字保留。

use ulua_common::records::dense_hash_set::DenseHashSet;

use crate::{
  functions::{follow_type, follow_type_pack},
  records::{type_cacher::TypeCacher, type_once_visitor::TypeOnceVisitor},
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

impl TypeCacher {
  pub fn cache(&self, ty: TypeId) {
    unsafe { (*self.cached_types).insert(follow_type::follow(ty)) };
  }
}

impl TypeCacher {
  pub fn is_cached(&self, ty: TypeId) -> bool {
    unsafe { (*self.cached_types).contains(&follow_type::follow(ty)) }
  }
}

impl TypeCacher {
  pub fn is_uncacheable_type_id(&self, ty: TypeId) -> bool {
    self.uncacheable.contains(&follow_type::follow(ty))
  }

  pub(crate) fn is_uncacheable_type_pack_id(&self, tp: TypePackId) -> bool {
    self
      .uncacheable_packs
      .contains(&follow_type_pack::follow(tp))
  }
}

impl TypeCacher {
  pub fn mark_uncacheable_type_id(&mut self, ty: TypeId) {
    let followed = follow_type::follow(ty);
    self.uncacheable.insert(followed);
  }

  pub(crate) fn mark_uncacheable_type_pack_id(&mut self, tp: TypePackId) {
    let followed = follow_type_pack::follow(tp);
    self.uncacheable_packs.insert(followed);
  }
}

impl TypeCacher {
  pub fn new(cached_types: *mut DenseHashSet<TypeId>) -> Self {
    Self {
      base: TypeOnceVisitor::new("TypeCacher".to_string(), true),
      cached_types,
      uncacheable: DenseHashSet::default(),
      uncacheable_packs: DenseHashSet::default(),
    }
  }
}
