use ulua_ast::{
  records::{ast_expr::AstExpr, ast_expr_table::AstExprTable},
  rtti::ast_node_is,
  visit::ast_expr_visit_ref,
};
use ulua_common::records::{dense_hash_map::DenseHashMap, dense_hash_set::DenseHashSet};

use crate::{
  records::{
    arena_handle::{alias, alias_ref},
    ast_expr_table_finder::AstExprTableFinder,
  },
  type_aliases::type_id::TypeId,
};

/// 对应 C++ `findUniqueTypes(NotNull<DenseHashSet<TypeId>>, AstExpr*,
/// NotNull<const DenseHashMap<...>>)`（`cpp/Analysis/src/AstUtils.cpp:70`）。
///
/// 降 safe 说明：集合/映射形参原为借用裸指针（调用点写的就是
/// `&mut x as *mut _`），收窄为引用后由借用检查器承担全部前提；
/// `expr` 的 `&mut` 即节点存活且独占的类型系统证明，遍历经引用门面
/// `ast_expr_visit_ref` 全链路 safe（遍历器只读 AST、写集合，无并存别名）。
pub fn find_unique_types(
  unique_types: &mut DenseHashSet<TypeId>,
  expr: &mut AstExpr,
  ast_types: &DenseHashMap<*const AstExpr, TypeId>,
) {
  let mut finder = AstExprTableFinder::new(unique_types, ast_types);
  ast_expr_visit_ref(expr, &mut finder);
}

/// 对应 C++ 迭代器版 `findUniqueTypes(uniqueTypes, begin, end, astTypes)`
/// （`cpp/Analysis/src/AstUtils.cpp:77`）。降 safe：集合/映射形参收引用；
/// `iter` 产出的每个 `*mut AstExpr` 是 parser arena 存活节点（地址不移动），
/// 判型与解引用统一经 `alias_ref`/`alias` 句柄门面收口，循环体无 `unsafe`。
fn find_unique_types_iter<I>(
  unique_types: &mut DenseHashSet<TypeId>,
  iter: I,
  ast_types: &DenseHashMap<*const AstExpr, TypeId>,
) where
  I: IntoIterator<Item = *mut AstExpr>,
{
  for expr in iter {
    if ast_node_is::<AstExprTable>(&alias_ref(expr).base) {
      let expr = alias(expr);
      find_unique_types(unique_types, expr, ast_types);
    }
  }
}

/// 对应 C++ `findUniqueTypes(uniqueTypes, AstArray<AstExpr*>, astTypes)` 族
/// （`cpp/Analysis/src/AstUtils.cpp:94/103`）。降 safe：切片元素前提转由
/// `find_unique_types_iter` 的窄块逐元素证成。
pub fn find_unique_types_exprs(
  unique_types: &mut DenseHashSet<TypeId>,
  exprs: &[*mut AstExpr],
  ast_types: &DenseHashMap<*const AstExpr, TypeId>,
) {
  find_unique_types_iter(unique_types, exprs.iter().copied(), ast_types);
}
