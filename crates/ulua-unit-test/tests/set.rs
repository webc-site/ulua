extern crate alloc;

mod set_clear_resets_size {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Set.test.cpp:34:set_clear_resets_size`
  //! Source: `tests/Set.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Set.test.cpp
  //! - source_includes:
  //!   - includes -> source_file tests/ScopedFlags.h
  //!   - includes -> source_file Analysis/include/Luau/Set.h
  //! - incoming:
  //!   - declares <- source_file tests/Set.test.cpp
  //! - outgoing:
  //!   - translates_to -> rust_item set_clear_resets_size

  #[cfg(test)]
  #[test]
  fn set_clear_resets_size() {
    use ulua_analysis::records::set::Set;

    let mut s1 = Set::<i32>::new(0);
    s1.insert(&1);
    s1.insert(&2);
    assert_eq!(s1.size(), 2);

    s1.clear();
    assert_eq!(s1.size(), 0);
    assert!(s1.empty());
  }
}

mod set_empty_set_size_0 {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Set.test.cpp:12:set_empty_set_size_0`
  //! Source: `tests/Set.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Set.test.cpp
  //! - source_includes:
  //!   - includes -> source_file tests/ScopedFlags.h
  //!   - includes -> source_file Analysis/include/Luau/Set.h
  //! - incoming:
  //!   - declares <- source_file tests/Set.test.cpp
  //! - outgoing:
  //!   - translates_to -> rust_item set_empty_set_size_0

  #[cfg(test)]
  #[test]
  fn set_empty_set_size_0() {
    use ulua_analysis::records::set::Set;

    let s1 = Set::<i32>::new(0);
    assert_eq!(s1.size(), 0);
    assert!(s1.empty());
  }
}

mod set_erase_using_const_ref_argument {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Set.test.cpp:132:set_erase_using_const_ref_argument`
  //! Source: `tests/Set.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Set.test.cpp
  //! - source_includes:
  //!   - includes -> source_file tests/ScopedFlags.h
  //!   - includes -> source_file Analysis/include/Luau/Set.h
  //! - incoming:
  //!   - declares <- source_file tests/Set.test.cpp
  //! - outgoing:
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item set_erase_using_const_ref_argument

  #[cfg(test)]
  #[test]
  fn set_erase_using_const_ref_argument() {
    use alloc::string::String;

    use ulua_analysis::records::set::Set;

    let mut s1 = Set::<String>::new(String::new());
    s1.insert(&String::from("x"));
    s1.insert(&String::from("y"));

    let key = String::from("y");
    s1.erase(&key);

    assert!(s1.count(&String::from("x")) != 0);
    assert_eq!(s1.count(&String::from("y")), 0);
  }
}

mod set_erase_works_and_decreases_size {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Set.test.cpp:46:set_erase_works_and_decreases_size`
  //! Source: `tests/Set.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Set.test.cpp
  //! - source_includes:
  //!   - includes -> source_file tests/ScopedFlags.h
  //!   - includes -> source_file Analysis/include/Luau/Set.h
  //! - incoming:
  //!   - declares <- source_file tests/Set.test.cpp
  //! - outgoing:
  //!   - translates_to -> rust_item set_erase_works_and_decreases_size

  #[cfg(test)]
  #[test]
  fn set_erase_works_and_decreases_size() {
    use ulua_analysis::records::set::Set;

    let mut s1 = Set::<i32>::new(0);
    s1.insert(&1);
    s1.insert(&2);
    assert_eq!(s1.size(), 2);
    assert!(s1.contains(&1));
    assert!(s1.contains(&2));

    s1.erase(&1);
    assert_eq!(s1.size(), 1);
    assert!(!s1.contains(&1));
    assert!(s1.contains(&2));

    s1.erase(&2);
    assert_eq!(s1.size(), 0);
    assert!(s1.empty());
    assert!(!s1.contains(&1));
    assert!(!s1.contains(&2));
  }
}

