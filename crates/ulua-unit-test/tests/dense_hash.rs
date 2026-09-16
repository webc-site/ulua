//! Port of `cpp/tests/DenseHash.test.cpp`（555 行，31 个 TEST_CASE）。
//!
//! 被测对象：`ulua_common::records::dense_hash_map::DenseHashMap`、
//! `dense_hash_set::DenseHashSet`（核心为 `dense_hash_table::DenseHashTable`）。
//! 对应 C++ 实现：`cpp/Common/include/Luau/DenseHash.h`。
//!
//! 移植说明：
//! - Rust 版构造保留 `empty_key` 哨兵（C++ 新版用 BitSet 代替哨兵，这是已
//!   记录的接口差异）；整型键统一取 `i32::MIN` 作哨兵，避开测试键 0..N。
//! - C++ 移动构造/移动赋值在 Rust 是所有权转移，源不可再访问，`map_move` /
//!   `set_move` 中"moved-from 后 size==0"断言不可表达，仅保留内容断言。
//! - C++ "迭代中修改容器"用例（引用/迭代器失效检测）在借用规则下改为先
//!   快照再回写，值语义断言与 C++ 一致。
//! - `set_erase_destroys_element_destructor` 等析构计数用例：`shared_ptr`
//!   镜像为 `Option<Arc<_>>`（`None` 即哨兵），`use_count` 对应
//!   `Arc::strong_count`。
//! - `emplace_*` 用例见 `vec_deque.rs`；`CHECK_NOTHROW` 在无异常语义的
//!   Rust 中退化为直接调用。

mod map_insert_and_find {
  //! Node: `cxx:Test:Luau.UnitTest:tests/DenseHash.test.cpp:12:map_insert_and_find`
  //! Source: `tests/DenseHash.test.cpp:12-35`

  #[test]
  fn map_insert_and_find() {
    use ulua_common::records::dense_hash_map::DenseHashMap;

    let mut m: DenseHashMap<i32, i32> = DenseHashMap::new(i32::MIN);

    m.insert(1, 10);
    m.insert(2, 20);
    m.insert(3, 30);

    assert_eq!(m.size(), 3);

    let v1 = m.find(&1);
    let v2 = m.find(&2);
    let v3 = m.find(&3);
    let v4 = m.find(&4);

    assert!(v1.is_some());
    assert!(v2.is_some());
    assert!(v3.is_some());
    assert!(v4.is_none());

    assert_eq!(*v1.unwrap(), 10);
    assert_eq!(*v2.unwrap(), 20);
    assert_eq!(*v3.unwrap(), 30);
  }
}

mod map_overwrite {
  //! Node: `cxx:Test:Luau.UnitTest:tests/DenseHash.test.cpp:37:map_overwrite`
  //! Source: `tests/DenseHash.test.cpp:37-48`

  #[test]
  fn map_overwrite() {
    use ulua_common::records::dense_hash_map::DenseHashMap;

    let mut m: DenseHashMap<i32, i32> = DenseHashMap::new(i32::MIN);

    m.insert(1, 10);
    m.insert(1, 42);

    assert_eq!(m.size(), 1);
    let v = m.find(&1);
    assert!(v.is_some());
    assert_eq!(*v.unwrap(), 42);
  }
}

mod map_contains {
  //! Node: `cxx:Test:Luau.UnitTest:tests/DenseHash.test.cpp:50:map_contains`
  //! Source: `tests/DenseHash.test.cpp:50-58`

  #[test]
  fn map_contains() {
    use ulua_common::records::dense_hash_map::DenseHashMap;

    let mut m: DenseHashMap<i32, i32> = DenseHashMap::new(i32::MIN);

    m.insert(5, 50);

    assert!(m.contains(&5));
    assert!(!m.contains(&6));
  }
}

mod map_try_insert {
  //! Node: `cxx:Test:Luau.UnitTest:tests/DenseHash.test.cpp:60:map_try_insert`
  //! Source: `tests/DenseHash.test.cpp:60-71`

