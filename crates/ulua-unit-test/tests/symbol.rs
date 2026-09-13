extern crate alloc;

mod symbol_equality_and_hashing_of_globals {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Symbol.test.cpp:15:symbol_equality_and_hashing_of_globals`
  //! Source: `tests/Symbol.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Symbol.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Symbol.h
  //!   - includes -> source_file Ast/include/Luau/Ast.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/Symbol.test.cpp
  //! - outgoing:
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> method SymDef::name (Analysis/include/Luau/ControlFlowGraph.h)
  //!   - type_ref -> record AstName (Ast/include/Luau/Ast.h)
  //!   - type_ref -> record Symbol (Analysis/include/Luau/Symbol.h)
  //!   - translates_to -> rust_item symbol_equality_and_hashing_of_globals

  #[cfg(test)]
  #[test]
  fn symbol_equality_and_hashing_of_globals() {
    use std::{
      collections::{HashMap, hash_map::DefaultHasher},
      ffi::CString,
      hash::{Hash, Hasher},
    };

    use ulua_analysis::records::symbol::Symbol;
    use ulua_ast::records::ast_name::AstName;

    fn hash_symbol(symbol: &Symbol) -> u64 {
      let mut hasher = DefaultHasher::new();
      symbol.hash(&mut hasher);
      hasher.finish()
    }

    let s1 = CString::new("name").unwrap();
    let s2 = CString::new("name").unwrap();

    let one = AstName::ast_name_c_char(s1.as_ptr());
    let two = AstName::ast_name_c_char(s2.as_ptr());

    let n1 = Symbol::from_global(one);
    let n2 = Symbol::from_global(two);

    assert_eq!(n1, n1);
    assert_eq!(n1, n2);
    assert_eq!(n2, n2);

    assert_eq!(
      hash_symbol(&Symbol::from_global(one)),
      hash_symbol(&Symbol::from_global(one))
    );
    assert_eq!(
      hash_symbol(&Symbol::from_global(one)),
      hash_symbol(&Symbol::from_global(two))
    );
    assert_eq!(
      hash_symbol(&Symbol::from_global(two)),
      hash_symbol(&Symbol::from_global(two))
    );

    let mut the_map = HashMap::new();
    the_map.insert(n1, 5);
    the_map.insert(n2, 1);

    assert_eq!(1, the_map.len());
  }
}

mod symbol_equality_and_hashing_of_locals {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Symbol.test.cpp:42:symbol_equality_and_hashing_of_locals`
  //! Source: `tests/Symbol.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Symbol.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Symbol.h
  //!   - includes -> source_file Ast/include/Luau/Ast.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/Symbol.test.cpp
  //! - outgoing:
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> method SymDef::name (Analysis/include/Luau/ControlFlowGraph.h)
  //!   - type_ref -> record AstLocal (Ast/include/Luau/Ast.h)
  //!   - type_ref -> record AstName (Ast/include/Luau/Ast.h)
  //!   - type_ref -> record Location (Ast/include/Luau/Location.h)
  //!   - type_ref -> record Symbol (Analysis/include/Luau/Symbol.h)
  //!   - translates_to -> rust_item symbol_equality_and_hashing_of_locals

  #[cfg(test)]
  #[test]
  fn symbol_equality_and_hashing_of_locals() {
    use core::ptr::null_mut;
    use std::{
      collections::{HashMap, hash_map::DefaultHasher},
      ffi::CString,
      hash::{Hash, Hasher},
    };

    use ulua_analysis::records::symbol::Symbol;
    use ulua_ast::records::{ast_local::AstLocal, ast_name::AstName, location::Location};

    fn hash_symbol(symbol: &Symbol) -> u64 {
      let mut hasher = DefaultHasher::new();
      symbol.hash(&mut hasher);
      hasher.finish()
    }

    let s1 = CString::new("name").unwrap();
    let s2 = CString::new("name").unwrap();

    let mut one = AstLocal::new(
      AstName::ast_name_c_char(s1.as_ptr()),
      Location::default(),
      null_mut(),
      0,
      0,
      null_mut(),
      false,
    );
    let mut two = AstLocal::new(
      AstName::ast_name_c_char(s2.as_ptr()),
      Location::default(),
      &mut one,
      0,
      0,
      null_mut(),
      false,
    );

    let n1 = Symbol::from_local(&mut one);
    let n2 = Symbol::from_local(&mut two);

    assert_eq!(n1, n1);
    assert_ne!(n1, n2);
    assert_eq!(n2, n2);

    assert_eq!(
      hash_symbol(&Symbol::from_local(&mut one)),
      hash_symbol(&Symbol::from_local(&mut one))
    );
    assert_ne!(
      hash_symbol(&Symbol::from_local(&mut one)),
      hash_symbol(&Symbol::from_local(&mut two))
    );
    assert_eq!(
      hash_symbol(&Symbol::from_local(&mut two)),
      hash_symbol(&Symbol::from_local(&mut two))
    );

    let mut the_map = HashMap::new();
    the_map.insert(n1, 5);
    the_map.insert(n2, 1);

    assert_eq!(2, the_map.len());
  }
}

mod symbol_equality_of_empty_symbols {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/Symbol.test.cpp:69:symbol_equality_of_empty_symbols`
  //! Source: `tests/Symbol.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/Symbol.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Symbol.h
  //!   - includes -> source_file Ast/include/Luau/Ast.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/Symbol.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> method SymDef::name (Analysis/include/Luau/ControlFlowGraph.h)
  //!   - type_ref -> record AstName (Ast/include/Luau/Ast.h)
  //!   - type_ref -> record AstLocal (Ast/include/Luau/Ast.h)
  //!   - type_ref -> record Location (Ast/include/Luau/Location.h)
  //!   - type_ref -> record Symbol (Analysis/include/Luau/Symbol.h)
  //!   - translates_to -> rust_item symbol_equality_of_empty_symbols

  #[cfg(test)]
  #[test]
  fn symbol_equality_of_empty_symbols() {
    use core::ptr::null_mut;
    use std::ffi::CString;

    use ulua_analysis::records::symbol::Symbol;
    use ulua_ast::records::{ast_local::AstLocal, ast_name::AstName, location::Location};
    use ulua_common::FFlag;
    use ulua_unit_test::type_aliases::scoped_fast_flag::ScopedFastFlag;

    let _sff = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let s1 = CString::new("name").unwrap();
    let s2 = CString::new("name").unwrap();

    let one = AstName::ast_name_c_char(s1.as_ptr());
    let mut two = AstLocal::new(
      AstName::ast_name_c_char(s2.as_ptr()),
      Location::default(),
      null_mut(),
      0,
      0,
      null_mut(),
      false,
    );

    let global = Symbol::from_global(one);
    let local = Symbol::from_local(&mut two);
    let empty1 = Symbol::default();
    let empty2 = Symbol::default();

    assert_ne!(empty1, global);
    assert_ne!(empty1, local);
    assert_eq!(empty1, empty2);
  }
}
