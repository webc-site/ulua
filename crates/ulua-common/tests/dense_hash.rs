//! Oracle validation for the hand-ported `DenseHashMap`/`DenseHashSet`
//! (occupancy-bitmap rewrite of `cpp/Common/include/Luau/DenseHash.h`):
//! differential fuzz against `BTreeMap`/`BTreeSet` across insert/erase churn —
//! this is what exercises the TAOCP 6.4-R backward-shift `doErase` and the
//! `usedTable` probes — plus the behaviour change the bitmap brings: a key equal
//! to the `empty_key` placeholder is storable (the legacy sentinel build could
//! not represent it).

use std::{
  collections::{BTreeMap, BTreeSet},
  hash::{Hash, Hasher},
  ptr::{null, null_mut},
};

use ulua_common::records::{
  dense_hash_map::DenseHashMap,
  dense_hash_set::DenseHashSet,
  dense_hash_table::{DenseEq, DenseHasher},
};

mod common;

use common::Rng;

/// 空槽占位键：位图版实现里它只是 `Vec<Item>` 空槽的填充值，不再是哨兵。
const PLACEHOLDER: u32 = u32::MAX;

#[test]
fn map_matches_btree_oracle_under_churn() {
  let mut dense: DenseHashMap<u32, u32> = DenseHashMap::new(PLACEHOLDER);
  let mut oracle: BTreeMap<u32, u32> = BTreeMap::new();
  let mut rng = Rng(0x5eed_1234_9abc_def0);

  for step in 0..20_000u64 {
    // 键域覆盖 PLACEHOLDER 本身：位图版必须把它当作普通键。
    let key = if step % 31 == 0 {
      PLACEHOLDER
    } else {
      u32::try_from(rng.below(300)).expect("键域在 u32 内")
    };
    match rng.below(3) {
      0 | 1 => {
        let value = u32::try_from(rng.below(u64::from(u32::MAX))).expect("值域在 u32 内");
        dense.insert(key, value);
        oracle.insert(key, value);
      }
      2 => {
        dense.erase(&key);
        oracle.remove(&key);
      }
      _ => unreachable!("below(3) 只可能返回 0..3"),
    }

    assert_eq!(dense.size(), oracle.len(), "step {step}: size 漂移");
    assert_eq!(
      dense.contains(&key),
      oracle.contains_key(&key),
      "step {step}"
    );
    assert_eq!(
      dense.find(&key).copied(),
      oracle.get(&key).copied(),
      "step {step}"
    );
  }

  let mut dense_keys: Vec<u32> = dense.iter().map(|(k, _)| *k).collect();
  dense_keys.sort_unstable();
  assert_eq!(
    dense_keys,
    oracle.keys().copied().collect::<Vec<_>>(),
    "迭代必须恰好产出全部占用键"
  );
}

#[test]
fn set_matches_btree_oracle_under_churn() {
  let mut dense: DenseHashSet<u32> = DenseHashSet::new(PLACEHOLDER);
  let mut oracle: BTreeSet<u32> = BTreeSet::new();
  let mut rng = Rng(0xc0ff_eeba_ddca_fe01);

  for _ in 0..20_000u64 {
    let key = u32::try_from(rng.below(257)).expect("键域在 u32 内");
    if rng.below(4) == 0 {
      assert_eq!(
        dense.try_insert(key),
        oracle.insert(key),
        "try_insert 的 fresh 判定漂移"
      );
    } else {
      dense.insert(key);
      oracle.insert(key);
    }
    if rng.below(5) == 0 {
      dense.erase(&key);
      oracle.remove(&key);
    }
  }

  assert_eq!(dense.size(), oracle.len());
  let mut dense_keys: Vec<u32> = dense.iter().copied().collect();
  dense_keys.sort_unstable();
  assert_eq!(dense_keys, oracle.iter().copied().collect::<Vec<_>>());
}

