//! Source: `Analysis/src/AstJsonEncoder.cpp:1562-1570` (hand-ported)
use alloc::{string::String, vec::Vec};

use ulua_ast::{
  records::{ast_node::AstNode, comment::Comment},
  visit::ast_node_visit,
};

use crate::records::ast_json_encoder::AstJsonEncoder;
/// # Safety
/// 调用方须保证 `node` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
pub unsafe fn to_json(node: *mut AstNode, comment_locations: Vec<Comment>) -> String {
  let mut encoder = AstJsonEncoder::ast_json_encoder_ast_json_encoder();
  encoder.write_raw_string_view("{\"root\":");
  unsafe {
    ast_node_visit(node, &mut encoder);
  }
  encoder.write_raw_string_view(",\"commentLocations\":[");
  encoder.write_comments(comment_locations);
  encoder.write_raw_string_view("]}");
  encoder.str()
}
