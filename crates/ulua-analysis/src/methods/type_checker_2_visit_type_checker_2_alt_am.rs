//! Faithful port of `TypeChecker2::visit(AstExprFunction*)` (TypeChecker2.cpp:2026-2148).
use alloc::format;
use core::ffi::CStr;

use ulua_ast::records::{ast_expr_function::AstExprFunction, ast_node::AstNode, ast_stat::AstStat};

use crate::{
  enums::type_context::TypeContext,
  functions::{
    begin_type_pack::begin, end_type_pack::end, follow_type::follow_type_id,
    follow_type_pack::follow_type_pack_id, get_type_alt_j::get_type_id,
    to_string_to_string_alt_c::to_string_type_id,
  },
  records::{
    code_too_complex::CodeTooComplex, extra_information::ExtraInformation,
    function_exits_without_returning::FunctionExitsWithoutReturning, function_type::FunctionType,
    generic_error::GenericError, in_conditional_context::InConditionalContext,
    internal_error::InternalError, never_type::NeverType, type_checker_2::TypeChecker2,
  },
  type_aliases::error_type::ErrorType,
};
impl TypeChecker2 {
  pub fn visit_ast_expr_function(&mut self, function: &AstExprFunction) {
    let fn_ref = function;
    let location = fn_ref.base.base.location;

    // InConditionalContext flipper(&typeContext, TypeContext::Default);
    let _flipper =
      unsafe { InConditionalContext::new(&mut self.type_context, TypeContext::Default) };

    // auto StackPusher = pushStack(fn);
    // AstNode 子对象在偏移 0，指针值与 push_stack 期望一致。
    let _stack_pusher = self.type_checker_2_push_stack(
      (function as *const AstExprFunction).cast::<AstNode>() as *mut AstNode,
    );

    self.visit_generics(fn_ref.generics, fn_ref.generic_packs);

    let inferred_fn_ty = self.lookup_type(&function.base);
    self.function_decl_stack.push(inferred_fn_ty);

    // std::shared_ptr<const NormalizedType> normalizedFnTy = normalizer.normalize(inferredFnTy);
    // Rust 的 normalize 返回非空 Arc，C++ 的 null 分支不可表示（保守保留结构）。
    let mut normalized_fn_ty = Some(self.normalizer.normalize(inferred_fn_ty));

    if normalized_fn_ty.is_none() {
      self.report_error_type_error_data_location(CodeTooComplex::default().into(), &location);
    } else if get_type_id::<ErrorType>(normalized_fn_ty.as_ref().unwrap().errors).is_some() {
      // If we have an error type, we don't want to do anything else involving the normalized type
      normalized_fn_ty = None;
    } else if !normalized_fn_ty.as_ref().unwrap().has_functions() {
      self.report_error_type_error_data_location(
        InternalError::new(format!(
          "Internal error: Lambda has non-function type {}",
          to_string_type_id(inferred_fn_ty)
        ))
        .into(),
        &location,
      );
      self.function_decl_stack.pop();
      return;
    } else {
      let normalized = normalized_fn_ty.as_ref().unwrap();
      if normalized.functions.parts.size() != 1 {
        self.report_error_type_error_data_location(
          InternalError::new(format!(
            "Unexpected: Lambda has unexpected type {}",
            to_string_type_id(inferred_fn_ty)
          ))
          .into(),
          &location,
        );
        self.function_decl_stack.pop();
        return;
      }

      let inferred_ftv_ty = normalized.functions.parts.front();
      // C++ LUAU_ASSERT(inferredFtv)（TypeChecker2.cpp:2065-2066）。
      let inferred_ftv =
        get_type_id::<FunctionType>(inferred_ftv_ty).expect("front part is a function");

      // There is no way to write an annotation for the self argument, so we
      // cannot do anything to check it.
      let mut arg_it = begin(inferred_ftv.arg_types);
      let arg_end = end(inferred_ftv.arg_types);
      if !fn_ref.self_.is_null() {
        arg_it.operator_inc();
      }

      let args = fn_ref.args.as_slice();
      for &arg in args {
        if !arg_it.operator_ne(&arg_end) {
          break;
        }

        // SAFETY: arg 指向 AST arena 节点；iterator 解引用指向类型 arena。
        let (arg_ref, inferred_arg_ty) = unsafe { (&*arg, *arg_it.operator_deref()) };

        if !arg_ref.annotation.is_null() {
          // we need to typecheck any argument annotations themselves.
          unsafe { self.visit_ast_type(arg_ref.annotation) };

          // SAFETY: annotation 非 null，由语法树构造方保证有效。
          let annotated_arg_ty = self.lookup_annotation(unsafe { &*arg_ref.annotation });

          self.test_is_subtype_type_id_type_id_location(
            inferred_arg_ty,
            annotated_arg_ty,
            arg_ref.location,
          );
        }

        // Some Luau constructs can result in an argument type being
        // reduced to never by inference. In this case, we want to
        // report an error at the function, instead of reporting an
        // error at every callsite.
        if get_type_id::<NeverType>(follow_type_id(inferred_arg_ty)).is_some() {
          // If the annotation simplified to never, we don't want to
          // even look at contributors.
          let mut explicitly_never = false;
          if !arg_ref.annotation.is_null() {
            // SAFETY: 同上。
            let annotated_arg_ty = self.lookup_annotation(unsafe { &*arg_ref.annotation });
            explicitly_never = get_type_id::<NeverType>(annotated_arg_ty).is_some();
          }

          // Not following here is deliberate.
          // SAFETY: self.module 与类型检查会话同寿（C++ 同契约）。
          let contributors = unsafe { &*self.module }
            .upper_bound_contributors
            .find(&inferred_arg_ty)
            .cloned();
          if let Some(contributors) = contributors
            && !explicitly_never
          {
            // SAFETY: name.value 指向 AST arena 内 NUL 结尾字面量。
            let arg_name = unsafe { CStr::from_ptr(arg_ref.name.value) }
              .to_string_lossy()
              .into_owned();
            self.report_error_type_error_data_location(
              GenericError::new(format!(
                "Parameter '{}' has been reduced to never. This function is not callable with any possible value.",
                arg_name
              ))
              .into(),
              &arg_ref.location,
            );
            for (site, component) in contributors {
              self.report_error_type_error_data_location(
                ExtraInformation::new(format!(
                  "Parameter '{}' is required to be a subtype of '{}' here.",
                  arg_name,
                  to_string_type_id(component)
                ))
                .into(),
                &site,
              );
            }
          }
        }

        arg_it.operator_inc();
      }

      // we need to typecheck the vararg annotation, if it exists.
      if fn_ref.vararg && !fn_ref.vararg_annotation.is_null() {
        self.visit_ast_type_pack(fn_ref.vararg_annotation);
      }

      // SAFETY: fn_ref.body 指向 AST arena 节点。
      let reaches_implicit_return = !self
        .type_checker_2_get_fallthrough(fn_ref.body as *const AstStat)
        .is_null();
      if reaches_implicit_return
        && !self.allows_no_return_values(unsafe { follow_type_pack_id(inferred_ftv.ret_types) })
      {
        let end_location = unsafe { self.get_end_location(function) };
        self.report_error_type_error_data_location(
          FunctionExitsWithoutReturning {
            expected_return_type: inferred_ftv.ret_types,
          }
          .into(),
          &end_location,
        );
      }
    }

    self.visit_ast_stat_block(fn_ref.body);

    // we need to typecheck the return annotation itself, if it exists.
    if !fn_ref.return_annotation.is_null() {
      self.visit_ast_type_pack(fn_ref.return_annotation);
    }

    // If the function type has a function annotation, we need to see if we can suggest an annotation
    if let Some(normalized) = normalized_fn_ty.as_ref() {
      let part = normalized.functions.parts.front();
      self.type_checker_2_suggest_annotations(function, part);
    }

    self.function_decl_stack.pop();
  }
}
