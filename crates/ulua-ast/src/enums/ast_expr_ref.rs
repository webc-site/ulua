use crate::{
  records::{
    ast_expr::AstExpr, ast_expr_binary::AstExprBinary, ast_expr_call::AstExprCall,
    ast_expr_constant_bool::AstExprConstantBool, ast_expr_constant_integer::AstExprConstantInteger,
    ast_expr_constant_nil::AstExprConstantNil, ast_expr_constant_number::AstExprConstantNumber,
    ast_expr_constant_string::AstExprConstantString, ast_expr_error::AstExprError,
    ast_expr_function::AstExprFunction, ast_expr_global::AstExprGlobal,
    ast_expr_group::AstExprGroup, ast_expr_if_else::AstExprIfElse,
    ast_expr_index_expr::AstExprIndexExpr, ast_expr_index_name::AstExprIndexName,
    ast_expr_instantiate::AstExprInstantiate, ast_expr_interp_string::AstExprInterpString,
    ast_expr_local::AstExprLocal, ast_expr_table::AstExprTable,
    ast_expr_type_assertion::AstExprTypeAssertion, ast_expr_unary::AstExprUnary,
    ast_expr_varargs::AstExprVarargs, ast_node::AstNode, location::Location,
  },
  rtti::{AstNodeClass, AstNodeView, ast_node_as_unchecked},
};

