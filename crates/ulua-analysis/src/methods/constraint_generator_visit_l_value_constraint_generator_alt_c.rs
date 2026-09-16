use ulua_ast::records::{ast_expr::AstExpr, ast_expr_global::AstExprGlobal};
use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::{
    as_mutable_type::as_mutable_type_id, follow_type::follow_type_id, get_type_alt_j::get,
  },
  records::{
    blocked_type::BlockedType, constraint_generator::ConstraintGenerator,
    subtype_constraint::SubtypeConstraint, symbol::Symbol,
  },
  type_aliases::{
    constraint_v::ConstraintV, scope_ptr_type::ScopePtr, type_id::TypeId, type_variant::TypeVariant,
  },
};

impl ConstraintGenerator {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn visit_l_value_scope_ptr_ast_expr_global_type_id(
    &mut self,
    scope: &ScopePtr,
    global: *mut AstExprGlobal,
    rhs_type: TypeId,
  ) {
    let global_name = unsafe { (*global).name };
    let annotated_ty = scope.lookup_symbol(Symbol::from_global(global_name));
    if let Some(annotated_ty_val) = annotated_ty {
      let def = unsafe { (*self.dfg).get_def(global as *const AstExprGlobal as *const AstExpr) };
      unsafe {
        *(*self.root_scope).lvalue_types.get_or_insert(def) = rhs_type;
      }

      // Ignore possible self-assignment, it doesn't create a new constraint.
      let followed_rhs = follow_type_id(rhs_type);
      if annotated_ty_val == followed_rhs {
        return;
      }

      let followed_annotation = follow_type_id(annotated_ty_val);
      // 对照 C++:3886 `if (auto bt = get<BlockedType>(...); bt && uninitializedGlobals.contains(...))`
      if let Some(bt) = get::<BlockedType>(followed_annotation)
        && self.uninitialized_globals.contains(&global_name)
      {
        LUAU_ASSERT!(bt.get_owner().is_null());
        self.uninitialized_globals.erase(&global_name);
        // SAFETY: followed_annotation 是 arena 内的类型句柄，与 C++ asMutable 同契约。
        unsafe {
          (*as_mutable_type_id(followed_annotation)).ty = TypeVariant::Bound(rhs_type);
        }
      }

      self.add_constraint_scope_ptr_location_constraint_v(
        scope,
        unsafe { (*global).base.base.location },
        ConstraintV::Subtype(SubtypeConstraint {
          sub_type: rhs_type,
          super_type: annotated_ty_val,
        }),
      );
    }
  }
}
