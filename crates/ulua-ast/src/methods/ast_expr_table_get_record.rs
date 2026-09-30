//! `AstExprTable::getRecord` (`Ast/src/Ast.cpp:396`).

use crate::{
  records::{
    ast_expr::AstExpr,
    ast_expr_constant_string::AstExprConstantString,
    ast_expr_table::{AstExprTable, ItemKind},
  },
  rtti::ast_node_try_as_ptr,
};

impl AstExprTable {
  /// 记录面：仅 [`Self::get_record_str`] 使用（cpp 的 `getRecord` 只有 string
  /// 键入口），不再对外暴露字节键变体。
  fn get_record_bytes(&self, key: &[u8]) -> Option<*mut AstExpr> {
    for item in self.items.iter() {
      if item.kind == ItemKind::Record
        && let Some(string_expr) = (unsafe {
          // Safety: item.key 是 parser 写入 items 数组（arena）的键表达式指针，null 或存活节点；ast_node_try_as_ptr 判空与 class index 命中后才给出 &T，命中即类型正确（repr(C) 基址重合）。
          ast_node_try_as_ptr::<AstExprConstantString>(item.key)
        })
        && string_expr.value.as_bytes() == key
      {
        return Some(item.value);
      }
    }
    None
  }

  #[inline]
  pub fn get_record_str(&self, key: &str) -> Option<*mut AstExpr> {
    self.get_record_bytes(key.as_bytes())
  }
}