mod set_insertion_works_and_increases_size {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Set.test.cpp:19:set_insertion_works_and_increases_size`
  //! Source: `tests/Set.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Set.test.cpp
  //! - source_includes:
  //!   - includes -> source_file tests/ScopedFlags.h
  //!   - includes -> source_file Analysis/include/Luau/Set.h
  //! - incoming:
  //!   - declares <- source_file tests/Set.test.cpp
  //! - outgoing:
  //!   - translates_to -> rust_item set_insertion_works_and_increases_size

  #[cfg(test)]
  #[test]
  fn set_insertion_works_and_increases_size() {
    use ulua_analysis::records::set::Set;

    let mut s1 = Set::<i32>::new(0);
    assert_eq!(s1.size(), 0);
    assert!(s1.empty());

    s1.insert(&1);
    assert!(s1.contains(&1));
    assert_eq!(s1.size(), 1);

    s1.insert(&2);
    assert!(s1.contains(&2));
    assert_eq!(s1.size(), 2);
  }
}

mod set_iterate_over_set {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Set.test.cpp:67:set_iterate_over_set`
  //! Source: `tests/Set.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Set.test.cpp
  //! - source_includes:
  //!   - includes -> source_file tests/ScopedFlags.h
  //!   - includes -> source_file Analysis/include/Luau/Set.h
  //! - incoming:
  //!   - declares <- source_file tests/Set.test.cpp
  //! - outgoing:
  //!   - translates_to -> rust_item set_iterate_over_set

  #[cfg(test)]
  #[test]
  fn set_iterate_over_set() {
    use ulua_analysis::records::set::Set;

    let mut s1 = Set::<i32>::new(0);
    s1.insert(&1);
    s1.insert(&2);
    s1.insert(&3);
    assert_eq!(s1.size(), 3);

    let sum: i32 = s1.iter().copied().sum();
    assert_eq!(sum, 6);
  }
}

mod set_iterate_over_set_skips_erased_elements {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Set.test.cpp:83:set_iterate_over_set_skips_erased_elements`
  //! Source: `tests/Set.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Set.test.cpp
  //! - source_includes:
  //!   - includes -> source_file tests/ScopedFlags.h
  //!   - includes -> source_file Analysis/include/Luau/Set.h
  //! - incoming:
  //!   - declares <- source_file tests/Set.test.cpp
  //! - outgoing:
  //!   - translates_to -> rust_item set_iterate_over_set_skips_erased_elements

  #[cfg(test)]
  #[test]
  fn set_iterate_over_set_skips_erased_elements() {
    use ulua_analysis::records::set::Set;

    let mut s1 = Set::<i32>::new(0);
    for value in 1..=6 {
      s1.insert(&value);
    }
    assert_eq!(s1.size(), 6);

    s1.erase(&2);
    s1.erase(&4);
    s1.erase(&6);

    let sum: i32 = s1.iter().copied().sum();
    assert_eq!(sum, 9);
  }
}

mod set_iterate_over_set_skips_first_element_if_it_is_erased {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Set.test.cpp:106:set_iterate_over_set_skips_first_element_if_it_is_erased`
  //! Source: `tests/Set.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Set.test.cpp
  //! - source_includes:
  //!   - includes -> source_file tests/ScopedFlags.h
  //!   - includes -> source_file Analysis/include/Luau/Set.h
  //! - incoming:
  //!   - declares <- source_file tests/Set.test.cpp
  //! - outgoing:
  //!   - type_ref -> record DenseHashSet (Common/include/Luau/DenseHash.h)
  //!   - calls -> function first (Analysis/src/TypePack.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item set_iterate_over_set_skips_first_element_if_it_is_erased

  #[cfg(test)]
  #[test]
  fn set_iterate_over_set_skips_first_element_if_it_is_erased() {
    use alloc::{string::String, vec::Vec};

    use ulua_analysis::records::set::Set;

    let mut s1 = Set::<String>::new(String::new());
    s1.insert(&String::from("x"));
    s1.insert(&String::from("y"));
    s1.erase(&String::from("y"));

    let out: Vec<String> = s1.iter().cloned().collect();
    assert_eq!(1, out.len());
  }
}
