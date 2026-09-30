//! 测试 ulua-common 集合门面（hashbrown DefaultHashBuilder 与 Deterministic 别名）。

use foldhash::fast::FixedState;
use ulua_common::collections::{
  DefaultBuildHasher, DeterministicHashMap, DeterministicHashSet, HashMap, HashSet, fast_hash,
};

#[test]
fn hash_map_and_set_inherent_constructors() {
  // 原生 hashbrown 提供 inherent new() 与 with_capacity()，哈希构建器为 DefaultBuildHasher
  let _hasher = DefaultBuildHasher::default();
  let mut map: HashMap<String, i32> = HashMap::new();
  assert!(map.is_empty());
  map.insert("alpha".to_string(), 1);
  assert_eq!(map.get("alpha"), Some(&1));

  let mut map_cap: HashMap<i32, i32> = HashMap::with_capacity(32);
  assert!(map_cap.capacity() >= 32);
  map_cap.insert(10, 20);
  assert_eq!(map_cap.get(&10), Some(&20));

  let mut set: HashSet<String> = HashSet::new();
  assert!(set.is_empty());
  set.insert("beta".to_string());
  assert!(set.contains("beta"));

  let mut set_cap: HashSet<i32> = HashSet::with_capacity(32);
  assert!(set_cap.capacity() >= 32);
  set_cap.insert(42);
  assert!(set_cap.contains(&42));
}

#[test]
fn deterministic_map_and_set() {
  let mut det_map: DeterministicHashMap<u32, &'static str> =
    DeterministicHashMap::with_hasher(FixedState::default());
  det_map.insert(1, "one");
  det_map.insert(2, "two");
  assert_eq!(det_map.get(&1), Some(&"one"));
  assert_eq!(det_map.len(), 2);

  let mut det_map_default: DeterministicHashMap<u32, &'static str> =
    DeterministicHashMap::default();
  det_map_default.insert(3, "three");
  assert_eq!(det_map_default.get(&3), Some(&"three"));

  let mut det_set: DeterministicHashSet<u32> =
    DeterministicHashSet::with_hasher(FixedState::default());
  det_set.insert(100);
  assert!(det_set.contains(&100));

  let mut det_set_default: DeterministicHashSet<u32> = DeterministicHashSet::default();
  det_set_default.insert(200);
  assert!(det_set_default.contains(&200));
}

#[test]
fn fast_hash_consistency() {
  let val1 = "luau_rocks";
  let val2 = "luau_rocks";
  assert_eq!(fast_hash(&val1), fast_hash(&val2));
  assert_ne!(fast_hash(&"foo"), fast_hash(&"bar"));
}
