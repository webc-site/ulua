//! Source: `Analysis/src/AstJsonEncoder.cpp:1555-1560` (hand-ported)
use alloc::string::String;

use ulua_ast::{records::ast_node::AstNode, visit::ast_node_visit};

use crate::records::ast_json_encoder::AstJsonEncoder;
/// # Safety
/// 调用方须保证 `node` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
pub unsafe fn to_json(node: *mut AstNode) -> String {
  let mut encoder = AstJsonEncoder::ast_json_encoder_ast_json_encoder();
  unsafe {
    ast_node_visit(node, &mut encoder);
  }
  encoder.str()
}
