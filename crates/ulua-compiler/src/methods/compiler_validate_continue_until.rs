use core::ptr::null_mut;

use ulua_ast::{
  records::{
    ast_expr::AstExpr, ast_stat::AstStat, ast_stat_block::AstStatBlock,
    ast_stat_local::AstStatLocal, ast_stat_local_function::AstStatLocalFunction,
  },
  rtti,
  visit::ast_expr_visit,
};
use ulua_common::records::dense_hash_set::DenseHashSet;

use crate::records::{
  compile_error::CompileError, compiler::Compiler, undefined_local_visitor::UndefinedLocalVisitor,
};

impl Compiler {
  /// # Safety
  /// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
  pub unsafe fn validate_continue_until(
    &mut self,
    cont: *mut AstStat,
    condition: *mut AstExpr,
    body: *mut AstStatBlock,
    start: usize,
  ) {
    let mut visitor = UndefinedLocalVisitor {
      self_: self as *mut Compiler,
      undef: null_mut(),
      locals: DenseHashSet::new(null_mut()),
    };

    unsafe {
      let body_ref = &*body;
      for i in start..body_ref.body.size {
        let stat = *body_ref.body.data.add(i);
        let local_stat = rtti::ast_node_as::<AstStatLocal>(stat as *mut _);
        if !local_stat.is_null() {
          for &var in (*local_stat).vars.iter() {
            visitor.locals.insert(var);
          }
        } else {
          let func_stat = rtti::ast_node_as::<AstStatLocalFunction>(stat as *mut _);
          if !func_stat.is_null() {
            visitor.locals.insert((*func_stat).name);
          }
        }
      }

      ast_expr_visit(condition, &mut visitor);

      if !visitor.undef.is_null() {
        CompileError::raise(
          &(*condition).base.location,
          format_args!(
            "Local {} used in the repeat..until condition is undefined because continue statement on line {} jumps over it",
            (*visitor.undef).name,
            (*cont).base.location.begin.line + 1
          ),
        );
      }
    }
  }
}
