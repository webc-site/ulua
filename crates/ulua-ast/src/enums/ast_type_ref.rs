use crate::{
  records::{
    ast_node::AstNode, ast_type::AstType, ast_type_error::AstTypeError,
    ast_type_function::AstTypeFunction, ast_type_group::AstTypeGroup,
    ast_type_intersection::AstTypeIntersection, ast_type_optional::AstTypeOptional,
    ast_type_reference::AstTypeReference, ast_type_singleton_bool::AstTypeSingletonBool,
    ast_type_singleton_string::AstTypeSingletonString, ast_type_table::AstTypeTable,
    ast_type_typeof::AstTypeTypeof, ast_type_union::AstTypeUnion, location::Location,
  },
  rtti::{AstNodeClass, AstNodeView, ast_node_as_unchecked},
};

/// 类型注解节点的只读引用判别枚举。
///
/// 封装基于 RTTI `class_index` 的下转逻辑，使消费方可以通过安全的
/// `match` 模式匹配具体类型注解，无需在每个分支手写 `unsafe { ast_node_as_unchecked }`。
#[derive(Debug, Clone, Copy)]
pub enum AstTypeRef<'a> {
  Reference(&'a AstTypeReference),
  Table(&'a AstTypeTable),
  Function(&'a AstTypeFunction),
  Typeof(&'a AstTypeTypeof),
  Union(&'a AstTypeUnion),
  Intersection(&'a AstTypeIntersection),
  Group(&'a AstTypeGroup),
  SingletonBool(&'a AstTypeSingletonBool),
  SingletonString(&'a AstTypeSingletonString),
  Optional(&'a AstTypeOptional),
  Error(&'a AstTypeError),
}

impl<'a> AstTypeRef<'a> {
  /// 尝试从基类类型注解引用构建具体类型注解枚举。
  /// 若 `class_index` 不属于已知类型注解节点类型，返回 `None`。
  #[inline]
  pub fn try_from_type(ty: &'a AstType) -> Option<Self> {
    match ty.base.class_index {
      AstTypeReference::CLASS_INDEX => Some(Self::Reference(unsafe { ast_node_as_unchecked(ty) })),
      AstTypeTable::CLASS_INDEX => Some(Self::Table(unsafe { ast_node_as_unchecked(ty) })),
      AstTypeFunction::CLASS_INDEX => Some(Self::Function(unsafe { ast_node_as_unchecked(ty) })),
      AstTypeTypeof::CLASS_INDEX => Some(Self::Typeof(unsafe { ast_node_as_unchecked(ty) })),
      AstTypeUnion::CLASS_INDEX => Some(Self::Union(unsafe { ast_node_as_unchecked(ty) })),
      AstTypeIntersection::CLASS_INDEX => {
        Some(Self::Intersection(unsafe { ast_node_as_unchecked(ty) }))
      }
      AstTypeGroup::CLASS_INDEX => Some(Self::Group(unsafe { ast_node_as_unchecked(ty) })),
      AstTypeSingletonBool::CLASS_INDEX => {
        Some(Self::SingletonBool(unsafe { ast_node_as_unchecked(ty) }))
      }
      AstTypeSingletonString::CLASS_INDEX => {
        Some(Self::SingletonString(unsafe { ast_node_as_unchecked(ty) }))
      }
      AstTypeOptional::CLASS_INDEX => Some(Self::Optional(unsafe { ast_node_as_unchecked(ty) })),
      AstTypeError::CLASS_INDEX => Some(Self::Error(unsafe { ast_node_as_unchecked(ty) })),
      _ => None,
    }
  }

  /// 从基类类型注解引用构建具体类型注解枚举。
  ///
  /// # Panics
  /// 若 `ty.base.class_index` 不属于已知类型注解节点类型，触发 panic。
  #[inline]
  pub fn from_type(ty: &'a AstType) -> Self {
    Self::try_from_type(ty).expect("AstType class_index 必须为合法的类型注解节点类型")
  }

  /// 获取该类型注解节点的基类 `AstNode` 引用。
  #[inline]
  pub fn as_ast_node(&self) -> &'a AstNode {
    match *self {
      Self::Reference(n) => n.as_ast_node(),
      Self::Table(n) => n.as_ast_node(),
      Self::Function(n) => n.as_ast_node(),
      Self::Typeof(n) => n.as_ast_node(),
      Self::Union(n) => n.as_ast_node(),
      Self::Intersection(n) => n.as_ast_node(),
      Self::Group(n) => n.as_ast_node(),
      Self::SingletonBool(n) => n.as_ast_node(),
      Self::SingletonString(n) => n.as_ast_node(),
      Self::Optional(n) => n.as_ast_node(),
      Self::Error(n) => n.as_ast_node(),
    }
  }

  /// 获取该类型注解节点的源码位置。
  #[inline]
  pub fn location(&self) -> Location {
    self.as_ast_node().location
  }
}

impl<'a> From<&'a AstType> for AstTypeRef<'a> {
  #[inline]
  fn from(ty: &'a AstType) -> Self {
    Self::from_type(ty)
  }
}

impl AstNodeView for AstTypeRef<'_> {
  #[inline]
  fn as_ast_node(&self) -> &AstNode {
    self.as_ast_node()
  }
}
