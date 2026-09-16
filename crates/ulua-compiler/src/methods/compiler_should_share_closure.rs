use core::{ptr::null_mut, slice::from_raw_parts};

use ulua_ast::{
  records::{ast_expr_function::AstExprFunction, ast_node::AstNode},
  rtti::ast_node_as,
};

use crate::records::compiler::Compiler;

impl Compiler {
  pub fn should_share_closure(&mut self, func: *mut AstExprFunction) -> bool {
    // 借用拆分：仅复制 upvals 的 (ptr, len) 供循环使用。循环体内对
    // functions/variables 只做 find 读查询、对 upvals Vec 无任何写入，
    // 数据在迭代期间保持稳定，免去原先为逃逸借用而做的整表 clone。
    let (upvals_ptr, upvals_len) = match self.functions.find(&func) {
      Some(f) => (f.upvals.as_ptr(), f.upvals.len()),
      None => return false,
    };

    for &uv in unsafe { from_raw_parts(upvals_ptr, upvals_len) } {
      let ul = match self.variables.find(&uv) {
        Some(ul) => *ul,
        None => return false,
      };

      if ul.written {
        return false;
      }

      unsafe {
        if (*uv).function_depth != 0 || (*uv).loop_depth != 0 {
          let uf = if !ul.init.is_null() {
            ast_node_as::<AstExprFunction>(ul.init as *mut AstNode)
          } else {
            null_mut()
          };

          if uf.is_null() {
            return false;
          }

          if uf != func && !self.should_share_closure(uf) {
            return false;
          }
        }
      }
    }

    true
  }
}