  #[test]
  fn map_try_insert() {
    use ulua_common::records::dense_hash_map::DenseHashMap;

    let mut m: DenseHashMap<i32, i32> = DenseHashMap::new(i32::MIN);

    let (ref1, fresh1) = m.try_insert(1, 10);
    assert!(fresh1);
    assert_eq!(*ref1, 10);

    let (ref2, fresh2) = m.try_insert(1, 99);
    assert!(!fresh2);
    assert_eq!(*ref2, 10);
  }
}

mod map_try_insert_move {
  //! Node: `cxx:Test:Luau.UnitTest:tests/DenseHash.test.cpp:73:map_try_insert_move`
  //! Source: `tests/DenseHash.test.cpp:73-84`

  #[test]
  fn map_try_insert_move() {
    use ulua_common::records::dense_hash_map::DenseHashMap;

    let mut m: DenseHashMap<i32, String> = DenseHashMap::new(i32::MIN);

    let (ref1, fresh1) = m.try_insert(1, String::from("hello"));
    assert!(fresh1);
    assert_eq!(ref1, "hello");

    let (ref2, fresh2) = m.try_insert(1, String::from("world"));
    assert!(!fresh2);
    assert_eq!(ref2, "hello");
  }
}

mod map_clear {
  //! Node: `cxx:Test:Luau.UnitTest:tests/DenseHash.test.cpp:86:map_clear`
  //! Source: `tests/DenseHash.test.cpp:86-98`

  #[test]
  fn map_clear() {
    use ulua_common::records::dense_hash_map::DenseHashMap;

    let mut m: DenseHashMap<i32, i32> = DenseHashMap::new(i32::MIN);

    for i in 0..10 {
      m.insert(i, i * 10);
    }

    assert_eq!(m.size(), 10);
    m.clear();
    assert_eq!(m.size(), 0);
    assert!(m.empty());
    assert!(m.find(&0).is_none());
  }
}

mod map_iteration {
  //! Node: `cxx:Test:Luau.UnitTest:tests/DenseHash.test.cpp:100:map_iteration`
  //! Source: `tests/DenseHash.test.cpp:100-120`

  #[test]
  fn map_iteration() {
    use ulua_common::records::dense_hash_map::DenseHashMap;

    let mut m: DenseHashMap<i32, i32> = DenseHashMap::new(i32::MIN);

    for i in 0..5 {
      m.insert(i, i * 2);
    }

    let mut sum_keys = 0;
    let mut sum_values = 0;
    let mut count = 0;
    for (k, v) in m.iter() {
      sum_keys += k;
      sum_values += v;
      count += 1;
    }

    assert_eq!(count, 5);
    assert_eq!(sum_keys, 1 + 2 + 3 + 4);
    assert_eq!(sum_values, 2 + 4 + 6 + 8);
  }
}

mod map_rehash {
  //! Node: `cxx:Test:Luau.UnitTest:tests/DenseHash.test.cpp:122:map_rehash`
  //! Source: `tests/DenseHash.test.cpp:122-137`

  #[test]
  fn map_rehash() {
    use ulua_common::records::dense_hash_map::DenseHashMap;

    let mut m: DenseHashMap<i32, i32> = DenseHashMap::new(i32::MIN);

    for i in 0..100 {
      m.insert(i, i);
    }

    assert_eq!(m.size(), 100);

    for i in 0..100 {
      let v = m.find(&i);
      assert!(v.is_some());
      assert_eq!(*v.unwrap(), i);
    }
  }
}

mod map_copy {
  //! Node: `cxx:Test:Luau.UnitTest:tests/DenseHash.test.cpp:139:map_copy`
  //! Source: `tests/DenseHash.test.cpp:139-159`

