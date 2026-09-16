use crate::{
  functions::find_attribute_in_array::find_attribute_in_array,
  records::{
    ast_attr::{AstAttr, AstAttrType},
    ast_expr_function::AstExprFunction,
  },
};

impl AstExprFunction {
  pub fn get_attribute(&self, attribute_type: AstAttrType) -> *mut AstAttr {
    find_attribute_in_array(self.attributes, attribute_type)
  }
}
