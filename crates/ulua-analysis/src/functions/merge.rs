use crate::type_aliases::{refinement_map::RefinementMap, type_id::TypeId};

pub fn merge(l: &mut RefinementMap, r: &RefinementMap, f: &dyn Fn(TypeId, TypeId) -> TypeId) {
  for (k, a) in r {
    if let Some(existing) = l.get_mut(k) {
      *existing = f(*existing, *a);
    } else {
      l.insert(k.clone(), *a);
    }
  }
}