/// 位图版的核心行为：`empty_key` 只是空槽填充值，等于它的键可以存取、迭代、擦除。
#[test]
fn placeholder_key_is_an_ordinary_key() {
  let mut map: DenseHashMap<u32, String> = DenseHashMap::new(PLACEHOLDER);
  map.insert(PLACEHOLDER, String::from("placeholder"));
  map.insert(7, String::from("seven"));

  assert_eq!(map.size(), 2);
  assert_eq!(
    map.find(&PLACEHOLDER).map(String::as_str),
    Some("placeholder")
  );
  assert_eq!(
    map.get(&PLACEHOLDER).map(String::as_str),
    Some("placeholder")
  );
  let mut pairs: Vec<(u32, &str)> = map.iter().map(|(k, v)| (*k, v.as_str())).collect();
  pairs.sort_unstable();
  assert_eq!(pairs, vec![(7, "seven"), (PLACEHOLDER, "placeholder")]);

  map.erase(&7);
  assert_eq!(
    map.find(&PLACEHOLDER).map(String::as_str),
    Some("placeholder")
  );
  map.erase(&PLACEHOLDER);
  assert!(map.find(&PLACEHOLDER).is_none());
  assert!(map.empty());
}

/// 指针键 `default()` 门面：与旧形 `new(null_mut())` 等价起步（空表），
/// 且 null 恰为占位键，也能当普通键正常 get/insert/erase（哨兵可存取）。
#[test]
fn pointer_default_facade_stores_sentinel_key() {
  let mut map: DenseHashMap<*mut u32, i32> = DenseHashMap::default();
  let legacy: DenseHashMap<*mut u32, i32> = DenseHashMap::new(null_mut());
  assert_eq!(
    map.size(),
    legacy.size(),
    "default() 与 new(null_mut()) 起步等价"
  );
  assert!(map.is_empty());

  let mut target = 7u32;
  let key = &mut target as *mut u32;
  map.insert(key, 1);
  map.insert(null_mut(), 2);
  assert_eq!(map.size(), 2);
  assert_eq!(map.find(&key).copied(), Some(1));
  assert_eq!(map.get(&null_mut()).copied(), Some(2));
  assert!(map.contains_key(&null_mut()));

  map.erase(&null_mut());
  assert!(!map.contains(&null_mut()));
  assert_eq!(map.find(&key).copied(), Some(1), "擦除占位键不得波及真实键");
  map.erase(&key);
  assert!(map.empty());

  let mut set: DenseHashSet<*const u32> = DenseHashSet::default();
  let const_key = key.cast_const();
  assert!(set.try_insert(null()), "首次插入哨兵键应为 fresh");
  assert!(!set.try_insert(null()), "重复插入哨兵键不应判 fresh");
  set.insert(const_key);
  assert_eq!(set.size(), 2);
  assert!(set.contains(&null()));
  assert_eq!(set.iter().count(), 2, "迭代必须产出占位键在内的全部占用项");
  set.erase(&null());
  assert!(!set.contains(&null()));
  assert_eq!(set.find(&const_key).copied(), Some(const_key));
}

/// 可变迭代：每个占用槽恰好访问一次，且能改值；`iter_mut` 不能泄漏空槽。
#[test]
fn iter_mut_visits_each_occupied_slot_once() {
  let mut map: DenseHashMap<u32, u32> = DenseHashMap::new(PLACEHOLDER);
  for key in 0..500u32 {
    map.insert(key, key);
  }

  for (_, value) in map.iter_mut() {
    *value *= 2;
  }
  for key in 0..500u32 {
    assert_eq!(
      map.find(&key).copied(),
      Some(key * 2),
      "键 {key} 的值未被 iter_mut 更新"
    );
  }

  let mut seen: BTreeSet<u32> = BTreeSet::new();
  for (key, _) in map.iter_mut() {
    assert!(seen.insert(*key), "键 {key} 被重复产出");
  }
  assert_eq!(seen.len(), 500);
}

