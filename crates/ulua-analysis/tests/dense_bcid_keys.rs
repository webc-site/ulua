//! `BlockedConstraintId` 键面 `default()` 门面单测（b16-variant3-dense 收口波）：
//! common `Variant3` 通型 [`DenseDefault`] impl（b16-bcid-default 定案的收口预案，
//! E0117 证明 impl 不可落本 analysis 包）落地后，
//! `DenseHashMap<BlockedConstraintId, _>::default()` 与旧形
//! `new(BlockedConstraintId::V0(null::<Type>()))` 起步逐位等价；并按占用位图
//! 契约（见 ulua-common `dense_hash_table` 模块文档）验证"位图判占用、哨兵可
//! 存取"：哨兵 `V0(null)` 作为普通键可 insert/命中/erase，且擦除哨兵不波及真实键。

use core::ptr::null;

use ulua_analysis::{
  records::{hash_blocked_constraint_id::HashBlockedConstraintId, r#type::Type},
  type_aliases::blocked_constraint_id::BlockedConstraintId,
};
use ulua_common::records::{dense_hash_map::DenseHashMap, dense_hash_table::DenseDefault};

type BcidMap = DenseHashMap<BlockedConstraintId, bool, HashBlockedConstraintId>;

/// `BlockedConstraintId::dense_default()` 必须是 `V0(null)`，与旧调用点内联
/// 传参 `BlockedConstraintId::V0(null::<Type>())` 逐位等价。
#[test]
fn bcid_dense_default_is_v0_null_sentinel() {
  let sentinel = BlockedConstraintId::dense_default();
  assert_eq!(sentinel, BlockedConstraintId::V0(null::<Type>()));
  assert_eq!(sentinel.index(), 0, "哨兵落在首变体 V0（TypeId 分量）");
  assert!(sentinel.get_if_0().is_some_and(|p| p.is_null()));
}

/// `default()` 门面与旧形 `new(V0(null))` 起步等价；哨兵键与真实键并存、
/// 正常存取（位图判占用契约），擦除哨兵不波及真实键。
#[test]
fn bcid_map_default_facade_stores_sentinel_key() {
  let mut map: BcidMap = BcidMap::default();
  let legacy: BcidMap = BcidMap::new(BlockedConstraintId::V0(null::<Type>()));
  assert_eq!(map.size(), legacy.size(), "default() 与 new(哨兵) 起步等价");
  assert!(map.is_empty());

  // 真实键：借用栈上活对象的地址伪装为 *const Type（仅按指针哈希/比较，不解引用）。
  let marker: u8 = 0;
  let real = BlockedConstraintId::V0(&marker as *const u8 as *const Type);
  let sentinel = BlockedConstraintId::dense_default();

  map.insert(sentinel.clone(), true);
  map.insert(real.clone(), false);
  assert_eq!(map.size(), 2, "哨兵键与真实键并存，占用由位图判定");
  assert_eq!(map.find(&sentinel), Some(&true), "V0(null) 哨兵键应可命中");
  assert_eq!(map.find(&real), Some(&false));

  map.erase(&sentinel);
  assert!(!map.contains(&sentinel));
  assert!(
    map.find(&real).is_some(),
    "擦除哨兵键不得波及真实键（旧代哨兵比较实现的丢键陷阱）"
  );
  map.erase(&real);
  assert!(map.empty());
}

/// 哨键 `try_insert` 的 fresh 语义与指针键门面测试对齐：首次 fresh、重复不
/// fresh——哨兵参与的是位图占用判定，不触发旧代"键等于哨兵即视为空槽"的误判。
#[test]
fn bcid_map_sentinel_try_insert_fresh_semantics() {
  let mut map: BcidMap = BcidMap::default();
  let sentinel = BlockedConstraintId::dense_default();
  assert!(
    map.try_insert(sentinel.clone(), true).1,
    "首次插入哨兵键应为 fresh"
  );
  assert!(
    !map.try_insert(sentinel.clone(), true).1,
    "重复插入哨兵键不应判 fresh"
  );
  assert_eq!(map.size(), 1);
}
