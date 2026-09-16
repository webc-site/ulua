use crate::{
  records::{
    find_simplification_blockers::FindSimplificationBlockers,
    iterative_type_visitor::IterativeTypeVisitorTrait,
  },
  type_aliases::type_id::TypeId,
};

pub fn must_defer_intersection(ty: TypeId) -> bool {
  let mut bts = FindSimplificationBlockers {
    base: Default::default(),
    found: false,
  };
  bts.find_simplification_blockers_find_simplification_blockers();
  bts.run_type_id(ty);
  bts.found
}