/// `operator[]` 语义：缺键插入值初始化项（`DenseDefault`），命中则返回同一槽位。
#[test]
fn index_like_access_value_initializes_on_miss() {
  let mut map: DenseHashMap<u32, u32> = DenseHashMap::new(PLACEHOLDER);
  *map.get_or_insert(1) = 10;
  assert_eq!(*map.get_or_insert(1), 10);
  assert_eq!(map.size(), 1);
  assert_eq!(
    *map.get_or_insert(2),
    0,
    "新槽位应为值初始化（cpp `Value()`）"
  );

  let (value, fresh) = map.try_insert(3, 30);
  assert!(fresh);
  *value += 1;
  assert_eq!(map.find(&3).copied(), Some(31));
  let (value, fresh) = map.try_insert(1, 999);
  assert!(!fresh, "已存在的键不应被覆盖");
  assert_eq!(*value, 10);
}

/// `clear` 的两条上游路径：小表保留容量、大表释放存储；两条都要清空位图。
/// cpp `clear(size_t thresholdToDestroy = 32)`（DenseHash.h:360-377）阈值作用于
/// **容量**：10 项容量停在 16（≤32，走就地销毁、保留存储），40 项涨到 64
/// （>32，走 destroy、释放存储）。两容器都灌 40 只会各跑同一 destroy 路径。
#[test]
fn clear_resets_occupancy_on_both_paths() {
  let mut small: DenseHashSet<u32> = DenseHashSet::new(PLACEHOLDER);
  let mut large: DenseHashSet<u32> = DenseHashSet::new(PLACEHOLDER);
  for key in 0..10u32 {
    small.insert(key);
  }
  for key in 0..40u32 {
    large.insert(key);
  }
  for (container, count) in [(&mut small, 10u32), (&mut large, 40u32)] {
    container.clear();
    assert!(container.empty());
    assert!(!container.contains(&0));
    assert_eq!(container.iter().count(), 0, "位图未清空");
    // 清空后重新填充：旧代实现里哨兵残留会让这里误判命中。
    for key in 0..count {
      container.insert(key);
    }
    assert_eq!(
      container.size(),
      usize::try_from(count).expect("项数在 usize 内")
    );
  }
}

#[test]
fn dense_hash_traits_and_idioms() {
  // DenseHashMap: len, is_empty, IntoIterator (&, &mut), Extend, FromIterator
  let mut map: DenseHashMap<u32, u32> = [(1, 10), (2, 20)].into_iter().collect();
  assert_eq!(map.len(), 2);
  assert!(!map.is_empty());

  map.extend([(3, 30), (4, 40)]);
  assert_eq!(map.len(), 4);

  // IntoIterator for &DenseHashMap
  let mut sum_k = 0;
  let mut sum_v = 0;
  for (&k, &v) in &map {
    sum_k += k;
    sum_v += v;
  }
  assert_eq!(sum_k, 1 + 2 + 3 + 4);
  assert_eq!(sum_v, 10 + 20 + 30 + 40);

  // IntoIterator for &mut DenseHashMap
  for (_, v) in &mut map {
    *v += 1;
  }
  assert_eq!(map.get(&1).copied(), Some(11));

  // DenseHashSet: len, is_empty, IntoIterator (&), Extend, FromIterator
  let mut set: DenseHashSet<u32> = [10, 20, 30].into_iter().collect();
  assert_eq!(set.len(), 3);
  assert!(!set.is_empty());

  set.extend([40, 50]);
  assert_eq!(set.len(), 5);

  let slice = [60, 70];
  set.extend(&slice);
  assert_eq!(set.len(), 7);

  // IntoIterator for &DenseHashSet
  let mut set_sum = 0;
  for &item in &set {
    set_sum += item;
  }
  assert_eq!(set_sum, 10 + 20 + 30 + 40 + 50 + 60 + 70);
}

// ---------------------------------------------------------------------------
// 对照 `cpp/tests/DenseHash.test.cpp` 的确定性用例（模糊测试之外的定点回归）：
// rehash / 满表覆写不 rehash / 迭代中合并 / 探测链擦除 / 集合相等 / 可变访问面。
// ---------------------------------------------------------------------------

