//! `InsertionOrderedMap` 对 cpp `Common/include/Luau/InsertionOrderedMap.h` 的
//! 语义对齐,测试原型为 `cpp/tests/InsertionOrderedMap.test.cpp`(共 7 个 TEST_CASE)。
//!
//! 三条上游契约:插入顺序即迭代顺序;重复 key 写入是 no-op(不覆盖);
//! erase 后其后元素的索引回移。
//! cpp:95 `destructuring_iterator_compiles` 仅为"可编译"冒烟例,由下方各
//! 测试的 `iter()` 解构模式覆盖,不单列。

use ulua_common::records::insertion_ordered_map::InsertionOrderedMap;

/// cpp:23-32 `map_insertion` 与 cpp:49-60 `insert_does_not_update` 的公共输入:
/// 两个互异 key 各插一次,再对首个 key 重复插入。
const FIRST_KEY: i32 = 1;
const SECOND_KEY: i32 = 2;

// cpp:23-32 map_insertion:两次插入后两个 key 均可查得(补 cpp 隐式期望的精确值)。
#[test]
fn map_insert_two_keys() {
  let mut m: InsertionOrderedMap<i32, i32> = InsertionOrderedMap::new();
  m.insert(FIRST_KEY, 1);
  m.insert(SECOND_KEY, 2);
  assert_eq!(m.size(), 2);
  assert_eq!(m.get(&FIRST_KEY), Some(&1), "{:?}", m);
  assert_eq!(m.get(&SECOND_KEY), Some(&2), "{:?}", m);
}

// cpp:34-47 map_lookup:命中已有 key 返回精确值;未知 key 返回 nullptr(即 None)。
#[test]
fn map_lookup_hit_and_miss() {
  let mut m: InsertionOrderedMap<i32, i32> = InsertionOrderedMap::new();
  m.insert(FIRST_KEY, 1);
  assert_eq!(m.get(&FIRST_KEY), Some(&1)); // cpp:42-43 REQUIRE(r) + *r == 1
  assert_eq!(m.get(&999), None); // cpp:45-46 get(新指针) == nullptr
}

// cpp:49-60 insert_does_not_update:同 key 二次 insert 是 no-op,旧值保留。
#[test]
fn insert_does_not_update_existing_key() {
  let mut m: InsertionOrderedMap<i32, i32> = InsertionOrderedMap::new();
  m.insert(FIRST_KEY, 1);
  m.insert(FIRST_KEY, 2);
  assert_eq!(m.get(&FIRST_KEY), Some(&1)); // cpp:59 *v == 1
  assert_eq!(m.size(), 1);
}

// cpp:62-93 insertion_order_is_iteration_order:迭代逐条锁定精确 (key, value) 对。
// (原 Rust 用例只锁 key 序列、值另经 get 抽查,断言弱于 cpp,已强化。)
#[test]
fn insertion_order_is_iteration_order() {
  let mut m: InsertionOrderedMap<i32, i32> = InsertionOrderedMap::new();
  // cpp 用三个不同指针 a/b/c 做 key、值恒 1;此处按 cpp 结构取 1/2/3 为 key。
  let inputs: &[(i32, i32)] = &[(3, 31), (1, 11), (2, 21)];
  for (k, v) in inputs {
    m.insert(*k, *v);
  }
  let pairs: Vec<(i32, i32)> = m.iter().map(|(k, v)| (*k, *v)).collect();
  // cpp:77-92 依次 CHECK a、b、c 三条 (key,value),++后到 end。
  assert_eq!(pairs, inputs.to_vec());
}

// cpp:108-125 map_erasure:erase 首个后 size 回 1、被删 key 三口径(contains/find/get)
// 皆不可查,另一 key 精确可查。
#[test]
fn map_erasure() {
  let mut m: InsertionOrderedMap<i32, i32> = InsertionOrderedMap::new();
  m.insert(FIRST_KEY, 1);
  m.insert(SECOND_KEY, 2);

  m.erase(&FIRST_KEY); // cpp:118 erase(map.find(a))
  assert_eq!(m.size(), 1); // cpp:119
  assert!(!m.contains(&FIRST_KEY), "{:?}", m); // cpp:120
  assert_eq!(m.get(&FIRST_KEY), None); // cpp:121
  assert_eq!(m.find(&FIRST_KEY), None);
  assert_eq!(m.get(&SECOND_KEY), Some(&2)); // cpp:123-124(强化为锁值)
}

// cpp:127-138 map_clear:清空后 size 为 0 且 key 不可查。(Rust 侧原无用例,补齐。)
#[test]
fn map_clear() {
  let mut m: InsertionOrderedMap<i32, i32> = InsertionOrderedMap::new();
  m.insert(FIRST_KEY, 1);
  m.clear();
  assert_eq!(m.size(), 0); // cpp:135
  assert!(m.is_empty());
  assert!(!m.contains(&FIRST_KEY), "{:?}", m); // cpp:136
  assert_eq!(m.get(&FIRST_KEY), None); // cpp:137
}

// 对齐 cpp 头文件 InsertionOrderedMap.h:70-76 `operator[]`(缺省插入后取 &mut):
// 首次取用写入缺省并可累加,二次取用命中同一条目不新增。
#[test]
fn get_or_default_matches_cpp_index_operator() {
  let mut m: InsertionOrderedMap<i32, i32> = InsertionOrderedMap::new();
  *m.get_or_default(5) = 50;
  assert_eq!(m.get(&5), Some(&50));
  *m.get_or_default(5) += 1; // existing: no new entry
  assert_eq!(m.get(&5), Some(&51));
  assert_eq!(m.size(), 1);
}

// 对齐 cpp 头文件 InsertionOrderedMap.h:121-134 `erase`:删中间项后其后
// 索引整体回移(find 即 erase 返回迭代器的位置语义);erase 不存在的 key 为 no-op。
#[test]
fn erase_reindexes_later_entries() {
  let mut m: InsertionOrderedMap<i32, i32> = InsertionOrderedMap::new();
  for k in [10, 20, 30, 40] {
    m.insert(k, k * 2);
  }
  m.erase(&20);
  assert_eq!(m.size(), 3);
  assert_eq!(m.find(&10), Some(0));
  assert_eq!(m.find(&30), Some(1));
  assert_eq!(m.find(&40), Some(2));
  assert_eq!(m.get(&40), Some(&80));
  m.erase(&999); // erasing end() is a no-op
  assert_eq!(m.size(), 3);
}

// 对齐 cpp 头文件 InsertionOrderedMap.h:61-68 的可变 `V* get(k)` 重载：命中改写槽内
// 原值且不新增条目，缺键返回 None（cpp 的 nullptr）。
#[test]
fn get_mut_matches_cpp_mutable_get() {
  let mut m: InsertionOrderedMap<i32, i32> = InsertionOrderedMap::new();
  m.insert(FIRST_KEY, 1);
  m.insert(SECOND_KEY, 2);

  assert_eq!(m.get_mut(&999), None); // cpp: get(未注册 key) == nullptr
  *m.get_mut(&FIRST_KEY).expect("已插入的 key 必命中") += 10;

  assert_eq!(m.get(&FIRST_KEY), Some(&11));
  assert_eq!(m.get(&SECOND_KEY), Some(&2), "改一个键不得波及他键");
  assert_eq!(m.size(), 2, "get_mut 不得新增条目");
  // 迭代序仍是插入序（cpp:62-93 的核心契约）。
  let pairs: Vec<(i32, i32)> = m.iter().map(|(k, v)| (*k, *v)).collect();
  assert_eq!(pairs, [(FIRST_KEY, 11), (SECOND_KEY, 2)]);
}
