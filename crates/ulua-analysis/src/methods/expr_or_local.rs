//! `expr_or_local` 方法汇总：原先按 cpp 符号逐方法拆分的同前缀小文件合并至此，行为逐字保留。

use core::ptr::null_mut;

use ulua_ast::{
  functions::get_identifier::get_identifier,
  records::{ast_expr::AstExpr, ast_local::AstLocal, ast_name::AstName, location::Location},
};

use crate::records::{
  arena_handle::{alias_opt, alias_ref},
  expr_or_local::ExprOrLocal,
};

impl ExprOrLocal {
  pub fn get_expr(&self) -> *mut AstExpr {
    self.expr
  }
}

impl ExprOrLocal {
  #[inline]
  pub fn get_local(&self) -> *mut AstLocal {
    self.local
  }
}

impl ExprOrLocal {
  pub fn get_location(&self) -> Option<Location> {
    let expr = self.get_expr();
    if !expr.is_null() {
      return Some(alias_ref(expr).base.location);
    }

    let local = self.get_local();
    if !local.is_null() {
      return Some(alias_ref(local).location);
    }

    None
  }
}

impl ExprOrLocal {
  pub fn get_name(&self) -> Option<AstName> {
    let expr = self.get_expr();
    if !expr.is_null() {
      let name = get_identifier(alias_opt(expr));
      if !name.is_null() {
        return Some(name);
      }
    } else {
      let local = self.get_local();
      if !local.is_null() {
        return Some(alias_ref(local).name);
      }
    }
    None
  }
}

impl ExprOrLocal {
  pub fn set_expr(&mut self, new_expr: *mut AstExpr) {
    // 既有约定（review.md §2）：写一臂即清另一臂为 null，是联合槽互斥不变量的显式转手动作
    // （空态唯一定义在 records/expr_or_local.rs::default），字段类型本批次不改。
    self.expr = new_expr;
    self.local = null_mut();
  }
}

impl ExprOrLocal {
  pub fn set_local(&mut self, new_local: *mut AstLocal) {
    // 既有约定（review.md §2）：同 `set_expr` 的互斥转手，清空即另一臂归 null。
    self.local = new_local;
    self.expr = null_mut();
  }
}
