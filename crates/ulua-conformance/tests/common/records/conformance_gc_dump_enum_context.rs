use crate::common::records::heap::Heap;

/// cpp `Conformance.test.cpp:3562-3567` 的 `struct EnumContext`：堆快照 + 是否已经
/// 看到那条 100000 字符量级的长字符串。
///
/// `errors` 收集 node 回调里对上游 `CHECK` 的偏离（回调签名受 C ABI 限制不能返回
/// `Result`，且必须遍历完整个堆才能报出全部偏差），用例末尾统一断言为空。
#[derive(Default)]
pub struct ConformanceGcDumpEnumContext {
  pub heap: Heap,
  pub seen_target_string: bool,
  pub errors: Vec<String>,
}
