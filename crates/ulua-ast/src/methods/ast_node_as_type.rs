use core::ptr::{NonNull, from_mut, from_ref};

use crate::{
  records::{ast_node::AstNode, ast_type::AstType},
  rtti::is_type_class,
};

impl AstNode {
  /// cpp `AstNode::asType()`：命中 `AstType` 家族则给出类型视图，否则 `None`。
  /// 形态取舍与 [`Self::as_expr`] 一致。
  #[inline]
  pub fn as_type(&mut self) -> Option<NonNull<AstType>> {
    if is_type_class(self.class_index) {
      // Safety: class_index 命中 + #[repr(C)] 单继承 ⇒ 偏移 0 即存活 AstType；非空。
      Some(unsafe { NonNull::new_unchecked(from_mut(self).cast::<AstType>()) })
    } else {
      None
    }
  }

  /// cpp `const AstNode::asType() const`：只读判别下转，安全形态
  /// `Option<&AstType>`（论证同 [`Self::as_expr_const`]）。
  #[inline]
  pub fn as_type_const(&self) -> Option<&AstType> {
    if is_type_class(self.class_index) {
      // Safety: 同 as_expr_const——判型命中 + repr(C) 基址重合 + 借用存活。
      Some(unsafe { NonNull::new_unchecked(from_ref(self).cast::<AstType>().cast_mut()).as_ref() })
    } else {
      None
    }
  }
}