/// 强制同桶哈希器：`do_hash` 拿 `0 * FIBONACCI >> shift == 0`，所有键都落在桶 0，
/// 于是插入序即线性探测链序 —— 用来确定性地压 `doErase` 的 TAOCP 6.4-R 前移，
/// 不依赖默认哈希的散射结果。
#[derive(Default)]
struct SameBucket;

impl DenseHasher<i32> for SameBucket {
  fn hash(&self, _key: &i32) -> usize {
    0
  }
}

/// 键的哈希/相等只看 `id`，`aux` 是可原位改写的附属位——cpp
/// `AstNameTable::getOrAddWithType` 用 `const_cast` 原地重写非自有名字指针
/// （hash/eq 不变故槽位仍有效）的 Rust 形态。
#[derive(Clone)]
struct SlotKey {
  id: u32,
  aux: u32,
}

impl Hash for SlotKey {
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.id.hash(state);
  }
}

impl PartialEq for SlotKey {
  fn eq(&self, other: &Self) -> bool {
    self.id == other.id
  }
}

impl Eq for SlotKey {}

/// cpp `map_rehash`/`set_rehash`（DenseHash.test.cpp:122-137,321-334）：跨多次容量
/// 翻倍后全部键精确可查，值逐项相等（原 Rust 侧只有模糊覆盖，无定点用例）。
#[test]
fn rehash_keeps_every_key_value_findable() {
  let mut map: DenseHashMap<i32, i32> = DenseHashMap::new(-1);
  let mut set: DenseHashSet<i32> = DenseHashSet::new(-1);
  for i in 0..100 {
    map.insert(i, i * 10);
    set.insert(i);
  }
  assert_eq!(map.size(), 100);
  assert_eq!(set.size(), 100);
  for i in 0..100 {
    assert_eq!(map.find(&i).copied(), Some(i * 10), "map 键 {i} 漂移");
    assert_eq!(set.find(&i).copied(), Some(i), "set 键 {i} 漂移");
  }
}

/// cpp `map_overwrite_when_full_shouldnt_rehash`（:181-198）：恰好灌到 3/4 负载阈值
/// （12 项 = 容量 16 的阈值）后整批覆写已有键——`rehash_if_full` 的 `find` 守卫必须
/// 让它走"不增长"分支。迭代器存活那半边在 Rust 由借用检查器结构性保证（持迭代器
/// 时无法再 `&mut` 插入），这里锁定可观察面：项数不变、每项值恰为旧值 +1。
#[test]
fn overwrite_when_full_shouldnt_rehash() {
  let mut map: DenseHashMap<i32, i32> = DenseHashMap::new(-1);
  for i in 0..12 {
    map.insert(i, i);
  }
  assert_eq!(map.size(), 12);

  let keys: Vec<i32> = map.iter().map(|(k, _)| *k).collect();
  assert_eq!(keys.len(), 12);
  for k in &keys {
    *map.get_or_insert(*k) += 1;
  }

  assert_eq!(map.size(), 12, "覆写已有键不得改变项数");
  for i in 0..12 {
    assert_eq!(map.find(&i).copied(), Some(i + 1), "键 {i} 的覆写值漂移");
  }
}

/// cpp `map_merge_with_rehash_while_iterating`（:200-235）：12 项与 16 项（重叠 8..12）
/// 相加合并，结果 24 项且逐键精确（重叠键为两倍值）。
#[test]
fn merge_two_maps_with_rehash_yields_exact_union() {
  let mut m1: DenseHashMap<i32, i32> = DenseHashMap::new(-1);
  for i in 0..12 {
    m1.insert(i, i);
  }
  let mut m2: DenseHashMap<i32, i32> = DenseHashMap::new(-1);
  for i in 8..24 {
    m2.insert(i, i);
  }
  assert_eq!((m1.size(), m2.size()), (12, 16));

  // 先把两侧快照取出来，再写回 m1：Rust 借用规则替代了 cpp 的"迭代中插入"约束。
  let m1_snapshot: Vec<(i32, i32)> = m1.iter().map(|(k, v)| (*k, *v)).collect();
  for (k, a) in m1_snapshot {
    if let Some(b) = m2.find(&k) {
      m1.insert(k, a + b);
    }
  }
  let m2_snapshot: Vec<(i32, i32)> = m2.iter().map(|(k, v)| (*k, *v)).collect();
  for (k, a) in m2_snapshot {
    if !m1.contains(&k) {
      m1.insert(k, a);
    }
  }

  assert_eq!(m1.size(), 24);
  for i in 0..24 {
    let expected = if !(8..12).contains(&i) { i } else { i + i };
    assert_eq!(m1.find(&i).copied(), Some(expected), "合并后键 {i} 值漂移");
  }
}

