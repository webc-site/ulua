use alloc::{string::String, sync::Arc, vec::Vec};
use core::{ffi::CStr, ptr::null_mut};

use ulua_ast::records::{ast_type::AstType, ast_type_reference::AstTypeReference};
use ulua_common::{FFlag, macros::luau_assert::LUAU_ASSERT};

use crate::{
  enums::polarity::Polarity,
  functions::{
    finite::finite, first::first, follow_type::follow_type_id, get_mutable_type::get_mutable,
    get_type_alt_j::get_type_id, size_type_pack::size,
  },
  records::{
    blocked_type::BlockedType, constraint_generator::ConstraintGenerator,
    generic_error::GenericError, generic_type::GenericType, module::Module,
    pending_expansion_type::PendingExpansionType, reduce_constraint::ReduceConstraint,
    scope::Scope, type_alias_expansion_constraint::TypeAliasExpansionConstraint, type_fun::TypeFun,
    type_function_instance_type::TypeFunctionInstanceType,
    unapplied_type_function::UnappliedTypeFunction,
  },
  type_aliases::{
    constraint_v::ConstraintV, scope_ptr_type::ScopePtr, type_error_data::TypeErrorData,
    type_id::TypeId, type_pack_id::TypePackId,
  },
};
impl ConstraintGenerator {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn resolve_reference_type(
    &mut self,
    scope: &ScopePtr,
    ty: *mut AstType,
    ref_: *mut AstTypeReference,
    in_type_arguments: bool,
    replace_error_with_fresh: bool,
  ) -> TypeId {
    let scope_ptr = scope.as_ref() as *const Scope as *mut Scope;
    let mut result: TypeId;

    if FFlag::DebugLuauMagicTypes.get() {
      let ref_name_str = unsafe {
        CStr::from_ptr((*ref_).name.value)
          .to_string_lossy()
          .into_owned()
      };

      if ref_name_str == "_luau_ice" {
        let location = unsafe { (*ty).base.location };
        unsafe { (*self.ice).ice_string_location("_luau_ice encountered", &location) };
      } else if ref_name_str == "_luau_print" {
        let location = unsafe { (*ty).base.location };
        let params = unsafe { &(*ref_).parameters };
        if params.size != 1 || unsafe { (*params.data.add(0)).r#type }.is_null() {
          let err = GenericError::new(String::from("_luau_print requires one generic parameter"));
          self.report_error(location, TypeErrorData::GenericError(err));
          unsafe {
            let module_ptr = Arc::as_ptr(self.module.as_ref().unwrap()) as *mut Module;
            *(*module_ptr)
              .ast_resolved_types
              .get_or_insert(ty as *const _) = (*self.builtin_types).error_type;
            return (*self.builtin_types).error_type;
          }
        } else {
          let param_ty = unsafe { (*params.data.add(0)).r#type };
          return self.resolve_type_constraint_generator_alt_b(
            scope_ptr,
            param_ty,
            in_type_arguments,
            false,
          );
        }
      } else if ref_name_str == "_luau_blocked_type" {
        return unsafe { (*self.arena).add_type(BlockedType::default()) };
      }
    }

    let name_str = unsafe {
      CStr::from_ptr((*ref_).name.value)
        .to_string_lossy()
        .into_owned()
    };
    let alias: Option<TypeFun> = if let Some(prefix_name) = unsafe { (*ref_).prefix } {
      let prefix_str = unsafe {
        CStr::from_ptr(prefix_name.value)
          .to_string_lossy()
          .into_owned()
      };
      scope.lookup_imported_type(&prefix_str, &name_str)
    } else {
      scope.lookup_type(&name_str)
    };

    if let Some(ref alias_ref) = alias {
      if !alias_ref.type_params.is_empty()
        || !alias_ref.type_pack_params.is_empty()
        || unsafe { (*ref_).has_parameter_list }
      {
        let mut parameters: Vec<TypeId> = Vec::new();
        let mut pack_parameters: Vec<TypePackId> = Vec::new();

        let params_array = unsafe { &(*ref_).parameters };
        for &param in params_array.as_slice() {
          if !param.r#type.is_null() {
            let param_ty =
              self.resolve_type_constraint_generator_alt_b(scope_ptr, param.r#type, true, false);
            parameters.push(param_ty);
          } else if !param.type_pack.is_null() {
            let tp = unsafe {
              self.resolve_type_pack_scope_ptr_ast_type_pack_bool_bool(
                scope_ptr,
                param.type_pack,
                true,
                false,
              )
            };

            // If we need more regular type_arguments, we can use single
            // element type packs to fill those in.
            if parameters.len() < alias_ref.type_params.len()
              && unsafe { size(tp, null_mut()) } == 1
              && unsafe { finite(tp, null_mut()) }
              && let Some(ty_val) = first(tp, false)
            {
              parameters.push(ty_val);
              continue;
            }
            pack_parameters.push(tp);
          } else {
            LUAU_ASSERT!(false);
          }
        }

        let pending = PendingExpansionType::pending_expansion_type_pending_expansion_type(
          unsafe { (*ref_).prefix },
          unsafe { (*ref_).name },
          parameters,
          pack_parameters,
        );
        result = unsafe { (*self.arena).add_type(pending) };

        if !in_type_arguments {
          let location = unsafe { (*ty).base.location };
          let constraint = TypeAliasExpansionConstraint { target: result };
          self.add_constraint_scope_ptr_location_constraint_v(
            scope,
            location,
            ConstraintV::TypeAliasExpansion(constraint),
          );
        }
      } else {
        result = alias_ref.r#type();
      }
    } else {
      result = unsafe { (*self.builtin_types).error_type };
      if replace_error_with_fresh {
        result = self.fresh_type(scope, Polarity::Mixed);
      }
    }

    let follow_result = follow_type_id(result);
    // 对照 C++：`if (get<TypeFunctionInstanceType>(result))`
    if get_type_id::<TypeFunctionInstanceType>(follow_result).is_some() {
      let location = unsafe { (*ty).base.location };
      self.report_error(
        location,
        TypeErrorData::UnappliedTypeFunction(UnappliedTypeFunction::default()),
      );
      let reduce_constraint = ReduceConstraint { ty: result };
      self.add_constraint_scope_ptr_location_constraint_v(
        scope,
        location,
        ConstraintV::Reduce(reduce_constraint),
      );
    }

    let follow_result = follow_type_id(result);
    // 对照 C++：`if (auto gt = getMutable<GenericType>(follow(result))) gt->polarity = ...`
    if let Some(generic_type) = get_mutable::<GenericType>(follow_result) {
      let current_polarity = generic_type.polarity;
      generic_type.polarity = (current_polarity & Polarity::Mixed) | self.polarity;
    }

    result
  }
}
