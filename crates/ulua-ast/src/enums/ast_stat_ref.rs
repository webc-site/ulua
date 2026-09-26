use crate::{
  records::{
    ast_stat::AstStat, ast_stat_assign::AstStatAssign, ast_stat_block::AstStatBlock,
    ast_stat_break::AstStatBreak, ast_stat_class::AstStatClass,
    ast_stat_compound_assign::AstStatCompoundAssign, ast_stat_continue::AstStatContinue,
    ast_stat_declare_extern_type::AstStatDeclareExternType,
    ast_stat_declare_function::AstStatDeclareFunction,
    ast_stat_declare_global::AstStatDeclareGlobal, ast_stat_error::AstStatError,
    ast_stat_expr::AstStatExpr, ast_stat_for::AstStatFor, ast_stat_for_in::AstStatForIn,
    ast_stat_function::AstStatFunction, ast_stat_if::AstStatIf, ast_stat_local::AstStatLocal,
    ast_stat_local_function::AstStatLocalFunction, ast_stat_repeat::AstStatRepeat,
    ast_stat_return::AstStatReturn, ast_stat_type_alias::AstStatTypeAlias,
    ast_stat_type_function::AstStatTypeFunction, ast_stat_while::AstStatWhile,
  },
  rtti::define_ast_ref_enum,
};

define_ast_ref_enum! {
  /// 语句节点的只读引用判别枚举。
  ///
  /// 封装基于 RTTI `class_index` 的下转逻辑，使消费方可以通过安全的
  /// `match` 模式匹配具体语句类型，无需在每个分支手写 `unsafe { ast_node_as_unchecked }`。
  AstStatRef<'a> : AstStat ;
  try try_from_stat , from from_stat , expect "AstStat class_index 必须为合法的语句节点类型" ;
  variants {
    Block(AstStatBlock),
    If(AstStatIf),
    While(AstStatWhile),
    Repeat(AstStatRepeat),
    Break(AstStatBreak),
    Continue(AstStatContinue),
    Return(AstStatReturn),
    Expr(AstStatExpr),
    Local(AstStatLocal),
    For(AstStatFor),
    ForIn(AstStatForIn),
    Assign(AstStatAssign),
    CompoundAssign(AstStatCompoundAssign),
    Function(AstStatFunction),
    LocalFunction(AstStatLocalFunction),
    TypeAlias(AstStatTypeAlias),
    TypeFunction(AstStatTypeFunction),
    DeclareFunction(AstStatDeclareFunction),
    DeclareGlobal(AstStatDeclareGlobal),
    DeclareExternType(AstStatDeclareExternType),
    DeclareClass(AstStatClass),
    Error(AstStatError),
  }
}
