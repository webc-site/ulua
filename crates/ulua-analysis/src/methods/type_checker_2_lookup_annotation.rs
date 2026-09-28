//! `TypeChecker2::lookupAnnotation`（TypeChecker2.cpp 对照）。
use alloc::format;

use ulua_ast::{
  records::{
    ast_node::AstNode, ast_type::AstType, ast_type_or_pack::AstTypeOrPack,
    ast_type_reference::AstTypeReference,
  },
  rtti::ast_node_try_as,
};
use ulua_common::fflag;

use crate::{
  functions::{
    follow_type,
    magic_names::{LUAU_FORCE_CONSTRAINT_SOLVING_INCOMPLETE, LUAU_PRINT},
    set_print_line::luau_print_line,
    to_string_to_string::to_string_type_id,
  },
  records::{
    constraint_solving_incomplete_error::ConstraintSolvingIncompleteError,
    type_checker_2::TypeChecker2,
  },
  type_aliases::type_id::TypeId,
};
impl TypeChecker2 {
  pub fn lookup_annotation(&mut self, annotation: &AstType) -> TypeId {
    if fflag::DebugLuauMagicTypes.get() {
      // SAFETY: AstType 是 #[repr(C)] 单继承，base(AstNode) 在偏移 0，cast 有效。
      let node = unsafe { &*((annotation as *const AstType).cast::<AstNode>()) };
      if let Some(ref_ty) = ast_node_try_as::<AstTypeReference>(node) {
        // SAFETY: name.value 指向 AST arena 内的 NUL 结尾字面量。
        let name = ref_ty.name.as_str_or_empty();

        if name == LUAU_PRINT
          && let Some(&param) = ref_ty.parameters.first()
        {
          if let AstTypeOrPack::Type(arg) = param {
            // 变体载荷即 arena 存活节点（`AstTypeOrPack::Type` 构造契约），
            // 与 cpp 判过非空后解引用 `param.type` 同值；其余形态走 cpp 的
            // 「type 为 null 直落后续分支」路径。
            let arg_ty = self.lookup_annotation(arg);
            let line = format!(
              "{LUAU_PRINT} ({}, {}): {}\n",
              annotation.base.location.begin.line,
              annotation.base.location.begin.column,
              to_string_type_id(arg_ty)
            );

            if let Some(print_line) = luau_print_line() {
              print_line(&line);
            }

            return follow_type::follow(arg_ty);
          }
        } else if name == LUAU_FORCE_CONSTRAINT_SOLVING_INCOMPLETE {
          let location = annotation.base.location;
          self.report_error_type_error_data_location(
            ConstraintSolvingIncompleteError::default().into(),
            &location,
          );
          // SAFETY: builtin_types 由构造方保证有效（C++ 同契约）。
          return self.builtin_types_ref().any_type;
        }
      }
    }

    // SAFETY: self.module 与类型检查会话同寿（C++ 同契约）。
    let module = self.module_ref();
    let ty = module
      .ast_resolved_types
      .find(&(annotation as *const AstType));

    if module.constraint_generation_did_not_complete && ty.is_none() {
      // SAFETY: 同上。
      return self.builtin_types_ref().any_type;
    }

    let ty = *ty.expect("Type annotation must be resolved");
    self.check_for_type_function_inhabitance(follow_type::follow(ty), annotation.base.location)
  }
}
