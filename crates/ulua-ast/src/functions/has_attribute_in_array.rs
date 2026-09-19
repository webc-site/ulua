use crate::{
  functions::find_attribute_in_array::find_attribute_in_array,
  records::{
    ast_array::AstArray,
    ast_attr::{AstAttr, AstAttrType},
  },
};

pub(crate) fn has_attribute_in_array(
  attributes: AstArray<*mut AstAttr>,
  attribute_type: AstAttrType,
) -> bool {
  !find_attribute_in_array(attributes, attribute_type).is_null()
}
