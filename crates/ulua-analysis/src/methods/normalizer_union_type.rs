use crate::{
  functions::{follow_type::follow_type_id, get_type_alt_j::get_type_id},
  records::{
    any_type::AnyType, never_type::NeverType, normalizer::Normalizer, type_ids::TypeIds,
    union_type::UnionType,
  },
  type_aliases::type_id::TypeId,
};

impl Normalizer {
  pub fn union_type(&mut self, mut here: TypeId, mut there: TypeId) -> TypeId {
    self.consume_fuel();

    here = follow_type_id(here);
    there = follow_type_id(there);

    if here == there {
      return here;
    }

    let here_never = get_type_id::<NeverType>(here).is_some();
    let there_any = get_type_id::<AnyType>(there).is_some();
    if here_never || there_any {
      return there;
    }

    let there_never = get_type_id::<NeverType>(there).is_some();
    let here_any = get_type_id::<AnyType>(here).is_some();
    if there_never || here_any {
      return here;
    }

    let mut tmps = TypeIds::new();

    if let Some(utv) = get_type_id::<UnionType>(here) {
      let mut heres = TypeIds::new();
      for &ty in &utv.options {
        heres.insert_type_id(ty);
        tmps.insert_type_id(ty);
      }
      let key = self.cache_type_ids(heres);
      self.cached_unions.insert(key, here);
    } else {
      tmps.insert_type_id(here);
    }

    if let Some(utv) = get_type_id::<UnionType>(there) {
      let mut theres = TypeIds::new();
      for &ty in &utv.options {
        theres.insert_type_id(ty);
        tmps.insert_type_id(ty);
      }
      let key = self.cache_type_ids(theres);
      self.cached_unions.insert(key, there);
    } else {
      tmps.insert_type_id(there);
    }

    let cache_key = self.cache_type_ids(tmps.clone());
    if let Some(&cached) = self.cached_unions.get(&cache_key) {
      return cached;
    }

    let parts: Vec<TypeId> = tmps.order.clone();

    // SAFETY: self.arena 指向 Normalizer 常驻的类型 arena
    let result = unsafe { (*self.arena).add_type(UnionType { options: parts }) };
    self.cached_unions.insert(cache_key, result);

    result
  }
}
