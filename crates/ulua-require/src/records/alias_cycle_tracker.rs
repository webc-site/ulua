use alloc::vec::Vec;

use crate::functions::path_bytes::ALIAS_PREFIX;

/// cpp 环检测消息的头部。
const CYCLE_PREFIX: &[u8] = b"detected alias cycle (";
/// 环字符串中相邻别名之间的分隔（cpp `" -> "`）。
const CYCLE_ARROW: &[u8] = b" -> ";

/// 对应 cpp `AliasCycleTracker`：别名以字节串存储（`std::string`），
/// 非 UTF-8 别名字节参与环检测与消息拼装时不被改写。
///
/// 单存储 `ordered`：链长 = 配置嵌套深度（极小），线性查重免哈希集合双份存储。
/// 构造统一走 `Default`（对应 C++ `AliasCycleTracker{}`），递归按值移动，无需 Clone。
#[derive(Default)]
pub struct AliasCycleTracker {
  ordered: Vec<Vec<u8>>,
}

impl AliasCycleTracker {
  /// 记录别名；已在环中出现则返回字节精确的环消息（对应 cpp `add(std::string)`）。
  pub fn add(&mut self, alias: Vec<u8>) -> Option<Vec<u8>> {
    // 链长 = 配置嵌套深度（极小），线性查重（slice::contains）即可
    if self.ordered.contains(&alias) {
      return Some(self.cycle_message(&alias));
    }

    self.ordered.push(alias);
    None
  }

  /// 拼出 `detected alias cycle (@a -> @b -> @a)` 形态的完整环消息：全部按
  /// 字节拼装，非 UTF-8 别名字节原样保留（cpp `AliasCycleTracker::add` 命中
  /// 环时的返回值）。
  fn cycle_message(&self, repeated: &[u8]) -> Vec<u8> {
    let mut message =
      Vec::with_capacity(CYCLE_PREFIX.len() + repeated.len() * 2 + 16 + CYCLE_ARROW.len());
    message.extend_from_slice(CYCLE_PREFIX);
    // 自 repeated 起遍历环上各别名（cpp `add` 头部即环起点）
    for item in self.ordered.iter().skip_while(|item| *item != repeated) {
      message.push(ALIAS_PREFIX);
      message.extend_from_slice(item);
      message.extend_from_slice(CYCLE_ARROW);
    }
    message.push(ALIAS_PREFIX);
    message.extend_from_slice(repeated);
    message.push(b')');
    message
  }
}