  #[test]
  fn map_copy() {
    use ulua_common::records::dense_hash_map::DenseHashMap;

    let mut m: DenseHashMap<i32, i32> = DenseHashMap::new(i32::MIN);

    for i in 0..10 {
      m.insert(i, i * 3);
    }

    let mut copy = m.clone();

    assert_eq!(copy.size(), 10);
    for i in 0..10 {
      let v = copy.find(&i);
      assert!(v.is_some());
      assert_eq!(*v.unwrap(), i * 3);
    }

    // Mutating the copy doesn't affect the original
    copy.insert(0, 999);
    assert_eq!(*m.find(&0).unwrap(), 0);
  }
}

mod map_move {
  //! Node: `cxx:Test:Luau.UnitTest:tests/DenseHash.test.cpp:161:map_move`
  //! Source: `tests/DenseHash.test.cpp:161-179`
  //!
  //! C++ 移动构造后断言源 `m.size() == 0`；Rust 所有权转移后源不可访问，
  //! 该断言不可表达，保留移动目标的内容断言。

  #[test]
  fn map_move() {
    use ulua_common::records::dense_hash_map::DenseHashMap;

    let mut m: DenseHashMap<i32, i32> = DenseHashMap::new(i32::MIN);

    for i in 0..10 {
      m.insert(i, i);
    }

    let moved = m;

    assert_eq!(moved.size(), 10);
    for i in 0..10 {
      let v = moved.find(&i);
      assert!(v.is_some());
      assert_eq!(*v.unwrap(), i);
    }
  }
}

mod map_overwrite_when_full_shouldnt_rehash {
  //! Node: `cxx:Test:Luau.UnitTest:tests/DenseHash.test.cpp:181:map_overwrite_when_full_shouldnt_rehash`
  //! Source: `tests/DenseHash.test.cpp:181-198`
  //!
  //! C++ 在迭代中回写（验证满表 overwrite 不触发 rehash、引用不失效）；
  //! Rust 借用规则下先快照 (k, v) 再回写，值语义断言一致。

  #[test]
  fn map_overwrite_when_full_shouldnt_rehash() {
    use ulua_common::records::dense_hash_map::DenseHashMap;

    let mut m: DenseHashMap<i32, i32> = DenseHashMap::new(i32::MIN);
    for i in 0..12 {
      m.insert(i, i);
    }

    assert_eq!(m.size(), 12);

    let updates: Vec<(i32, i32)> = m.iter().map(|(&k, &a)| (k, a + 1)).collect();
    for (k, a) in updates {
      m.insert(k, a);
    }

    for i in 0..m.size() {
      let a = m.find(&(i as i32));
      assert!(a.is_some());
      assert_eq!(i + 1, *a.unwrap() as usize);
    }
  }
}

mod map_merge_with_rehash_while_iterating {
  //! Node: `cxx:Test:Luau.UnitTest:tests/DenseHash.test.cpp:200:map_merge_with_rehash_while_iterating`
  //! Source: `tests/DenseHash.test.cpp:200-235`
  //!
  //! 迭代中修改改为快照后回写（见 `map_overwrite_when_full_shouldnt_rehash`）。

  #[test]
  fn map_merge_with_rehash_while_iterating() {
    use ulua_common::records::dense_hash_map::DenseHashMap;

    let mut m1: DenseHashMap<i32, i32> = DenseHashMap::new(i32::MIN);
    for i in 0..12 {
      m1.insert(i, i);
    }

    let mut m2: DenseHashMap<i32, i32> = DenseHashMap::new(i32::MIN);
    for i in 8..24 {
      m2.insert(i, i);
    }

    assert_eq!(m1.size(), 12);
    assert_eq!(m2.size(), 16);

    let merges: Vec<(i32, i32)> = m1
      .iter()
      .filter_map(|(&k, &a)| m2.find(&k).map(|&b| (k, a + b)))
      .collect();
    for (k, v) in merges {
      m1.insert(k, v);
    }

    let additions: Vec<(i32, i32)> = m2
      .iter()
      .filter(|item| !m1.contains(item.0))
      .map(|(&k, &a)| (k, a))
      .collect();
    for (k, v) in additions {
      m1.insert(k, v);
    }

    assert_eq!(m1.size(), 24);
    for i in 0..m1.size() {
      let a = m1.find(&(i as i32));
      assert!(a.is_some());
      let a = *a.unwrap() as usize;
      if !(8..12).contains(&i) {
        assert_eq!(i, a);
      } else {
        assert_eq!(i + i, a);
      }
    }
  }
}

