use ulua_ast::records::{ast_stat::AstStat, ast_stat_block::AstStatBlock};

/// 取 `AstStatBlock` 体中第 `index` 条语句的裸指针（cpp `root->body[i]`）。
///
/// `root` 以 `&AstStatBlock` 接收：调用点通过 `PtrRef::as_ref_opt()` 安全物化，
/// 本函数只做索引与 `as_ptr`，无需 `unsafe`。
pub fn block_statement(root: &AstStatBlock, index: usize) -> *mut AstStat {
  root.body[index].as_ptr()
}
