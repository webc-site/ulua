use crate::{
  functions::{has_attribute_in_array::find_attribute_in_raw, optional_node::opt_ptr},
  records::{
    ast_attr::{AstAttr, AstAttrType},
    ast_type_function::AstTypeFunction,
  },
};

impl AstTypeFunction {
  pub fn get_attribute(&self, attribute_type: AstAttrType) -> *mut AstAttr {
    opt_ptr(find_attribute_in_raw(self.attributes, attribute_type))
  }
}