/// 确定性探测链擦除（cpp `map_erase_chain` :486-511 的定点版）：同桶哈希器把全部键
/// 压成一条线性探测链，擦除链首/链中/链尾都必须由 backward-shift 保住其余元素可达。
/// 活键集合逐步建模，逐键比对，兼查 `map_erase_and_reinsert`（:470-484）与
/// `map_erase_nonexistent`（:456-468）。
#[test]
fn erase_repairs_probe_chain_deterministically() {
  let mut map: DenseHashMap<i32, i32, SameBucket> = DenseHashMap::new(-1);
  let mut live: BTreeSet<i32> = BTreeSet::new();
  for key in 0..12 {
    map.insert(key, key * 100);
    live.insert(key);
  }
  assert_eq!(map.size(), 12);

  // 链首（键 0）、链中（键 5）、链尾（键 11）各擦一次，每次都校验全链可达。
  for erased in [0, 5, 11] {
    map.erase(&erased);
    live.remove(&erased);
    assert!(map.find(&erased).is_none(), "键 {erased} 擦后仍可查");

    for key in 0..12 {
      let expected = live.contains(&key).then_some(key * 100);
      assert_eq!(
        map.find(&key).copied(),
        expected,
        "擦除 {erased} 后键 {key} 断链"
      );
    }
    let iterated: BTreeSet<i32> = map.iter().map(|(k, _)| *k).collect();
    assert_eq!(iterated, live, "迭代不得产出空洞或重复产出");
    assert_eq!(map.size(), live.len());

    // 擦除后同键可重插（cpp `map_erase_and_reinsert`）：换值插入后按新值命中。
    map.insert(erased, erased * 7);
    live.insert(erased);
    assert_eq!(map.find(&erased).copied(), Some(erased * 7));
    assert_eq!(map.size(), live.len());

    map.erase(&erased);
    live.remove(&erased);
  }

  // 擦不存在的键是 no-op（cpp `map_erase_nonexistent`）。
  map.erase(&999);
  assert_eq!(map.size(), live.len(), "擦除缺键不得动表");
  assert_eq!(map.size(), 9);
}

/// cpp `set_equality`（:366-381）：`==` 是集合相等——同元素不同插入序相等；长度不同
/// 或元素缺失则不等。
#[test]
fn set_equality_is_set_equality() {
  let mut a: DenseHashSet<u32> = DenseHashSet::new(u32::MAX);
  let mut b: DenseHashSet<u32> = DenseHashSet::new(u32::MAX);
  for k in [1, 2, 3] {
    a.insert(k);
  }
  for k in [3, 1, 2] {
    b.insert(k);
  }
  assert_eq!(a, b, "插入序不得影响集合相等");

  b.insert(4);
  assert_ne!(a, b, "多一个元素必须不等");
  b.erase(&4);
  assert_eq!(a, b);
  b.erase(&2);
  assert_ne!(a, b, "少一个元素必须不等");
}

