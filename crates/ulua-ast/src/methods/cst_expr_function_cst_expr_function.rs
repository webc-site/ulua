use crate::{
  records::{
    ast_array::AstArray, cst_expr_function::CstExprFunction, cst_node::CstNode, position::Position,
  },
  rtti::CstNodeClass,
};

impl CstExprFunction {
  pub fn new() -> Self {
    Self {
      base: CstNode {
        class_index: <Self as CstNodeClass>::CLASS_INDEX,
      },
      function_keyword_position: Position::missing(),
      open_generics_position: Position::missing(),
      generics_comma_positions: AstArray::default(),
      close_generics_position: Position::missing(),
      args_annotation_colon_positions: AstArray::default(),
      args_comma_positions: AstArray::default(),
      vararg_annotation_colon_position: Position::missing(),
      return_specifier_position: Position::missing(),
    }
  }
}

impl Default for CstExprFunction {
  fn default() -> Self {
    Self::new()
  }
}

pub fn cst_expr_function_cst_expr_function() -> CstExprFunction {
  CstExprFunction::new()
}
