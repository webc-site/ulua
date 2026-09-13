//! `TypeChecker2::visitCall`（TypeChecker2.cpp:1608-1930）。
use alloc::{format, string::String, vec::Vec};
use core::ptr::{NonNull, null_mut};

use ulua_ast::{
  records::{
    ast_expr::AstExpr, ast_expr_call::AstExprCall, ast_expr_index_name::AstExprIndexName,
    ast_node::AstNode,
  },
  rtti::ast_node_try_as,
};
use ulua_common::{
  FFlag,
  records::{dense_hash_set::DenseHashSet, variant::Variant2},
};

use crate::{
  enums::{normalization_result::NormalizationResult, value::Value},
  functions::{
    extend_type_pack::extend_type_pack, find_unique_types_ast_utils_alt_d::find_unique_types,
    flatten_type_pack::flatten_type_pack_id, follow_type::follow_type_id,
    get_parameter_extents::get_parameter_extents, get_type_alt_j::get_type_id,
    is_optional::is_optional, is_variadic_type_pack::is_variadic,
    report_available_overloads::report_available_overloads,
    should_suppress_errors_type_utils::should_suppress_errors,
  },
  records::{
    ambiguous_function_call::AmbiguousFunctionCall,
    any_type::AnyType,
    cannot_call_non_function::CannotCallNonFunction,
    count_mismatch::{CountMismatch, CountMismatchContext},
    function_type::FunctionType,
    generic_error::GenericError,
    internal_error::InternalError,
    intersection_type::IntersectionType,
    magic_function_type_check_context::MagicFunctionTypeCheckContext,
    multiple_nonviable_overloads::MultipleNonviableOverloads,
    never_type::NeverType,
    normalization_too_complex::NormalizationTooComplex,
    optional_value_access::OptionalValueAccess,
    overload_resolver::OverloadResolver,
    txn_log::TxnLog,
    type_checker_2::TypeChecker2,
    type_pack::TypePack,
    union_type::UnionType,
  },
  type_aliases::{error_type::ErrorType, type_error_data::TypeErrorData, type_id::TypeId},
};
impl TypeChecker2 {
  pub fn visit_call(&mut self, call: &AstExprCall) {
    // SAFETY: self.module 与类型检查会话同寿；本函数多处经裸指针 place 读写
    // module（errors/internal_types/ast_types），C++ 同契约。
    let module = unsafe { &mut *self.module };
    let func_node = call.func as *const AstNode;
    let Some(original_call_ty) = module.ast_original_call_types.find(&func_node).copied() else {
      return;
    };

    let mut fn_ty = follow_type_id(original_call_ty);
    if get_type_id::<AnyType>(fn_ty).is_some()
      || get_type_id::<ErrorType>(fn_ty).is_some()
      || get_type_id::<NeverType>(fn_ty).is_some()
    {
      return;
    }

    if is_optional(fn_ty) {
      match Value::from(unsafe { should_suppress_errors(&mut self.normalizer, fn_ty) }) {
        Value::Suppress => {}
        Value::NormalizationFailed => {
          self.report_error_type_error_data_location(
            TypeErrorData::NormalizationTooComplex(NormalizationTooComplex::default()),
            // SAFETY: call.func 指向 AST arena 节点。
            unsafe { &(*call.func).base.location },
          );
          self.report_error_type_error_data_location(
            TypeErrorData::OptionalValueAccess(OptionalValueAccess { optional: fn_ty }),
            // SAFETY: 同上。
            unsafe { &(*call.func).base.location },
          );
        }
        Value::DoNotSuppress => {
          self.report_error_type_error_data_location(
            TypeErrorData::OptionalValueAccess(OptionalValueAccess { optional: fn_ty }),
            // SAFETY: 同上。
            unsafe { &(*call.func).base.location },
          );
        }
      }
      return;
    }

    if FFlag::LuauExplicitTypeInstantiationSupport.get() && call.type_arguments.size != 0 {
      self.check_type_instantiation(
        &call.base,
        fn_ty,
        &call.base.base.location,
        call.type_arguments,
      );
    }

    // SAFETY: AstExprCall 的 AstNode 子对象在偏移 0，指针值与 C++ map key 一致。
    let call_node = unsafe { &*((call as *const AstExprCall).cast::<AstNode>()) } as *const AstNode;
    if let Some(selected_overload_ty) = module.ast_overload_resolved_types.find(&call_node).copied()
    {
      let scope = self.find_innermost_scope(call.base.base.location);
      // SAFETY: self.subtyping 由构造方保证有效（C++ 同契约）。
      let result = unsafe {
        (*self.subtyping).is_subtype_type_id_type_id_not_null_scope(
          original_call_ty,
          selected_overload_ty,
          scope,
        )
      };
      if result.is_subtype {
        fn_ty = follow_type_id(selected_overload_ty);
      }
      self.report_errors(result.errors);
      if result.normalization_too_complex {
        return;
      }
    }

    let Some(fty) = get_type_id::<FunctionType>(fn_ty) else {
      let mut args = TypePack {
        head: Vec::<TypeId>::new(),
        tail: None,
      };
      let mut arg_exprs: Vec<*mut AstExpr> = Vec::new();

      // The `call->self` prelude in C++ `visitCall` runs before the
      // FunctionType/else split (TypeChecker2.cpp:1624-1634), so the
      // method receiver `self` must be prepended onto `args` here too.
      if call.self_ {
        // SAFETY: call.func 指向 AST arena 节点。
        let index_expr = ast_node_try_as::<AstExprIndexName>(unsafe { &(*call.func).base });
        let Some(index_expr) = index_expr else {
          self.report_error_type_error_data_location(
            TypeErrorData::InternalError(InternalError {
              message: "method call expression has no 'self'".into(),
            }),
            &call.base.base.location,
          );
          return;
        };

        // SAFETY: index_expr->expr 指向 AST arena 节点。
        args
          .head
          .push(self.lookup_type(unsafe { &*index_expr.expr }));
        arg_exprs.push(index_expr.expr);
      }

      let args_slice = call.args.as_slice();
      for (idx, &arg_expr) in args_slice.iter().enumerate() {
        arg_exprs.push(arg_expr);
        let is_last = idx + 1 == args_slice.len();

        if is_last
          && let Some(last_arg_pack) = module
            .ast_type_packs
            .find(&(arg_expr as *const AstExpr))
            .copied()
        {
          let (last_arg_head, last_arg_tail) = flatten_type_pack_id(last_arg_pack);
          args.head.extend(last_arg_head);
          args.tail = last_arg_tail;
          continue;
        }

        if let Some(arg_ty) = module.ast_types.find(&(arg_expr as *const AstExpr)) {
          args.head.push(*arg_ty);
        } else if is_last {
          // SAFETY: builtin_types 由构造方保证有效（C++ 同契约）。
          args.tail = Some(unsafe { (*self.builtin_types).any_type_pack });
        } else {
          // SAFETY: 同上。
          args.head.push(unsafe { (*self.builtin_types).any_type });
        }
      }

      let args_pack = module.internal_types.add_type_pack_t(args);
      let scope = self.find_innermost_scope(call.base.base.location);
      let mut resolver = unsafe {
        OverloadResolver::new(
          self.builtin_types,
          &mut module.internal_types as *mut _,
          &mut self.normalizer as *mut _,
          self.type_function_runtime,
          scope,
          self.ice,
          self.limits,
          call.base.base.location,
        )
      };
      let mut unique_types: DenseHashSet<TypeId> = DenseHashSet::new(null_mut());
      // SAFETY: find_unique_types（清单外）为 unsafe fn；指针指向本函数局部。
      unsafe {
        find_unique_types(
          &mut unique_types as *mut DenseHashSet<TypeId>,
          &arg_exprs,
          &module.ast_types as *const _,
        )
      };

      // SAFETY: call.func 指向 AST arena 节点。
      let func_loc = unsafe { (*call.func).base.location };
      let result = resolver.resolve_overload(
        fn_ty,
        args_pack,
        func_loc,
        &mut unique_types as *mut DenseHashSet<TypeId>,
        false,
      );
      if !result.ok.is_empty() {
        if result.ok.len() > 1 {
          self.report_error_type_error_data_location(
            TypeErrorData::AmbiguousFunctionCall(AmbiguousFunctionCall::new(fn_ty, args_pack)),
            &call.base.base.location,
          );
        }
        return;
      }

      if result.incompatible_overloads.len() == 1 {
        for (ty, reasons) in result.incompatible_overloads.iter() {
          match reasons {
            Variant2::V0(reasonings) => {
              for reason in reasonings.iter() {
                resolver.report_errors(
                  &mut module.errors,
                  *ty,
                  func_loc,
                  &module.name,
                  args_pack,
                  &arg_exprs,
                  reason,
                );
              }
            }
            Variant2::V1(errors) => {
              self.report_errors(errors.clone());
            }
          }
        }
        return;
      }

      if result.incompatible_overloads.len() > 1 {
        let (arg_head, _) = flatten_type_pack_id(args_pack);
        let mut overloads_to_report = Vec::new();
        for (overload_ty, _) in result.incompatible_overloads.iter() {
          if !self.is_error_suppressing_location_type_id(call.base.base.location, *overload_ty) {
            overloads_to_report.push(*overload_ty);
          }
        }
        if !overloads_to_report.is_empty() {
          self.report_error_type_error_data_location(
            TypeErrorData::MultipleNonviableOverloads(MultipleNonviableOverloads::new(
              arg_head.len(),
            )),
            &call.base.base.location,
          );
          report_available_overloads(
            &mut module.errors,
            call.base.base.location,
            &module.name,
            &overloads_to_report,
          );
        }
        return;
      }

      if result.arity_mismatches.len() == 1 {
        let mismatch_ty = follow_type_id(result.arity_mismatches[0]);
        if let Some(mismatch_fn) = get_type_id::<FunctionType>(mismatch_ty) {
          let is_variadic = is_variadic(mismatch_fn.arg_types);
          let (min_params, opt_max_params) =
            unsafe { get_parameter_extents(TxnLog::empty(), mismatch_fn.arg_types, true) };
          let (arg_head, _) = flatten_type_pack_id(args_pack);
          self.report_error_type_error_data_location(
            TypeErrorData::CountMismatch(CountMismatch {
              expected: min_params,
              maximum: opt_max_params,
              actual: arg_head.len(),
              context: CountMismatchContext::Arg,
              is_variadic,
              function: String::new(),
            }),
            &func_loc,
          );
          return;
        }
      }

      if !result.arity_mismatches.is_empty() {
        let (arg_head, _) = flatten_type_pack_id(args_pack);
        self.report_error_type_error_data_location(
          TypeErrorData::GenericError(GenericError::new(format!(
            "No overload for function accepts {} arguments.",
            arg_head.len()
          ))),
          &func_loc,
        );
        report_available_overloads(
          &mut module.errors,
          func_loc,
          &module.name,
          &result.arity_mismatches,
        );
        return;
      }

      if !result.non_functions.is_empty() {
        let norm = self.normalizer.try_normalize(fn_ty);
        let hit_limits = norm.as_ref().is_none_or(|norm| {
          self.normalizer.is_inhabited_normalized_type(norm.as_ref())
            == NormalizationResult::HitLimits
        });
        if hit_limits {
          self.report_error_type_error_data_location(
            TypeErrorData::NormalizationTooComplex(NormalizationTooComplex::default()),
            &func_loc,
          );
        }

        if norm
          .as_ref()
          .is_none_or(|norm| !norm.should_suppress_errors())
        {
          self.report_error_type_error_data_location(
            TypeErrorData::CannotCallNonFunction(CannotCallNonFunction { ty: fn_ty }),
            &func_loc,
          );
        }
      } else if get_type_id::<IntersectionType>(fn_ty).is_none()
        && get_type_id::<UnionType>(fn_ty).is_none()
      {
        self.report_error_type_error_data_location(
          TypeErrorData::CannotCallNonFunction(CannotCallNonFunction { ty: fn_ty }),
          &func_loc,
        );
      }
      return;
    };

    let args_slice = call.args.as_slice();
    let mut arg_exprs: Vec<*mut AstExpr> = Vec::new();
    let self_offset = if call.self_ { 1 } else { 0 };
    let use_bidirectional_args =
      fty.generics.is_empty() && fty.generic_packs.is_empty() && !args_slice.is_empty();
    let params_head = if use_bidirectional_args {
      unsafe {
        extend_type_pack(
          &mut module.internal_types,
          self.builtin_types,
          fty.arg_types,
          args_slice.len() + self_offset,
          Vec::new(),
        )
      }
      .head
    } else {
      Vec::new()
    };

    let mut args = TypePack {
      head: Vec::<TypeId>::new(),
      tail: None,
    };

    if call.self_ {
      // SAFETY: call.func 指向 AST arena 节点。
      let index_expr = ast_node_try_as::<AstExprIndexName>(unsafe { &(*call.func).base });
      let Some(index_expr) = index_expr else {
        self.report_error_type_error_data_location(
          TypeErrorData::InternalError(InternalError {
            message: "method call expression has no 'self'".into(),
          }),
          &call.base.base.location,
        );
        return;
      };

      // SAFETY: index_expr->expr 指向 AST arena 节点。
      args
        .head
        .push(self.lookup_type(unsafe { &*index_expr.expr }));
      arg_exprs.push(index_expr.expr);
    }

    for (idx, &arg_expr) in args_slice.iter().enumerate() {
      let is_last = idx + 1 == args_slice.len();

      if is_last
        && let Some(last_arg_pack) = module
          .ast_type_packs
          .find(&(arg_expr as *const AstExpr))
          .copied()
      {
        let (last_arg_head, last_arg_tail) = flatten_type_pack_id(last_arg_pack);
        args.head.extend(last_arg_head);
        args.tail = last_arg_tail;
        continue;
      }

      // SAFETY: arg_expr 指向 AST arena 节点。
      let arg_expr_type = self.lookup_type(unsafe { &*arg_expr });
      arg_exprs.push(arg_expr);
      if use_bidirectional_args
        && idx + self_offset < params_head.len()
        && !self.is_error_suppressing_location_type_id(
          // SAFETY: 同上。
          unsafe { (*arg_expr).base.location },
          arg_expr_type,
        )
      {
        // test_literal_or_ast_type_is_subtype（清单外）仍收裸指针。
        unsafe {
          self.test_literal_or_ast_type_is_subtype(arg_expr, params_head[idx + self_offset])
        };
        args.head.push(params_head[idx + self_offset]);
      } else {
        args.head.push(arg_expr_type);
      }
    }

    // SAFETY: self.module 同上。
    let args_tp = unsafe { (*self.module).internal_types.add_type_pack_t(args.clone()) };
    if let Some(original_ftv) = get_type_id::<FunctionType>(follow_type_id(original_call_ty))
      && let Some(magic) = original_ftv.magic.as_ref()
    {
      let scope = self.find_innermost_scope(call.base.base.location);
      // SAFETY: typechecker/builtin_types/scope 均由构造方保证有效（C++ 同契约）。
      let used_magic = unsafe {
        (magic.type_check)(&MagicFunctionTypeCheckContext {
          typechecker: NonNull::new_unchecked(self as *mut TypeChecker2),
          builtin_types: NonNull::new_unchecked(self.builtin_types),
          call_site: call,
          arguments: args_tp,
          check_scope: NonNull::new_unchecked(scope),
        })
      };

      if used_magic {
        return;
      }
    }

    if args.tail.is_none() {
      let actual = args.head.len();
      let (min_params, opt_max_params) =
        unsafe { get_parameter_extents(TxnLog::empty(), fty.arg_types, false) };

      if actual < min_params || opt_max_params.is_some_and(|max_params| actual > max_params) {
        self.report_error_type_error_data_location(
          TypeErrorData::CountMismatch(CountMismatch {
            expected: min_params,
            maximum: opt_max_params,
            actual,
            context: CountMismatchContext::Arg,
            is_variadic: is_variadic(fty.arg_types),
            function: String::new(),
          }),
          // SAFETY: call.func 指向 AST arena 节点。
          unsafe { &(*call.func).base.location },
        );
        return;
      }
    }

    let args_pack = module.internal_types.add_type_pack_t(args);
    let scope = self.find_innermost_scope(call.base.base.location);
    let mut resolver = unsafe {
      OverloadResolver::new(
        self.builtin_types,
        &mut module.internal_types as *mut _,
        &mut self.normalizer as *mut _,
        self.type_function_runtime,
        scope,
        self.ice,
        self.limits,
        call.base.base.location,
      )
    };
    let mut unique_types: DenseHashSet<TypeId> = DenseHashSet::new(null_mut());
    // SAFETY: 同上。
    unsafe {
      find_unique_types(
        &mut unique_types as *mut DenseHashSet<TypeId>,
        &arg_exprs,
        &module.ast_types as *const _,
      )
    };

    // SAFETY: call.func 指向 AST arena 节点。
    let func_loc = unsafe { (*call.func).base.location };
    let result = resolver.resolve_overload(
      fn_ty,
      args_pack,
      func_loc,
      &mut unique_types as *mut DenseHashSet<TypeId>,
      false,
    );

    if !result.potential_overloads.is_empty() {
      self.report_error_type_error_data_location(
        TypeErrorData::InternalError(InternalError {
          message: "Internal error: outstanding free or blocked type in function call".into(),
        }),
        &call.base.base.location,
      );
    }

    if !result.ok.is_empty() {
      if result.ok.len() > 1 {
        self.report_error_type_error_data_location(
          TypeErrorData::AmbiguousFunctionCall(AmbiguousFunctionCall::new(fn_ty, args_pack)),
          &call.base.base.location,
        );
      }

      self.lookup_type(&call.base);
      return;
    }

    if result.incompatible_overloads.len() == 1 {
      for (ty, reasons) in result.incompatible_overloads.iter() {
        match reasons {
          Variant2::V0(reasonings) => {
            for reason in reasonings.iter() {
              resolver.report_errors(
                &mut module.errors,
                *ty,
                func_loc,
                &module.name,
                args_pack,
                &arg_exprs,
                reason,
              );
            }
          }
          Variant2::V1(errors) => {
            self.report_errors(errors.clone());
          }
        }
      }
      return;
    }

    let (arg_head, _) = flatten_type_pack_id(args_pack);
    if result.incompatible_overloads.len() > 1 {
      let mut overloads_to_report = Vec::new();
      for (overload_ty, _) in result.incompatible_overloads.iter() {
        if !self.is_error_suppressing_location_type_id(call.base.base.location, *overload_ty) {
          overloads_to_report.push(*overload_ty);
        }
      }

      if !overloads_to_report.is_empty() {
        self.report_error_type_error_data_location(
          TypeErrorData::MultipleNonviableOverloads(MultipleNonviableOverloads::new(
            arg_head.len(),
          )),
          &call.base.base.location,
        );
        report_available_overloads(
          &mut module.errors,
          call.base.base.location,
          &module.name,
          &overloads_to_report,
        );
      }
      return;
    }

    if result.arity_mismatches.len() == 1 {
      let mismatch_ty = follow_type_id(result.arity_mismatches[0]);
      if let Some(mismatch_fn) = get_type_id::<FunctionType>(mismatch_ty) {
        let is_variadic = is_variadic(mismatch_fn.arg_types);
        let (min_params, opt_max_params) =
          unsafe { get_parameter_extents(TxnLog::empty(), mismatch_fn.arg_types, true) };
        self.report_error_type_error_data_location(
          TypeErrorData::CountMismatch(CountMismatch {
            expected: min_params,
            maximum: opt_max_params,
            actual: arg_head.len(),
            context: CountMismatchContext::Arg,
            is_variadic,
            function: String::new(),
          }),
          &func_loc,
        );
        return;
      }
    }

    if !result.arity_mismatches.is_empty() {
      self.report_error_type_error_data_location(
        TypeErrorData::GenericError(GenericError::new(format!(
          "No overload for function accepts {} arguments.",
          arg_head.len()
        ))),
        &func_loc,
      );
      report_available_overloads(
        &mut module.errors,
        func_loc,
        &module.name,
        &result.arity_mismatches,
      );
      return;
    }

    if !result.non_functions.is_empty() {
      let norm = self.normalizer.try_normalize(fn_ty);
      let hit_limits = norm.as_ref().is_none_or(|norm| {
        self.normalizer.is_inhabited_normalized_type(norm.as_ref())
          == NormalizationResult::HitLimits
      });
      if hit_limits {
        self.report_error_type_error_data_location(
          TypeErrorData::NormalizationTooComplex(NormalizationTooComplex::default()),
          &func_loc,
        );
      }

      if norm
        .as_ref()
        .is_none_or(|norm| !norm.should_suppress_errors())
      {
        self.report_error_type_error_data_location(
          TypeErrorData::CannotCallNonFunction(CannotCallNonFunction { ty: fn_ty }),
          &func_loc,
        );
      }
    }
  }
}
