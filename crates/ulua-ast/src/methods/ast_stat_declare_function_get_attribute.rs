use crate::{
  functions::find_attribute_in_array::find_attribute_in_array,
  records::{
    ast_attr::{AstAttr, AstAttrType},
    ast_stat_declare_function::AstStatDeclareFunction,
  },
};

impl AstStatDeclareFunction {
  pub fn get_attribute(&self, attribute_type: AstAttrType) -> *mut AstAttr {
    find_attribute_in_array(self.attributes, attribute_type)
  }
}

pub fn ast_stat_declare_function_get_attribute(
  this: &AstStatDeclareFunction,
  attribute_type: AstAttrType,
) -> *mut AstAttr {
  this.get_attribute(attribute_type)
}
