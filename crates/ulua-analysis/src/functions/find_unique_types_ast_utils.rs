use ulua_ast::{
  records::{ast_expr::AstExpr, ast_expr_table::AstExprTable},
  rtti::ast_node_is,
  visit::ast_expr_visit,
};
use ulua_common::records::{dense_hash_map::DenseHashMap, dense_hash_set::DenseHashSet};

use crate::{records::ast_expr_table_finder::AstExprTableFinder, type_aliases::type_id::TypeId};

/// 对应 C++ `findUniqueTypes(NotNull<DenseHashSet<TypeId>>, AstExpr*,
/// NotNull<const DenseHashMap<...>>)`（`cpp/Analysis/src/AstUtils.cpp:70`）。
///
/// 降 safe 说明：集合/映射形参原为借用裸指针（调用点写的就是
/// `&mut x as *mut _`），收窄为引用后由借用检查器承担全部前提；
/// `expr` 的 `&mut` 已保证节点存活且独占，解引用只剩 `ast_expr_visit`
/// 一处窄 `unsafe` 块（遍历器只读 AST、写集合，无并存别名）。
pub fn find_unique_types(
  unique_types: &mut DenseHashSet<TypeId>,
  expr: &mut AstExpr,
  ast_types: &DenseHashMap<*const AstExpr, TypeId>,
) {
  let mut finder = AstExprTableFinder::new(unique_types, ast_types);
  // Safety: expr 由 &mut 借用保证为存活且独占的 AST 节点，遍历期间 finder 只读
  // 节点字段、只写自己的集合，单线程无并存可变借用。
  unsafe {
    ast_expr_visit(expr, &mut finder);
  }
}

/// 对应 C++ 迭代器版 `findUniqueTypes(uniqueTypes, begin, end, astTypes)`
/// （`cpp/Analysis/src/AstUtils.cpp:77`）。降 safe：集合/映射形参收引用；
/// `iter` 产出的每个 `*mut AstExpr` 是 parser arena 存活节点（地址不移动），
/// 判型读取 `base` 首字段（repr(C) 同址）收进循环体内窄 `unsafe` 块。
pub fn find_unique_types_iter<I>(
  unique_types: &mut DenseHashSet<TypeId>,
  iter: I,
  ast_types: &DenseHashMap<*const AstExpr, TypeId>,
) where
  I: IntoIterator<Item = *mut AstExpr>,
{
  for expr in iter {
    // Safety: expr 由调用点（args/varargs 数组元素）保证为 arena 存活非空对齐
    // 节点，此处仅读 base.class_index 判型，不解引用整棵子树。
    if unsafe { ast_node_is::<AstExprTable>(&(*expr).base) } {
      // Safety: 守卫已判定该 arena 存活节点为 AstExprTable；bump arena 单线程遍历
      // 此刻无其它借用，`&mut *expr` 重建独占可变引用（repr(C) 基址同址）。
      let expr = unsafe { &mut *expr };
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
