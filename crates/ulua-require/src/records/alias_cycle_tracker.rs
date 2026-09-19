use alloc::vec::Vec;

use ulua_common::records::dense_hash_set::DenseHashSet;

/// 对应 cpp `AliasCycleTracker`：别名以字节串存储（`std::string`），
/// 非 UTF-8 别名字节参与环检测与消息拼装时不被改写。
#[derive(Debug, Clone, Default)]
pub struct AliasCycleTracker {
  pub(crate) seen: DenseHashSet<Vec<u8>>,
  pub(crate) ordered: Vec<Vec<u8>>,
}
