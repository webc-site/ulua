use ulua_ast::records::ast_stat_block::AstStatBlock;

use crate::records::cfg_builder::CfgBuilder;

impl CfgBuilder {
  /// # Safety
  /// 调用方须保证 `statement` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub unsafe fn lower_ast_stat_block(&mut self, statement: *mut AstStatBlock) {
    unsafe {
      let body = (*statement).body;
      for &stat in body.as_slice() {
        self.lower_ast_stat(stat);
      }
    }
  }
}
