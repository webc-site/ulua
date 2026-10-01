use crate::{
  functions::{find_attribute_in_array::find_attribute_in_array, optional_node::opt_ptr},
  records::{
    ast_attr::{AstAttr, AstAttrType},
    ast_expr_function::AstExprFunction,
  },
};

impl AstExprFunction {
  pub fn get_attribute(&self, attribute_type: AstAttrType) -> *mut AstAttr {
    opt_ptr(find_attribute_in_array(
      self.attributes.iter(),
      attribute_type,
    ))
  }
}
