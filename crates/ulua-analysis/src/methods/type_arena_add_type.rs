use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  records::{r#type::Type, type_arena::TypeArena},
  type_aliases::{type_id::TypeId, type_variant::TypeVariant},
};
impl TypeArena {
  pub fn add_type<T>(&mut self, tv: T) -> TypeId
  where
    T: Into<Type> + 'static,
  {
    let tv_into: Type = tv.into();

    if let TypeVariant::Union(union_type) = &tv_into.ty {
      LUAU_ASSERT!(union_type.options.len() >= 2);
    }

    if let TypeVariant::Singleton(singleton_type) = &tv_into.ty
      && self.collect_singleton_stats
    {
      self.record_singleton_stats(singleton_type);
    }

    self.add_tv(tv_into)
  }
}
