use crate::{
  functions::{begin_type::begin_union_type, follow_type, get_type},
  records::{
    any_type::AnyType, never_type::NeverType, normalizer::Normalizer, type_ids::TypeIds,
    union_type::UnionType,
  },
  type_aliases::type_id::TypeId,
};

impl Normalizer {
  pub fn union_type(&mut self, mut here: TypeId, mut there: TypeId) -> TypeId {
    self.consume_fuel();

    here = follow_type::follow(here);
    there = follow_type::follow(there);

    if here == there {
      return here;
    }

    let here_never = get_type::get::<NeverType>(here).is_some();
    let there_any = get_type::get::<AnyType>(there).is_some();
    if here_never || there_any {
      return there;
    }

    let there_never = get_type::get::<NeverType>(there).is_some();
    let here_any = get_type::get::<AnyType>(here).is_some();
    if there_never || here_any {
      return here;
    }

    let mut tmps = TypeIds::new();

    if let Some(utv) = get_type::get::<UnionType>(here) {
      let mut heres = TypeIds::new();
      // C++ `heres.insert(begin(utv), end(utv))`——UnionTypeIterator 展平
      // 嵌套 union 并 follow,裸遍历 options 会漏掉嵌套成员。
      for ty in begin_union_type(utv) {
        heres.insert_type_id(ty);
        tmps.insert_type_id(ty);
      }
      let key = self.cache_type_ids(heres);
      self.cached_unions.insert(key, here);
    } else {
      tmps.insert_type_id(here);
    }

    if let Some(utv) = get_type::get::<UnionType>(there) {
      let mut theres = TypeIds::new();
      for ty in begin_union_type(utv) {
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

    // 契约：归一化期 arena 已接线（wired_arena_mut 断言），单线程驱动无并存别名。
    let result = self
      .wired_arena_mut()
      .add_type(UnionType { options: parts });
    self.cached_unions.insert(cache_key, result);

    result
  }
}
