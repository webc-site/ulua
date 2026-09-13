use ulua_ast::{
  records::{ast_array::AstArray, ast_expr::AstExpr, ast_node::AstNode, ast_stat::AstStat},
  visit::ast_expr_visit,
};

use crate::{
  enums::kind::Kind,
  records::{assignment::Assignment, compiler::Compiler},
};

impl Compiler {
  /// # Safety
  /// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
  pub(crate) unsafe fn resolve_assign_conflicts(
    &mut self,
    stat: *mut AstStat,
    vars: &mut [Assignment],
    values: &AstArray<*mut AstExpr>,
  ) {
    unsafe {
      let mut visitor = self.visitor_visitor();

      for (i, var) in vars.iter().enumerate() {
        let li = &var.lvalue;
        if li.kind == Kind::Local {
          if i < values.size {
            ast_expr_visit(*values.data.add(i), &mut visitor);
          }
          let reg = li.reg as usize;
          visitor.assigned[reg / 64] |= 1 << (reg % 64);
        }
      }

      for (i, var) in vars.iter().enumerate() {
        let li = &var.lvalue;
        if li.kind != Kind::Local && i < values.size {
          ast_expr_visit(*values.data.add(i), &mut visitor);
        }
      }

      for i in vars.len()..values.size {
        ast_expr_visit(*values.data.add(i), &mut visitor);
      }

      for var in vars.iter() {
        let li = &var.lvalue;
        if matches!(
          li.kind,
          Kind::IndexName | Kind::IndexNumber | Kind::IndexExpr
        ) {
          let reg = li.reg as usize;
          if (visitor.assigned[reg / 64] & (1 << (reg % 64))) != 0 {
            visitor.conflict[reg / 64] |= 1 << (reg % 64);
          }
        }
        if li.kind == Kind::IndexExpr {
          let idx = li.index as usize;
          if (visitor.assigned[idx / 64] & (1 << (idx % 64))) != 0 {
            visitor.conflict[idx / 64] |= 1 << (idx % 64);
          }
        }
      }

      for var in vars.iter_mut() {
        let li = &var.lvalue;
        if li.kind == Kind::Local {
          let reg = li.reg as usize;
          if (visitor.conflict[reg / 64] & (1 << (reg % 64))) != 0 {
            var.conflict_reg = self.alloc_reg(stat as *mut AstNode, 1);
          }
        }
      }
    }
  }
}
