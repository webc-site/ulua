use alloc::vec::Vec;

use crate::records::alias_cycle_tracker::AliasCycleTracker;

/// 环字符串中相邻别名之间的分隔（cpp `" -> "`）。
const CYCLE_ARROW: &[u8] = b" -> ";
/// 别名前缀字符。
const ALIAS_PREFIX: u8 = b'@';

impl AliasCycleTracker {
  /// 拼接 `@a -> @b -> @a` 形态的环：全部按字节追加，非 UTF-8 别名字节原样保留。
  pub(crate) fn append_cycle(&self, result: &mut Vec<u8>, repeated: &[u8]) {
    for item in self.ordered.iter().skip_while(|item| *item != repeated) {
      result.push(ALIAS_PREFIX);
      result.extend_from_slice(item);
      result.extend_from_slice(CYCLE_ARROW);
    }
    result.push(ALIAS_PREFIX);
    result.extend_from_slice(repeated);
  }
}
