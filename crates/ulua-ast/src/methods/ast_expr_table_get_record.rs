//! `AstExprTable::getRecord` (`Ast/src/Ast.cpp:396`).

use crate::records::{
  ast_expr::AstExpr,
  ast_expr_constant_string::AstExprConstantString,
  ast_expr_table::{AstExprTable, ItemKind},
  node_handle::OptNode,
};

impl AstExprTable {
  /// 记录面：仅 [`Self::get_record_str`] 使用（cpp 的 `getRecord` 只有 string
  /// 键入口），不再对外暴露字节键变体。
  fn get_record_bytes(&self, key: &[u8]) -> Option<*mut AstExpr> {
    for item in self.items.iter() {
      if item.kind == ItemKind::Record
        // `item.key` 是 parser 写入 items 数组（arena）的键表达式槽位（null 或
        // 存活节点）：经句柄门面 `OptNode::from_ptr` 折叠可空性，判型下转走
        // 生命周期正确的句柄 `try_as`（class index 命中才给出 &T，repr(C) 基址
        // 重合），借用半径由本迭代局部句柄供给，不再锻造假 'static。
        && let Some(string_expr) = OptNode::from_ptr(item.key).try_as::<AstExprConstantString>()
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
