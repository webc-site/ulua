use core::ffi::CStr;

use ulua_ast::{
  records::{
    ast_node::AstNode, ast_stat_block::AstStatBlock, ast_stat_function::AstStatFunction,
    ast_stat_local_function::AstStatLocalFunction,
  },
  rtti::ast_node_as,
};

use crate::records::lint_duplicate_function::LintDuplicateFunction;
impl LintDuplicateFunction {
  /// # Safety
  /// 调用方须保证 `block` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn visit_ast_stat_block(&mut self, block: *mut AstStatBlock) -> bool {
    self.defns.clear();

    let block_ref = unsafe { &*block };
    let body = block_ref.body;

    for &stat in body.as_slice() {
      let node = stat as *mut AstNode;

      unsafe {
        let func = ast_node_as::<AstStatFunction>(node);
        if !func.is_null() {
          self.track_function(
            (*(*func).name).base.location,
            &self.build_name((*func).name),
          );
          continue;
        }

        let local_func = ast_node_as::<AstStatLocalFunction>(node);
        if !local_func.is_null() {
          let name = (*(*local_func).name).name;
          if !name.value.is_null() {
            let name = CStr::from_ptr(name.value).to_string_lossy();
            self.track_function((*(*local_func).name).location, &name);
          }
        }
      }
    }

    true
  }
}
