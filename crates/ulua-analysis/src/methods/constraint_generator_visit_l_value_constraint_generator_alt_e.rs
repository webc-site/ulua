use alloc::{string::String, sync::Arc};
use core::str::from_utf8;

use ulua_ast::{
  records::{
    ast_expr::AstExpr, ast_expr_constant_string::AstExprConstantString,
    ast_expr_index_expr::AstExprIndexExpr, ast_node::AstNode,
  },
  rtti::ast_node_as,
};

use crate::{
  functions::get_mutable_type::get_mutable_type_id,
  records::{
    assign_index_constraint::AssignIndexConstraint, assign_prop_constraint::AssignPropConstraint,
    blocked_type::BlockedType, constraint_generator::ConstraintGenerator, module::Module,
  },
  type_aliases::{constraint_v::ConstraintV, scope_ptr_type::ScopePtr, type_id::TypeId},
};
impl ConstraintGenerator {
  // ConstraintGenerator::visitLValue(const ScopePtr&, AstExprIndexExpr*, TypeId)
  // (ConstraintGenerator.cpp:3838).
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn visit_l_value_scope_ptr_ast_expr_index_expr_type_id(
    &mut self,
    scope: &ScopePtr,
    expr: *mut AstExprIndexExpr,
    rhs_type: TypeId,
  ) {
    let index = unsafe { (*expr).index };
    let constant_string = unsafe { ast_node_as::<AstExprConstantString>(index as *mut AstNode) };
    if !constant_string.is_null() {
      let lhs_ty = self
        .check_scope_ptr_ast_expr(scope, unsafe { (*expr).expr })
        .ty;
      let prop_ty = unsafe { (*self.arena).add_type(BlockedType::default()) };

      if let Some(module) = &self.module {
        let module_ptr = Arc::as_ptr(module) as *mut Module;
        unsafe {
          *(*module_ptr)
            .ast_types
            .get_or_insert(expr as *const AstExpr) = prop_ty;
          // FIXME? Singleton strings exist.
          *(*module_ptr)
            .ast_types
            .get_or_insert(index as *const AstExpr) = (*self.builtin_types).string_type;
        }
      }

      let prop_name: String =
        unsafe { String::from(from_utf8((*constant_string).value.as_bytes()).unwrap_or("")) };

      let incremented = self.record_property_assignment(lhs_ty);

      let apc = self.add_constraint_scope_ptr_location_constraint_v(
        scope,
        unsafe { (*expr).base.base.location },
        ConstraintV::AssignProp(AssignPropConstraint {
          lhs_type: lhs_ty,
          prop_name,
          rhs_type,
          prop_location: Some(unsafe { (*index).base.location }),
          prop_type: prop_ty,
          decrement_prop_count: incremented,
        }),
      );

      // prop_ty 刚由 add_type(BlockedType) 分配，必命中；对照 C++:3927
      // `getMutable<BlockedType>(propTy)->setOwner(apc)`
      let blocked = get_mutable_type_id::<BlockedType>(prop_ty).unwrap();
      blocked.set_owner(apc as *const _);

      return;
    }

    let lhs_ty = self
      .check_scope_ptr_ast_expr(scope, unsafe { (*expr).expr })
      .ty;
    let index_ty = self.check_scope_ptr_ast_expr(scope, index).ty;
    let prop_ty = unsafe { (*self.arena).add_type(BlockedType::default()) };

    if let Some(module) = &self.module {
      let module_ptr = Arc::as_ptr(module) as *mut Module;
      unsafe {
        *(*module_ptr)
          .ast_types
          .get_or_insert(expr as *const AstExpr) = prop_ty;
      }
    }

    let aic = self.add_constraint_scope_ptr_location_constraint_v(
      scope,
      unsafe { (*expr).base.base.location },
      ConstraintV::AssignIndex(AssignIndexConstraint {
        lhs_type: lhs_ty,
        index_type: index_ty,
        rhs_type,
        prop_type: prop_ty,
      }),
    );

    // prop_ty 刚由 add_type(BlockedType) 分配，必命中；对照 C++:3937
    // `getMutable<BlockedType>(propTy)->setOwner(aic)`
    let blocked = get_mutable_type_id::<BlockedType>(prop_ty).unwrap();
    blocked.set_owner(aic as *const _);
  }
}
