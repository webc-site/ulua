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
    ast_expr_varargs::AstExprVarargs,
  },
  rtti::define_ast_ref_enum,
};

define_ast_ref_enum! {
  /// 表达式节点的只读引用判别枚举。
  ///
  /// 封装基于 RTTI `class_index` 的下转逻辑，使消费方可以通过安全的
  /// `match` 模式匹配具体表达式类型，无需在每个分支手写 `unsafe { ast_node_as_unchecked }`。
  AstExprRef<'a> : AstExpr ;
  try try_from_expr , from from_expr , expect "AstExpr class_index 必须为合法的表达式节点类型" ;
  variants {
    Binary(AstExprBinary),
    Call(AstExprCall),
    ConstantBool(AstExprConstantBool),
    ConstantInteger(AstExprConstantInteger),
    ConstantNil(AstExprConstantNil),
    ConstantNumber(AstExprConstantNumber),
    ConstantString(AstExprConstantString),
    Error(AstExprError),
    Function(AstExprFunction),
    Global(AstExprGlobal),
    Group(AstExprGroup),
    IfElse(AstExprIfElse),
    IndexExpr(AstExprIndexExpr),
    IndexName(AstExprIndexName),
    Instantiate(AstExprInstantiate),
    InterpString(AstExprInterpString),
    Local(AstExprLocal),
    Table(AstExprTable),
    TypeAssertion(AstExprTypeAssertion),
    Unary(AstExprUnary),
    Varargs(AstExprVarargs),
  }
}
