use crate::type_aliases::{refinement_map::RefinementMap, type_id::TypeId};

/// C++ `void merge(RefinementMap& l, const RefinementMap& r, std::function<TypeId(TypeId, TypeId)> f)`
/// (`Analysis/src/LValue.cpp:91-101`)：每个 key 至多调用一次 `f`，故 Rust 侧用
/// `impl FnMut` 即可，回调可以合法地持有可变状态。
pub fn merge(
  l: &mut RefinementMap,
  r: &RefinementMap,
  mut f: impl FnMut(TypeId, TypeId) -> TypeId,
) {
  for (k, a) in r {
    if let Some(existing) = l.get_mut(k) {
      *existing = f(*existing, *a);
    } else {
      l.insert(k.clone(), *a);
    }
  }
}
