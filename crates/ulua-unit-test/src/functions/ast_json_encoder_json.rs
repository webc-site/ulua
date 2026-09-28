use alloc::string::String;

use ulua_analysis::functions::to_json_ast_json_encoder::to_json as encode_to_json;
use ulua_ast::rtti::AstNodePtr;

pub fn json(node: impl AstNodePtr) -> String {
  // `as_ast_node()` 门面上转收口 repr(C) 单继承基址重合；
  // Safety: node 为 fixture 内存活的 AST 节点（json 序列化只读遍历，不伪造借用）。
  unsafe { encode_to_json(node.as_ast_node()) }
}
