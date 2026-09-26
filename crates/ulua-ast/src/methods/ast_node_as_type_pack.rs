use core::ptr::{NonNull, from_mut, from_ref};

use crate::{
  enums::ast_type_pack_ref::AstTypePackRef,
  records::{ast_node::AstNode, ast_type_pack::AstTypePack},
  rtti::is_type_pack_class,
};

impl AstNode {
  /// cpp `AstNode::asTypePack()`：命中 `AstTypePack` 家族则给出类型包视图，否则 `None`。
  /// 形态取舍与 [`Self::as_expr`] 一致。
  #[inline]
  pub fn as_type_pack(&mut self) -> Option<NonNull<AstTypePack>> {
    if is_type_pack_class(self.class_index) {
      // Safety: class_index 命中 + #[repr(C)] 单继承 ⇒ 偏移 0 即存活 AstTypePack；非空。
      Some(unsafe { NonNull::new_unchecked(from_mut(self).cast::<AstTypePack>()) })
    } else {
      None
    }
  }

  /// cpp `const AstNode::asTypePack() const`：只读判别下转，安全形态
  /// `Option<&AstTypePack>`（论证同 [`Self::as_expr_const`]）。
  #[inline]
  pub fn as_type_pack_const(&self) -> Option<&AstTypePack> {
    if is_type_pack_class(self.class_index) {
      // Safety: 同 as_expr_const——判型命中 + repr(C) 基址重合 + 借用存活。
      Some(unsafe {
        NonNull::new_unchecked(from_ref(self).cast::<AstTypePack>().cast_mut()).as_ref()
      })
    } else {
      None
    }
  }

  /// 尝试将通用 AST 节点下转为具体类型包引用枚举。若节点不属于 `AstTypePack` 家族，返回 `None`。
  #[inline]
  pub fn try_as_pack_ref(&self) -> Option<AstTypePackRef<'_>> {
    self
      .as_type_pack_const()
      .and_then(AstTypePack::try_as_pack_ref)
  }

  /// 将通用 AST 节点下转为具体类型包引用枚举。
  ///
  /// # Panics
  /// 若节点不属于 `AstTypePack` 家族，触发 panic。
  #[inline]
  pub fn as_pack_ref(&self) -> AstTypePackRef<'_> {
    self
      .try_as_pack_ref()
      .expect("AstNode class_index 必须为合法的类型包节点类型")
  }
}
