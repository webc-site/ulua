//! 统一哈希容器别名：全工作区单一定义在 `ulua_common::collections`
//! （hashbrown + foldhash `fast::FixedState`，固定种子 BuildHasher）。
//!
//! 此处仅做 re-export，保持 `crate::type_aliases::collections::*` 调用点不变；
//! 不再在本 crate 内另立 std 容器别名，避免双重定义漂移。

pub use ulua_common::collections::{
  DefaultBuildHasher, DeterministicHashMap, DeterministicHashSet, HashMap, HashSet, fast_hash,
};
