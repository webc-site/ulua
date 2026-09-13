use crate::{
  enums::{inhabited::Inhabited, relation::Relation},
  functions::{
    follow_type::follow_type_id, get_type_alt_j::get_type_id,
    relate_simplify_alt_b::relate_type_id_type_id,
  },
  records::{
    intersection_type::IntersectionType, type_ids::TypeIds, type_simplifier::TypeSimplifier,
  },
  type_aliases::type_id::TypeId,
};

pub fn intersect_one_with_intersection(
  simplifier: &mut TypeSimplifier,
  source: &mut TypeIds,
  dest: &mut TypeIds,
  candidate: TypeId,
) -> Inhabited {
  // `get<T>` requires a followed type (it asserts the argument is not a BoundType).
  // The C++ pipeline guarantees the candidate is followed at this point (TypeIds
  // entries are followed on insert); a recursed sub-part of an IntersectionType is
  // not necessarily followed, so we follow here to preserve that invariant.
  let candidate = follow_type_id(candidate);

  if dest.count(candidate) > 0 {
    return Inhabited::Yes;
  }

  if let Some(itv) = get_type_id::<IntersectionType>(candidate) {
    for &sub_part in &itv.parts {
      if intersect_one_with_intersection(simplifier, source, dest, sub_part) == Inhabited::No {
        return Inhabited::No;
      }
    }

    return Inhabited::Yes;
  }

  if source.empty() {
    dest.insert_type_id(candidate);
    return Inhabited::Yes;
  }
  for &ty in &source.order {
    match relate_type_id_type_id(candidate, ty) {
      Relation::Disjoint => return Inhabited::No,
      Relation::Subset => dest.insert_type_id(candidate),
      Relation::Coincident | Relation::Superset => dest.insert_type_id(ty),
      Relation::Intersects => {
        if let Some(simplified) = simplifier.basic_intersect(candidate, ty) {
          dest.insert_type_id(simplified);
        } else {
          dest.insert_type_id(candidate);
          dest.insert_type_id(ty);
        }
      }
    }
  }
  Inhabited::Yes
}
