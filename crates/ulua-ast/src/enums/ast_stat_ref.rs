use crate::{
  records::{
    ast_node::AstNode,
    ast_stat::AstStat,
    ast_stat_assign::AstStatAssign,
    ast_stat_block::AstStatBlock,
    ast_stat_break::AstStatBreak,
    ast_stat_class::{AstStatClass, AstStatDeclareClass},
    ast_stat_compound_assign::AstStatCompoundAssign,
    ast_stat_continue::AstStatContinue,
    ast_stat_declare_extern_type::AstStatDeclareExternType,
    ast_stat_declare_function::AstStatDeclareFunction,
    ast_stat_declare_global::AstStatDeclareGlobal,
    ast_stat_error::AstStatError,
    ast_stat_expr::AstStatExpr,
    ast_stat_for::AstStatFor,
    ast_stat_for_in::AstStatForIn,
    ast_stat_function::AstStatFunction,
    ast_stat_if::AstStatIf,
    ast_stat_local::AstStatLocal,
    ast_stat_local_function::AstStatLocalFunction,
    ast_stat_repeat::AstStatRepeat,
    ast_stat_return::AstStatReturn,
    ast_stat_type_alias::AstStatTypeAlias,
    ast_stat_type_function::AstStatTypeFunction,
    ast_stat_while::AstStatWhile,
    location::Location,
  },
  rtti::{AstNodeClass, AstNodeView, ast_node_as_unchecked},
};

