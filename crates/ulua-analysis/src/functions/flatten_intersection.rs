//! Source: `Analysis/src/Type.cpp:134-164` (hand-ported)

use alloc::{collections::VecDeque, vec, vec::Vec};

use crate::{
  functions::{follow_type::follow, get_type::get},
  records::intersection_type::IntersectionType,
  type_aliases::{collections::HashSet, type_id::TypeId},
};
pub fn flatten_intersection(ty: TypeId) -> Vec<TypeId> {
  if get::<IntersectionType>(follow(ty)).is_none() {
    return vec![ty];
  }

  let mut seen = HashSet::<TypeId>::new();
  let mut queue: VecDeque<TypeId> = VecDeque::from(vec![ty]);

  let mut result = Vec::new();

  while let Some(front) = queue.pop_front() {
    let current = follow(front);

    if seen.contains(&current) {
      continue;
    }

    seen.insert(current);

    if let Some(itv) = get::<IntersectionType>(current) {
      for &t in itv.parts.iter() {
        queue.push_back(t);
      }
    } else {
      result.push(current);
    }
  }

  result
}
