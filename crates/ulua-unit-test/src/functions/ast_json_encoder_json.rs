use alloc::string::String;

use ulua_analysis::functions::to_json_ast_json_encoder::to_json as encode_to_json;
use ulua_ast::rtti::AstNodePtr;

pub fn json(node: impl AstNodePtr) -> String {
  encode_to_json(unsafe { node.as_ast_node().as_mut() })
}
