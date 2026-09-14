use crate::{
  functions::has_attribute_in_array::has_attribute_in_array,
  records::{ast_attr::AstAttrType, ast_type_function::AstTypeFunction},
};

impl AstTypeFunction {
  pub fn has_attribute(&self, attribute_type: AstAttrType) -> bool {
    has_attribute_in_array(self.attributes, attribute_type)
  }
}
