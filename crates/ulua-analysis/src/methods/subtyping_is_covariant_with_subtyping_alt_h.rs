use crate::{
  enums::{subtyping_suppression_policy::SubtypingSuppressionPolicy, type_field::TypeField},
  functions::{follow_type::follow_type_id, get_type_alt_j::get_type_id},
  records::{
    any_type::AnyType, error_type::ErrorType, function_type::FunctionType,
    intersection_type::IntersectionType, metatable_type::MetatableType,
    negation_type::NegationType, never_type::NeverType, scope::Scope, subtyping::Subtyping,
    subtyping_environment::SubtypingEnvironment, subtyping_result::SubtypingResult,
    table_type::TableType, union_type::UnionType, unknown_type::UnknownType,
  },
  type_aliases::{component::Component, type_id::TypeId},
};

impl Subtyping {
  pub fn is_covariant_with_subtyping_environment_negation_type_type_id_not_null_scope(
    &mut self,
    env: &mut SubtypingEnvironment,
    sub_negation: &NegationType,
    super_ty: TypeId,
    scope: *mut Scope,
  ) -> SubtypingResult {
    let negated_ty = follow_type_id(sub_negation.ty);

    let mut result = SubtypingResult::default();

    if get_type_id::<NeverType>(negated_ty).is_some() {
      result = self.is_covariant_with_subtyping_environment_type_id_type_id_not_null_scope(
        env,
        unsafe { (*self.builtin_types).unknown_type },
        super_ty,
        scope,
      );
      result.with_sub_component(Component::TypeField(TypeField::Negated));
    } else if get_type_id::<UnknownType>(negated_ty).is_some() {
      result = self.is_covariant_with_subtyping_environment_type_id_type_id_not_null_scope(
        env,
        unsafe { (*self.builtin_types).never_type },
        super_ty,
        scope,
      );
      result.with_sub_component(Component::TypeField(TypeField::Negated));
    } else if get_type_id::<AnyType>(negated_ty).is_some() {
      result = self.is_covariant_with_subtyping_environment_type_id_type_id_not_null_scope(
        env, negated_ty, super_ty, scope,
      );
      result.with_sub_component(Component::TypeField(TypeField::Negated));
    } else if let Some(u) = get_type_id::<UnionType>(negated_ty) {
      result = SubtypingResult {
        is_subtype: true,
        ..Default::default()
      };

      for ty in &u.options {
        if let Some(negated_part) = get_type_id::<NegationType>(follow_type_id(*ty)) {
          let mut inner = self
            .is_covariant_with_subtyping_environment_type_id_type_id_not_null_scope(
              env,
              negated_part.ty,
              super_ty,
              scope,
            );
          inner.with_sub_component(Component::TypeField(TypeField::Negated));
          result.and_also(inner, SubtypingSuppressionPolicy::Any);
        } else {
          let negated_tmp = NegationType { ty: *ty };
          result.and_also(
            self.is_covariant_with_subtyping_environment_negation_type_type_id_not_null_scope(
              env,
              &negated_tmp,
              super_ty,
              scope,
            ),
            SubtypingSuppressionPolicy::Any,
          );
        }
      }
    } else if let Some(i) = get_type_id::<IntersectionType>(negated_ty) {
      result = SubtypingResult {
        is_subtype: false,
        ..Default::default()
      };

      for ty in &i.parts {
        if let Some(negated_part) = get_type_id::<NegationType>(follow_type_id(*ty)) {
          let mut inner = self
            .is_covariant_with_subtyping_environment_type_id_type_id_not_null_scope(
              env,
              negated_part.ty,
              super_ty,
              scope,
            );
          inner.with_sub_component(Component::TypeField(TypeField::Negated));
          result.or_else(inner);
        } else {
          let negated_tmp = NegationType { ty: *ty };
          result.or_else(
            self.is_covariant_with_subtyping_environment_negation_type_type_id_not_null_scope(
              env,
              &negated_tmp,
              super_ty,
              scope,
            ),
          );
        }
      }
    } else if get_type_id::<ErrorType>(negated_ty).is_some()
      || get_type_id::<FunctionType>(negated_ty).is_some()
      || get_type_id::<TableType>(negated_ty).is_some()
      || get_type_id::<MetatableType>(negated_ty).is_some()
    {
      unsafe {
        (*self.ice_reporter).ice_string("attempting to negate a non-testable type");
      }
    } else {
      result = SubtypingResult {
        is_subtype: false,
        ..Default::default()
      };
      result.with_sub_component(Component::TypeField(TypeField::Negated));
    }

    result
  }
}
