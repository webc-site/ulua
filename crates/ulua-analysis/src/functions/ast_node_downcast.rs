use ulua_ast::{
  records::ast_node::AstNode,
  rtti::{AstNodeClass, ast_node_as_unchecked},
};

/// RTTI `&AstNode` → `&T` 下转门面单点（原 9 处 visit/check 分派模块里的逐字
/// 私有副本收口）。调用方均处于 `match node.class_index { T::CLASS_INDEX => ... }`
/// 判定臂内，动态类型已静态保证为 T，直接 unchecked 下转（cpp `as<T>()` 命中后
/// 同样直接 `static_cast` 解引用；家族类索引互斥由 rtti 的 `rtti_indices_unique`
/// 测试保证）。
#[inline]
pub(crate) fn ast_node_downcast<T: AstNodeClass>(node: &AstNode) -> &T {
  // Safety: 调用方均处于类索引判定臂内，动态类型已静态保证为 T。
  unsafe { ast_node_as_unchecked::<T>(node) }
}
