use std::collections::HashSet;

use crate::{records::union_type::UnionType, type_aliases::type_id::TypeId};

pub fn is_subset(super_: &UnionType, sub: &UnionType) -> bool {
  let mut super_types: HashSet<TypeId> = HashSet::new();

  for id in &super_.options {
    super_types.insert(*id);
  }

  for id in &sub.options {
    if !super_types.contains(id) {
      return false;
    }
  }

  true
}
