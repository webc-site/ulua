use crate::{
  records::{
    cst_expr_explicit_type_instantiation::CstExprExplicitTypeInstantiation, cst_node::CstNode,
    cst_type_instantiation::CstTypeInstantiation,
  },
  rtti::CstNodeClass,
};

impl CstExprExplicitTypeInstantiation {
  pub fn new(instantiation: CstTypeInstantiation) -> Self {
    Self {
      base: CstNode {
        class_index: <Self as CstNodeClass>::CLASS_INDEX,
      },
      instantiation,
    }
  }
}

pub fn cst_expr_explicit_type_instantiation_cst_expr_explicit_type_instantiation(
  instantiation: CstTypeInstantiation,
) -> CstExprExplicitTypeInstantiation {
  CstExprExplicitTypeInstantiation::new(instantiation)
}
