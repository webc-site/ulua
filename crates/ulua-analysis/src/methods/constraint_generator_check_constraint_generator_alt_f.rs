//! Source: `Analysis/src/ConstraintGenerator.cpp:3281-3285` (hand-ported)
//! C++ `Inference ConstraintGenerator::check(const ScopePtr& scope, AstExprIndexName* indexName)`.
use alloc::string::String;
use core::ffi::CStr;

use ulua_ast::records::{ast_expr::AstExpr, ast_expr_index_name::AstExprIndexName};

use crate::{
  records::{constraint_generator::ConstraintGenerator, inference::Inference},
  type_aliases::scope_ptr_type::ScopePtr,
};

impl ConstraintGenerator {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn check_scope_ptr_ast_expr_index_name(
    &mut self,
    scope: &ScopePtr,
    index_name: *mut AstExprIndexName,
  ) -> Inference {
    unsafe {
      let key = (*self.dfg).get_refinement_key(index_name as *const AstExpr);
      let index: String = CStr::from_ptr((*index_name).index.value)
        .to_string_lossy()
        .into_owned();
      self.check_index_name(
        scope,
        key,
        (*index_name).expr,
        &index,
        (*index_name).index_location,
      )
    }
  }
}
