use alloc::string::String;

use crate::{
  functions::get_type_alt_j::get_type_id,
  records::{primitive_type::PrimitiveType, type_mismatch::TypeMismatch, unifier::Unifier},
  type_aliases::{type_error_data::TypeErrorData, type_id::TypeId},
};

impl Unifier {
  pub fn unifier_try_unify_primitives(&mut self, sub_ty: TypeId, super_ty: TypeId) {
    // C++ Unifier.cpp:1635 tryUnifyPrimitives
    let (Some(super_prim), Some(sub_prim)) = (
      get_type_id::<PrimitiveType>(super_ty),
      get_type_id::<PrimitiveType>(sub_ty),
    ) else {
      self.ice_string("passed non primitive types to unifyPrimitives");
      return;
    };

    if super_prim.r#type != sub_prim.r#type {
      let context = self.unifier_mismatch_context();
      self.report_error_location_type_error_data(
        self.location,
        TypeErrorData::TypeMismatch(TypeMismatch {
          wanted_type: super_ty,
          given_type: sub_ty,
          reason: String::new(),
          error: None,
          context,
        }),
      );
    }
  }
}