/// 表达式节点的只读引用判别枚举。
///
/// 封装基于 RTTI `class_index` 的下转逻辑，使消费方可以通过安全的
/// `match` 模式匹配具体表达式类型，无需在每个分支手写 `unsafe { ast_node_as_unchecked }`。
#[derive(Debug, Clone, Copy)]
pub enum AstExprRef<'a> {
  Binary(&'a AstExprBinary),
  Call(&'a AstExprCall),
  ConstantBool(&'a AstExprConstantBool),
  ConstantInteger(&'a AstExprConstantInteger),
  ConstantNil(&'a AstExprConstantNil),
  ConstantNumber(&'a AstExprConstantNumber),
  ConstantString(&'a AstExprConstantString),
  Error(&'a AstExprError),
  Function(&'a AstExprFunction),
  Global(&'a AstExprGlobal),
  Group(&'a AstExprGroup),
  IfElse(&'a AstExprIfElse),
  IndexExpr(&'a AstExprIndexExpr),
  IndexName(&'a AstExprIndexName),
  Instantiate(&'a AstExprInstantiate),
  InterpString(&'a AstExprInterpString),
  Local(&'a AstExprLocal),
  Table(&'a AstExprTable),
  TypeAssertion(&'a AstExprTypeAssertion),
  Unary(&'a AstExprUnary),
  Varargs(&'a AstExprVarargs),
}

impl<'a> AstExprRef<'a> {
  /// 尝试从基类表达式引用构建具体表达式枚举。
  /// 若 `class_index` 不属于已知表达式类型，返回 `None`。
  #[inline]
  pub fn try_from_expr(expr: &'a AstExpr) -> Option<Self> {
    match expr.base.class_index {
      AstExprBinary::CLASS_INDEX => Some(Self::Binary(unsafe { ast_node_as_unchecked(expr) })),
      AstExprCall::CLASS_INDEX => Some(Self::Call(unsafe { ast_node_as_unchecked(expr) })),
      AstExprConstantBool::CLASS_INDEX => {
        Some(Self::ConstantBool(unsafe { ast_node_as_unchecked(expr) }))
      }
      AstExprConstantInteger::CLASS_INDEX => Some(Self::ConstantInteger(unsafe {
        ast_node_as_unchecked(expr)
      })),
      AstExprConstantNil::CLASS_INDEX => {
        Some(Self::ConstantNil(unsafe { ast_node_as_unchecked(expr) }))
      }
      AstExprConstantNumber::CLASS_INDEX => {
        Some(Self::ConstantNumber(unsafe { ast_node_as_unchecked(expr) }))
      }
      AstExprConstantString::CLASS_INDEX => {
        Some(Self::ConstantString(unsafe { ast_node_as_unchecked(expr) }))
      }
      AstExprError::CLASS_INDEX => Some(Self::Error(unsafe { ast_node_as_unchecked(expr) })),
      AstExprFunction::CLASS_INDEX => Some(Self::Function(unsafe { ast_node_as_unchecked(expr) })),
      AstExprGlobal::CLASS_INDEX => Some(Self::Global(unsafe { ast_node_as_unchecked(expr) })),
      AstExprGroup::CLASS_INDEX => Some(Self::Group(unsafe { ast_node_as_unchecked(expr) })),
      AstExprIfElse::CLASS_INDEX => Some(Self::IfElse(unsafe { ast_node_as_unchecked(expr) })),
      AstExprIndexExpr::CLASS_INDEX => {
        Some(Self::IndexExpr(unsafe { ast_node_as_unchecked(expr) }))
      }
      AstExprIndexName::CLASS_INDEX => {
        Some(Self::IndexName(unsafe { ast_node_as_unchecked(expr) }))
      }
      AstExprInstantiate::CLASS_INDEX => {
        Some(Self::Instantiate(unsafe { ast_node_as_unchecked(expr) }))
      }
      AstExprInterpString::CLASS_INDEX => {
        Some(Self::InterpString(unsafe { ast_node_as_unchecked(expr) }))
      }
      AstExprLocal::CLASS_INDEX => Some(Self::Local(unsafe { ast_node_as_unchecked(expr) })),
      AstExprTable::CLASS_INDEX => Some(Self::Table(unsafe { ast_node_as_unchecked(expr) })),
      AstExprTypeAssertion::CLASS_INDEX => {
        Some(Self::TypeAssertion(unsafe { ast_node_as_unchecked(expr) }))
      }
      AstExprUnary::CLASS_INDEX => Some(Self::Unary(unsafe { ast_node_as_unchecked(expr) })),
      AstExprVarargs::CLASS_INDEX => Some(Self::Varargs(unsafe { ast_node_as_unchecked(expr) })),
      _ => None,
    }
  }

  /// 从基类表达式引用构建具体表达式枚举。
  ///
  /// # Panics
  /// 若 `expr.base.class_index` 不属于已知表达式类型，触发 panic。
  #[inline]
  pub fn from_expr(expr: &'a AstExpr) -> Self {
    Self::try_from_expr(expr).expect("AstExpr class_index 必须为合法的表达式节点类型")
  }

  /// 获取该表达式节点的基类 `AstNode` 引用。
  #[inline]
  pub fn as_ast_node(&self) -> &'a AstNode {
    match *self {
      Self::Binary(n) => n.as_ast_node(),
      Self::Call(n) => n.as_ast_node(),
      Self::ConstantBool(n) => n.as_ast_node(),
      Self::ConstantInteger(n) => n.as_ast_node(),
      Self::ConstantNil(n) => n.as_ast_node(),
      Self::ConstantNumber(n) => n.as_ast_node(),
      Self::ConstantString(n) => n.as_ast_node(),
      Self::Error(n) => n.as_ast_node(),
      Self::Function(n) => n.as_ast_node(),
      Self::Global(n) => n.as_ast_node(),
      Self::Group(n) => n.as_ast_node(),
      Self::IfElse(n) => n.as_ast_node(),
      Self::IndexExpr(n) => n.as_ast_node(),
      Self::IndexName(n) => n.as_ast_node(),
      Self::Instantiate(n) => n.as_ast_node(),
      Self::InterpString(n) => n.as_ast_node(),
      Self::Local(n) => n.as_ast_node(),
      Self::Table(n) => n.as_ast_node(),
      Self::TypeAssertion(n) => n.as_ast_node(),
      Self::Unary(n) => n.as_ast_node(),
      Self::Varargs(n) => n.as_ast_node(),
    }
  }

  /// 获取该表达式节点的源码位置。
  #[inline]
  pub fn location(&self) -> Location {
    self.as_ast_node().location
  }
}

impl<'a> From<&'a AstExpr> for AstExprRef<'a> {
  #[inline]
  fn from(expr: &'a AstExpr) -> Self {
    Self::from_expr(expr)
  }
}

impl AstNodeView for AstExprRef<'_> {
  #[inline]
  fn as_ast_node(&self) -> &AstNode {
    self.as_ast_node()
  }
}