mod map_string_keys {
  //! Node: `cxx:Test:Luau.UnitTest:tests/DenseHash.test.cpp:237:map_string_keys`
  //! Source: `tests/DenseHash.test.cpp:237-250`

  #[test]
  fn map_string_keys() {
    use ulua_common::records::dense_hash_map::DenseHashMap;

    let mut m: DenseHashMap<String, i32> = DenseHashMap::new(String::new());

    m.insert(String::from("hello"), 1);
    m.insert(String::from("world"), 2);
    m.insert(String::from("test"), 3);

    assert_eq!(m.size(), 3);
    assert_eq!(*m.find(&String::from("hello")).unwrap(), 1);
    assert_eq!(*m.find(&String::from("world")).unwrap(), 2);
    assert_eq!(*m.find(&String::from("test")).unwrap(), 3);
    assert!(m.find(&String::from("missing")).is_none());
  }
}

mod set_insert_and_find {
  //! Node: `cxx:Test:Luau.UnitTest:tests/DenseHash.test.cpp:252:set_insert_and_find`
  //! Source: `tests/DenseHash.test.cpp:252-265`

  #[test]
  fn set_insert_and_find() {
    use ulua_common::records::dense_hash_set::DenseHashSet;

    let mut s: DenseHashSet<i32> = DenseHashSet::new(i32::MIN);

    s.insert(1);
    s.insert(2);
    s.insert(3);

    assert_eq!(s.size(), 3);
    assert!(s.contains(&1));
    assert!(s.contains(&2));
    assert!(s.contains(&3));
    assert!(!s.contains(&4));
  }
}

mod set_duplicate_insert {
  //! Node: `cxx:Test:Luau.UnitTest:tests/DenseHash.test.cpp:267:set_duplicate_insert`
  //! Source: `tests/DenseHash.test.cpp:267-277`

  #[test]
  fn set_duplicate_insert() {
    use ulua_common::records::dense_hash_set::DenseHashSet;

    let mut s: DenseHashSet<i32> = DenseHashSet::new(i32::MIN);

    s.insert(1);
    s.insert(1);
    s.insert(1);

    assert_eq!(s.size(), 1);
    assert!(s.contains(&1));
  }
}

mod set_try_insert {
  //! Node: `cxx:Test:Luau.UnitTest:tests/DenseHash.test.cpp:279:set_try_insert`
  //! Source: `tests/DenseHash.test.cpp:279-286`

  #[test]
  fn set_try_insert() {
    use ulua_common::records::dense_hash_set::DenseHashSet;

    let mut s: DenseHashSet<i32> = DenseHashSet::new(i32::MIN);

    assert!(s.try_insert(1));
    assert!(!s.try_insert(1));
    assert_eq!(s.size(), 1);
  }
}

mod set_clear {
  //! Node: `cxx:Test:Luau.UnitTest:tests/DenseHash.test.cpp:288:set_clear`
  //! Source: `tests/DenseHash.test.cpp:288-300`

  #[test]
  fn set_clear() {
    use ulua_common::records::dense_hash_set::DenseHashSet;

    let mut s: DenseHashSet<i32> = DenseHashSet::new(i32::MIN);

    for i in 0..10 {
      s.insert(i);
    }

    assert_eq!(s.size(), 10);
    s.clear();
    assert_eq!(s.size(), 0);
    assert!(s.empty());
    assert!(!s.contains(&0));
  }
}

