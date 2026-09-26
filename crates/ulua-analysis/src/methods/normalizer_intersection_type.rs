use crate::{
  functions::{begin_type::begin_intersection_type, follow_type, get_type},
  records::{
    any_type::AnyType, intersection_type::IntersectionType, never_type::NeverType,
    normalizer::Normalizer, type_ids::TypeIds,
  },
  type_aliases::type_id::TypeId,
};

impl Normalizer {
  pub fn intersection_type(&mut self, here: TypeId, there: TypeId) -> TypeId {
    self.consume_fuel();

    let here = follow_type::follow(here);
    let there = follow_type::follow(there);

    if here == there {
      return here;
    }

    if get_type::get::<NeverType>(here).is_some() || get_type::get::<AnyType>(there).is_some() {
      return here;
    }

    if get_type::get::<NeverType>(there).is_some() || get_type::get::<AnyType>(here).is_some() {
      return there;
    }

    let mut tmps = TypeIds::new();

    if let Some(utv) = get_type::get::<IntersectionType>(here) {
      let mut heres = TypeIds::new();
      // C++ `heres.insert(begin(utv), end(utv))`——IntersectionTypeIterator
      // 展平嵌套 intersection 并 follow,裸遍历 parts 会漏掉嵌套成员。
      for ty in begin_intersection_type(utv) {
        heres.insert_type_id(ty);
        tmps.insert_type_id(ty);
      }
      let key = self.cache_type_ids(heres);
      self.cached_intersections.insert(key, here);
    } else {
      tmps.insert_type_id(here);
    }

    if let Some(utv) = get_type::get::<IntersectionType>(there) {
      let mut theres = TypeIds::new();
      for ty in begin_intersection_type(utv) {
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
    // 契约：归一化期 arena 必已接线（模块外调用方先走判空上报分支），
    // wired_arena_mut 内断言，违例为确定性 panic。
    let result = self.wired_arena_mut().add_type(IntersectionType { parts });
    self.cached_intersections.insert(cache_key, result);

    result
  }
}