/// 语句节点的只读引用判别枚举。
///
/// 封装基于 RTTI `class_index` 的下转逻辑，使消费方可以通过安全的
/// `match` 模式匹配具体语句类型，无需在每个分支手写 `unsafe { ast_node_as_unchecked }`。
#[derive(Debug, Clone, Copy)]
pub enum AstStatRef<'a> {
  Block(&'a AstStatBlock),
  If(&'a AstStatIf),
  While(&'a AstStatWhile),
  Repeat(&'a AstStatRepeat),
  Break(&'a AstStatBreak),
  Continue(&'a AstStatContinue),
  Return(&'a AstStatReturn),
  Expr(&'a AstStatExpr),
  Local(&'a AstStatLocal),
  For(&'a AstStatFor),
  ForIn(&'a AstStatForIn),
  Assign(&'a AstStatAssign),
  CompoundAssign(&'a AstStatCompoundAssign),
  Function(&'a AstStatFunction),
  LocalFunction(&'a AstStatLocalFunction),
  TypeAlias(&'a AstStatTypeAlias),
  TypeFunction(&'a AstStatTypeFunction),
  DeclareFunction(&'a AstStatDeclareFunction),
  DeclareGlobal(&'a AstStatDeclareGlobal),
  DeclareExternType(&'a AstStatDeclareExternType),
  DeclareClass(&'a AstStatDeclareClass),
  Error(&'a AstStatError),
}

impl<'a> AstStatRef<'a> {
  /// 尝试从基类语句引用构建具体语句枚举。
  /// 若 `class_index` 不属于已知语句类型，返回 `None`。
  #[inline]
  pub fn try_from_stat(stat: &'a AstStat) -> Option<Self> {
    match stat.base.class_index {
      AstStatBlock::CLASS_INDEX => Some(Self::Block(unsafe { ast_node_as_unchecked(stat) })),
      AstStatIf::CLASS_INDEX => Some(Self::If(unsafe { ast_node_as_unchecked(stat) })),
      AstStatWhile::CLASS_INDEX => Some(Self::While(unsafe { ast_node_as_unchecked(stat) })),
      AstStatRepeat::CLASS_INDEX => Some(Self::Repeat(unsafe { ast_node_as_unchecked(stat) })),
      AstStatBreak::CLASS_INDEX => Some(Self::Break(unsafe { ast_node_as_unchecked(stat) })),
      AstStatContinue::CLASS_INDEX => Some(Self::Continue(unsafe { ast_node_as_unchecked(stat) })),
      AstStatReturn::CLASS_INDEX => Some(Self::Return(unsafe { ast_node_as_unchecked(stat) })),
      AstStatExpr::CLASS_INDEX => Some(Self::Expr(unsafe { ast_node_as_unchecked(stat) })),
      AstStatLocal::CLASS_INDEX => Some(Self::Local(unsafe { ast_node_as_unchecked(stat) })),
      AstStatFor::CLASS_INDEX => Some(Self::For(unsafe { ast_node_as_unchecked(stat) })),
      AstStatForIn::CLASS_INDEX => Some(Self::ForIn(unsafe { ast_node_as_unchecked(stat) })),
      AstStatAssign::CLASS_INDEX => Some(Self::Assign(unsafe { ast_node_as_unchecked(stat) })),
      AstStatCompoundAssign::CLASS_INDEX => {
        Some(Self::CompoundAssign(unsafe { ast_node_as_unchecked(stat) }))
      }
      AstStatFunction::CLASS_INDEX => Some(Self::Function(unsafe { ast_node_as_unchecked(stat) })),
      AstStatLocalFunction::CLASS_INDEX => {
        Some(Self::LocalFunction(unsafe { ast_node_as_unchecked(stat) }))
      }
      AstStatTypeAlias::CLASS_INDEX => {
        Some(Self::TypeAlias(unsafe { ast_node_as_unchecked(stat) }))
      }
      AstStatTypeFunction::CLASS_INDEX => {
        Some(Self::TypeFunction(unsafe { ast_node_as_unchecked(stat) }))
      }
      AstStatDeclareFunction::CLASS_INDEX => Some(Self::DeclareFunction(unsafe {
        ast_node_as_unchecked(stat)
      })),
      AstStatDeclareGlobal::CLASS_INDEX => {
        Some(Self::DeclareGlobal(unsafe { ast_node_as_unchecked(stat) }))
      }
      AstStatDeclareExternType::CLASS_INDEX => Some(Self::DeclareExternType(unsafe {
        ast_node_as_unchecked(stat)
      })),
      AstStatClass::CLASS_INDEX => Some(Self::DeclareClass(unsafe { ast_node_as_unchecked(stat) })),
      AstStatError::CLASS_INDEX => Some(Self::Error(unsafe { ast_node_as_unchecked(stat) })),
      _ => None,
    }
  }

  /// 从基类语句引用构建具体语句枚举。
  ///
  /// # Panics
  /// 若 `stat.base.class_index` 不属于已知语句类型，触发 panic。
  #[inline]
  pub fn from_stat(stat: &'a AstStat) -> Self {
    Self::try_from_stat(stat).expect("AstStat class_index 必须为合法的语句节点类型")
  }

  /// 获取该语句节点的基类 `AstNode` 引用。
  #[inline]
  pub fn as_ast_node(&self) -> &'a AstNode {
    match *self {
      Self::Block(n) => n.as_ast_node(),
      Self::If(n) => n.as_ast_node(),
      Self::While(n) => n.as_ast_node(),
      Self::Repeat(n) => n.as_ast_node(),
      Self::Break(n) => n.as_ast_node(),
      Self::Continue(n) => n.as_ast_node(),
      Self::Return(n) => n.as_ast_node(),
      Self::Expr(n) => n.as_ast_node(),
      Self::Local(n) => n.as_ast_node(),
      Self::For(n) => n.as_ast_node(),
      Self::ForIn(n) => n.as_ast_node(),
      Self::Assign(n) => n.as_ast_node(),
      Self::CompoundAssign(n) => n.as_ast_node(),
      Self::Function(n) => n.as_ast_node(),
      Self::LocalFunction(n) => n.as_ast_node(),
      Self::TypeAlias(n) => n.as_ast_node(),
      Self::TypeFunction(n) => n.as_ast_node(),
      Self::DeclareFunction(n) => n.as_ast_node(),
      Self::DeclareGlobal(n) => n.as_ast_node(),
      Self::DeclareExternType(n) => n.as_ast_node(),
      Self::DeclareClass(n) => n.as_ast_node(),
      Self::Error(n) => n.as_ast_node(),
    }
  }

  /// 获取该语句节点的源码位置。
  #[inline]
  pub fn location(&self) -> Location {
    self.as_ast_node().location
  }
}

impl<'a> From<&'a AstStat> for AstStatRef<'a> {
  #[inline]
  fn from(stat: &'a AstStat) -> Self {
    Self::from_stat(stat)
  }
}

impl AstNodeView for AstStatRef<'_> {
  #[inline]
  fn as_ast_node(&self) -> &AstNode {
    self.as_ast_node()
  }
}