mod set_iteration {
  //! Node: `cxx:Test:Luau.UnitTest:tests/DenseHash.test.cpp:302:set_iteration`
  //! Source: `tests/DenseHash.test.cpp:302-319`

  #[test]
  fn set_iteration() {
    use ulua_common::records::dense_hash_set::DenseHashSet;

    let mut s: DenseHashSet<i32> = DenseHashSet::new(i32::MIN);

    for i in 0..5 {
      s.insert(i);
    }

    let mut sum = 0;
    let mut count = 0;
    for k in s.iter() {
      sum += k;
      count += 1;
    }

    assert_eq!(count, 5);
    assert_eq!(sum, 1 + 2 + 3 + 4);
  }
}

mod set_rehash {
  //! Node: `cxx:Test:Luau.UnitTest:tests/DenseHash.test.cpp:321:set_rehash`
  //! Source: `tests/DenseHash.test.cpp:321-334`

  #[test]
  fn set_rehash() {
    use ulua_common::records::dense_hash_set::DenseHashSet;

    let mut s: DenseHashSet<i32> = DenseHashSet::new(i32::MIN);

    for i in 0..100 {
      s.insert(i);
    }

    assert_eq!(s.size(), 100);

    for i in 0..100 {
      assert!(s.contains(&i));
    }

    assert!(!s.contains(&100));
  }
}

mod set_copy {
  //! Node: `cxx:Test:Luau.UnitTest:tests/DenseHash.test.cpp:336:set_copy`
  //! Source: `tests/DenseHash.test.cpp:336-348`

  #[test]
  fn set_copy() {
    use ulua_common::records::dense_hash_set::DenseHashSet;

    let mut s: DenseHashSet<i32> = DenseHashSet::new(i32::MIN);

    for i in 0..10 {
      s.insert(i);
    }

    let copy = s.clone();

    assert_eq!(copy.size(), 10);
    for i in 0..10 {
      assert!(copy.contains(&i));
    }
  }
}

mod set_move {
  //! Node: `cxx:Test:Luau.UnitTest:tests/DenseHash.test.cpp:350:set_move`
  //! Source: `tests/DenseHash.test.cpp:350-364`
  //!
  //! 源容器断言不可表达（同 `map_move`）。

  #[test]
  fn set_move() {
    use ulua_common::records::dense_hash_set::DenseHashSet;

    let mut s: DenseHashSet<i32> = DenseHashSet::new(i32::MIN);

    for i in 0..10 {
      s.insert(i);
    }

    let moved = s;

    assert_eq!(moved.size(), 10);
    for i in 0..10 {
      assert!(moved.contains(&i));
    }
  }
}

mod set_equality {
  //! Node: `cxx:Test:Luau.UnitTest:tests/DenseHash.test.cpp:366:set_equality`
  //! Source: `tests/DenseHash.test.cpp:366-381`

  #[test]
  fn set_equality() {
    use ulua_common::records::dense_hash_set::DenseHashSet;

    let mut a: DenseHashSet<i32> = DenseHashSet::new(i32::MIN);
    let mut b: DenseHashSet<i32> = DenseHashSet::new(i32::MIN);

    for i in 0..5 {
      a.insert(i);
      b.insert(4 - i);
    }

    assert_eq!(a, b);

    b.insert(99);
    assert_ne!(a, b);
  }
}

mod set_pointer_keys {
  //! Node: `cxx:Test:Luau.UnitTest:tests/DenseHash.test.cpp:383:set_pointer_keys`
  //! Source: `tests/DenseHash.test.cpp:383-405`
  //!
  //! C++ `Test*` 键镜像为裸指针 `*const Test`；`shared_ptr` 由 `Arc` 持有保活。

  struct Test {
    #[expect(dead_code)]
    a: i32,
  }

