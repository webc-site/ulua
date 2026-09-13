//! Node: `cxx:Function:Luau.Analysis:Analysis/src/AstUtils.cpp:70:find_unique_types`
//! Source: `Analysis/src/AstUtils.cpp`
//!
//! Faithful port of:
//! ```cpp
//! void findUniqueTypes(NotNull<DenseHashSet<TypeId>> uniqueTypes, AstExpr* expr,
//!     NotNull<const DenseHashMap<const AstExpr*, TypeId>> ast_types)
//! {
//!     AstExprTableFinder finder{uniqueTypes, ast_types};
//!     expr->visit(&finder);
//! }
//! ```

use ulua_ast::{records::ast_expr::AstExpr, visit::ast_expr_visit};
use ulua_common::records::{dense_hash_map::DenseHashMap, dense_hash_set::DenseHashSet};

use crate::{records::ast_expr_table_finder::AstExprTableFinder, type_aliases::type_id::TypeId};

/// # Safety
/// 调用方须保证满足 C++ 原实现的调用契约。
pub unsafe fn find_unique_types(
  unique_types: *mut DenseHashSet<TypeId>,
  expr: *mut AstExpr,
  ast_types: *const DenseHashMap<*const AstExpr, TypeId>,
) {
  unsafe {
    let mut finder = AstExprTableFinder::new(unique_types, ast_types);
    ast_expr_visit(expr, &mut finder);
  }
}
