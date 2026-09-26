use crate::{
  functions::get_type,
  records::{primitive_type::PrimitiveType, unifier::Unifier},
  type_aliases::type_id::TypeId,
};

impl Unifier {
  pub fn unifier_try_unify_primitives(&mut self, sub_ty: TypeId, super_ty: TypeId) {
    // C++ Unifier.cpp:1635 tryUnifyPrimitives
    let (Some(super_prim), Some(sub_prim)) = (
      get_type::get::<PrimitiveType>(super_ty),
      get_type::get::<PrimitiveType>(sub_ty),
    ) else {
      self.ice_string("passed non primitive types to unifyPrimitives");
      return;
    };

    if super_prim.r#type != sub_prim.r#type {
      self.unifier_report_type_mismatch(super_ty, sub_ty);
    }
  }
}
