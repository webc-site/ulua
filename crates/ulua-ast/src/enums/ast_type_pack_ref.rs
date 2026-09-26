use crate::{
  records::{
    ast_node::AstNode, ast_type_pack::AstTypePack, ast_type_pack_explicit::AstTypePackExplicit,
    ast_type_pack_generic::AstTypePackGeneric, ast_type_pack_variadic::AstTypePackVariadic,
    location::Location,
  },
  rtti::{AstNodeClass, AstNodeView, ast_node_as_unchecked},
};

/// 类型包节点的只读引用判别枚举。
///
/// 封装基于 RTTI `class_index` 的下转逻辑，使消费方可以通过安全的
/// `match` 模式匹配具体类型包，无需在每个分支手写 `unsafe { ast_node_as_unchecked }`。
#[derive(Debug, Clone, Copy)]
pub enum AstTypePackRef<'a> {
  Explicit(&'a AstTypePackExplicit),
  Variadic(&'a AstTypePackVariadic),
  Generic(&'a AstTypePackGeneric),
}

impl<'a> AstTypePackRef<'a> {
  /// 尝试从基类类型包引用构建具体类型包枚举。
  /// 若 `class_index` 不属于已知类型包节点类型，返回 `None`。
  #[inline]
  pub fn try_from_pack(pack: &'a AstTypePack) -> Option<Self> {
    match pack.base.class_index {
      AstTypePackExplicit::CLASS_INDEX => {
        Some(Self::Explicit(unsafe { ast_node_as_unchecked(pack) }))
      }
      AstTypePackVariadic::CLASS_INDEX => {
        Some(Self::Variadic(unsafe { ast_node_as_unchecked(pack) }))
      }
      AstTypePackGeneric::CLASS_INDEX => {
        Some(Self::Generic(unsafe { ast_node_as_unchecked(pack) }))
      }
      _ => None,
    }
  }

  /// 从基类类型包引用构建具体类型包枚举。
  ///
  /// # Panics
  /// 若 `pack.base.class_index` 不属于已知类型包节点类型，触发 panic。
  #[inline]
  pub fn from_pack(pack: &'a AstTypePack) -> Self {
    Self::try_from_pack(pack).expect("AstTypePack class_index 必须为合法的类型包节点类型")
  }

  /// 获取该类型包节点的基类 `AstNode` 引用。
  #[inline]
  pub fn as_ast_node(&self) -> &'a AstNode {
    match *self {
      Self::Explicit(n) => n.as_ast_node(),
      Self::Variadic(n) => n.as_ast_node(),
      Self::Generic(n) => n.as_ast_node(),
    }
  }

  /// 获取该类型包节点的源码位置。
  #[inline]
  pub fn location(&self) -> Location {
    self.as_ast_node().location
  }
}

impl<'a> From<&'a AstTypePack> for AstTypePackRef<'a> {
  #[inline]
  fn from(pack: &'a AstTypePack) -> Self {
    Self::from_pack(pack)
  }
}

impl AstNodeView for AstTypePackRef<'_> {
  #[inline]
  fn as_ast_node(&self) -> &AstNode {
    self.as_ast_node()
  }
}
