use alloc::string::String;
use core::ptr::from_mut;

use ulua_ast::rtti::AstNodePtr;

use crate::functions::ast_json_encoder_json::json;

pub fn json_ref<T>(node: &mut T) -> String
where
  *mut T: AstNodePtr,
{
  json(from_mut(node))
}
