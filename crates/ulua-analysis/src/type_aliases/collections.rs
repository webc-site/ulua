//! 统一哈希容器别名：std 容器换用 foldhash `FixedState`（固定种子 BuildHasher）。
//!
//! 固定种子使迭代序跨进程稳定；哈希质量与吞吐优于 SipHash（std `RandomState`）。

use std::collections::{HashMap as StdHashMap, HashSet as StdHashSet};

use foldhash::fast::FixedState;

pub type HashMap<K, V> = StdHashMap<K, V, FixedState>;

pub type HashSet<T> = StdHashSet<T, FixedState>;

pub use foldhash::{HashMapExt, HashSetExt};
