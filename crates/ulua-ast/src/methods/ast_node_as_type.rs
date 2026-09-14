use core::ptr::null_mut;

use crate::{
  records::{
    ast_node::AstNode, ast_type::AstType, ast_type_error::AstTypeError,
    ast_type_function::AstTypeFunction, ast_type_group::AstTypeGroup,
    ast_type_intersection::AstTypeIntersection, ast_type_optional::AstTypeOptional,
    ast_type_reference::AstTypeReference, ast_type_singleton_bool::AstTypeSingletonBool,
    ast_type_singleton_string::AstTypeSingletonString, ast_type_table::AstTypeTable,
    ast_type_typeof::AstTypeTypeof, ast_type_union::AstTypeUnion,
  },
  rtti::AstNodeClass,
};

impl AstNode {
  pub fn as_type(&self) -> *mut AstType {
    let is_type = matches!(
      self.class_index,
      AstTypeError::CLASS_INDEX
        | AstTypeFunction::CLASS_INDEX
        | AstTypeGroup::CLASS_INDEX
        | AstTypeIntersection::CLASS_INDEX
        | AstTypeOptional::CLASS_INDEX
        | AstTypeReference::CLASS_INDEX
        | AstTypeSingletonBool::CLASS_INDEX
        | AstTypeSingletonString::CLASS_INDEX
        | AstTypeTable::CLASS_INDEX
        | AstTypeTypeof::CLASS_INDEX
        | AstTypeUnion::CLASS_INDEX
    );

    if is_type {
      self as *const AstNode as *mut AstType
    } else {
      null_mut()
    }
  }
}

#[inline]
pub fn ast_node_as_type(node: &AstNode) -> *mut AstType {
  node.as_type()
}
