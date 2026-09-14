use crate::{
  records::{
    ast_array::AstArray, ast_expr::AstExpr, ast_expr_call::AstExprCall, ast_node::AstNode,
    ast_type_or_pack::AstTypeOrPack, location::Location,
  },
  rtti::AstNodeClass,
};

impl AstExprCall {
  pub fn new(
    location: Location,
    func: *mut AstExpr,
    args: AstArray<*mut AstExpr>,
    self_: bool,
    explicit_types: AstArray<AstTypeOrPack>,
    arg_location: Location,
  ) -> Self {
    Self {
      base: AstExpr {
        base: AstNode {
          class_index: <Self as AstNodeClass>::CLASS_INDEX,
          location,
        },
      },
      func,
      type_arguments: explicit_types,
      args,
      self_,
      arg_location,
    }
  }
}

pub fn ast_expr_call_ast_expr_call(
  location: Location,
  func: *mut AstExpr,
  args: AstArray<*mut AstExpr>,
  self_: bool,
  explicit_types: AstArray<AstTypeOrPack>,
  arg_location: Location,
) -> AstExprCall {
  AstExprCall::new(location, func, args, self_, explicit_types, arg_location)
}
