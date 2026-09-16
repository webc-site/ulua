use ulua_ast::records::location::Location;

use crate::{
  records::{
    reasonings::Reasonings, subtyping_result::SubtypingResult, type_checker_2::TypeChecker2,
  },
  type_aliases::type_pack_id::TypePackId,
};

impl TypeChecker2 {
  // C++ `Reasonings TypeChecker2::explainReasonings(TypePackId, TypePackId,
  // Location, const SubtypingResult&)` — forwards to the templated
  // `explainReasonings_`.
  pub fn explain_reasonings_type_pack_id_type_pack_id_location_subtyping_result(
    &mut self,
    sub_tp: TypePackId,
    super_tp: TypePackId,
    location: Location,
    r: &SubtypingResult,
  ) -> Reasonings {
    self.explain_reasonings_generic(sub_tp, super_tp, location, r)
  }
}
