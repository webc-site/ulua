use crate::{
  enums::{subtyping_suppression_policy::SubtypingSuppressionPolicy, type_field::TypeField},
  functions::{follow_type::follow_type_id, get_2::get2, get_type_alt_j::get_type_id},
  records::{
    any_type::AnyType,
    boolean_singleton::BooleanSingleton,
    error_type::ErrorType,
    extern_type::ExternType,
    function_type::FunctionType,
    intersection_type::IntersectionType,
    metatable_type::MetatableType,
    negation_type::NegationType,
    never_type::NeverType,
    primitive_type::{PrimitiveType, Type as PrimType},
    scope::Scope,
    singleton_type::SingletonType,
    string_singleton::StringSingleton,
    subtyping::Subtyping,
    subtyping_environment::SubtypingEnvironment,
    subtyping_result::SubtypingResult,
    table_type::TableType,
    union_type::UnionType,
    unknown_type::UnknownType,
  },
  type_aliases::{component::Component, type_id::TypeId},
};

impl Subtyping {
  pub fn is_covariant_with_subtyping_environment_type_id_negation_type_not_null_scope(
    &mut self,
    env: &mut SubtypingEnvironment,
    sub_ty: TypeId,
    super_negation: &NegationType,
    scope: *mut Scope,
  ) -> SubtypingResult {
    let negated_ty = follow_type_id(super_negation.ty);

    let mut result = SubtypingResult::default();

    // sub_ty 是否为 ExternType（get2 纯查询，提前绑定以简化 if 链条件）
    let sub_extern = !get2::<ExternType, PrimitiveType, _>(sub_ty, negated_ty)
      .first
      .is_null();

    if get_type_id::<NeverType>(negated_ty).is_some() {
      // ¬never ~ unknown
      result = self.is_covariant_with_subtyping_environment_type_id_type_id_not_null_scope(
        env,
        sub_ty,
        unsafe { (*self.builtin_types).unknown_type },
        scope,
      );
    } else if get_type_id::<UnknownType>(negated_ty).is_some() {
      // ¬unknown ~ never
      result = self.is_covariant_with_subtyping_environment_type_id_type_id_not_null_scope(
        env,
        sub_ty,
        unsafe { (*self.builtin_types).never_type },
        scope,
      );
    } else if get_type_id::<AnyType>(negated_ty).is_some() {
      // ¬any ~ any
      result = self.is_covariant_with_subtyping_environment_type_id_type_id_not_null_scope(
        env, sub_ty, negated_ty, scope,
      );
    } else if let Some(u) = get_type_id::<UnionType>(negated_ty) {
      // ¬(A ∪ B) ~ ¬A ∩ ¬B
      // follow intersection rules: A & B <: T iff A <: T && B <: T
      result = SubtypingResult {
        is_subtype: true,
        ..Default::default()
      };

      for ty in &u.options {
        if let Some(negated_part) = get_type_id::<NegationType>(follow_type_id(*ty)) {
          result.and_also(
            self.is_covariant_with_subtyping_environment_type_id_type_id_not_null_scope(
              env,
              sub_ty,
              negated_part.ty,
              scope,
            ),
            SubtypingSuppressionPolicy::Any,
          );
        } else {
          let negated_tmp = NegationType { ty: *ty };
          result.and_also(
            self.is_covariant_with_subtyping_environment_type_id_negation_type_not_null_scope(
              env,
              sub_ty,
              &negated_tmp,
              scope,
            ),
            SubtypingSuppressionPolicy::Any,
          );
        }
      }
    } else if let Some(i) = get_type_id::<IntersectionType>(negated_ty) {
      // ¬(A ∩ B) ~ ¬A ∪ ¬B
      // follow union rules: A | B <: T iff A <: T || B <: T
      result = SubtypingResult {
        is_subtype: false,
        ..Default::default()
      };

      for ty in &i.parts {
        if let Some(negated_part) = get_type_id::<NegationType>(follow_type_id(*ty)) {
          result.or_else(
            self.is_covariant_with_subtyping_environment_type_id_type_id_not_null_scope(
              env,
              sub_ty,
              negated_part.ty,
              scope,
            ),
          );
        } else {
          let negated_tmp = NegationType { ty: *ty };
          result.or_else(
            self.is_covariant_with_subtyping_environment_type_id_negation_type_not_null_scope(
              env,
              sub_ty,
              &negated_tmp,
              scope,
            ),
          );
        }
      }
    } else if let p = get2::<PrimitiveType, PrimitiveType, _>(sub_ty, negated_ty)
      && !p.first.is_null()
    {
      // number <: ¬boolean
      // number </: ¬number
      let p = get2::<PrimitiveType, PrimitiveType, _>(sub_ty, negated_ty);
      result = SubtypingResult {
        is_subtype: unsafe { (*p.first).r#type != (*p.second).r#type },
        ..Default::default()
      };
    } else if let p = get2::<SingletonType, PrimitiveType, _>(sub_ty, negated_ty)
      && !p.first.is_null()
    {
      let p = get2::<SingletonType, PrimitiveType, _>(sub_ty, negated_ty);
      // "foo" </: ¬string
      if unsafe {
        (*p.first).variant.get_if::<StringSingleton>().is_some()
          && (*p.second).r#type == PrimType::String
      } || unsafe {
        (*p.first).variant.get_if::<BooleanSingleton>().is_some()
          && (*p.second).r#type == PrimType::Boolean
      } {
        result = SubtypingResult {
          is_subtype: false,
          ..Default::default()
        };
      }
      // other cases are true
      else {
        result = SubtypingResult {
          is_subtype: true,
          ..Default::default()
        };
      }
    } else if let p = get2::<PrimitiveType, SingletonType, _>(sub_ty, negated_ty)
      && !p.first.is_null()
    {
      let p = get2::<PrimitiveType, SingletonType, _>(sub_ty, negated_ty);
      if unsafe {
        (*p.first).r#type == PrimType::String
          && (*p.second).variant.get_if::<StringSingleton>().is_some()
      } || unsafe {
        (*p.first).r#type == PrimType::Boolean
          && (*p.second).variant.get_if::<BooleanSingleton>().is_some()
      } {
        result = SubtypingResult {
          is_subtype: false,
          ..Default::default()
        };
      } else {
        result = SubtypingResult {
          is_subtype: true,
          ..Default::default()
        };
      }
    }
    // the top class type is not actually a primitive type, so the negation of
    // any one of them includes the top class type.
    else if sub_extern {
      result = SubtypingResult {
        is_subtype: true,
        ..Default::default()
      };
    } else if get_type_id::<PrimitiveType>(negated_ty).is_some()
      && (get_type_id::<TableType>(sub_ty).is_some()
        || get_type_id::<MetatableType>(sub_ty).is_some())
    {
      let p = get_type_id::<PrimitiveType>(negated_ty).unwrap();
      result = SubtypingResult {
        is_subtype: p.r#type != PrimType::Table,
        ..Default::default()
      };
    } else if let p = get2::<FunctionType, PrimitiveType, _>(sub_ty, negated_ty)
      && !p.first.is_null()
    {
      let p = get2::<FunctionType, PrimitiveType, _>(sub_ty, negated_ty);
      result = SubtypingResult {
        is_subtype: unsafe { (*p.second).r#type != PrimType::Function },
        ..Default::default()
      };
    } else if let p = get2::<SingletonType, SingletonType, _>(sub_ty, negated_ty)
      && !p.first.is_null()
    {
      let p = get2::<SingletonType, SingletonType, _>(sub_ty, negated_ty);
      result = SubtypingResult {
        is_subtype: unsafe { *p.first != *p.second },
        ..Default::default()
      };
    } else if let p = get2::<ExternType, ExternType, _>(sub_ty, negated_ty)
      && !p.first.is_null()
    {
      let p = get2::<ExternType, ExternType, _>(sub_ty, negated_ty);
      let inner = self
        .is_covariant_with_subtyping_environment_extern_type_extern_type_not_null_scope(
          env,
          unsafe { &*p.first },
          unsafe { &*p.second },
          scope,
        );
      result = SubtypingResult::negate(&inner);
    } else if let p = get2::<FunctionType, ExternType, _>(sub_ty, negated_ty)
      && !p.first.is_null()
    {
      result = SubtypingResult {
        is_subtype: true,
        ..Default::default()
      };
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
    }

    result.with_super_component(Component::TypeField(TypeField::Negated));
    result
  }
}
