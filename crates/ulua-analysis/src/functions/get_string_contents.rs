use alloc::string::String;

use ulua_ast::{
  records::{
    ast_expr_constant_string::AstExprConstantString, ast_expr_interp_string::AstExprInterpString,
    ast_node::AstNode,
  },
  rtti::ast_node_try_as,
};
use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::records::arena_handle::alias_opt;

/// `node` 为 parse-arena 节点句柄（可空，null → None）；解引用收口 `alias_opt`，
/// 业务侧 safe 调用。对应 C++ `static std::optional<std::string> getStringContents(const AstNode* node)` (`cpp/Analysis/src/AutocompleteCore.cpp:1822`)。
pub fn get_string_contents(node: *const AstNode) -> Option<String> {
  // null 折叠为 None，与原 `is_null` 早退逐格等价。
  let node_ref = alias_opt(node)?;

  if let Some(string_node) = ast_node_try_as::<AstExprConstantString>(node_ref) {
    return Some(String::from_utf8_lossy(string_node.value.as_bytes()).into_owned());
  }
  if let Some(interp_string) = ast_node_try_as::<AstExprInterpString>(node_ref)
    && interp_string.expressions.is_empty()
  {
    LUAU_ASSERT!(interp_string.strings.len() == 1);
    let first_string_array = interp_string.strings.as_slice().first()?;
    return Some(String::from_utf8_lossy(first_string_array.as_bytes()).into_owned());
  }

  None
}

// r7-tlossy1 让位台账（本文件票面 2 枚：让 2）——:25/:32 `String::from_utf8_lossy(...).into_owned()`
// 源为词法字节流（字面量可携非 UTF-8 字节，cpp `getStringContents` 直返
// `std::string(data,size)` 不校验，Rust `Option<String>` 出口 lossy 系表示面）；
// 出口 owned 下限各 1 malloc。消费面 `autocomplete_string_params.rs:80` 持
// `candidate_string.clone()` 进回调闭包（Option<String> 值转移），改 Cow/借用需给
// raw-ptr 契约造 arena 生命周期并贯通 require_suggester/回调签名，外溢 ≥3 调用面，
// 外溢候裁。IDE 补全触发路径非每节点热面。
