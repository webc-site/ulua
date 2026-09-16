// ConstraintGenerator::visit(const ScopePtr&, AstStatFor*) (ConstraintGenerator.cpp:1554-1585).
use alloc::string::String;

use ulua_ast::records::{ast_expr::AstExpr, ast_node::AstNode, ast_stat_for::AstStatFor};

use crate::{
  enums::{control_flow::ControlFlow, polarity::Polarity},
  records::{
    binding::Binding, constraint_generator::ConstraintGenerator, scope::Scope,
    subtype_constraint::SubtypeConstraint, symbol::Symbol,
  },
  type_aliases::{constraint_v::ConstraintV, scope_ptr_type::ScopePtr, type_id::TypeId},
};
impl ConstraintGenerator {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn visit_scope_ptr_ast_stat_for(
    &mut self,
    scope: &ScopePtr,
    for_: *mut AstStatFor,
  ) -> ControlFlow {
    let for_ref = unsafe { &*for_ };
    let var = for_ref.var;

    let mut annotation_ty: TypeId = unsafe { (*self.builtin_types).number_type };
    if !unsafe { (*var).annotation }.is_null() {
      annotation_ty = self.resolve_type(
        scope.as_ref() as *const Scope as *mut Scope,
        unsafe { (*var).annotation },
        /* in_type_arguments */ false,
        /* replace_error_with_fresh */ false,
        Polarity::Positive,
      );
    }

    // C++ inferNumber lambda.
    let number_ty: TypeId = unsafe { (*self.builtin_types).number_type };
    let infer_number = |this: &mut Self, expr: *mut AstExpr| {
      if expr.is_null() {
        return;
      }
      let t = this.check_scope_ptr_ast_expr(scope, expr).ty;
      this.add_constraint_scope_ptr_location_constraint_v(
        scope,
        unsafe { (*expr).base.location },
        ConstraintV::Subtype(SubtypeConstraint {
          sub_type: t,
          super_type: number_ty,
        }),
      );
    };

    infer_number(self, for_ref.from);
    infer_number(self, for_ref.to);
    infer_number(self, for_ref.step);

    let for_scope: ScopePtr =
      unsafe { self.child_scope(&for_ref.base.base as *const AstNode as *mut AstNode, scope) };
    let for_scope_raw = for_scope.as_ref() as *const Scope as *mut Scope;

    unsafe {
      (*for_scope_raw).bindings.insert(
        Symbol::from_local(var),
        Binding {
          type_id: annotation_ty,
          location: (*var).location,
          deprecated: false,
          deprecated_suggestion: String::new(),
          documentation_symbol: None,
        },
      );
    }

    let def = unsafe { (*self.dfg).get_def_local(var) };
    unsafe {
      *(*for_scope_raw).lvalue_types.get_or_insert(def) = annotation_ty;
    }
    self.update_r_value_refinements_scope_ptr_def_id_type_id(&for_scope, def, annotation_ty);

    self.visit_scope_ptr_ast_stat_block(&for_scope, for_ref.body);

    unsafe {
      (*(scope.as_ref() as *const Scope as *mut Scope)).inherit_assignments(&for_scope);
    }

    ControlFlow::None
  }
}
