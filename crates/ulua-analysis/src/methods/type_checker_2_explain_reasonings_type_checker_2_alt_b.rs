use ulua_ast::records::location::Location;

use crate::{
  records::{
    reasonings::Reasonings, subtyping_result::SubtypingResult, type_checker_2::TypeChecker2,
  },
  type_aliases::type_id::TypeId,
};

impl TypeChecker2 {
  // C++ `Reasonings TypeChecker2::explainReasonings(TypeId, TypeId, Location,
  // const SubtypingResult&)` (TypeChecker2.cpp:3136-3139) — forwards to the
  // templated `explainReasonings_`.
  pub fn explain_reasonings_type_id_type_id_location_subtyping_result(
    &mut self,
    sub_ty: TypeId,
    super_ty: TypeId,
    location: Location,
    r: &SubtypingResult,
  ) -> Reasonings {
    self.explain_reasonings_generic(sub_ty, super_ty, location, r)
  }
}
