//! `VisitKey` / `VisitKeyRef`（seen-set 地址身份键）三契约钉（§8 迁移自
//! `src/records/visit_key.rs` 单元测试：被测面 `from_ptr`/派生 `Hash`/`Eq`/`Ord`
//! 与 `DenseDefault` 均为 pub，外部可达，无需泄密内部实现）：
//! `Hash`/`Eq` 逐字节转发到内部指针、`Ord`（`VisitKey` 独享，供 `BTreeMap` 使用）
//! 即裸地址序、`DenseDefault` 的 null 占位可 insert/find/erase 往返。
//! 参照同仓 `tests/dense_bcid_keys.rs` / `tests/dense_identifier_keys.rs` 风格。

use core::{
  hash::{Hash, Hasher},
  hint::black_box,
};
use std::collections::{HashMap, HashSet, hash_map::DefaultHasher};

use ulua_analysis::records::visit_key::{VisitKey, VisitKeyRef};
use ulua_common::records::{dense_hash_set::DenseHashSet, dense_hash_table::DenseDefault};

/// 把键喂给默认 hasher，取最终 `u64`——用于直断「同址⇒同哈希」，与集合
/// 去重面互补（集合面只证 Eq 一致，不逐位比哈希）。
fn hash_of<K: Hash>(key: &K) -> u64 {
  let mut h = DefaultHasher::new();
  key.hash(&mut h);
  h.finish()
}

/// 契约①`Hash`/`Eq` 逐字节转发到地址：同一地址的两次 `from_ptr` 恒相等且
/// 同哈希；不同栈变量地址（两个活对象）恒不等，且在 `HashMap` 里各占独立
/// 桶（异址去重为二、同址去重为二之一）。键永不解引用，故只用地址身份。
#[test]
fn visit_key_hash_eq_forward_address_identity() {
  let left: u8 = 1;
  let right: u8 = 2;
  // 强制两个局部都保活、地址不同（否则本用例的「异址」前提失效，令其显式失败）。
  let (left_addr, right_addr) = (
    black_box(&left) as *const u8,
    black_box(&right) as *const u8,
  );
  assert_ne!(left_addr, right_addr, "两个活栈变量地址应互异");

  let a = VisitKey::from_ptr(left_addr);
  let a_again = VisitKey::from_ptr(left_addr);
  let b = VisitKey::from_ptr(right_addr);

  assert_eq!(a, a_again, "同址必相等");
  assert_eq!(hash_of(&a), hash_of(&a_again), "同址必同哈希");
  assert_ne!(a, b, "异址必不等");

  // 集合面：同址坍缩、异址独立共存（证明 Eq/Hash 转发与地址一一对应）。
  let mut map: HashMap<VisitKey, u8> = HashMap::new();
  map.insert(a, 10);
  map.insert(a_again, 11);
  map.insert(b, 12);
  assert_eq!(map.len(), 2, "同址去重、异址并存");
  assert_eq!(
    *map.get(&a).unwrap(),
    11,
    "重复 from_ptr 命中同一条目并覆盖"
  );
  assert_eq!(*map.get(&b).unwrap(), 12);
}

/// 契约②`Ord` 即裸地址序（`BTreeMap` 依赖之）：对一组打乱顺序的合成地址，
/// 经派生 `Ord` 排序的键序列与按 `usize` 地址排序的期望序列逐元素全等。
/// 用合成地址（仅携带位模式、永不被解引用，符合模块头「地址即身份」契约）
/// 构造非单调输入，避免平台相对地址序导致断言退化。
#[test]
fn visit_key_ord_is_raw_address_order() {
  let addrs: [usize; 5] = [0x1234, 0x100, 0xffff_ffff_ffff, 0x1, 0x2000];
  let keys: Vec<VisitKey> = addrs
    .iter()
    .map(|&addr| VisitKey::from_ptr(addr as *const u8))
    .collect();

  let mut by_ord = keys.clone();
  by_ord.sort(); // 派生 Ord：转发到内部 `*mut ()` 的 `Ord`（地址序）

  let mut by_addr = addrs.to_vec();
  by_addr.sort_unstable(); // 期望序：`p as usize` 升序
  let expected: Vec<VisitKey> = by_addr
    .iter()
    .map(|&addr| VisitKey::from_ptr(addr as *const u8))
    .collect();

  assert_ne!(keys, by_ord, "输入须为乱序，否则本用例退化为恒等");
  assert_eq!(
    by_ord, expected,
    "VisitKey 的 Ord 序必须与裸指针 `as usize` 序逐元素全等"
  );

  // 直接钉相邻比较：升序地址 ⇒ 键的 `<` 全序与地址 `<` 一致。
  for w in by_addr.windows(2) {
    let (lo, hi) = (
      VisitKey::from_ptr(w[0] as *const u8),
      VisitKey::from_ptr(w[1] as *const u8),
    );
    assert!(lo < hi, "地址升序必映射到键的严格升序");
  }
}

