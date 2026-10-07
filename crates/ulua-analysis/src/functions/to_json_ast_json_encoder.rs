use alloc::{string::String, vec::Vec};

use ulua_ast::{
  records::{ast_node::AstNode, comment::Comment},
  visit::dispatch_node,
};

use crate::records::ast_json_encoder::AstJsonEncoder;

/// 对 AST 进行 JSON 序列化。
/// cpp `Analysis/src/AstJsonEncoder.cpp:1591`（`std::string toJson(AstNode*)`）。
pub fn to_json(node: Option<&mut AstNode>) -> String {
  let mut encoder = AstJsonEncoder::ast_json_encoder_ast_json_encoder();
  if let Some(node) = node {
    dispatch_node(node, &mut encoder);
  }
  encoder.str()
}

/// 对 AST 连同注释位置进行 JSON 序列化。
/// cpp `Analysis/src/AstJsonEncoder.cpp:1598`（`std::string toJson(AstNode*, const std::vector<Comment>&)`）。
pub fn to_json_with_comment_locations(
  node: Option<&mut AstNode>,
  comment_locations: Vec<Comment>,
) -> String {
  let mut encoder = AstJsonEncoder::ast_json_encoder_ast_json_encoder();
  encoder.write_raw_string_view("{\"root\":");
  if let Some(node) = node {
    dispatch_node(node, &mut encoder);
  }
  encoder.write_raw_string_view(",\"commentLocations\":[");
  encoder.write_comments(comment_locations);
  encoder.write_raw_string_view("]}");
  encoder.str()
}
