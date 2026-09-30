use alloc::vec::Vec;

use ulua_ast::records::ast_expr::AstExpr;
use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::type_aliases::type_id::TypeId;

/// 生命周期化访问器：`ast_types`/`to_block` 由驱动方以独占借用注入
/// （原 `*mut` 字段的引用化，同一契约由借用类型承载）。
#[derive(Debug)]
pub struct BlockedTypeInLiteralVisitor<'a> {
  pub(crate) ast_types: &'a mut DenseHashMap<*const AstExpr, TypeId>,
  pub(crate) to_block: &'a mut Vec<TypeId>,
}