/// 可变访问面：`find_mut`/`get_mut`（map）与 `insert_mut`/`find_mut`（set，cpp
/// `const_cast` 惯用法的 Rust 拼写）改到的是槽内原项，命中同一条目不新增。
#[test]
fn mutable_accessors_update_slot_in_place() {
  let mut map: DenseHashMap<i32, i32> = DenseHashMap::new(-1);
  map.insert(1, 10);
  assert_eq!(map.find_mut(&1).copied(), Some(10));
  assert_eq!(map.get_mut(&1).copied(), Some(10));
  assert!(map.find_mut(&2).is_none(), "缺键的可变查找必须为 None");
  if let Some(v) = map.get_mut(&1) {
    *v += 5;
  }
  assert_eq!(map.find(&1).copied(), Some(15));
  assert_eq!(map.size(), 1);

  let seed = SlotKey { id: 0, aux: 0 };
  let mut set: DenseHashSet<SlotKey> = DenseHashSet::new(seed);
  set.insert(SlotKey { id: 1, aux: 0 });
  // 二次 `insert_mut` 命中同一槽位（项数不变），原位改写 `aux` 后仍按 `id` 命中。
  set.insert_mut(SlotKey { id: 1, aux: 0 }).aux = 42;
  assert_eq!(set.size(), 1, "同键二次插入不得新增");
  assert_eq!(
    set.find(&SlotKey { id: 1, aux: 0 }).map(|k| k.aux),
    Some(42),
    "insert_mut 未改写槽内原项"
  );
  if let Some(slot) = set.find_mut(&SlotKey { id: 1, aux: 7 }) {
    slot.aux = 43;
  }
  assert_eq!(set.get(&SlotKey { id: 1, aux: 0 }).map(|k| k.aux), Some(43));
}

/// cpp `DenseHashMap(size_t buckets)` 构造变体（DenseHash.h:836-839，消费点
/// `Substitution.cpp:161-163`）的 `reserve_buckets`：空表扩桶后照常全键可查；
/// 非空表为 no-op，不得丢项。
#[test]
fn reserve_buckets_expands_empty_and_noops_nonempty() {
  let mut map: DenseHashMap<i32, i32> = DenseHashMap::new(-1);
  map.reserve_buckets(64);
  for i in 0..40 {
    map.insert(i, i);
  }
  assert_eq!(map.size(), 40);
  for i in 0..40 {
    assert_eq!(map.find(&i).copied(), Some(i));
  }

  // 已有 40 项时再 reserve 是 no-op（保留全部项）。
  map.reserve_buckets(1024);
  assert_eq!(map.size(), 40, "非空表 reserve 不得丢项");
  for i in 0..40 {
    assert_eq!(map.find(&i).copied(), Some(i));
  }
}

/// 自定义 `Hash`/`Eq` 模板参数注入口（cpp `DenseHash<K, V, Hash, Eq>` 的等价面）：
/// 同桶哈希器把全部键压进一条探测链，配"差值 ≤2 判等"的等值器，命中必须由注入的
/// functor 决定而非内置 `PartialEq`；并且 `setKey` 覆写槽内键（cpp `ItemInterfaceMap2`
/// 语义）后，按旧键查询仍按判等命中同一槽。
#[test]
fn custom_eq_functor_drives_lookup() {
  /// 差值不超过 2 即视为同键。
  #[derive(Default)]
  struct NearEq;

  impl DenseEq<i32> for NearEq {
    fn eq(&self, a: &i32, b: &i32) -> bool {
      (*a - *b).abs() <= 2
    }
  }

  let mut map: DenseHashMap<i32, i32, SameBucket, NearEq> = DenseHashMap::new(-1);
  map.insert(10, 1);
  map.insert(60, 2);
  assert_eq!(map.size(), 2);
  // 11 与槽内键 10 判等 → 命中同槽；50 与链上两键判等皆假 → 未命中。
  assert_eq!(map.find(&11).copied(), Some(1));
  assert_eq!(map.find(&50).copied(), None);
  // 内建的 `PartialEq` 形态下 10 与 12 不同键；注入 NearEq 后必须命中同一槽并覆写其键。
  map.insert(12, 3);
  assert_eq!(map.size(), 2, "判等命中必须覆写同槽而非新增");
  assert_eq!(
    map.find(&10).copied(),
    Some(3),
    "槽内键被改写后按判等仍可达"
  );
  assert_eq!(map.find(&60).copied(), Some(2), "他键槽位不得被波及");
}

