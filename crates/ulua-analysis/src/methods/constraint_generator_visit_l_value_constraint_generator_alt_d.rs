use alloc::{string::String, sync::Arc};
use core::ffi::CStr;

use ulua_ast::records::{ast_expr::AstExpr, ast_expr_index_name::AstExprIndexName};

use crate::{
  functions::get_mutable_type::get_mutable_type_id,
  records::{
    assign_prop_constraint::AssignPropConstraint, blocked_type::BlockedType,
    constraint_generator::ConstraintGenerator, module::Module,
  },
  type_aliases::{constraint_v::ConstraintV, scope_ptr_type::ScopePtr, type_id::TypeId},
};
impl ConstraintGenerator {
  // ConstraintGenerator::visitLValue(const ScopePtr&, AstExprIndexName*, TypeId)
  // (ConstraintGenerator.cpp:3825).
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn visit_l_value_scope_ptr_ast_expr_index_name_type_id(
    &mut self,
    scope: &ScopePtr,
    expr: *mut AstExprIndexName,
    rhs_type: TypeId,
  ) {
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
      }
    }

    let incremented = self.record_property_assignment(lhs_ty);

    let prop_name: String = unsafe {
      CStr::from_ptr((*expr).index.value)
        .to_string_lossy()
        .into_owned()
    };

    let apc = self.add_constraint_scope_ptr_location_constraint_v(
      scope,
      unsafe { (*expr).base.base.location },
      ConstraintV::AssignProp(AssignPropConstraint {
        lhs_type: lhs_ty,
        prop_name,
        rhs_type,
        prop_location: Some(unsafe { (*expr).index_location }),
        prop_type: prop_ty,
        decrement_prop_count: incremented,
      }),
    );

    // prop_ty 刚由 add_type(BlockedType) 分配，必命中；对照 C++:3904
    // `getMutable<BlockedType>(propTy)->setOwner(apc)`
    let blocked = get_mutable_type_id::<BlockedType>(prop_ty).unwrap();
    blocked.set_owner(apc as *const _);
  }
}
