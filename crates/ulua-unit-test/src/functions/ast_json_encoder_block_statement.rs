use ulua_ast::records::{ast_stat::AstStat, ast_stat_block::AstStatBlock};
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe fn block_statement(root: *mut AstStatBlock, index: usize) -> *mut AstStat {
  unsafe { *(*root).body.data.add(index) }
}
