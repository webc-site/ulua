use alloc::{string::String, vec::Vec};

use ulua_ast::{
  records::{ast_node::AstNode, comment::Comment},
  visit::ast_node_visit,
};

use crate::records::ast_json_encoder::AstJsonEncoder;

/// # Safety
/// 调用方须保证 `node` 非空、对齐，指向 parser arena 中一棵完整存活至本次序列化结束、地址稳定的 AST；
/// encoder 只以共享借用遍历读取、不写 AST，`ast_node_visit` 同前提。cpp `Analysis/src/AstJsonEncoder.cpp:1591`
/// （`std::string toJson(AstNode*)`）。单线程遍历内无并存可变别名。
pub unsafe fn to_json(node: *mut AstNode) -> String {
  let mut encoder = AstJsonEncoder::ast_json_encoder_ast_json_encoder();
  // Safety: `node` 按函数级契约非 null，指向 parser arena 中一棵完整存活且对齐的 AST；
  // `ast_node_visit` 是 `unsafe fn`，其契约（同一 `node`）在遍历期间成立，encoder 只以
  // 共享借用读取节点字段、不写 AST，单线程遍历内无并存可变别名。
  unsafe {
    ast_node_visit(node, &mut encoder);
  }
  encoder.str()
}

/// # Safety
/// 调用方须保证 `node` 非空、对齐，指向 parser arena 中一棵完整存活至本次序列化结束、地址稳定的 AST；
/// encoder 只读遍历不写 AST，`comment_locations` 为传入即自有的值。cpp `Analysis/src/AstJsonEncoder.cpp:1598`
/// （`std::string toJson(AstNode*, const std::vector<Comment>&)`）。单线程。
pub unsafe fn to_json_with_comment_locations(
  node: *mut AstNode,
  comment_locations: Vec<Comment>,
) -> String {
  let mut encoder = AstJsonEncoder::ast_json_encoder_ast_json_encoder();
  encoder.write_raw_string_view("{\"root\":");
  // Safety: `node` 按函数级契约非 null，指向 parser arena 中一棵完整存活且对齐的 AST；
  // `ast_node_visit` 是 `unsafe fn`，同一 `node` 前提在遍历期间成立，encoder 只以共享
  // 借用读取节点字段、不写 AST，单线程遍历内无并存可变别名。
  unsafe {
    ast_node_visit(node, &mut encoder);
  }
  encoder.write_raw_string_view(",\"commentLocations\":[");
  encoder.write_comments(comment_locations);
  encoder.write_raw_string_view("]}");
  encoder.str()
}
