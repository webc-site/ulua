//! Port of `cpp/tests/InsertionOrderedMap.test.cpp`（140 行，7 个 TEST_CASE）。
//!
//! 被测对象：`ulua_common::records::insertion_ordered_map::InsertionOrderedMap`。
//! 对应 C++ 实现：`cpp/Common/include/Luau/InsertionOrderedMap.h`。
//!
//! 移植说明：
//! - C++ `MapFixture::makePtr` 返回 `unique_ptr` 托管指针，镜像为
//!   `Vec<Box<i32>>` 托管 + 裸指针键（地址恒等、生命周期由 fixture 保证）。
//! - C++ `erase(map.find(a))`（按迭代器删）对应 Rust `erase(&a)`（按键删，
//!   内部查位），语义一致。

use std::collections::VecDeque;

/// C++ `MapFixture`：托管一批 `int` 对象，键取其地址。`Box` 保证元素地址
/// 在后续 `make_ptr` 后保持稳定（`Vec` 扩容会搬移元素），`VecDeque` 承载
/// 增长的托管列表（clippy 禁用 `Vec<Box<_>>`）。
struct MapFixture {
  ptrs: VecDeque<Box<i32>>,
}

impl MapFixture {
  fn new() -> Self {
    MapFixture {
      ptrs: VecDeque::new(),
    }
  }

  fn make_ptr(&mut self) -> *const i32 {
    self.ptrs.push_back(Box::new(0));
    let ptr: *const i32 = &**self.ptrs.back().unwrap();
    ptr
  }
}

mod map_insertion {
  //! Node: `cxx:Test:Luau.UnitTest:tests/InsertionOrderedMap.test.cpp:23:map_insertion`
  //! Source: `tests/InsertionOrderedMap.test.cpp:23-32`

  #[test]
  fn map_insertion() {
    use ulua_common::records::insertion_ordered_map::InsertionOrderedMap;

    use super::MapFixture;

    let mut fixture = MapFixture::new();
    let mut map: InsertionOrderedMap<*const i32, i32> = InsertionOrderedMap::new();

    let a = fixture.make_ptr();
    let b = fixture.make_ptr();

    map.insert(a, 1);
    map.insert(b, 2);
  }
}

mod map_lookup {
  //! Node: `cxx:Test:Luau.UnitTest:tests/InsertionOrderedMap.test.cpp:34:map_lookup`
  //! Source: `tests/InsertionOrderedMap.test.cpp:34-47`

  #[test]
  fn map_lookup() {
    use ulua_common::records::insertion_ordered_map::InsertionOrderedMap;

    use super::MapFixture;

    let mut fixture = MapFixture::new();
    let mut map: InsertionOrderedMap<*const i32, i32> = InsertionOrderedMap::new();

    let a = fixture.make_ptr();
    map.insert(a, 1);

    let r = map.get(&a);
    assert!(r.is_some());
    assert_eq!(*r.unwrap(), 1);

    let r = map.get(&fixture.make_ptr());
    assert!(r.is_none());
  }
}

mod insert_does_not_update {
  //! Node: `cxx:Test:Luau.UnitTest:tests/InsertionOrderedMap.test.cpp:49:insert_does_not_update`
  //! Source: `tests/InsertionOrderedMap.test.cpp:49-60`

  #[test]
  fn insert_does_not_update() {
    use ulua_common::records::insertion_ordered_map::InsertionOrderedMap;

    use super::MapFixture;

    let mut fixture = MapFixture::new();
    let mut map: InsertionOrderedMap<*const i32, i32> = InsertionOrderedMap::new();

    let k = fixture.make_ptr();
    map.insert(k, 1);
    map.insert(k, 2);

    let v = map.get(&k);
    assert!(v.is_some());
    assert_eq!(*v.unwrap(), 1);
  }
}