// ---------------------------------------------------------------------------
// r7-rc-4：`String` 键容器 `&str` 借用视图查询口（*_str）与 owned 口的双口等价。
// 正确性自证：hash 逐位一致（含空串/内含 NUL/多字节 UTF-8/超长键，Rust `String`
// 恒为合法 UTF-8，无"非 UTF-8"维度）；同插入集下 find/find_str 与 BTree 三方一致；
// erase_str 只擦字节相等键。
// ---------------------------------------------------------------------------

/// 边界键集：空串（同时是 `new(String::new())` 的 empty_key 占位，位图版必须
/// 可命中）、内含 NUL、多字节 UTF-8、跨 64 位字边界的长键。
fn str_edge_cases() -> Vec<String> {
  vec![
    String::new(),
    "a".into(),
    "key\0with\0nul".into(),
    "\0".into(),
    "多字节 🚀 键".into(),
    "x".repeat(63),
    "y".repeat(64),
    "z".repeat(1000),
  ]
}

/// 由随机源产出 [0, 6) 片段拼出的键（含空片段→空串高频出现）。
fn rand_str_key(rng: &mut Rng) -> String {
  const FRAGMENTS: [&str; 8] = ["a", "b", "\u{0}", "é", "🚀", "long_key", "%n", "KEY"];
  let n = rng.below(6);
  let mut s = String::new();
  for _ in 0..n {
    s.push_str(FRAGMENTS[rng.below(FRAGMENTS.len() as u64) as usize]);
  }
  s
}

/// hash 保真的直接钉死：同一内容经 `&str` 与 `&String` 进 `dense_hash_of`
/// 必须给出同一 usize（std `impl Hash for String` 纯转发 `str`，字节流相同；
/// wasm32 下 `finish() as usize` 的截断也在同一函数里同路径施加）。
#[test]
fn dense_hash_of_is_identical_for_str_and_string() {
  use ulua_common::type_aliases::dense_hash_default::dense_hash_of;

  for case in str_edge_cases() {
    assert_eq!(
      dense_hash_of(case.as_str()),
      dense_hash_of(&case),
      "String/str 哈希漂移: {case:?}"
    );
  }
  let mut rng = Rng(0xa1b2_c3d4_e5f6_0708);
  for _ in 0..2000 {
    let case = rand_str_key(&mut rng);
    assert_eq!(dense_hash_of(case.as_str()), dense_hash_of(&case));
  }
}

/// map 双口等价模糊测试：每个随机步都断言 `find(&String)`、`find_str(&str)`、
/// `contains`/`contains_str`、`contains_key_str` 与 BTreeMap 对照一致；擦除全部
/// 走 `erase_str`（借用口驱动的 do_erase 即 TAOCP 前移路径）。
#[test]
fn map_str_view_matches_owned_view_under_churn() {
  let mut dense: DenseHashMap<String, u32> = DenseHashMap::new(String::new());
  let mut oracle: BTreeMap<String, u32> = BTreeMap::new();

  for (i, k) in str_edge_cases().into_iter().enumerate() {
    dense.insert(k.clone(), i as u32 + 1);
    oracle.insert(k, i as u32 + 1);
  }

  let mut rng = Rng(0x57c0_ffee_dead_beef);
  for step in 0..4000u64 {
    let key = rand_str_key(&mut rng);
    match rng.below(3) {
      0 | 1 => {
        let value = u32::try_from(rng.below(u64::from(u32::MAX))).expect("值域在 u32 内");
        dense.insert(key.clone(), value);
        oracle.insert(key.clone(), value);
      }
      _ => {
        dense.erase_str(&key);
        oracle.remove(&key);
      }
    }
    assert_eq!(
      dense.find(&key).copied(),
      dense.find_str(&key).copied(),
      "step {step}: find/find_str 对 {key:?} 分歧"
    );
    assert_eq!(
      dense.find_str(&key).copied(),
      oracle.get(&key).copied(),
      "step {step}: find_str 与 oracle 分歧"
    );
    assert_eq!(dense.get_str(&key).copied(), dense.find_str(&key).copied());
    assert!(dense.contains(&key) == dense.contains_str(&key));
    assert!(dense.contains_key_str(&key) == dense.contains_str(&key));
    assert_eq!(dense.size(), oracle.len(), "step {step}: size 漂移");
  }

  // 全表终检：oracle 每个键的双口都可读，find_mut_str 原位改写后双口同见新值。
  for (k, v) in &oracle {
    assert_eq!(dense.find_str(k).copied(), Some(*v), "终检命中: {k:?}");
    let expected = v.wrapping_add(7);
    let slot = dense.find_mut_str(k).expect("借用口可变命中");
    *slot = expected;
    assert_eq!(
      dense.find(k).copied(),
      Some(expected),
      "owned 口见新值: {k:?}"
    );
    assert_eq!(
      dense.find_str(k).copied(),
      Some(expected),
      "借用口见新值: {k:?}"
    );
  }
}

