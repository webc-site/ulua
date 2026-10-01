//! Source: `Analysis/include/Luau/Set.h:55-65` (hand-ported)

use alloc::string::String;
use core::hash::Hash;

use crate::records::set::Set;
impl<T: Clone + Hash + PartialEq> Set<T> {
  /// C++ `void erase(T&& element)` / `void erase(const T& element)` —
  /// tombstones the entry (sets it false) rather than removing the slot.
  pub fn erase(&mut self, element: &T) {
    let entry = self.mapping.get_or_insert(element.clone());

    if *entry {
      *entry = false;
      self.entry_count -= 1;
    }
  }
}

/// `Set<String>` 的 `&str` 借用视图擦除口（r7-tset1；实形裁定与整体论证见
/// [`set_insert_set`](super::set_insert_set) 专化块文档）。
///
/// 与泛型口逐位复现，含其冷径语义：泛型 [`erase`](Self::erase) 走
/// `get_or_insert`，**键缺席时会先在 mapping 里插一个 false 墓碑占槽**
/// （不动 `entry_count`，但影响后续探测链与 `iter()` 落位序）——本口如实
/// 保形该副作用。热径（命中真实键）经 `get_mut_str` 单探改写，零分配，
/// 旧口同径为 2 malloc（调用方 `String::from` + 体内 `clone`）；冷径
/// （缺席）保留 1 malloc（`String::from` 物化占位键），旧口同径 2 malloc。
impl Set<String> {
  /// `erase` 的 `&str` 借用口。
  pub fn erase_str(&mut self, s: &str) {
    let hit = self.mapping.get_mut_str(s).map(|entry| {
      let was_present = *entry;
      *entry = false;
      was_present
    });
    match hit {
      Some(true) => self.entry_count -= 1,
      // 命中假槽：与旧口 `if *entry` 假分支同为 no-op。
      Some(false) => {}
      None => {
        // 缺席态保形：复现旧口 `get_or_insert` 的 false 墓碑占位（新槽默认
        // 值即 false，故无须再写）。
        self.mapping.get_or_insert(String::from(s));
      }
    }
  }
}

#[cfg(test)]
mod tests {
  //! `erase_str` 与 owned 口 `erase` 双口等值，覆盖三条判定路径：命中真槽
  //! （tombstone + entry_count 减一）、命中假槽（no-op）、缺席（保形
  //! `get_or_insert` 占位副作用——以 `mapping.size()` 直断，`pub(crate)` 字段
  //! 仅 crate 内测试可见，故本钉不落 tests/ 集成文件）。

  use alloc::string::{String, ToString};

  use crate::records::set::Set;

  #[test]
  fn erase_str_matches_owned_erase_including_miss_side_effect() {
    let mut new_port: Set<String> = Set::new(String::new());
    let mut old_port: Set<String> = Set::new(String::new());
    for s in ["x", "y", "多字节 🚀"] {
      assert!(new_port.insert_str(s));
      assert!(old_port.insert(&s.to_string()));
    }

    // 命中真槽：双口同擦同键，size/可见性逐点一致。
    new_port.erase_str("y");
    old_port.erase(&"y".to_string());
    assert_eq!(new_port.size(), 2);
    assert_eq!(new_port.size(), old_port.size(), "entry_count 双口一致");
    assert_eq!(new_port.count_str("y"), 0);
    assert_eq!(new_port.count_str("y"), old_port.count(&String::from("y")));

    // 命中假槽（重复擦）：双口都 no-op，槽位数不动。
    let slots_new = new_port.mapping.size();
    new_port.erase_str("y");
    old_port.erase(&"y".to_string());
    assert_eq!(new_port.mapping.size(), slots_new, "假槽再擦不再占位");
    assert_eq!(new_port.size(), 2);

    // 缺席态：双口都在 mapping 里多占一个 false 墓碑、entry_count 不动、
    // count/contains/iter 均不可见。
    let slots_before = new_port.mapping.size();
    new_port.erase_str("never");
    old_port.erase(&"never".to_string());
    assert_eq!(
      new_port.mapping.size(),
      slots_before + 1,
      "缺席 erase_str 须复现 get_or_insert 占位副作用"
    );
    assert_eq!(
      new_port.mapping.size(),
      old_port.mapping.size(),
      "缺席态双口槽位数保形"
    );
    assert_eq!(new_port.size(), 2, "墓碑不动 entry_count");
    assert_eq!(new_port.count_str("never"), 0);
    assert!(!new_port.contains(&String::from("never")));
    assert!(!new_port.iter().any(|k| k == "never"), "iter 跳过假槽");

    // 终态：内容面 PartialEq / iter 多重集一致。
    assert_eq!(new_port, old_port);
    let mut a: Vec<String> = new_port.iter().cloned().collect();
    let mut b: Vec<String> = old_port.iter().cloned().collect();
    a.sort();
    b.sort();
    assert_eq!(a, b);
  }
}