mod insertion_order_is_iteration_order {
  //! Node: `cxx:Test:Luau.UnitTest:tests/InsertionOrderedMap.test.cpp:62:insertion_order_is_iteration_order`
  //! Source: `tests/InsertionOrderedMap.test.cpp:62-93`

  #[test]
  fn insertion_order_is_iteration_order() {
    use ulua_common::records::insertion_ordered_map::InsertionOrderedMap;

    use super::MapFixture;

    // This one is a little hard to prove, in that if the ordering guarantees
    // fail this test isn't guaranteed to fail, but it is strictly better than
    // nothing.

    let mut fixture = MapFixture::new();
    let mut map: InsertionOrderedMap<*const i32, i32> = InsertionOrderedMap::new();
    let a = fixture.make_ptr();
    let b = fixture.make_ptr();
    let c = fixture.make_ptr();
    map.insert(a, 1);
    map.insert(b, 1);
    map.insert(c, 1);

    let mut it = map.iter();

    let item = it.next();
    assert!(item.is_some());
    assert_eq!(*item.unwrap().0, a);
    assert_eq!(*item.unwrap().1, 1);

    let item = it.next();
    assert!(item.is_some());
    assert_eq!(*item.unwrap().0, b);
    assert_eq!(*item.unwrap().1, 1);

    let item = it.next();
    assert!(item.is_some());
    assert_eq!(*item.unwrap().0, c);
    assert_eq!(*item.unwrap().1, 1);

    assert!(it.next().is_none());
  }
}

mod destructuring_iterator_compiles {
  //! Node: `cxx:Test:Luau.UnitTest:tests/InsertionOrderedMap.test.cpp:95:destructuring_iterator_compiles`
  //! Source: `tests/InsertionOrderedMap.test.cpp:95-106`
  //!
  //! 本用例只验证能编译、空迭代不执行。

  #[test]
  fn destructuring_iterator_compiles() {
    use ulua_common::records::insertion_ordered_map::InsertionOrderedMap;

    use super::MapFixture;

    let mut fixture = MapFixture::new();
    let map: InsertionOrderedMap<*const i32, i32> = InsertionOrderedMap::new();
    let _ = fixture.make_ptr();

    for (k, v) in map.iter() {
      // Checks here solely to silence unused variable warnings.
      assert!(!k.is_null());
      assert!(*v > 0);
    }
  }
}

mod map_erasure {
  //! Node: `cxx:Test:Luau.UnitTest:tests/InsertionOrderedMap.test.cpp:108:map_erasure`
  //! Source: `tests/InsertionOrderedMap.test.cpp:108-125`

  #[test]
  fn map_erasure() {
    use ulua_common::records::insertion_ordered_map::InsertionOrderedMap;

    use super::MapFixture;

    let mut fixture = MapFixture::new();
    let mut map: InsertionOrderedMap<*const i32, i32> = InsertionOrderedMap::new();

    let a = fixture.make_ptr();
    let b = fixture.make_ptr();

    map.insert(a, 1);
    map.insert(b, 2);

    // C++ `erase(map.find(a))`：按 `a` 定位删除。
    map.erase(&a);
    assert_eq!(map.size(), 1);
    assert!(!map.contains(&a));
    assert!(map.get(&a).is_none());

    let v = map.get(&b);
    assert!(v.is_some());
  }
}

mod map_clear {
  //! Node: `cxx:Test:Luau.UnitTest:tests/InsertionOrderedMap.test.cpp:127:map_clear`
  //! Source: `tests/InsertionOrderedMap.test.cpp:127-138`

  #[test]
  fn map_clear() {
    use ulua_common::records::insertion_ordered_map::InsertionOrderedMap;

    use super::MapFixture;

    let mut fixture = MapFixture::new();
    let mut map: InsertionOrderedMap<*const i32, i32> = InsertionOrderedMap::new();
    let a = fixture.make_ptr();

    map.insert(a, 1);

    map.clear();
    assert_eq!(map.size(), 0);
    assert!(!map.contains(&a));
    assert!(map.get(&a).is_none());
  }
}
