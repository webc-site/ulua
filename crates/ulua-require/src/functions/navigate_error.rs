//! 导航错误消息拼装：全部按字节拼接，与 cpp `std::string` 的 `+` 完全等价
//! （路径/别名/组件可能含非 UTF-8 字节，转成 `String` 会破坏字节语义）。

use alloc::vec::Vec;

use crate::{enums::navigate_result::NavigateResult, functions::path_bytes::ALIAS_PREFIX};

/// `NavigateResult::Ambiguous` 时统一追加的后缀（cpp 各分支同一字面量）。
pub(crate) const AMBIGUOUS_SUFFIX: &[u8] = b" (ambiguous)";
/// 非法别名消息的固定尾部。
const NOT_A_VALID_ALIAS: &[u8] = b" is not a valid alias";

/// Ambiguous 结果对应的后缀，其它结果为空。
pub(crate) fn ambiguous(result: NavigateResult) -> &'static [u8] {
  if result == NavigateResult::Ambiguous {
    AMBIGUOUS_SUFFIX
  } else {
    &[]
  }
}

/// 拼 `prefix + '"' + value + '"' + suffix`：cpp `"..." + x + "\"..."` 的等价形态。
pub(crate) fn quoted(prefix: &[u8], value: &[u8], suffix: &[u8]) -> Vec<u8> {
  let mut message = Vec::with_capacity(prefix.len() + value.len() + suffix.len() + 2);
  message.extend_from_slice(prefix);
  message.push(b'"');
  message.extend_from_slice(value);
  message.push(b'"');
  message.extend_from_slice(suffix);
  message
}

/// 拼 `message + suffix`。
pub(crate) fn suffixed(message: &[u8], suffix: &[u8]) -> Vec<u8> {
  let mut result = Vec::with_capacity(message.len() + suffix.len());
  result.extend_from_slice(message);
  result.extend_from_slice(suffix);
  result
}

/// 拼 `@<alias> is not a valid alias[ (ambiguous)]`（override / fallback 共用文案）。
pub(crate) fn invalid_alias(alias: &[u8], ambiguous: bool) -> Vec<u8> {
  // 精确容量 = '@' + alias + 固定尾部 +（可选）ambiguous 后缀，免增长重分配
  // （原 `alias.len() + 40` 魔法余量折算为常量长度加法，恒不小于实际所需）
  let mut message =
    Vec::with_capacity(alias.len() + 1 + NOT_A_VALID_ALIAS.len() + AMBIGUOUS_SUFFIX.len());
  message.push(ALIAS_PREFIX);
  message.extend_from_slice(alias);
  message.extend_from_slice(NOT_A_VALID_ALIAS);
  if ambiguous {
    message.extend_from_slice(AMBIGUOUS_SUFFIX);
  }
  message
}
