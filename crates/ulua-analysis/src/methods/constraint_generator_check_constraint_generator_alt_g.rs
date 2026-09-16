//! Source: `Analysis/src/ConstraintGenerator.cpp:3287-3316` (hand-ported)
//! C++ `Inference ConstraintGenerator::check(const ScopePtr& scope, AstExprIndexExpr* indexExpr)`.
use alloc::{string::String, sync::Arc};
use core::{ptr::null_mut, str::from_utf8};

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
    blocked_type::BlockedType, constraint_generator::ConstraintGenerator,
    has_indexer_constraint::HasIndexerConstraint, inference::Inference, module::Module,
  },
  type_aliases::{constraint_v::ConstraintV, def_id_def::DefId, scope_ptr_type::ScopePtr},
};
impl ConstraintGenerator {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn check_scope_ptr_ast_expr_index_expr(
    &mut self,
    scope: &ScopePtr,
    index_expr: *mut AstExprIndexExpr,
  ) -> Inference {
    unsafe {
      let constant_string =
        ast_node_as::<AstExprConstantString>((*index_expr).index as *mut AstNode);
      if !constant_string.is_null() {
        if let Some(module) = &self.module {
          let module_ptr = Arc::as_ptr(module) as *mut Module;
          *(*module_ptr)
            .ast_types
            .get_or_insert((*index_expr).index as *const AstExpr) =
            (*self.builtin_types).string_type;
        }
        let key = (*self.dfg).get_refinement_key(index_expr as *const AstExpr);
        let index: String =
          String::from(from_utf8((*constant_string).value.as_bytes()).unwrap_or(""));
        return self.check_index_name(
          scope,
          key,
          (*index_expr).expr,
          &index,
          (*index_expr).base.base.location,
        );
      }

      let obj = self.check_scope_ptr_ast_expr(scope, (*index_expr).expr).ty;
      let index_type = self.check_scope_ptr_ast_expr(scope, (*index_expr).index).ty;

      let result = (*self.arena).add_type(BlockedType::default());

      let key = (*self.dfg).get_refinement_key(index_expr as *const AstExpr);
      if !key.is_null() {
        // C++ default `prototype = true`.
        if let Some(ty) = self.lookup(
          scope,
          (*index_expr).base.base.location,
          (*key).def as DefId,
          true,
        ) {
          let refinement = self
            .refinement_arena
            .proposition_refinement_key_type_id(key, (*self.builtin_types).truthy_type);
          return Inference::inference_type_id_refinement_id(ty, refinement);
        }
        self.update_r_value_refinements_scope_ptr_def_id_type_id(
          scope,
          (*key).def as DefId,
          result,
        );
      }

      let c = self.add_constraint_scope_ptr_location_constraint_v(
        scope,
        (*(*index_expr).expr).base.location,
        ConstraintV::HasIndexer(HasIndexerConstraint {
          result_type: result,
          subject_type: obj,
          index_type,
        }),
      );
      // result 刚由 add_type(BlockedType) 分配，必命中；对照 C++:3349
      // `getMutable<BlockedType>(result)->setOwner(c)`
      let blocked = get_mutable_type_id::<BlockedType>(result).unwrap();
      blocked.set_owner(c as *const _);

      if !key.is_null() {
        let refinement = self
          .refinement_arena
          .proposition_refinement_key_type_id(key, (*self.builtin_types).truthy_type);
        Inference::inference_type_id_refinement_id(result, refinement)
      } else {
        Inference::inference_type_id_refinement_id(result, null_mut())
      }
    }
  }
}
