extern crate alloc;

// Source: `tests/Set.test.cpp`
#[test]
fn set_clear_resets_size() {
  use ulua_analysis::records::set::Set;

  let mut s1 = Set::<i32>::new(0);
  s1.insert(&1);
  s1.insert(&2);
  assert_eq!(s1.size(), 2);

  s1.clear();
  assert_eq!(s1.size(), 0);
  assert!(s1.empty());
}

// Source: `tests/Set.test.cpp`
#[test]
fn set_empty_set_size_0() {
  use ulua_analysis::records::set::Set;

  let s1 = Set::<i32>::new(0);
  assert_eq!(s1.size(), 0);
  assert!(s1.empty());
}

// Source: `tests/Set.test.cpp`
#[test]
fn set_erase_using_const_ref_argument() {
  use alloc::string::String;

  use ulua_analysis::records::set::Set;

  let mut s1 = Set::<String>::new(String::new());
  s1.insert(&String::from("x"));
  s1.insert(&String::from("y"));

  let key = String::from("y");
  s1.erase(&key);

  assert!(s1.count(&String::from("x")) != 0);
  assert_eq!(s1.count(&String::from("y")), 0);
}

// Source: `tests/Set.test.cpp`
#[test]
fn set_erase_works_and_decreases_size() {
  use ulua_analysis::records::set::Set;

  let mut s1 = Set::<i32>::new(0);
  s1.insert(&1);
  s1.insert(&2);
  assert_eq!(s1.size(), 2);
  assert!(s1.contains(&1));
  assert!(s1.contains(&2));

  s1.erase(&1);
  assert_eq!(s1.size(), 1);
  assert!(!s1.contains(&1));
  assert!(s1.contains(&2));

  s1.erase(&2);
  assert_eq!(s1.size(), 0);
  assert!(s1.empty());
  assert!(!s1.contains(&1));
  assert!(!s1.contains(&2));
}

// Source: `tests/Set.test.cpp`
#[test]
fn set_insertion_works_and_increases_size() {
  use ulua_analysis::records::set::Set;

  let mut s1 = Set::<i32>::new(0);
  assert_eq!(s1.size(), 0);
  assert!(s1.empty());

  s1.insert(&1);
  assert!(s1.contains(&1));
  assert_eq!(s1.size(), 1);

  s1.insert(&2);
  assert!(s1.contains(&2));
  assert_eq!(s1.size(), 2);
}

// Source: `tests/Set.test.cpp`
#[test]
fn set_iterate_over_set() {
  use ulua_analysis::records::set::Set;

  let mut s1 = Set::<i32>::new(0);
  s1.insert(&1);
  s1.insert(&2);
  s1.insert(&3);
  assert_eq!(s1.size(), 3);

  let sum: i32 = s1.iter().copied().sum();
  assert_eq!(sum, 6);
}

// Source: `tests/Set.test.cpp`
#[test]
fn set_iterate_over_set_skips_erased_elements() {
  use ulua_analysis::records::set::Set;

  let mut s1 = Set::<i32>::new(0);
  for value in 1..=6 {
    s1.insert(&value);
  }
  assert_eq!(s1.size(), 6);

  s1.erase(&2);
  s1.erase(&4);
  s1.erase(&6);

  let sum: i32 = s1.iter().copied().sum();
  assert_eq!(sum, 9);
}

// Source: `tests/Set.test.cpp`
#[test]
fn set_iterate_over_set_skips_first_element_if_it_is_erased() {
  use alloc::{string::String, vec::Vec};

  use ulua_analysis::records::set::Set;

  let mut s1 = Set::<String>::new(String::new());
  s1.insert_str("x");
  s1.insert_str("y");
  s1.erase_str("y");

  let out: Vec<String> = s1.iter().cloned().collect();
  assert_eq!(1, out.len());
}

// ==== r7-tset1 尾台账（Set<String> *_str 专化口，rc-4 插侧补位）====
// 已迁 3 枚：set_iterate_over_set_skips_first_element_if_it_is_erased :130-132
//   （insert_str×2 实测 2 malloc→1；erase_str×1 命中径实测 2 malloc→0）。
// 让 5 枚（同窗 tstr15 席正改 set.rs 让位台账，本席触达限定=验证站点行+本块）：
//   set_erase_using_const_ref_argument——:36/:37 insert(&String::from(..)) 可换
//   insert_str 各降 1 malloc；:39/:40 `key` 绑定改 &str 直传 erase_str 合计降 2
//   （调用方物化+体内 clone 双免）；:42/:43 count(&String::from(..)) 可换
//   count_str 各降 1 malloc（借用口零分配）。
// 行为守恒：insert_str/erase_str/count_str 三对双口等价钉（含缺席态
//   get_or_insert 占位副作用保形钉）已 inline 落于 ulua-analysis
//   src/methods/set_{insert_set,erase_set,count}.rs tests mod，本文件不重复。

// ── tstr15 堆物化收口台账（分支 r7-tstr15，基 ae64575；行号为改前基线）──
// 票面 grep 8 行 = 真站点 8 枚（无台账文字噪声）。收 0 / 让 8。
// 依波账票77 裁定：本文件 8 枚站点归 tstr15 让位、收口候 tset1 后票，
// 本票不动测试体（合并绑定虽可省临时物化，但会与 tset1 改写同面撞车，且
// `Set<String>` 门面缺 `*_str` 专化口才是真正的闸，先修闸）。
// 让位清单（消费口三查询/插口皆 `&T`＝`&String`，直传 `&str` 实测定罪
// E0308 `expected &String, found &str` 共 5 探针位，覆盖 insert/count/contains/erase）：
//   :36/:37/:130/:131 insert —— crates/ulua-analysis/src/methods/set_insert_set.rs:8
//     （体 = `mapping.get_or_insert(element.clone())` ⇒ 每枚 2 次堆物化：
//     临时串 + 表内克隆，此即票77 登记之"双 malloc 真矿"）
//   :39/:40（绑定 key 供 erase 用）与 :132 —— erase(element: &T)
//     methods/set_erase_set.rs:9（同样 `get_or_insert(clone)`）
//   :42/:43 count —— methods/set_count.rs:8（`mapping.find(element)` 按值键）
//     旁证 contains —— methods/set_contains.rs:7（转 count，同 `&T` 口）
//   稠密侧已有 `*_str` 专化先例 crates/ulua-common/src/records/dense_hash_map.rs:235-277
//   （find_str/get_str/find_mut_str/contains_str/erase_str），但 Set 门面与插侧空缺。
// 解锁条件：tset1 基建席为 `Set<String>` 补 insert_str/erase_str/count_str
//   （rc-4 稠密 `*_str` 先例）⇒ 本 8 枚一次收口，insert 位每枚省 1、
//   count/erase 位每枚省 1（临时串归零）。
// 账面：除本台账外零改动，测试函数 8 枚与基线 8 passed 逐枚一致。
