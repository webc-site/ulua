//! Source: `Analysis/include/Luau/Set.h:34-47` (hand-ported)

use alloc::string::String;
use core::hash::Hash;

use crate::records::set::Set;
impl<T: Clone + Hash + PartialEq> Set<T> {
  /// C++ `bool insert(const T& element)` — true when newly inserted.
  pub fn insert(&mut self, element: &T) -> bool {
    let entry = self.mapping.get_or_insert(element.clone());
    let fresh = !*entry;

    if fresh {
      *entry = true;
      self.entry_count += 1;
    }

    fresh
  }
}

/// `Set<String>` 的 `&str` 借用视图插入口（r7-tset1，rc-4 插侧空缺补位）。
///
/// 实形裁定：专化 impl（`impl Set<String>`）而非泛型口——沿用 rc-4 仲裁
/// "专化 impl 弃泛型 Borrow"先例（`dense_hash_map.rs`/`dense_hash_set.rs`
/// 专化块文档）：借用口成立的全部前提是 `String`/`str` 的哈希与等值逐位
/// 一致，该性质只在默认 functor 下可证。`Set` 的 mapping 恒为
/// `DenseHashMap<T, bool>`（无 H/E 泛型参，即默认 functor 不可定制），
/// 论证条件结构性满足；本组口内部直用其 `find_str`/`get_mut_str` 借用核。
///
/// 语义与泛型口逐位复现：`String::from(s)` 与同内容 `&String` 的 `clone`
/// 字节相等，喂入同一 [`get_or_insert`] 探测链得同一槽位，fresh 判定与
/// `entry_count` 增减路径完全一致；差异仅在堆物化次数——旧口调用形
/// `insert(&String::from(x))` 现场分配 1 次 + 函数体 `element.clone()`
/// 再分配 1 次（set_insert_set.rs:9 实证 2 malloc），新口入口一次
/// `String::from(s)` 合计 1 malloc。
///
/// [`get_or_insert`]: ulua_common::records::dense_hash_map::DenseHashMap::get_or_insert
impl Set<String> {
  /// `insert` 的 `&str` 借用口：以单次 `String::from(s)` 物化 owned 键后
  /// 走与 [`insert`](Self::insert) 逐位相同的槽位写入路径。true 当且仅当
  /// 键为新增。
  pub fn insert_str(&mut self, s: &str) -> bool {
    let entry = self.mapping.get_or_insert(String::from(s));
    let fresh = !*entry;

    if fresh {
      *entry = true;
      self.entry_count += 1;
    }

    fresh
  }
}

#[cfg(test)]
mod tests {
  //! `insert_str` 与 owned 口 `insert(&String)` 双口等价（r7-tset1 行为守恒，
  //! 参照 `ulua-common/tests/dense_hash.rs` rc-4 双口用例风格）：同内容边界键集上
  //! fresh/重复返回值逐点一致；`insert_str` 落键后 `contains(&String::from(s))`=true；
  //! 底层槽位数（含空串占位键场景）双口保形。
  //! §8 留证：断言直读 `pub(crate)` 字段 `mapping`（槽位数保形钉），外部测试
  //! 无从读取，迁移须泄 pub 扩面，故保留 src。

  use alloc::string::{String, ToString};

  use crate::records::set::Set;

  fn edge_cases() -> Vec<String> {
    vec![
      String::new(),
      "x".to_string(),
      "key\0with\0nul".to_string(),
      "多字节 🚀 键".to_string(),
      "l".repeat(1000),
    ]
  }

  #[test]
  fn insert_str_matches_owned_insert() {
    let cases = edge_cases();
    let mut new_port: Set<String> = Set::new(String::new());
    let mut old_port: Set<String> = Set::new(String::new());

    for s in &cases {
      assert!(new_port.insert_str(s), "空表插入必 fresh: {s:?}");
      assert!(old_port.insert(s), "owned 口同键必 fresh: {s:?}");
    }
    assert_eq!(new_port.size(), cases.len());
    assert_eq!(new_port, old_port, "同序插入后双口集合相等");

    // 重复插入：双口都报 !fresh，size/槽位数不动。
    for s in &cases {
      assert!(!new_port.insert_str(s), "重复插入必 !fresh: {s:?}");
      assert!(!old_port.insert(s), "owned 口重复同键必 !fresh: {s:?}");
    }
    assert_eq!(new_port.size(), cases.len(), "重复插入不动 entry_count");
    assert_eq!(
      new_port.mapping.size(),
      old_port.mapping.size(),
      "底层槽位数（含空串键）双口保形"
    );

    // 任务钉：insert_str(s) 后旧口 contains(&String::from(s))=true，count 双口一致。
    for s in &cases {
      assert!(
        new_port.contains(s),
        "insert_str 落键后 contains 必真: {s:?}"
      );
      assert_eq!(new_port.count_str(s), new_port.count(s));
      assert_eq!(new_port.count_str(s), 1);
    }
  }
}