  #[test]
  fn set_pointer_keys() {
    use std::{ptr::null, sync::Arc};

    use ulua_common::records::dense_hash_set::DenseHashSet;

    let ta = Arc::new(Test { a: 1 });
    let tb = Arc::new(Test { a: 2 });

    let mut s: DenseHashSet<*const Test> = DenseHashSet::new(null());
    s.insert(Arc::as_ptr(&ta));
    s.insert(Arc::as_ptr(&tb));

    assert_eq!(s.size(), 2);
    assert!(s.contains(&Arc::as_ptr(&ta)));
    assert!(s.contains(&Arc::as_ptr(&tb)));
    assert!(!s.contains(&null()));
  }
}

mod map_pointer_keys {
  //! Node: `cxx:Test:Luau.UnitTest:tests/DenseHash.test.cpp:407:map_pointer_keys`
  //! Source: `tests/DenseHash.test.cpp:407-429`

  struct Test {
    #[expect(dead_code)]
    a: i32,
  }

  #[test]
  fn map_pointer_keys() {
    use std::{ptr::null, sync::Arc};

    use ulua_common::records::dense_hash_map::DenseHashMap;

    let ta = Arc::new(Test { a: 1 });
    let tb = Arc::new(Test { a: 2 });

    let mut m: DenseHashMap<*const Test, i32> = DenseHashMap::new(null());
    m.insert(Arc::as_ptr(&ta), 1);
    m.insert(Arc::as_ptr(&tb), 2);

    assert_eq!(m.size(), 2);
    assert_eq!(*m.find(&Arc::as_ptr(&ta)).unwrap(), 1);
    assert_eq!(*m.find(&Arc::as_ptr(&tb)).unwrap(), 2);
    assert!(m.find(&null()).is_none());
  }
}

mod map_erase {
  //! Node: `cxx:Test:Luau.UnitTest:tests/DenseHash.test.cpp:431:map_erase`
  //! Source: `tests/DenseHash.test.cpp:431-454`

  #[test]
  fn map_erase() {
    use ulua_common::records::dense_hash_map::DenseHashMap;

    let mut m: DenseHashMap<i32, i32> = DenseHashMap::new(i32::MIN);

    for i in 0..10 {
      m.insert(i, i * 10);
    }

    assert_eq!(m.size(), 10);

    m.erase(&5);

    assert_eq!(m.size(), 9);
    assert!(m.find(&5).is_none());

    // Other elements are still accessible
    for i in 0..10 {
      if i == 5 {
        continue;
      }
      let v = m.find(&i);
      assert!(v.is_some());
      assert_eq!(*v.unwrap(), i * 10);
    }
  }
}

mod map_erase_nonexistent {
  //! Node: `cxx:Test:Luau.UnitTest:tests/DenseHash.test.cpp:456:map_erase_nonexistent`
  //! Source: `tests/DenseHash.test.cpp:456-468`

  #[test]
  fn map_erase_nonexistent() {
    use ulua_common::records::dense_hash_map::DenseHashMap;

    let mut m: DenseHashMap<i32, i32> = DenseHashMap::new(i32::MIN);

    m.insert(1, 10);
    m.insert(2, 20);

    m.erase(&99);

    assert_eq!(m.size(), 2);
    assert_eq!(*m.find(&1).unwrap(), 10);
    assert_eq!(*m.find(&2).unwrap(), 20);
  }
}

mod map_erase_and_reinsert {
  //! Node: `cxx:Test:Luau.UnitTest:tests/DenseHash.test.cpp:470:map_erase_and_reinsert`
  //! Source: `tests/DenseHash.test.cpp:470-484`

  #[test]
  fn map_erase_and_reinsert() {
    use ulua_common::records::dense_hash_map::DenseHashMap;

    let mut m: DenseHashMap<i32, i32> = DenseHashMap::new(i32::MIN);

    m.insert(1, 10);
    m.insert(2, 20);
    m.insert(3, 30);

    m.erase(&2);
    assert!(m.find(&2).is_none());

    m.insert(2, 99);
    assert_eq!(*m.find(&2).unwrap(), 99);
    assert_eq!(m.size(), 3);
  }
}