/// 契约③`DenseDefault` 的 null 占位可正常 insert/find/erase：位图判占用，
/// null 键（=`dense_default()`）作为普通键存取自如，且与真键并存、擦除 null
/// 不波及真键（旧代「键等于哨兵即空槽」误判的反例）。镜像 bcid/identifier 先例。
#[test]
fn visit_key_dense_default_null_slot_roundtrip() {
  let mut set: DenseHashSet<VisitKey> = DenseHashSet::default();
  assert!(set.empty(), "default() 起步为空");

  let null_key = VisitKey::dense_default();
  // 与 null 占位形状不同的真键：借用活栈对象地址（仅按指针哈希/比较，不解引用）。
  let marker: u8 = 0;
  let real = VisitKey::from_ptr(black_box(&marker) as *const u8);

  set.insert(null_key);
  set.insert(real);
  assert_eq!(set.size(), 2, "null 占位与真键并存，占用由位图判定");
  assert!(set.contains(&null_key), "null 键应可命中");
  assert!(set.contains(&real));

  set.erase(&null_key);
  assert!(!set.contains(&null_key), "擦除后 null 键应缺席");
  assert!(
    set.contains(&real),
    "擦除 null 占位不得波及真键（旧代哨兵比较实现的丢键陷阱）"
  );
  set.erase(&real);
  assert!(set.empty());
}

/// `VisitKeyRef`（`*const ()` 极性）hash/eq 转发最小版：同址相等、异址不等，
/// 与 `VisitKey` 同构（各自逐字节转发原极性，集合永不跨极性混用）。
#[test]
fn visit_key_ref_hash_eq_forward_address_identity() {
  let left: u8 = 1;
  let right: u8 = 2;
  let (la, ra) = (
    black_box(&left) as *const u8,
    black_box(&right) as *const u8,
  );
  assert_ne!(la, ra);

  let a = VisitKeyRef::from_ptr(la);
  let a_again = VisitKeyRef::from_ptr(la);
  let b = VisitKeyRef::from_ptr(ra);

  assert_eq!(a, a_again);
  assert_eq!(hash_of(&a), hash_of(&a_again), "同址必同哈希");
  assert_ne!(a, b, "异址必不等");

  let mut set: HashSet<VisitKeyRef> = HashSet::new();
  set.insert(a);
  set.insert(a_again);
  set.insert(b);
  assert_eq!(set.len(), 2, "同址坍缩、异址共存");
}

/// `VisitKeyRef` 的 null 占位（`*const T` 的 `DenseDefault` 逐位一致）可
/// insert/find/erase 往返，语义同 `VisitKey` 的 null 占位。
#[test]
fn visit_key_ref_dense_default_null_slot_roundtrip() {
  let mut set: DenseHashSet<VisitKeyRef> = DenseHashSet::default();
  let null_key = VisitKeyRef::dense_default();
  let marker: u8 = 0;
  let real = VisitKeyRef::from_ptr(black_box(&marker) as *const u8);

  set.insert(null_key);
  set.insert(real);
  assert_eq!(set.size(), 2);
  set.erase(&null_key);
  assert!(!set.contains(&null_key));
  assert!(set.contains(&real), "擦除 null 占位不得波及真键");
  set.erase(&real);
  assert!(set.empty());
}
