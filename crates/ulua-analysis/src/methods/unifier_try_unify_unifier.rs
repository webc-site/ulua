//! Source: `Analysis/src/Unifier.cpp` (Unifier::tryUnify(TypeId,...), L385-390)
use crate::{
  records::unifier::Unifier,
  type_aliases::{literal_properties::LiteralProperties, type_id::TypeId},
};

impl Unifier {
  /// `void Unifier::tryUnify(TypeId sub_ty, TypeId super_ty, bool isFunctionCall, bool isIntersection, const LiteralProperties* literalProperties)`
  pub fn try_unify_type_id_type_id_bool_bool_literal_properties_entry(
    &mut self,
    sub_ty: TypeId,
    super_ty: TypeId,
    is_function_call: bool,
    is_intersection: bool,
    literal_properties: Option<&LiteralProperties>,
  ) {
    unsafe {
      (*self.shared_state).counters.iteration_count = 0;
    }

    self.try_unify_type_id_type_id_bool_bool_literal_properties(
      sub_ty,
      super_ty,
      is_function_call,
      is_intersection,
      literal_properties,
    );
  }
}
