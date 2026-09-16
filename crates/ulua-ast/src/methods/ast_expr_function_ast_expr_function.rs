use crate::{
  records::{
    ast_array::AstArray, ast_attr::AstAttr, ast_expr::AstExpr, ast_expr_function::AstExprFunction,
    ast_generic_type::AstGenericType, ast_generic_type_pack::AstGenericTypePack,
    ast_local::AstLocal, ast_name::AstName, ast_node::AstNode, ast_stat_block::AstStatBlock,
    ast_type_pack::AstTypePack, location::Location,
  },
  rtti::AstNodeClass,
};

/// `AstExprFunction` 构造参数集合。
/// C++ 构造函数直译有 14 个参数，超出 Rust 惯用上限，打包成 struct 传递；
/// 字段与 C++ `AstExprFunction` 构造函数形参一一对应。
pub struct AstExprFunctionArgs {
  pub location: Location,
  pub attributes: AstArray<*mut AstAttr>,
  pub generics: AstArray<*mut AstGenericType>,
  pub generic_packs: AstArray<*mut AstGenericTypePack>,
  pub self_: *mut AstLocal,
  pub args: AstArray<*mut AstLocal>,
  pub vararg: bool,
  pub vararg_location: Location,
  pub body: *mut AstStatBlock,
  pub function_depth: usize,
  pub debugname: AstName,
  pub return_annotation: *mut AstTypePack,
  pub vararg_annotation: *mut AstTypePack,
  pub arg_location: Option<Location>,
}

impl AstExprFunction {
  pub fn new(a: AstExprFunctionArgs) -> Self {
    Self {
      base: AstExpr {
        base: AstNode {
          class_index: <Self as AstNodeClass>::CLASS_INDEX,
          location: a.location,
        },
      },
      attributes: a.attributes,
      generics: a.generics,
      generic_packs: a.generic_packs,
      self_: a.self_,
      args: a.args,
      return_annotation: a.return_annotation,
      vararg: a.vararg,
      vararg_location: a.vararg_location,
      vararg_annotation: a.vararg_annotation,
      body: a.body,
      function_depth: a.function_depth,
      debugname: a.debugname,
      arg_location: a.arg_location,
    }
  }
}

pub fn ast_expr_function_ast_expr_function(a: AstExprFunctionArgs) -> AstExprFunction {
  AstExprFunction::new(a)
}
