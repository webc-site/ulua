use core::ptr::null_mut;

use crate::records::{
  ast_array::AstArray,
  ast_attr::{AstAttr, AstAttrType},
};

pub(crate) fn find_attribute_in_array(
  attributes: AstArray<*mut AstAttr>,
  attribute_type: AstAttrType,
) -> *mut AstAttr {
  for &attribute in attributes.as_slice() {
    if attribute.is_null() {
      continue;
    }

    unsafe {
      if (*attribute).r#type == attribute_type {
        return attribute;
      }
    }
  }

  null_mut()
}
