use crate::{
  functions::find_attribute_in_array::find_attribute_in_array,
  records::{ast_attr::AstAttrType::Native, ast_expr_function::AstExprFunction},
};

impl AstExprFunction {
  pub fn has_native_attribute(&self) -> bool {
    find_attribute_in_array(self.attributes.iter(), Native).is_some()
  }
}
