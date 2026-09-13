use alloc::vec;
use core::ptr::null_mut;

use ulua_ast::records::location::Location;

use crate::{
  functions::{first::first, get_mutable_type::get_mutable_type_id},
  records::{
    blocked_type::BlockedType, constraint_generator::ConstraintGenerator, inference::Inference,
    inference_pack::InferencePack, unpack_constraint::UnpackConstraint,
  },
  type_aliases::{constraint_v::ConstraintV, scope_ptr_type::ScopePtr},
};
impl ConstraintGenerator {
  pub fn flatten_pack(
    &mut self,
    scope: &ScopePtr,
    location: Location,
    pack: InferencePack,
  ) -> Inference {
    let tp = pack.tp;
    let refinements = pack.refinements;

    let mut refinement = None;
    if !refinements.is_empty() {
      refinement = Some(refinements[0]);
    }

    if let Some(f) = first(tp, true) {
      return Inference::inference_type_id_refinement_id(f, refinement.unwrap_or(null_mut()));
    }

    // Create a blocked type: arena->addType(BlockedType{})
    let type_result = unsafe { (*self.arena).add_type(BlockedType::default()) };

    // addConstraint(scope, location, UnpackConstraint{{typeResult}, tp})
    let unpack_constraint = UnpackConstraint {
      result_pack: vec![type_result],
      source_pack: tp,
    };
    let constraint_ptr = self.add_constraint_scope_ptr_location_constraint_v(
      scope,
      location,
      ConstraintV::Unpack(unpack_constraint),
    );

    // type_result 刚由 add_type(BlockedType) 分配，必命中；对照 C++:5030
    // `getMutable<BlockedType>(typeResult)->setOwner(c)`
    let blocked = get_mutable_type_id::<BlockedType>(type_result).unwrap();
    blocked.set_owner(constraint_ptr as *const _);

    Inference::inference_type_id_refinement_id(type_result, refinement.unwrap_or(null_mut()))
  }
}
