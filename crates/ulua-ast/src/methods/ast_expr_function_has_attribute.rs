use crate::{
  functions::find_attribute_in_array::find_attribute_in_array,
  records::{ast_attr::AstAttrType, ast_expr_function::AstExprFunction},
};

impl AstExprFunction {
  pub fn has_attribute(&self, attribute_type: AstAttrType) -> bool {
    find_attribute_in_array(self.attributes.iter(), attribute_type).is_some()
  }
}
