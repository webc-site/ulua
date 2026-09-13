use std::collections::HashSet;

use ulua_analysis::{
  functions::{follow_type::follow_type_id, get_type_alt_j::get_type_id, merge as merge_analysis},
  records::{type_arena::TypeArena, union_type::UnionType},
  type_aliases::{refinement_map::RefinementMap, type_id::TypeId},
};
fn add_union_options(options: &mut Vec<TypeId>, seen: &mut HashSet<TypeId>, ty: TypeId) {
  let followed = follow_type_id(ty);
  let union = get_type_id::<UnionType>(followed);

  if let Some(union) = union {
    for option in &union.options {
      if seen.insert(*option) {
        options.push(*option);
      }
    }
  } else if seen.insert(ty) {
    options.push(ty);
  }
}

pub fn merge(arena: &mut TypeArena, l: &mut RefinementMap, r: &RefinementMap) {
  let arena = arena as *mut TypeArena;

  merge_analysis::merge(l, r, &|a, b| {
    let mut options = Vec::new();
    let mut seen = HashSet::new();

    add_union_options(&mut options, &mut seen, a);
    add_union_options(&mut options, &mut seen, b);

    if options.len() == 1 {
      options[0]
    } else {
      unsafe { (*arena).add_type(UnionType { options }) }
    }
  })
}
