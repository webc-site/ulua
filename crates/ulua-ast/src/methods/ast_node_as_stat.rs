use core::ptr::{NonNull, from_ref};

use crate::{
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
}
