//! `DenseHashDefault` 的 foldhash 快版替身 —— **仅供只查不迭代的热表使用**。
//!
//! 与 [`DenseHashDefault`](crate::type_aliases::dense_hash_default::DenseHashDefault)
//! （逐字节 FNV-1a）的唯一差别是哈希函数换成
//! `foldhash::fast` 的固定种子版：按 8/16 字节块折叠混合，取代 FNV 每字节一次
//! 乘法的串行链，长键（如 `TableShape` 的 2×32 个 i32）提速数倍。
//!
//! 固定种子保证哈希跨进程、跨运行稳定。但 `DenseHashMap` 的迭代序 = 桶序 =
//! 哈希序，换哈希器必然改变迭代顺序，因此本类型**不得**用于任何依赖迭代序的
//! 表（例如 `BytecodeBuilder::string_table` 会被遍历、`Config::aliases` 会在
//! `copy_from` 里被遍历）——这类表继续用
//! [`DenseHashDefault`](crate::type_aliases::dense_hash_default::DenseHashDefault)。

use core::{
  hash::{BuildHasher, Hash},
  marker::PhantomData,
};

use foldhash::fast::FixedState;

use crate::records::dense_hash_table::DenseHasher;

/// 固定种子哈希器状态：`with_seed(0)` 即 crate 默认种子，编译期常量。
const FIXED_STATE: FixedState = FixedState::with_seed(0);

/// 只查不迭代热表的默认键哈希器（foldhash fast，固定种子）。
#[derive(Clone, Copy)]
pub struct DenseHashFast<K>(PhantomData<K>);

// 手写 impl：derive 会强加 `K: Default`，而 PhantomData 无需任何约束
//（键可能是裸指针或 `String`）。
impl<K> Default for DenseHashFast<K> {
  fn default() -> Self {
    DenseHashFast(PhantomData)
  }
}

impl<K: Hash> DenseHasher<K> for DenseHashFast<K> {
  fn hash(&self, key: &K) -> usize {
    FIXED_STATE.hash_one(key) as usize
  }
}