/// set 双口等价 + 擦除精确性：`erase_str` 只擦字节相等键（前缀/扩写都不误擦），
/// `find_mut_str` 原位改写键内容后旧视图不再命中、新视图命中——与 owned 口的
/// `PartialEq` 语义逐位一致。
#[test]
fn set_str_view_matches_owned_view_and_erase_is_exact() {
  let mut dense: DenseHashSet<String> = DenseHashSet::new(String::new());
  let mut oracle: BTreeSet<String> = BTreeSet::new();

  for k in str_edge_cases() {
    dense.insert(k.clone());
    oracle.insert(k);
  }

  let mut rng = Rng(0x57c0_0dec_feed_face);
  for step in 0..4000u64 {
    let key = rand_str_key(&mut rng);
    if rng.below(3) == 0 {
      assert_eq!(dense.try_insert(key.clone()), !oracle.contains(&key));
      oracle.insert(key.clone());
    } else if rng.below(4) == 0 {
      dense.erase_str(&key);
      oracle.remove(&key);
    }
    assert_eq!(
      dense.find(&key).is_some(),
      dense.find_str(&key).is_some(),
      "step {step}: find/find_str 对 {key:?} 分歧"
    );
    assert_eq!(
      dense.contains(&key),
      dense.contains_str(&key),
      "step {step}"
    );
    assert_eq!(
      dense.find_str(&key),
      oracle.get(&key),
      "step {step}: 借用口与 oracle 分歧"
    );
    assert_eq!(dense.size(), oracle.len(), "step {step}: size 漂移");
  }

  // 擦除精确性：对表内键取真前缀/加后缀，erase_str 均不得误擦。
  let probe = "prefix_probe_key";
  dense.insert(probe.into());
  dense.erase_str("prefix_probe");
  assert!(dense.contains_str(probe), "真前缀不得命中");
  dense.erase_str("prefix_probe_keyX");
  assert!(dense.contains_str(probe), "扩写不得命中");
  dense.erase_str(probe);
  assert!(!dense.contains_str(probe));
  assert!(!dense.contains(&probe.to_string()), "双口终态一致");

  // 借用口可变访问面：find_mut_str 原位重写为**同字节**的新分配副本——即
  // `AstNameTable` const_cast 惯用法的合法形态（hash/eq 不变故槽位仍有效）。
  // 改写成不同字节会同时漂移哈希与桶位，属容器外契约，不在此测试范围。
  dense.insert("mutable_key".into());
  {
    let slot = dense.find_mut_str("mutable_key").expect("可变借用口命中");
    assert_eq!(slot.as_str(), "mutable_key");
    *slot = String::from("mutable_key");
  }
  assert!(
    dense.contains_str("mutable_key"),
    "同字节重写后借用口仍命中"
  );
  assert!(
    dense.contains(&"mutable_key".to_string()),
    "同字节重写后 owned 口仍命中"
  );
}
