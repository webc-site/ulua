//! `Set<String>` 双口等值钉（§8 迁移自 `src/methods/set_count.rs` 单元测试：
//! 被测面 `new`/`insert`/`insert_str`/`count`/`count_str`/`erase_str` 均为 pub，
//! 外部可达）：`count_str` 与 owned 口 `count` 在三种槽位态（真实键 / 墓碑
//! false 槽 / 缺席键）下逐点等值——判定核同为 `*entry` 读出，双口只换键的
//! 进入方式。

extern crate alloc;

use alloc::string::{String, ToString};

use ulua_analysis::records::set::Set;

/// `count_str` 与 owned 口 `count` 在三种槽位态（真实键 / 墓碑 false 槽 /
/// 缺席键）下逐点等值——判定核同为 `*entry` 读出，双口只换键的进入方式。
#[test]
fn count_str_matches_owned_count_across_states() {
  let mut set: Set<String> = Set::new(String::new());

  // 缺席态
  assert_eq!(set.count_str("absent"), 0);
  assert_eq!(set.count(&"absent".to_string()), 0);

  // 真实键态：混用两口插入，验证交叉可见
  assert!(set.insert_str("a"));
  assert!(set.insert(&"b".to_string()));
  assert_eq!(set.count_str("a"), 1);
  assert_eq!(set.count_str("b"), 1);
  assert_eq!(set.count(&String::from("a")), set.count_str("a"));
  assert_eq!(set.count(&String::from("b")), set.count_str("b"));

  // 墓碑 false 态：erase 后双口都报 0（find 命中但值假）
  set.erase_str("a");
  assert_eq!(set.count_str("a"), 0, "墓碑槽必计 0");
  assert_eq!(set.count(&String::from("a")), set.count_str("a"));
  assert_eq!(set.count_str("b"), 1, "擦 a 不波及 b");
}
