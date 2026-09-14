//! Faithful port of the function/function `isCovariantWith` overload
//! `Subtyping::isCovariantWith(env, const FunctionType* subFunction, const FunctionType* superFunction, scope)`
//! (Analysis/src/Subtyping.cpp:2378-2486).
use alloc::{string::String, vec::Vec};
use core::mem::swap;

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::{
    pack_field::PackField, subtyping_suppression_policy::SubtypingSuppressionPolicy,
    subtyping_variance::SubtypingVariance,
  },
  functions::{
    flatten_type_pack::flatten_type_pack_id, follow_type::follow_type_id,
    follow_type_pack::follow_type_pack_id, get_type_alt_j::get_type_id,
    get_type_pack::get_type_pack_id, merge_reasonings::k_empty_reasoning,
  },
  records::{
    function_type::FunctionType, generic_bounds::GenericBounds, generic_type::GenericType,
    generic_type_count_mismatch::GenericTypeCountMismatch, generic_type_pack::GenericTypePack,
    generic_type_pack_count_mismatch::GenericTypePackCountMismatch, path::Path, scope::Scope,
    subtyping::Subtyping, subtyping_environment::SubtypingEnvironment,
    subtyping_reasoning::SubtypingReasoning, subtyping_result::SubtypingResult,
    type_error::TypeError, variadic_type_pack::VariadicTypePack,
  },
  type_aliases::{
    component::Component, subtyping_reasonings::SubtypingReasonings,
    type_error_data::TypeErrorData, type_pack_id::TypePackId,
  },
};
impl Subtyping {
  /// C++ `isContravariantWith(env, subTp, superTp, scope)` instantiated for the
  /// `TypePackId` overload set: `isCovariantWith(env, superTp, subTp, scope)`
  /// (note the swap) followed by the contravariant reasoning transform. We
  /// inline it here rather than routing through the generic `isContravariantWith`
  /// helper, because that helper's `IntoCovOperand` dispatch models only the
  /// `TypeId` / `TableIndexer` instantiations; the pack overload is selected by
  /// C++ overload resolution and must call the pack `isCovariantWith` directly.
  fn is_contravariant_with_packs(
    &mut self,
    env: &mut SubtypingEnvironment,
    sub_tp: TypePackId,
    super_tp: TypePackId,
    scope: *mut Scope,
  ) -> SubtypingResult {
    let mut result = unsafe {
      self.is_covariant_with_subtyping_environment_type_pack_id_type_pack_id_not_null_scope(
        env, super_tp, sub_tp, scope,
      )
    };

    if result.reasoning.empty() {
      result.reasoning.insert(SubtypingReasoning {
        sub_path: Path::default(),
        super_path: Path::default(),
        variance: SubtypingVariance::Contravariant,
        is_property_modifier_violation: false,
      });
    } else {
      let mut updated = SubtypingReasonings::new(k_empty_reasoning());
      for r in result.reasoning.iter() {
        let mut r = r.clone();
        swap(&mut r.sub_path, &mut r.super_path);
        if r.variance == SubtypingVariance::Covariant {
          r.variance = SubtypingVariance::Contravariant;
        } else if r.variance == SubtypingVariance::Contravariant {
          r.variance = SubtypingVariance::Covariant;
        }
        updated.insert(r);
      }
      result.reasoning = updated;
    }

    result
  }

  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn is_covariant_with_subtyping_environment_function_type_function_type_not_null_scope(
    &mut self,
    env: &mut SubtypingEnvironment,
    sub_function: &FunctionType,
    super_function: &FunctionType,
    scope: *mut Scope,
  ) -> SubtypingResult {
    let mut result = SubtypingResult::default();

    if !sub_function.generics.is_empty() {
      for &g in sub_function.generics.iter() {
        let g = follow_type_id(g);
        if get_type_id::<GenericType>(g).is_some() {
          if let Some(bounds) = env.mapped_generics.find_mut(&g) {
            // g may shadow an existing generic, so push a fresh set of bounds
            bounds.push(GenericBounds::default());
          } else {
            *env.mapped_generics.get_or_insert(g) = alloc::vec![GenericBounds::default()];
          }
        }
      }
    }

    if !sub_function.generic_packs.is_empty() {
      let mut packs: Vec<TypePackId> = Vec::with_capacity(sub_function.generic_packs.len());

      for &g in sub_function.generic_packs.iter() {
        let g = unsafe { follow_type_pack_id(g) };
        if get_type_pack_id::<GenericTypePack>(g).is_some() {
          packs.push(g);
        }
      }

      env.mapped_generic_packs.push_frame(&packs);
    }

    {
      let mut arg_result = self.is_contravariant_with_packs(
        env,
        sub_function.arg_types,
        super_function.arg_types,
        scope,
      );
      arg_result.with_both_component(Component::PackField(PackField::Arguments));
      result.or_else(arg_result);

      // If subtyping failed in the argument packs, we should check if there's a hidden variadic tail and try ignoring it.
      // This might cause subtyping correctly because the sub type here may not have a hidden variadic tail or equivalent.
      if !result.is_subtype {
        let (arguments, tail) = flatten_type_pack_id(super_function.arg_types);

        let hidden_variadic = match tail {
          Some(t) => get_type_pack_id::<VariadicTypePack>(t).is_some_and(|v| v.hidden),
          None => false,
        };

        if hidden_variadic {
          let truncated = unsafe {
            (*self.arena).add_type_pack_vector_type_id_optional_type_pack_id(arguments, None)
          };
          let mut retry =
            self.is_contravariant_with_packs(env, sub_function.arg_types, truncated, scope);
          retry.with_both_component(Component::PackField(PackField::Arguments));
          result.or_else(retry);
        }
      }
    }

    let mut ret_result = unsafe {
      self.is_covariant_with_subtyping_environment_type_pack_id_type_pack_id_not_null_scope(
        env,
        sub_function.ret_types,
        super_function.ret_types,
        scope,
      )
    };
    ret_result.with_both_component(Component::PackField(PackField::Returns));
    result.and_also(ret_result, SubtypingSuppressionPolicy::Any);

    if unsafe {
      (*sub_function.arg_types).type_pack_var_operator_eq(&*super_function.arg_types)
        && (*sub_function.ret_types).type_pack_var_operator_eq(&*super_function.ret_types)
    } {
      // It's fine to upcast a function with generics to a function without.
      // Intuitively: a generic function should always be a subtype of its instantiations.
      if super_function.generics.len() != sub_function.generics.len()
        && !super_function.generics.is_empty()
      {
        result.and_also(
          SubtypingResult {
            is_subtype: false,
            ..Default::default()
          },
          SubtypingSuppressionPolicy::Any,
        );
        result.with_error(TypeError {
          location: unsafe { (*scope).location },
          module_name: String::new(),
          data: TypeErrorData::GenericTypeCountMismatch(GenericTypeCountMismatch {
            sub_ty_generic_count: super_function.generics.len(),
            super_ty_generic_count: sub_function.generics.len(),
          }),
        });
      }

      if super_function.generic_packs.len() != sub_function.generic_packs.len()
        && !super_function.generic_packs.is_empty()
      {
        result.and_also(
          SubtypingResult {
            is_subtype: false,
            ..Default::default()
          },
          SubtypingSuppressionPolicy::Any,
        );
        result.with_error(TypeError {
          location: unsafe { (*scope).location },
          module_name: String::new(),
          data: TypeErrorData::GenericTypePackCountMismatch(GenericTypePackCountMismatch {
            sub_ty_generic_pack_count: super_function.generic_packs.len(),
            super_ty_generic_pack_count: sub_function.generic_packs.len(),
          }),
        });
      }
    }

    if !sub_function.generics.is_empty() {
      for &g in sub_function.generics.iter() {
        let g = follow_type_id(g);
        if let Some(r#gen) = get_type_id::<GenericType>(g) {
          let generic_name = r#gen.name.clone();

          let last_bounds = {
            let bounds = env.mapped_generics.find(&g);
            LUAU_ASSERT!(bounds.is_some() && !bounds.unwrap().is_empty());
            bounds.unwrap().last().unwrap().clone()
          };

          let bounds_result =
            self.subtyping_check_generic_bounds(&last_bounds, env, scope, &generic_name);
          result.and_also(bounds_result, SubtypingSuppressionPolicy::Any);

          env.mapped_generics.find_mut(&g).unwrap().pop();
        }
      }
    }

    if !sub_function.generic_packs.is_empty() {
      env.mapped_generic_packs.pop_frame();
      // This result isn't cacheable, because we may need it to populate the generic pack mapping environment again later
      result.is_cacheable = false;
    }

    result
  }
}
