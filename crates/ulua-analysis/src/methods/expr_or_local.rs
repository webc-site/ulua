//! `expr_or_local` 方法汇总：原先按 cpp 符号逐方法拆分的同前缀小文件合并至此，行为逐字保留。

use core::ptr::null_mut;

use ulua_ast::{
  functions::get_identifier::get_identifier,
  records::{ast_expr::AstExpr, ast_local::AstLocal, ast_name::AstName, location::Location},
};

use crate::records::expr_or_local::ExprOrLocal;

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
      // Safety: 先判空再解引用。`expr` 是 ExprOrLocal 记录的 AST 节点裸指针，只可能由
      // parser 产出的 `*mut AstExpr` 填入（`Default` 为 null，表示「本变体未使用」），
      // 节点位于 AST arena 的 bump 块内、地址不移动，且 `&self` 期间该 arena 存活。
      // 这里只一次性拷贝 `base.location`（`#[repr(C)]` 首字段，与派生体基址重合的
      // `AstNode::location`），不构造长期引用，故无别名与悬垂风险。
      return Some(unsafe { (*expr).base.location });
    }

    let local = self.get_local();
    if !local.is_null() {
      // Safety: 同上——`local` 由 parser 建立的 `AstLocal` 节点地址填入（未使用时为 null，
      // 已由判空排除），AST arena 保活且块地址不移动；仅拷贝 `location` 这个 `Copy` 字段。
      return Some(unsafe { (*local).location });
    }

    None
  }
}

impl ExprOrLocal {
  pub fn get_name(&self) -> Option<AstName> {
    let expr = self.get_expr();
    if !expr.is_null() {
      // SAFETY: expr 判空后指向 arena 存活节点；`as_ref` 只把该指针交给只读的 `get_identifier`
      let name = get_identifier(unsafe { expr.as_ref() });
      if !name.is_null() {
        return Some(name);
      }
    } else {
      let local = self.get_local();
      if !local.is_null() {
        return Some(unsafe { (*local).name });
      }
    }
    None
  }
}

impl ExprOrLocal {
  pub fn set_expr(&mut self, new_expr: *mut AstExpr) {
    self.expr = new_expr;
    self.local = null_mut();
  }
}

impl ExprOrLocal {
  pub fn set_local(&mut self, new_local: *mut AstLocal) {
    self.local = new_local;
    self.expr = null_mut();
  }
}
