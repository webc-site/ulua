use crate::{
  enums::variance::Variance,
  functions::{get_singleton_type::get_singleton_type, get_type},
  records::{
    boolean_singleton::BooleanSingleton, primitive_type::PrimitiveType,
    singleton_type::SingletonType, string_singleton::StringSingleton, unifier::Unifier,
  },
  type_aliases::type_id::TypeId,
};

impl Unifier {
  pub fn unifier_try_unify_singletons(&mut self, sub_ty: TypeId, super_ty: TypeId) {
    // C++ Unifier.cpp:1646 tryUnifySingletons
    let super_prim = get_type::get::<PrimitiveType>(super_ty);
    let super_singleton = get_type::get::<SingletonType>(super_ty);
    let sub_singleton = get_type::get::<SingletonType>(sub_ty);

    let Some(sub_singleton) = sub_singleton else {
      self.ice_string("passed non singleton/primitive types to unifySingletons");
      return;
    };
    if super_prim.is_none() && super_singleton.is_none() {
      self.ice_string("passed non singleton/primitive types to unifySingletons");
      return;
    }

    // 同一 singleton，直接通过
    if let Some(super_singleton) = super_singleton
      && *super_singleton == *sub_singleton
    {
      return;
    }

    // 协变时：boolean 基类型接受 boolean singleton，string 基类型接受 string singleton
    if let Some(super_prim) = super_prim {
      let covariant = self.variance == Variance::Covariant;
      let boolean_ok = super_prim.r#type == PrimitiveType::BOOLEAN
        && get_singleton_type::<BooleanSingleton>(sub_singleton).is_some();
      let string_ok = super_prim.r#type == PrimitiveType::STRING
        && get_singleton_type::<StringSingleton>(sub_singleton).is_some();

      if covariant && (boolean_ok || string_ok) {
        return;
      }
    }

    self.unifier_report_type_mismatch(super_ty, sub_ty);
  }
}
