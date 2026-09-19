use crate::{
  functions::{follow_type::follow_type_id, get_type_alt_j::get_type_id},
  records::{
    any_type::AnyType, intersection_type::IntersectionType, never_type::NeverType,
    normalizer::Normalizer, type_ids::TypeIds,
  },
  type_aliases::type_id::TypeId,
};

impl Normalizer {
  pub fn intersection_type(&mut self, here: TypeId, there: TypeId) -> TypeId {
    self.consume_fuel();

    let here = follow_type_id(here);
    let there = follow_type_id(there);

    if here == there {
      return here;
    }

    if !get_type_id::<NeverType>(here).is_none() || !get_type_id::<AnyType>(there).is_none() {
      return here;
    }

    if !get_type_id::<NeverType>(there).is_none() || !get_type_id::<AnyType>(here).is_none() {
      return there;
    }

    let mut tmps = TypeIds::new();

    if !get_type_id::<IntersectionType>(here).is_none() {
      let utv = get_type_id::<IntersectionType>(here).unwrap();
      let mut heres = TypeIds::new();
      for &ty in &utv.parts {
        heres.insert_type_id(ty);
        tmps.insert_type_id(ty);
      }
      let key = self.cache_type_ids(heres);
      self.cached_intersections.insert(key, here);
    } else {
      tmps.insert_type_id(here);
    }

    if !get_type_id::<IntersectionType>(there).is_none() {
      let utv = get_type_id::<IntersectionType>(there).unwrap();
      let mut theres = TypeIds::new();
      for &ty in &utv.parts {
        theres.insert_type_id(ty);
        tmps.insert_type_id(ty);
      }
      let key = self.cache_type_ids(theres);
      self.cached_intersections.insert(key, there);
    } else {
      tmps.insert_type_id(there);
    }

    if tmps.size() == 1 {
      return tmps.front();
    }

    let cache_key = self.cache_type_ids(tmps.clone());
    if let Some(&cached) = self.cached_intersections.get(&cache_key) {
      return cached;
    }

    let parts = tmps.order.clone();
    let result = unsafe { (*self.arena).add_type(IntersectionType { parts }) };
    self.cached_intersections.insert(cache_key, result);

    result
  }
}
