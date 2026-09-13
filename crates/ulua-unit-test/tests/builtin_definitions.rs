extern crate alloc;

mod builtin_definitions_lib_documentation_symbols {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/BuiltinDefinitions.test.cpp:13:builtin_definitions_lib_documentation_symbols`
  //! Source: `tests/BuiltinDefinitions.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/BuiltinDefinitions.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Type.h
  //!   - includes -> source_file tests/ClassFixture.h
  //! - incoming:
  //!   - declares <- source_file tests/BuiltinDefinitions.test.cpp
  //! - outgoing:
  //!   - calls -> method SymDef::name (Analysis/include/Luau/ControlFlowGraph.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> method Symbol::c_str (Analysis/include/Luau/Symbol.h)
  //!   - calls -> method StringWriter::symbol (Ast/src/PrettyPrinter.cpp)
  //!   - type_ref -> record TableType (Analysis/include/Luau/Type.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record ExternType (Analysis/include/Luau/Type.h)
  //!   - calls -> method PathBuilder::prop (Analysis/src/TypePath.cpp)
  //!   - translates_to -> rust_item builtin_definitions_lib_documentation_symbols

  #[cfg(test)]
  #[test]
  fn builtin_definitions_lib_documentation_symbols() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::{follow_type::follow_type_id, get_type_alt_j::get_type_id},
      records::{extern_type::ExternType, table_type::TableType},
    };
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    let frontend = fixture.get_frontend();
    let global_scope = frontend.globals.global_scope();

    assert!(!global_scope.bindings.is_empty());

    for (name, binding) in &global_scope.bindings {
      let name_string = name.name();
      let expected_root_symbol = alloc::format!("@luau/global/{name_string}");

      assert_eq!(
        binding.documentation_symbol.as_deref(),
        Some(expected_root_symbol.as_str()),
        "expected symbol {expected_root_symbol} for global {name_string}, got {:?}",
        binding.documentation_symbol
      );

      let followed = follow_type_id(binding.type_id);
      let mut props: Option<Vec<(String, Option<String>)>> = None;

      if let Some(ttv) = get_type_id::<TableType>(followed) {
        props = Some(
          ttv
            .props
            .iter()
            .map(|(prop_name, prop)| (prop_name.clone(), prop.documentation_symbol.clone()))
            .collect(),
        );
      } else if let Some(etv) = get_type_id::<ExternType>(followed) {
        props = Some(
          etv
            .props
            .iter()
            .map(|(prop_name, prop)| (prop_name.clone(), prop.documentation_symbol.clone()))
            .collect(),
        );
      }

      if let Some(props) = props {
        for (prop_name, actual_prop_symbol) in props {
          let full_prop_name = alloc::format!("{name_string}.{prop_name}");
          let expected_prop_symbol = alloc::format!("{expected_root_symbol}.{prop_name}");

          assert_eq!(
            actual_prop_symbol.as_deref(),
            Some(expected_prop_symbol.as_str()),
            "expected symbol {expected_prop_symbol} for {full_prop_name}, got {:?}",
            actual_prop_symbol
          );
        }
      }
    }
  }
}
