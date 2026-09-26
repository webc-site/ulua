use core::ptr::{NonNull, from_ref};

use crate::{
  enums::ast_stat_ref::AstStatRef,
  records::{ast_node::AstNode, ast_stat::AstStat},
  rtti::is_stat_class,
};

impl AstNode {
  /// cpp `const AstNode::asStat() const`：只读判别下转，安全形态
  /// `Option<&AstStat>`（论证同 [`Self::as_expr_const`]）。
  #[inline]
  pub fn as_stat_const(&self) -> Option<&AstStat> {
    if is_stat_class(self.class_index) {
      // Safety: 同 as_expr_const——判型命中 + repr(C) 基址重合 + 借用存活。
      Some(unsafe { NonNull::new_unchecked(from_ref(self).cast::<AstStat>().cast_mut()).as_ref() })
    } else {
      None
    }
  }

  /// 尝试将通用 AST 节点下转为具体语句引用枚举。若节点不属于 `AstStat` 家族，返回 `None`。
  #[inline]
  pub fn try_as_stat_ref(&self) -> Option<AstStatRef<'_>> {
    self.as_stat_const().and_then(AstStat::try_as_stat_ref)
  }

  /// 将通用 AST 节点下转为具体语句引用枚举。
  ///
  /// # Panics
  /// 若节点不属于 `AstStat` 家族，触发 panic。
  #[inline]
  pub fn as_stat_ref(&self) -> AstStatRef<'_> {
    self
      .try_as_stat_ref()
      .expect("AstNode class_index 必须为合法的语句节点类型")
  }
}
