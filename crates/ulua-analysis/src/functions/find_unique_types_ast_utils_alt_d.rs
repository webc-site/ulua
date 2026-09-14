//! Node: `cxx:Function:Luau.Analysis:Analysis/src/AstUtils.cpp:103:find_unique_types`
//! Source: `Analysis/src/AstUtils.cpp`
//!
//! Faithful port of:
//! ```cpp
//! void findUniqueTypes(NotNull<DenseHashSet<TypeId>> uniqueTypes, const std::vector<AstExpr*>& exprs,
//!     NotNull<const DenseHashMap<const AstExpr*, TypeId>> ast_types)
//! {
//!     findUniqueTypes(uniqueTypes, exprs.begin(), exprs.end(), ast_types);
//! }
//! ```

use ulua_ast::records::ast_expr::AstExpr;
use ulua_common::records::{dense_hash_map::DenseHashMap, dense_hash_set::DenseHashSet};

use crate::{
  functions::find_unique_types_ast_utils_alt_b::find_unique_types_iter,
  type_aliases::type_id::TypeId,
};

/// # Safety
/// 调用方须保证满足 C++ 原实现的调用契约。
pub unsafe fn find_unique_types(
  unique_types: *mut DenseHashSet<TypeId>,
  exprs: &[*mut AstExpr],
  ast_types: *const DenseHashMap<*const AstExpr, TypeId>,
) {
  unsafe {
    find_unique_types_iter(unique_types, exprs.iter().copied(), ast_types);
  }
}
