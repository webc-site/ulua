use core::mem::swap;

use crate::{
  enums::inhabited::Inhabited,
  functions::intersect_one_with_intersection::intersect_one_with_intersection,
  records::{
    intersection_builder::IntersectionBuilder, type_ids::TypeIds, type_simplifier::TypeSimplifier,
  },
  type_aliases::type_id::TypeId,
};
impl TypeSimplifier {
  pub fn intersect_from_parts(&mut self, parts: TypeIds) -> TypeId {
    let builtin_types = unsafe { &*self.builtin_types };

    if parts.size() == 0 {
      return builtin_types.unknown_type;
    }

    if parts.size() == 1 {
      return parts.front();
    }

    let mut source = TypeIds::new();
    let mut dest = TypeIds::new();

    source.reserve(parts.size());
    dest.reserve(parts.size());

    for &part in &parts.order {
      if intersect_one_with_intersection(self, &mut source, &mut dest, part) == Inhabited::No {
        return builtin_types.never_type;
      }

      swap(&mut source, &mut dest);
      dest.clear_without_realloc();
    }

    let mut ib =
      IntersectionBuilder::new(self.arena as *mut _, builtin_types as *const _ as *mut _);

    for &ty in &source.order {
      ib.add(ty);
    }

    ib.build()
  }
}
