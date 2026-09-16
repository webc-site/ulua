use ulua_ast::{
  records::{
    ast_expr::AstExpr,
    ast_expr_binary::{AstExprBinary, AstExprBinaryOp},
    ast_node::AstNode,
  },
  rtti::ast_node_as,
};

use crate::records::compiler::Compiler;

impl Compiler {
  pub fn unroll_concats(&mut self, args: &mut Vec<*mut AstExpr>) {
    loop {
      if args.is_empty() {
        break;
      }

      let back = *args.last().unwrap();
      if back.is_null() {
        break;
      }

      // C++ `args.back()->as<AstExprBinary>()` — a CHECKED RTTI downcast that
      // returns null when the node is not an AstExprBinary. The model used
      // `as_expr() as *mut AstExprBinary`, a blind reinterpret that treats any
      // node (e.g. the trailing string in `a..b..c`) as a Binary and derefs
      // its garbage `op`/`left`/`right` -> SIGSEGV.
      let be = unsafe { ast_node_as::<AstExprBinary>(back as *mut AstNode) };
      if be.is_null() {
        break;
      }

      let op = unsafe { (*be).op };
      if op != AstExprBinaryOp::Concat {
        break;
      }

      args.pop();
      args.push(unsafe { (*be).left });
      args.push(unsafe { (*be).right });
    }
  }
}
