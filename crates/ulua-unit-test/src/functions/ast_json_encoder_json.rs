use alloc::string::String;

use ulua_analysis::functions::to_json_ast_json_encoder::to_json as encode_to_json;
use ulua_ast::rtti::AstNodePtr;

/// 节点身份指针 → analysis JSON encoder。
///
/// # Safety 边界说明（非 `unsafe fn`，块内 unsafe 的契约）
/// analysis `to_json` 的 encoder 以 visitor 写穿节点（cpp `JsonEncoder` 持非
/// const `AstNode*`），故此处是测试侧唯一把节点指针交出独占借用的边界；
/// `node` 来自 fixture arena 的存活节点，测试内单线程、调用即还、无并发借用。
pub fn json(node: impl AstNodePtr) -> String {
  // Safety: 见函数级边界说明。
  encode_to_json(unsafe { node.as_ast_node().as_mut() })
}
