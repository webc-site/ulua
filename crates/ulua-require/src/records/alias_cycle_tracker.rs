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
/// 构造只走 [`AliasCycleTracker::new`]，递归按值移动，无需 Clone/Default。
#[derive(Default)]
pub struct AliasCycleTracker {
  pub(crate) ordered: Vec<Vec<u8>>,
}

impl AliasCycleTracker {
  /// 空追踪器（对应 C++ `AliasCycleTracker{}`）。
  pub fn new() -> Self {
    Self {
      ordered: Vec::new(),
    }
  }

  /// 记录别名；已在环中出现则返回字节精确的环消息（对应 cpp `add(std::string)`）。
  pub fn add(&mut self, alias: Vec<u8>) -> Option<Vec<u8>> {
    // 链长 = 配置嵌套深度（极小），线性查重（slice::contains）即可
    if self.ordered.contains(&alias) {
      let mut message = Vec::with_capacity(CYCLE_PREFIX.len() + alias.len() * 2 + 16);
      message.extend_from_slice(CYCLE_PREFIX);
      self.append_cycle(&mut message, &alias);
      message.push(b')');
      return Some(message);
    }

    self.ordered.push(alias);
    None
  }

  /// 拼接 `@a -> @b -> @a` 形态的环：全部按字节追加，非 UTF-8 别名字节原样保留。
  fn append_cycle(&self, result: &mut Vec<u8>, repeated: &[u8]) {
    for item in self.ordered.iter().skip_while(|item| *item != repeated) {
      result.push(ALIAS_PREFIX);
      result.extend_from_slice(item);
      result.extend_from_slice(CYCLE_ARROW);
    }
    result.push(ALIAS_PREFIX);
    result.extend_from_slice(repeated);
  }
}