mod map_erase_chain {
  //! Node: `cxx:Test:Luau.UnitTest:tests/DenseHash.test.cpp:486:map_erase_chain`
  //! Source: `tests/DenseHash.test.cpp:486-511`

  #[test]
  fn map_erase_chain() {
    use ulua_common::records::dense_hash_map::DenseHashMap;

    // Insert elements that will form a probe chain, then erase from the middle
    let mut m: DenseHashMap<i32, i32> = DenseHashMap::new(i32::MIN);

    for i in 0..50 {
      m.insert(i, i);
    }

    // Erase every other element
    for i in (0..50).step_by(2) {
      m.erase(&i);
    }

    assert_eq!(m.size(), 25);

    // Odd elements should still be findable
    for i in (1..50).step_by(2) {
      let v = m.find(&i);
      assert!(v.is_some());
      assert_eq!(*v.unwrap(), i);
    }

    // Even elements should be gone
    for i in (0..50).step_by(2) {
      assert!(m.find(&i).is_none());
    }
  }
}

mod set_erase_destroys_element_destructor {
  //! Node: `cxx:Test:Luau.UnitTest:tests/DenseHash.test.cpp:513:set_erase_destroys_element_destructor`
  //! Source: `tests/DenseHash.test.cpp:513-529`
  //!
  //! 被删元素位于探测链末端时 erase 只清空该槽；若槽内元素未析构，
  //! `shared_ptr` 引用计数会虚高。`shared_ptr` 镜像为 `Option<Arc<_>>`。

  #[test]
  fn set_erase_destroys_element_destructor() {
    use std::sync::Arc;

    use ulua_common::records::dense_hash_set::DenseHashSet;

    let p = Arc::new(42);
    assert_eq!(Arc::strong_count(&p), 1);

    {
      let mut s: DenseHashSet<Option<Arc<i32>>> = DenseHashSet::new(None);
      s.insert(Some(p.clone()));
      assert_eq!(Arc::strong_count(&p), 2);

      s.erase(&Some(p.clone()));
      assert_eq!(Arc::strong_count(&p), 1);
    }
  }
}

mod map_erase_destroys_value_destructor {
  //! Node: `cxx:Test:Luau.UnitTest:tests/DenseHash.test.cpp:531:map_erase_destroys_value_destructor`
  //! Source: `tests/DenseHash.test.cpp:531-544`

  #[test]
  fn map_erase_destroys_value_destructor() {
    use std::sync::Arc;

    use ulua_common::records::dense_hash_map::DenseHashMap;

    let p = Arc::new(42);
    assert_eq!(Arc::strong_count(&p), 1);

    {
      let mut m: DenseHashMap<i32, Option<Arc<i32>>> = DenseHashMap::new(i32::MIN);
      m.insert(1, Some(p.clone()));
      assert_eq!(Arc::strong_count(&p), 2);

      m.erase(&1);
      assert_eq!(Arc::strong_count(&p), 1);
    }
  }
}

mod map_usable_with_luau_notnull {
  //! Node: `cxx:Test:Luau.UnitTest:tests/DenseHash.test.cpp:546:map_usable_with_Luau_notnull`
  //! Source: `tests/DenseHash.test.cpp:546-553`
  //!
  //! 哨兵需要一个永不入表的 `NotNull`：指向独立的栈变量 `sentinel_storage`，
  //! 与真实键 `x` 地址必然不同。

  #[test]
  fn map_usable_with_luau_notnull() {
    use ulua_analysis::records::not_null::NotNull;
    use ulua_common::records::dense_hash_map::DenseHashMap;

    let mut sentinel_storage: i32 = 0;
    let mut map: DenseHashMap<NotNull<i32>, i32> =
      DenseHashMap::new(NotNull::new(&mut sentinel_storage));
    let mut x = 1;
    let i = NotNull::new(&mut x);
    map.insert(i, x);
    assert!(map.contains(&i));
  }
}
