//! 高性能通用集合与哈希模块。
//!
//! 底层采用 `hashbrown`（SwissTable 高吞吐实现），已激活原生 `default-hasher`（由 `foldhash` 驱动）。
//! 默认哈希器 `hashbrown::DefaultHashBuilder` 提供高吞吐哈希；
//! 针对需要跨进程严格确定迭代序的场景，提供固定种子的 `DeterministicHashMap` / `DeterministicHashSet`。

use std::hash::{BuildHasher, Hash};

use foldhash::fast::FixedState;

pub type DefaultBuildHasher = hashbrown::DefaultHashBuilder;

/// 用工作区统一哈希器（foldhash `fast::FixedState`，固定种子）一次性算 usize 键哈希。
///
/// 取代各 crate 手搓 `std` `DefaultHasher`（SipHash）的热路径键哈希：哈希值仅需
/// 同一运行内对同一键一致（DenseHashMap/DenseHashSet 契约），不持久化、不跨进程
/// 比较，故换哈希函数安全。独立哈希逻辑只准出现在此处（一处定义）。
#[inline]
pub fn fast_hash<T: Hash + ?Sized>(value: &T) -> usize {
  FixedState::default().hash_one(value) as usize
}

/// 高性能哈希表（暴露统一的 hashbrown 默认哈希器）
pub type HashMap<K, V, S = hashbrown::DefaultHashBuilder> = hashbrown::HashMap<K, V, S>;

/// 高性能哈希集合（暴露统一的 hashbrown 默认哈希器）
pub type HashSet<T, S = hashbrown::DefaultHashBuilder> = hashbrown::HashSet<T, S>;

/// 确定性哈希表（带固定种子，供需要严格跨进程确定序的场景）
pub type DeterministicHashMap<K, V> = hashbrown::HashMap<K, V, FixedState>;

/// 确定性哈希集合（带固定种子，供需要严格跨进程确定序的场景）
pub type DeterministicHashSet<T> = hashbrown::HashSet<T, FixedState>;
