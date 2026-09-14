//! `TypeChecker2::lookupAnnotation`（TypeChecker2.cpp 对照）。
use alloc::format;
use core::ffi::CStr;

use ulua_ast::{
  records::{ast_node::AstNode, ast_type::AstType, ast_type_reference::AstTypeReference},
  rtti::ast_node_try_as,
};
use ulua_common::FFlag;

use crate::{
  functions::{
    follow_type::follow_type_id, set_print_line::LUAU_PRINT_LINE,
    to_string_to_string_alt_c::to_string_type_id,
  },
  records::{
    constraint_solving_incomplete_error::ConstraintSolvingIncompleteError,
    type_checker_2::TypeChecker2,
  },
  type_aliases::type_id::TypeId,
};
impl TypeChecker2 {
  pub fn lookup_annotation(&mut self, annotation: &AstType) -> TypeId {
    if FFlag::DebugLuauMagicTypes.get() {
      // SAFETY: AstType 是 #[repr(C)] 单继承，base(AstNode) 在偏移 0，cast 有效。
      let node = unsafe { &*((annotation as *const AstType).cast::<AstNode>()) };
      if let Some(ref_ty) = ast_node_try_as::<AstTypeReference>(node) {
        // SAFETY: name.value 指向 AST arena 内的 NUL 结尾字面量。
        let name = unsafe { CStr::from_ptr(ref_ty.name.value) }.to_string_lossy();

        if name == "_luau_print" && ref_ty.parameters.size > 0 {
          // SAFETY: parameters.data 指向 arena，size > 0 已判定。
          let param = unsafe { *ref_ty.parameters.data.add(0) };

          if !param.r#type.is_null() {
            // SAFETY: param.r#type 非 null，由语法树构造方保证有效。
            let arg_ty = self.lookup_annotation(unsafe { &*param.r#type });
            let line = format!(
              "_luau_print ({}, {}): {}\n",
              annotation.base.location.begin.line,
              annotation.base.location.begin.column,
              to_string_type_id(arg_ty)
            );

            unsafe {
              if let Some(print_line) = LUAU_PRINT_LINE {
                print_line(&line);
              }
            }

            return follow_type_id(arg_ty);
          }
        } else if name == "_luau_force_constraint_solving_incomplete" {
          let location = annotation.base.location;
          self.report_error_type_error_data_location(
            ConstraintSolvingIncompleteError::default().into(),
            &location,
          );
          // SAFETY: builtin_types 由构造方保证有效（C++ 同契约）。
          return unsafe { (*self.builtin_types).any_type };
        }
      }
    }

    // SAFETY: self.module 与类型检查会话同寿（C++ 同契约）。
    let module = unsafe { &*self.module };
    let ty = module
      .ast_resolved_types
      .find(&(annotation as *const AstType));

    if module.constraint_generation_did_not_complete && ty.is_none() {
      // SAFETY: 同上。
      return unsafe { (*self.builtin_types).any_type };
    }

    let ty = *ty.expect("Type annotation must be resolved");
    self.check_for_type_function_inhabitance(follow_type_id(ty), annotation.base.location)
  }
}
