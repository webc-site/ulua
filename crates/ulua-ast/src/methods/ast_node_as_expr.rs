use core::ptr::{NonNull, from_mut, from_ref};

use crate::{
  enums::ast_expr_ref::AstExprRef,
  records::{ast_expr::AstExpr, ast_node::AstNode},
  rtti::is_expr_class,
};

impl AstNode {
  /// cpp `AstNode::asExpr()`：命中 `AstExpr` 家族则给出表达式视图，否则 `None`
  /// （cpp 的 null 折叠进 Option）。判别表见 [`is_expr_class`]。
  ///
  /// 返回 `Option<NonNull<AstExpr>>` 而非 `Option<&mut AstExpr>`：arena 内节点随后
  /// 仍会经裸指针句柄被写穿（parser），长命 `&mut` 与既有形态互斥（同
  /// `optional_node` 对 `NonNull` 的选择）。解引用在消费点以 `as_ref()`/`as_mut()`
  /// 显式兑现。
  #[inline]
  pub fn as_expr(&mut self) -> Option<NonNull<AstExpr>> {
    if is_expr_class(self.class_index) {
      // Safety: class_index 命中 + #[repr(C)] 单继承 ⇒ self 所在 place 的偏移 0
      // 即初始化且良好对齐的 AstExpr；指针非空（来自 &mut）。
      Some(unsafe { NonNull::new_unchecked(from_mut(self).cast::<AstExpr>()) })
    } else {
      None
    }
  }

  /// cpp `const AstNode::asExpr() const`：只读判别下转，共享借用直接给出
  /// `Option<&AstExpr>`（`#[repr(C)]` 基址重合 + 借用存活 ⇒ 同一 place 前缀读
  /// 合法；arena 写穿场景请用 [`Self::as_expr`] 的 `NonNull` 形态）。
  #[inline]
  pub fn as_expr_const(&self) -> Option<&AstExpr> {
    if is_expr_class(self.class_index) {
      // Safety: class_index 命中 + repr(C) 单继承 ⇒ 同一 place 的偏移 0 即存活
      // AstExpr；`&self` 借用期内有效，共享读不造写权限。
      Some(unsafe { NonNull::new_unchecked(from_ref(self).cast::<AstExpr>().cast_mut()).as_ref() })
    } else {
      None
    }
  }

  /// 将通用 AST 节点下转为具体表达式引用枚举。若节点不属于 `AstExpr` 家族，返回 `None`。
  #[inline]
  pub fn try_as_expr_ref(&self) -> Option<AstExprRef<'_>> {
    self.as_expr_const().and_then(AstExpr::try_as_expr_ref)
  }

  /// 将通用 AST 节点下转为具体表达式引用枚举。
  ///
  /// # Panics
  /// 若节点不属于 `AstExpr` 家族，触发 panic。
  #[inline]
  pub fn as_expr_ref(&self) -> AstExprRef<'_> {
    self
      .try_as_expr_ref()
      .expect("AstNode class_index 必须为合法的表达式节点类型")
  }
}
