extern crate alloc;

mod to_string_bound_types {
  //! Ported from `tests/ToString.test.cpp`.

  #[cfg(test)]
  #[test]
  fn to_string_bound_types() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture
      .check_string_optional_frontend_options(&String::from("local a = 444    local b = a"), None);
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    assert_eq!(
      "number",
      to_string_type_id(fixture.require_type_string(&String::from("b")))
    );
  }
}

mod to_string_builtin_top_extern_types {
  //! Ported from `tests/ToString.test.cpp`.

  #[cfg(test)]
  #[test]
  fn to_string_builtin_top_extern_types() {
    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let builtins = fixture.get_builtins();

    assert_eq!("object", to_string_type_id(builtins.object_type));
    assert_eq!("class", to_string_type_id(builtins.class_type));
  }
}

mod to_string_checked_fn_to_string {
  //! Ported from `tests/ToString.test.cpp`.

  #[cfg(test)]
  #[test]
  fn to_string_checked_fn_to_string() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::enums::mode::Mode;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _old_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = Fixture::fixture_bool(false);
    fixture.load_definition(
      &String::from(
        r#"
@checked declare function abs(n: number) : number
"#,
      ),
      false,
    );

    let result = fixture.check_mode_string_optional_frontend_options(
      Mode::Nonstrict,
      &String::from(
        r#"
local f = abs
"#,
      ),
      None,
    );
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    assert_eq!(
      "@checked (number) -> number",
      to_string_type_id(fixture.require_type_string(&String::from("f")))
    );
  }
}

mod to_string_complex_intersections_printed_on_multiple_lines {
  //! Ported from `tests/ToString.test.cpp`.

  #[cfg(test)]
  #[test]
  fn to_string_complex_intersections_printed_on_multiple_lines() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_m::to_string_type_id_to_string_options,
      records::to_string_options::ToStringOptions,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local a: string & number & boolean
    "#,
      ),
      None,
    );
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let mut opts = ToStringOptions {
      use_line_breaks: true,
      composite_types_single_line_limit: 2,
      ..Default::default()
    };
    let a = fixture.require_type_string(&String::from("a"));
    assert_eq!(
      "boolean\n& number\n& string",
      to_string_type_id_to_string_options(a, &mut opts)
    );
  }
}

mod to_string_complex_unions_printed_on_multiple_lines {
  //! Ported from `tests/ToString.test.cpp`.

  #[cfg(test)]
  #[test]
  fn to_string_complex_unions_printed_on_multiple_lines() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_m::to_string_type_id_to_string_options,
      records::to_string_options::ToStringOptions,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local a: string | number | boolean
    "#,
      ),
      None,
    );
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let mut opts = ToStringOptions {
      composite_types_single_line_limit: 2,
      use_line_breaks: true,
      ..Default::default()
    };
    let a = fixture.require_type_string(&String::from("a"));
    assert_eq!(
      "boolean\n| number\n| string",
      to_string_type_id_to_string_options(a, &mut opts)
    );
  }
}

mod to_string_correct_stringification_user_defined_type_functions {
  //! Ported from `tests/ToString.test.cpp`.

  #[cfg(test)]
  #[test]
  fn to_string_correct_stringification_user_defined_type_functions() {
    use alloc::{string::String, vec};

    use ulua_analysis::{
      enums::reduction::Reduction,
      functions::to_string_to_string_alt_f::to_string_type_item,
      records::{
        r#type::Type, type_function::TypeFunction,
        type_function_instance_type::TypeFunctionInstanceType,
        type_function_reduction_result::TypeFunctionReductionResult,
      },
    };
    use ulua_ast::records::ast_name::AstName;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let number_type = fixture.get_builtins().number_type;
    let user = TypeFunction {
      name: String::from("user"),
      reducer: |_, _, _, _| TypeFunctionReductionResult {
        result: None,
        reduction_status: Reduction::Erroneous,
        blocked_types: vec![],
        blocked_packs: vec![],
        error: None,
        messages: vec![],
      },
      can_reduce_generics: false,
    };
    let user_func_name = b"woohoo\0";
    let tftt = TypeFunctionInstanceType::new_user_defined(
      &user,
      vec![number_type],
      vec![],
      AstName::ast_name_c_char(user_func_name.as_ptr().cast()),
    );
    let tv = Type::from(tftt);

    assert_eq!("woohoo<number>", to_string_type_item(&tv));
  }
}

mod to_string_cycle_rooted_in_a_pack {
  //! Ported from `tests/ToString.test.cpp`.

  #[cfg(test)]
  #[test]
  fn to_string_cycle_rooted_in_a_pack() {
    use alloc::string::String;

    use ulua_analysis::{
      enums::table_state::TableState,
      functions::{
        get_mutable_type_pack::get_mutable_type_pack_id,
        to_string_to_string_alt_d::to_string_type_pack_id,
      },
      records::{
        function_type::FunctionType, property_type::Property, table_type::TableType,
        type_arena::TypeArena, type_level::TypeLevel, type_pack::TypePack,
      },
      type_aliases::props_type::Props,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let number_type = fixture.get_builtins().number_type;
    let unknown_type = fixture.get_builtins().unknown_type;
    let mut arena = TypeArena::default();

    let the_pack = arena.add_type_pack_initializer_list_type_id(&[number_type, number_type]);
    let empty_pack = arena.add_type_pack_initializer_list_type_id(&[]);
    let base_method = arena.add_type(FunctionType::function_type_new(
      the_pack, empty_pack, None, false,
    ));

    let mut props = Props::new();
    props.insert(String::from("BaseField"), Property::readonly(unknown_type));
    props.insert(String::from("BaseMethod"), Property::readonly(base_method));

    let the_table = arena.add_type(
      TableType::table_type_props_optional_table_indexer_type_level_table_state(
        &props,
        None,
        TypeLevel::default(),
        TableState::Sealed,
      ),
    );

    let pack_ptr = get_mutable_type_pack_id::<TypePack>(the_pack).expect("expected pack");
    pack_ptr.head_mut()[0] = the_table;

    assert_eq!(
      "tp1 where tp1 = { read BaseField: unknown, read BaseMethod: (tp1) -> () }, number",
      to_string_type_pack_id(the_pack)
    );
  }
}

mod to_string_cyclic_table {
  //! Ported from `tests/ToString.test.cpp`.

  #[cfg(test)]
  #[test]
  fn to_string_cyclic_table() {
    use ulua_analysis::{
      functions::{
        get_mutable_type::get_mutable_type_id, to_string_to_string_alt_c::to_string_type_id,
      },
      records::{property_type::Property, table_type::TableType, type_arena::TypeArena},
    };

    let mut arena = TypeArena::default();
    let cyclic_table = arena.add_type(TableType::new());
    let table_one = get_mutable_type_id::<TableType>(cyclic_table).expect("expected cyclic table");
    table_one
      .props
      .insert("self".into(), Property::rw_type_id(cyclic_table));

    assert_eq!(
      "t1 where t1 = {| self: t1 |}",
      to_string_type_id(cyclic_table)
    );
  }
}

mod to_string_empty_table {
  //! Ported from `tests/ToString.test.cpp`.

  #[cfg(test)]
  #[test]
  fn to_string_empty_table() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::{
        to_string_to_string_alt_c::to_string_type_id,
        to_string_to_string_alt_m::to_string_type_id_to_string_options,
      },
      records::to_string_options::ToStringOptions,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local a: {}
    "#,
      ),
      None,
    );
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let a = fixture.require_type_string(&String::from("a"));
    assert_eq!("{  }", to_string_type_id(a));

    let mut opts = ToStringOptions {
      use_line_breaks: true,
      ..Default::default()
    };
    assert_eq!("{  }", to_string_type_id_to_string_options(a, &mut opts));
  }
}

mod to_string_exhaustive_to_string_of_cyclic_table {
  //! Ported from `tests/ToString.test.cpp`.

  #[cfg(test)]
  #[test]
  fn to_string_exhaustive_to_string_of_cyclic_table() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_m::to_string_type_id_to_string_options,
      records::to_string_options::ToStringOptions,
    };
    use ulua_common::FFlag;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        --!strict
        local Vec3 = {}
        Vec3.__index = Vec3
        function Vec3.new()
            return setmetatable({x=0, y=0, z=0}, Vec3)
        end

        export type Vec3 = typeof(Vec3.new())

        local thefun: any = function(self, o) return self end

        local multiply: ((Vec3, Vec3) -> Vec3) & ((Vec3, number) -> Vec3) = thefun

        Vec3.__mul = multiply

        local a = Vec3.new()
    "#,
      ),
      None,
    );
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let mut opts = ToStringOptions::new(true);
    let a = to_string_type_id_to_string_options(
      fixture.base.require_type_string(&String::from("a")),
      &mut opts,
    );

    assert_eq!(None, a.find("CYCLE"));
    assert_eq!(None, a.find("TRUNCATED"));

    if !FFlag::DebugLuauForceOldSolver.get() {
      assert_eq!(
        "t2 where t1 = { __index: t1, __mul: ((t2, number) -> t2) & ((t2, t2) -> t2), new: () -> t2 } ; t2 = { @metatable t1, { x: number, y: number, z: number } }",
        a
      );
    } else {
      assert_eq!(
        "t2 where t1 = {| __index: t1, __mul: ((t2, number) -> t2) & ((t2, t2) -> t2), new: () -> t2 |} ; t2 = { @metatable t1, { x: number, y: number, z: number } }",
        a
      );
    }
  }
}

mod to_string_free_types {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/ToString.test.cpp:52:to_string_free_types`
  //! Source: `tests/ToString.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/ToString.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypePack.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/ToString.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item to_string_free_types

  #[cfg(test)]
  #[test]
  fn to_string_free_types() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_common::FFlag;
    use ulua_unit_test::records::fixture::Fixture;

    if !FFlag::DebugLuauForceOldSolver.get() {
      return;
    }

    let mut fixture = Fixture::fixture_bool(false);

    let result = fixture.check_string_optional_frontend_options(&String::from("local a"), None);

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "'a",
      to_string_type_id(fixture.require_type_string(&String::from("a")))
    );
  }
}

mod to_string_free_types_stringify_the_same_regardless_of_solver {
  //! Ported from `tests/ToString.test.cpp`.
  use core::ptr::null_mut;

  #[cfg(test)]
  #[test]
  fn to_string_free_types_stringify_the_same_regardless_of_solver() {
    use ulua_analysis::{
      enums::polarity::Polarity,
      functions::to_string_to_string_alt_c::to_string_type_id,
      records::{free_type::FreeType, type_arena::TypeArena},
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let builtins = fixture.get_builtins();

    let mut arena = TypeArena::default();
    let t = arena.add_type(FreeType::free_type_scope_type_id_type_id_polarity(
      null_mut(),
      builtins.never_type,
      builtins.unknown_type,
      Polarity::Unknown,
    ));

    assert_eq!("'a", to_string_type_id(t));
  }
}

mod to_string_function_type_with_argument_names {
  //! Ported from `tests/ToString.test.cpp`.

  #[cfg(test)]
  #[test]
  fn to_string_function_type_with_argument_names() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_m::to_string_type_id_to_string_options,
      records::to_string_options::ToStringOptions,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from("type MyFunc = (a: number, string, c: number) -> string; local a : MyFunc"),
      None,
    );
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let mut opts = ToStringOptions {
      function_type_arguments: true,
      ..Default::default()
    };
    assert_eq!(
      "(a: number, string, c: number) -> string",
      to_string_type_id_to_string_options(
        fixture.require_type_string(&String::from("a")),
        &mut opts
      )
    );
  }
}

mod to_string_function_type_with_argument_names_and_self {
  //! Ported from `tests/ToString.test.cpp`.

  #[cfg(test)]
  #[test]
  fn to_string_function_type_with_argument_names_and_self() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_m::to_string_type_id_to_string_options,
      records::to_string_options::ToStringOptions,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
local tbl = {}
tbl.a = 2
function tbl:foo(b: number, c: number) return (self.a :: number) + b + c end
type Table = typeof(tbl)
type Foo = typeof(tbl.foo)
local u: Foo
"#,
      ),
      None,
    );
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let mut opts = ToStringOptions {
      function_type_arguments: true,
      ..Default::default()
    };
    let _ = to_string_type_id_to_string_options(
      fixture.require_type_string(&String::from("u")),
      &mut opts,
    );
  }
}

mod to_string_function_type_with_argument_names_generic {
  //! Ported from `tests/ToString.test.cpp`.

  #[cfg(test)]
  #[test]
  fn to_string_function_type_with_argument_names_generic() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_m::to_string_type_id_to_string_options,
      records::to_string_options::ToStringOptions,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from("local function f<a...>(n: number, ...: a...): (a...) return ... end"),
      None,
    );
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let mut opts = ToStringOptions {
      function_type_arguments: true,
      ..Default::default()
    };
    assert_eq!(
      "<a...>(n: number, a...) -> (a...)",
      to_string_type_id_to_string_options(
        fixture.require_type_string(&String::from("f")),
        &mut opts
      )
    );
  }
}

mod to_string_functions_are_always_parenthesized_in_unions_or_intersections {
  //! Ported from `tests/ToString.test.cpp`.

  #[cfg(test)]
  #[test]
  fn to_string_functions_are_always_parenthesized_in_unions_or_intersections() {
    use alloc::vec;

    use ulua_analysis::{
      functions::to_string_to_string_alt_f::to_string_type_item,
      records::{
        function_type::FunctionType, intersection_type::IntersectionType, r#type::Type,
        type_arena::TypeArena, union_type::UnionType,
      },
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let builtins = fixture.get_builtins();
    let mut arena = TypeArena::default();

    let string_and_number_pack =
      arena.add_type_pack_initializer_list_type_id(&[builtins.string_type, builtins.number_type]);
    let number_and_string_pack =
      arena.add_type_pack_initializer_list_type_id(&[builtins.number_type, builtins.string_type]);

    let sn_to_ns = Type::from(FunctionType::function_type_new(
      string_and_number_pack,
      number_and_string_pack,
      None,
      false,
    ));
    let ns_to_sn = Type::from(FunctionType::function_type_new(
      number_and_string_pack,
      string_and_number_pack,
      None,
      false,
    ));

    let utv = Type::from(UnionType {
      options: vec![&ns_to_sn as *const _, &sn_to_ns as *const _],
    });
    let itv = Type::from(IntersectionType {
      parts: vec![&ns_to_sn as *const _, &sn_to_ns as *const _],
    });

    assert_eq!(
      "((number, string) -> (string, number)) | ((string, number) -> (number, string))",
      to_string_type_item(&utv)
    );
    assert_eq!(
      "((number, string) -> (string, number)) & ((string, number) -> (number, string))",
      to_string_type_item(&itv)
    );
  }
}

mod to_string_generate_friendly_names_for_inferred_generics {
  //! Ported from `tests/ToString.test.cpp`.

  #[cfg(test)]
  #[test]
  fn to_string_generate_friendly_names_for_inferred_generics() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        function id(x) return x end

        function id2(a1, a2, a3, a4, a5, a6, a7, a8, a9, a10, a11, a12, a13, a14, a15, a16, a17, a18, a19, a20, a21, a22, a23, a24, a25, a26, a27, a28, a29, a30)
            return a1, a2, a3, a4, a5, a6, a7, a8, a9, a10, a11, a12, a13, a14, a15, a16, a17, a18, a19, a20, a21, a22, a23, a24, a25, a26, a27, a28, a29, a30
        end
    "#,
        ),
        None,
    );
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    assert_eq!(
      "<a>(a) -> a",
      to_string_type_id(fixture.require_type_string(&String::from("id")))
    );
    assert_eq!(
      "<a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s, t, u, v, w, x, y, z, a1, b1, c1, d1>(a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s, t, u, v, w, x, y, z, a1, b1, c1, d1) -> (a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s, t, u, v, w, x, y, z, a1, b1, c1, d1)",
      to_string_type_id(fixture.require_type_string(&String::from("id2")))
    );
  }
}

mod to_string_generic_packs_are_stringified_differently_from_generic_types {
  //! Ported from `tests/ToString.test.cpp`.

  #[cfg(test)]
  #[test]
  fn to_string_generic_packs_are_stringified_differently_from_generic_types() {
    use alloc::string::String;

    use ulua_analysis::{
      enums::polarity::Polarity,
      functions::{
        to_string_to_string_alt_f::to_string_type_item,
        to_string_to_string_alt_g::to_string_type_pack_var,
      },
      records::{
        generic_type::GenericType, generic_type_pack::GenericTypePack, r#type::Type,
        type_pack_var::TypePackVar,
      },
    };

    let tpv = TypePackVar::from(GenericTypePack::new_name(String::from("a")));
    assert_eq!("a...", to_string_type_pack_var(&tpv));

    let tv = Type::from(GenericType::generic_type_name_polarity(
      &String::from("a"),
      Polarity::Mixed,
    ));
    assert_eq!("a", to_string_type_item(&tv));
  }
}

mod to_string_intersection_parenthesized_only_if_needed {
  //! Ported from `tests/ToString.test.cpp`.

  #[cfg(test)]
  #[test]
  fn to_string_intersection_parenthesized_only_if_needed() {
    use alloc::vec;

    use ulua_analysis::{
      functions::to_string_to_string_alt_f::to_string_type_item,
      records::{intersection_type::IntersectionType, r#type::Type, union_type::UnionType},
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let builtins = fixture.get_builtins();
    let utv = Type::from(UnionType {
      options: vec![builtins.number_type, builtins.string_type],
    });
    let itv = Type::from(IntersectionType {
      parts: vec![&utv as *const _, builtins.boolean_type],
    });

    assert_eq!("(number | string) & boolean", to_string_type_item(&itv));
  }
}

mod to_string_long_disjunct_of_nil_is_nil_not_question_mark {
  //! Ported from `tests/ToString.test.cpp`.

  #[cfg(test)]
  #[test]
  fn to_string_long_disjunct_of_nil_is_nil_not_question_mark() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_m::to_string_type_id_to_string_options,
      records::to_string_options::ToStringOptions,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
      type nil_ty = nil | nil | nil | nil | nil
      local a : nil_ty = nil
  "#,
      ),
      None,
    );
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let mut opts = ToStringOptions {
      use_line_breaks: false,
      ..Default::default()
    };
    let a = fixture.require_type_string(&String::from("a"));
    assert_eq!("nil", to_string_type_id_to_string_options(a, &mut opts));
  }
}

mod to_string_metatable {
  //! Ported from `tests/ToString.test.cpp`.

  #[cfg(test)]
  #[test]
  fn to_string_metatable() {
    use ulua_analysis::{
      functions::to_string_to_string_alt_f::to_string_type_item,
      records::{metatable_type::MetatableType, table_type::TableType, r#type::Type},
    };

    let table = Type::from(TableType::new());
    let metatable = Type::from(TableType::new());
    let mtv = Type::from(MetatableType::new(
      &table as *const _,
      &metatable as *const _,
    ));

    assert_eq!("{ @metatable {|  |}, {|  |} }", to_string_type_item(&mtv));
  }
}

mod to_string_named_metatable {
  //! Ported from `tests/ToString.test.cpp`.

  #[cfg(test)]
  #[test]
  fn to_string_named_metatable() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_f::to_string_type_item,
      records::{metatable_type::MetatableType, table_type::TableType, r#type::Type},
    };

    let table = Type::from(TableType::new());
    let metatable = Type::from(TableType::new());
    let mtv = Type::from(MetatableType::new_named(
      &table as *const _,
      &metatable as *const _,
      String::from("NamedMetatable"),
    ));

    assert_eq!("NamedMetatable", to_string_type_item(&mtv));
  }
}

mod to_string_named_metatable_to_string_named_function {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/ToString.test.cpp:160:to_string_named_metatable_to_string_named_function`
  //! Source: `tests/ToString.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/ToString.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypePack.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/ToString.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - translates_to -> rust_item to_string_named_metatable_to_string_named_function

  #[cfg(test)]
  #[test]
  fn to_string_named_metatable_to_string_named_function() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::{
        follow_type::follow_type_id, get_type_alt_j::get_type_id,
        to_string_named_function_to_string::to_string_named_function_string_function_type,
      },
      records::function_type::FunctionType,
    };
    use ulua_common::FFlag;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    if !FFlag::DebugLuauForceOldSolver.get() {
      return;
    }

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function createTbl(): NamedMetatable
            return setmetatable({}, {})
        end
        type NamedMetatable = typeof(createTbl())
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    let ty = fixture.base.require_type_string(&String::from("createTbl"));
    let ftv = get_type_id::<FunctionType>(follow_type_id(ty))
      .expect("expected createTbl to be a function type");
    assert_eq!(
      "createTbl(): NamedMetatable",
      to_string_named_function_string_function_type("createTbl", ftv)
    );
  }
}

mod to_string_named_table {
  //! Ported from `tests/ToString.test.cpp`.

  #[cfg(test)]
  #[test]
  fn to_string_named_table() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_f::to_string_type_item,
      records::{table_type::TableType, r#type::Type},
    };

    let mut table = TableType::new();
    table.name = Some(String::from("TheTable"));
    let table = Type::from(table);

    assert_eq!("TheTable", to_string_type_item(&table));
  }
}

mod to_string_nil_or_nil_is_nil_not_question_mark {
  //! Ported from `tests/ToString.test.cpp`.

  #[cfg(test)]
  #[test]
  fn to_string_nil_or_nil_is_nil_not_question_mark() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_m::to_string_type_id_to_string_options,
      records::to_string_options::ToStringOptions,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
      type nil_ty = nil | nil
      local a : nil_ty = nil
  "#,
      ),
      None,
    );
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let mut opts = ToStringOptions {
      use_line_breaks: false,
      ..Default::default()
    };
    let a = fixture.require_type_string(&String::from("a"));
    assert_eq!("nil", to_string_type_id_to_string_options(a, &mut opts));
  }
}

mod to_string_no_parentheses_around_cyclic_function_type_in_intersection {
  //! Ported from `tests/ToString.test.cpp`.

  #[cfg(test)]
  #[test]
  fn to_string_no_parentheses_around_cyclic_function_type_in_intersection() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function f() return f end
        local a: ((number) -> ()) & typeof(f)
    "#,
      ),
      None,
    );
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    assert_eq!(
      "((number) -> ()) & t1 where t1 = () -> t1",
      to_string_type_id(fixture.require_type_string(&String::from("a")))
    );
  }
}

mod to_string_no_parentheses_around_cyclic_function_type_in_union {
  //! Ported from `tests/ToString.test.cpp`.

  #[cfg(test)]
  #[test]
  fn to_string_no_parentheses_around_cyclic_function_type_in_union() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type F = ((() -> number)?) -> F?
        local function f(p) return f end
        local g: F = f
    "#,
      ),
      None,
    );
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    assert_eq!(
      "t1 where t1 = ((() -> number)?) -> t1?",
      to_string_type_id(fixture.require_type_string(&String::from("g")))
    );
  }
}

mod to_string_no_parentheses_around_return_type_if_pack_has_an_empty_head_link {
  //! Ported from `tests/ToString.test.cpp`.

  #[cfg(test)]
  #[test]
  fn to_string_no_parentheses_around_return_type_if_pack_has_an_empty_head_link() {
    use alloc::vec;

    use ulua_analysis::{
      functions::to_string_to_string_alt_c::to_string_type_id,
      records::{function_type::FunctionType, type_arena::TypeArena},
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let string_type = fixture.get_builtins().string_type;
    let mut arena = TypeArena::default();

    let real_tail = arena.add_type_pack_initializer_list_type_id(&[string_type]);
    let empty_tail =
      arena.add_type_pack_vector_type_id_optional_type_pack_id(vec![], Some(real_tail));
    let arg_list = arena.add_type_pack_initializer_list_type_id(&[string_type]);

    let function_type = arena.add_type(FunctionType::function_type_new(
      arg_list, empty_tail, None, false,
    ));

    assert_eq!("(string) -> string", to_string_type_id(function_type));
  }
}

mod to_string_overloaded_functions_always_printed_on_multiple_lines {
  //! Ported from `tests/ToString.test.cpp`.

  #[cfg(test)]
  #[test]
  fn to_string_overloaded_functions_always_printed_on_multiple_lines() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_m::to_string_type_id_to_string_options,
      records::to_string_options::ToStringOptions,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local a: ((string) -> string) & ((number) -> number)
    "#,
      ),
      None,
    );
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let mut opts = ToStringOptions {
      use_line_breaks: true,
      ..Default::default()
    };
    let a = fixture.require_type_string(&String::from("a"));
    assert_eq!(
      "((number) -> number)\n& ((string) -> string)",
      to_string_type_id_to_string_options(a, &mut opts)
    );
  }
}

mod to_string_pick_distinct_names_for_mixed_explicit_and_implicit_generics {
  //! Ported from `tests/ToString.test.cpp`.

  #[cfg(test)]
  #[test]
  fn to_string_pick_distinct_names_for_mixed_explicit_and_implicit_generics() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_common::FFlag;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function foo<a>(x: a, y) end
    "#,
      ),
      None,
    );
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    if !FFlag::DebugLuauForceOldSolver.get() {
      assert_eq!(
        "<a>(a, unknown) -> ()",
        to_string_type_id(fixture.require_type_string(&String::from("foo")))
      );
    } else {
      assert_eq!(
        "<a, b>(a, b) -> ()",
        to_string_type_id(fixture.require_type_string(&String::from("foo")))
      );
    }
  }
}

mod to_string_primitive {
  //! Ported from `tests/ToString.test.cpp`.

  #[cfg(test)]
  #[test]
  fn to_string_primitive() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_common::FFlag;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from("local a = nil    local b = 44    local c = 'lalala'    local d = true"),
      None,
    );
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    if !FFlag::DebugLuauForceOldSolver.get() {
      assert_eq!(
        "nil",
        to_string_type_id(fixture.require_type_string(&String::from("a")))
      );
    } else {
      assert_ne!(
        "nil",
        to_string_type_id(fixture.require_type_string(&String::from("a")))
      );
    }

    assert_eq!(
      "number",
      to_string_type_id(fixture.require_type_string(&String::from("b")))
    );
    assert_eq!(
      "string",
      to_string_type_id(fixture.require_type_string(&String::from("c")))
    );
    assert_eq!(
      "boolean",
      to_string_type_id(fixture.require_type_string(&String::from("d")))
    );
  }
}

mod to_string_quit_stringifying_table_type_when_length_is_exceeded {
  //! Ported from `tests/ToString.test.cpp`.

  #[cfg(test)]
  #[test]
  fn to_string_quit_stringifying_table_type_when_length_is_exceeded() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_m::to_string_type_id_to_string_options,
      records::{
        property_type::Property, table_type::TableType, to_string_options::ToStringOptions,
        type_arena::TypeArena,
      },
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let number_type = fixture.get_builtins().number_type;
    let mut table = TableType::new();
    for c in b'a'..=b'o' {
      table.props.insert(
        String::from(char::from(c)),
        Property::rw_type_id(number_type),
      );
    }

    let mut arena = TypeArena::default();
    let tv = arena.add_type(table);

    let mut opts = ToStringOptions {
      exhaustive: false,
      max_table_length: 40,
      ..Default::default()
    };
    assert_eq!(
      "{| a: number, b: number, c: number, d: number, e: number, ... 10 more ... |}",
      to_string_type_id_to_string_options(tv, &mut opts)
    );
  }
}

mod to_string_quit_stringifying_type_when_length_is_exceeded {
  //! Ported from `tests/ToString.test.cpp`.

  #[cfg(test)]
  #[test]
  fn to_string_quit_stringifying_type_when_length_is_exceeded() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_m::to_string_type_id_to_string_options,
      records::to_string_options::ToStringOptions,
    };
    use ulua_common::FFlag;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function f0() end
        function f1(f) return f or f0 end
        function f2(f) return f or f1 end
        function f3(f) return f or f2 end
    "#,
      ),
      None,
    );
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    if !FFlag::DebugLuauForceOldSolver.get() {
      let mut opts = ToStringOptions {
        exhaustive: false,
        max_type_length: 20,
        ..Default::default()
      };
      assert_eq!(
        "() -> ()",
        to_string_type_id_to_string_options(
          fixture.require_type_string(&String::from("f0")),
          &mut opts
        )
      );
      assert_eq!(
        "<a>(a) -> (() -> ()) ... *TRUNCATED*",
        to_string_type_id_to_string_options(
          fixture.require_type_string(&String::from("f1")),
          &mut opts
        )
      );
      assert_eq!(
        "<b>(b) -> (<a>(a) -> (() -> ())... *TRUNCATED*",
        to_string_type_id_to_string_options(
          fixture.require_type_string(&String::from("f2")),
          &mut opts
        )
      );
      assert_eq!(
        "<c>(c) -> (<b>(b) -> (<a>(a) -> (() -> ())... *TRUNCATED*",
        to_string_type_id_to_string_options(
          fixture.require_type_string(&String::from("f3")),
          &mut opts
        )
      );
    } else {
      let mut opts = ToStringOptions {
        exhaustive: false,
        max_type_length: 40,
        ..Default::default()
      };
      assert_eq!(
        "() -> ()",
        to_string_type_id_to_string_options(
          fixture.require_type_string(&String::from("f0")),
          &mut opts
        )
      );
      assert_eq!(
        "(() -> ()) -> () -> ()",
        to_string_type_id_to_string_options(
          fixture.require_type_string(&String::from("f1")),
          &mut opts
        )
      );
      assert_eq!(
        "((() -> ()) -> () -> ()) -> (() -> ()) -> ... *TRUNCATED*",
        to_string_type_id_to_string_options(
          fixture.require_type_string(&String::from("f2")),
          &mut opts
        )
      );
      assert_eq!(
        "(((() -> ()) -> () -> ()) -> (() -> ()) -> ... *TRUNCATED*",
        to_string_type_id_to_string_options(
          fixture.require_type_string(&String::from("f3")),
          &mut opts
        )
      );
    }
  }
}

mod to_string_read_only_properties {
  //! Ported from `tests/ToString.test.cpp`.

  #[cfg(test)]
  #[test]
  fn to_string_read_only_properties() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_m::to_string_type_id_to_string_options,
      records::to_string_options::ToStringOptions,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type A = {x: string}
        type B = {read x: string}
    "#,
      ),
      None,
    );
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let mut opts = ToStringOptions::new(true);
    assert_eq!(
      "{ x: string }",
      to_string_type_id_to_string_options(
        fixture.require_type_alias(&String::from("A")),
        &mut opts
      )
    );

    let mut opts = ToStringOptions::new(true);
    assert_eq!(
      "{ read x: string }",
      to_string_type_id_to_string_options(
        fixture.require_type_alias(&String::from("B")),
        &mut opts
      )
    );
  }
}

mod to_string_record_type_compositions_generic {
  //! Ported from `tests/ToString.test.cpp`.

  #[cfg(test)]
  #[test]
  fn to_string_record_type_compositions_generic() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_detailed_to_string::to_string_detailed_type_id_to_string_options,
      records::to_string_options::ToStringOptions,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let check_result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type Object = {}
        type Box<T> = { inner: T }

        local x: Box<Object>
    "#,
      ),
      None,
    );
    assert_eq!(0, check_result.errors.len(), "{:?}", check_result.errors);

    let mut opts = ToStringOptions::default();
    let ty = fixture.require_type_string(&String::from("x"));
    let result = to_string_detailed_type_id_to_string_options(ty, &mut opts);

    assert_eq!(2, result.type_spans.len());

    let span_box = result.type_spans[0];
    assert_eq!(0, span_box.start_pos());
    assert_eq!(3, span_box.end_pos());
    assert_eq!(ty, span_box.r#type());

    let span_object = result.type_spans[1];
    assert_eq!(4, span_object.start_pos());
    assert_eq!(10, span_object.end_pos());
    assert_eq!(
      fixture.require_type_alias(&String::from("Object")),
      span_object.r#type()
    );
  }
}

mod to_string_record_type_compositions_table {
  //! Ported from `tests/ToString.test.cpp`.

  #[cfg(test)]
  #[test]
  fn to_string_record_type_compositions_table() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_detailed_to_string::to_string_detailed_type_id_to_string_options,
      records::to_string_options::ToStringOptions,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let check_result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type Table = {}
    "#,
      ),
      None,
    );
    assert_eq!(0, check_result.errors.len(), "{:?}", check_result.errors);

    let mut opts = ToStringOptions::default();
    let ty = fixture.require_type_alias(&String::from("Table"));
    let result = to_string_detailed_type_id_to_string_options(ty, &mut opts);

    assert_eq!(1, result.type_spans.len());
    let span = result.type_spans[0];
    assert_eq!(0, span.start_pos());
    assert_eq!(5, span.end_pos());
    assert_eq!(ty, span.r#type());
  }
}

mod to_string_record_type_compositions_union_handle_resorted_results {
  //! Ported from `tests/ToString.test.cpp`.

  #[cfg(test)]
  #[test]
  fn to_string_record_type_compositions_union_handle_resorted_results() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_detailed_to_string::to_string_detailed_type_id_to_string_options,
      records::to_string_options::ToStringOptions,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let check_result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type Zebra = {}
        type Alpha = {}

        type Composite = Zebra | Alpha
    "#,
      ),
      None,
    );
    assert_eq!(0, check_result.errors.len(), "{:?}", check_result.errors);

    let mut opts = ToStringOptions::default();
    let ty = fixture.require_type_alias(&String::from("Composite"));
    let result = to_string_detailed_type_id_to_string_options(ty, &mut opts);

    assert_eq!("Alpha | Zebra", result.name);
    assert_eq!(2, result.type_spans.len());

    let span_alpha = result.type_spans[0];
    assert_eq!(0, span_alpha.start_pos());
    assert_eq!(5, span_alpha.end_pos());
    assert_eq!(
      fixture.require_type_alias(&String::from("Alpha")),
      span_alpha.r#type()
    );

    let span_zebra = result.type_spans[1];
    assert_eq!(8, span_zebra.start_pos());
    assert_eq!(13, span_zebra.end_pos());
    assert_eq!(
      fixture.require_type_alias(&String::from("Zebra")),
      span_zebra.r#type()
    );
  }
}

mod to_string_record_type_compositions_union_intersection {
  //! Ported from `tests/ToString.test.cpp`.

  #[cfg(test)]
  #[test]
  fn to_string_record_type_compositions_union_intersection() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_detailed_to_string::to_string_detailed_type_id_to_string_options,
      records::to_string_options::ToStringOptions,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let check_result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type TableA = {}
        type TableB = {}

        type Composite1 = TableA | TableB
        type Composite2 = TableA & TableB
    "#,
      ),
      None,
    );
    assert_eq!(0, check_result.errors.len(), "{:?}", check_result.errors);

    for alias_name in ["Composite1", "Composite2"] {
      let mut opts = ToStringOptions::default();
      let ty = fixture.require_type_alias(&String::from(alias_name));
      let result = to_string_detailed_type_id_to_string_options(ty, &mut opts);

      assert_eq!(2, result.type_spans.len(), "{}", alias_name);

      let span_a = result.type_spans[0];
      assert_eq!(0, span_a.start_pos(), "{}", alias_name);
      assert_eq!(6, span_a.end_pos(), "{}", alias_name);
      assert_eq!(
        fixture.require_type_alias(&String::from("TableA")),
        span_a.r#type(),
        "{}",
        alias_name
      );

      let span_b = result.type_spans[1];
      assert_eq!(9, span_b.start_pos(), "{}", alias_name);
      assert_eq!(15, span_b.end_pos(), "{}", alias_name);
      assert_eq!(
        fixture.require_type_alias(&String::from("TableB")),
        span_b.r#type(),
        "{}",
        alias_name
      );
    }
  }
}

mod to_string_self_recursive_instantiated_param {
  //! Ported from `tests/ToString.test.cpp`.

  #[cfg(test)]
  #[test]
  fn to_string_self_recursive_instantiated_param() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::{
        get_mutable_type::get_mutable_type_id, to_string_to_string_alt_c::to_string_type_id,
      },
      records::{table_type::TableType, type_arena::TypeArena},
    };

    let mut arena = TypeArena::default();
    let table_ty = arena.add_type(TableType::new());
    let ttv = get_mutable_type_id::<TableType>(table_ty).expect("expected table");
    ttv.name = Some(String::from("Table"));
    ttv.instantiated_type_params.push(table_ty);

    assert_eq!("Table<Table>", to_string_type_id(table_ty));
  }
}

mod to_string_simple_intersections_printed_on_one_line {
  //! Ported from `tests/ToString.test.cpp`.

  #[cfg(test)]
  #[test]
  fn to_string_simple_intersections_printed_on_one_line() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_m::to_string_type_id_to_string_options,
      records::to_string_options::ToStringOptions,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local a: string & number
    "#,
      ),
      None,
    );
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let mut opts = ToStringOptions {
      use_line_breaks: true,
      ..Default::default()
    };
    let a = fixture.require_type_string(&String::from("a"));
    assert_eq!(
      "number & string",
      to_string_type_id_to_string_options(a, &mut opts)
    );
  }
}

mod to_string_simple_unions_printed_on_one_line {
  //! Ported from `tests/ToString.test.cpp`.

  #[cfg(test)]
  #[test]
  fn to_string_simple_unions_printed_on_one_line() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_m::to_string_type_id_to_string_options,
      records::to_string_options::ToStringOptions,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local a: number | boolean
    "#,
      ),
      None,
    );
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let mut opts = ToStringOptions {
      use_line_breaks: true,
      ..Default::default()
    };
    let a = fixture.require_type_string(&String::from("a"));
    assert_eq!(
      "boolean | number",
      to_string_type_id_to_string_options(a, &mut opts)
    );
  }
}

mod to_string_stringifying_array_uses_array_syntax {
  //! Ported from `tests/ToString.test.cpp`.

  #[cfg(test)]
  #[test]
  fn to_string_stringifying_array_uses_array_syntax() {
    use alloc::string::String;

    use ulua_analysis::{
      enums::table_state::TableState,
      functions::to_string_to_string_alt_f::to_string_type_item,
      records::{
        property_type::Property, table_indexer::TableIndexer, table_type::TableType, r#type::Type,
      },
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let builtins = fixture.get_builtins();
    let mut ttv = TableType::new();
    ttv.state = TableState::Sealed;
    ttv.indexer = Some(TableIndexer {
      index_type: builtins.number_type,
      index_result_type: builtins.string_type,
      is_read_only: false,
    });

    assert_eq!("{string}", to_string_type_item(&Type::from(ttv.clone())));

    ttv.props.insert(
      String::from("A"),
      Property::rw_type_id(builtins.number_type),
    );
    assert_eq!(
      "{ [number]: string, A: number }",
      to_string_type_item(&Type::from(ttv.clone()))
    );

    ttv.props.clear();
    ttv.state = TableState::Unsealed;
    assert_eq!("{string}", to_string_type_item(&Type::from(ttv)));
  }
}

mod to_string_stringifying_cyclic_intersection_type_bails_early {
  //! Ported from `tests/ToString.test.cpp`.

  #[cfg(test)]
  #[test]
  fn to_string_stringifying_cyclic_intersection_type_bails_early() {
    use alloc::vec;

    use ulua_analysis::{
      functions::{
        get_mutable_type::get_mutable_type_id, to_string_to_string_alt_c::to_string_type_id,
      },
      records::{intersection_type::IntersectionType, type_arena::TypeArena},
    };

    let mut arena = TypeArena::default();
    let tv = arena.add_type(IntersectionType { parts: vec![] });
    let itv = get_mutable_type_id::<IntersectionType>(tv).expect("expected intersection type");
    itv.parts.push(tv);
    itv.parts.push(tv);

    assert_eq!("t1 where t1 = t1 & t1", to_string_type_id(tv));
  }
}

mod to_string_stringifying_cyclic_union_type_bails_early {
  //! Ported from `tests/ToString.test.cpp`.

  #[cfg(test)]
  #[test]
  fn to_string_stringifying_cyclic_union_type_bails_early() {
    use alloc::vec;

    use ulua_analysis::{
      functions::{
        get_mutable_type::get_mutable_type_id, to_string_to_string_alt_c::to_string_type_id,
      },
      records::{type_arena::TypeArena, union_type::UnionType},
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let string_type = fixture.get_builtins().string_type;
    let number_type = fixture.get_builtins().number_type;

    let mut arena = TypeArena::default();
    let tv = arena.add_type(UnionType {
      options: vec![string_type, number_type],
    });
    let utv = get_mutable_type_id::<UnionType>(tv).expect("expected union type");
    utv.options.push(tv);
    utv.options.push(tv);

    assert_eq!("t1 where t1 = number | string", to_string_type_id(tv));
  }
}

mod to_string_stringifying_table_type_correctly_use_matching_table_state_braces {
  //! Ported from `tests/ToString.test.cpp`.

  #[cfg(test)]
  #[test]
  fn to_string_stringifying_table_type_correctly_use_matching_table_state_braces() {
    use alloc::string::String;

    use ulua_analysis::{
      enums::table_state::TableState,
      functions::to_string_to_string_alt_m::to_string_type_id_to_string_options,
      records::{
        property_type::Property, table_type::TableType, to_string_options::ToStringOptions,
        type_arena::TypeArena,
      },
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let number_type = fixture.get_builtins().number_type;
    let mut table = TableType::new();
    table.state = TableState::Sealed;
    for c in b'a'..=b'j' {
      table.props.insert(
        String::from(char::from(c)),
        Property::rw_type_id(number_type),
      );
    }

    let mut arena = TypeArena::default();
    let tv = arena.add_type(table);

    let mut opts = ToStringOptions {
      max_table_length: 40,
      ..Default::default()
    };
    assert_eq!(
      "{ a: number, b: number, c: number, d: number, e: number, ... 5 more ... }",
      to_string_type_id_to_string_options(tv, &mut opts)
    );
  }
}

mod to_string_stringifying_table_type_is_still_capped_when_exhaustive {
  //! Ported from `tests/ToString.test.cpp`.

  #[cfg(test)]
  #[test]
  fn to_string_stringifying_table_type_is_still_capped_when_exhaustive() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_m::to_string_type_id_to_string_options,
      records::{
        property_type::Property, table_type::TableType, to_string_options::ToStringOptions,
        type_arena::TypeArena,
      },
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let number_type = fixture.get_builtins().number_type;
    let mut table = TableType::new();
    for c in b'a'..=b'g' {
      table.props.insert(
        String::from(char::from(c)),
        Property::rw_type_id(number_type),
      );
    }

    let mut arena = TypeArena::default();
    let tv = arena.add_type(table);

    let mut opts = ToStringOptions {
      exhaustive: true,
      max_table_length: 40,
      ..Default::default()
    };
    assert_eq!(
      "{| a: number, b: number, c: number, d: number, e: number, ... 2 more ... |}",
      to_string_type_id_to_string_options(tv, &mut opts)
    );
  }
}

mod to_string_stringifying_type_is_still_capped_when_exhaustive {
  //! Ported from `tests/ToString.test.cpp`.

  #[cfg(test)]
  #[test]
  fn to_string_stringifying_type_is_still_capped_when_exhaustive() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_m::to_string_type_id_to_string_options,
      records::to_string_options::ToStringOptions,
    };
    use ulua_common::FFlag;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function f0() end
        function f1(f) return f or f0 end
        function f2(f) return f or f1 end
        function f3(f) return f or f2 end
    "#,
      ),
      None,
    );
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    if !FFlag::DebugLuauForceOldSolver.get() {
      let mut opts = ToStringOptions {
        exhaustive: true,
        max_type_length: 20,
        ..Default::default()
      };
      assert_eq!(
        "() -> ()",
        to_string_type_id_to_string_options(
          fixture.require_type_string(&String::from("f0")),
          &mut opts
        )
      );
      assert_eq!(
        "<a>(a) -> (() -> ()) ... *TRUNCATED*",
        to_string_type_id_to_string_options(
          fixture.require_type_string(&String::from("f1")),
          &mut opts
        )
      );
      assert_eq!(
        "<b>(b) -> (<a>(a) -> (() -> ())... *TRUNCATED*",
        to_string_type_id_to_string_options(
          fixture.require_type_string(&String::from("f2")),
          &mut opts
        )
      );
      assert_eq!(
        "<c>(c) -> (<b>(b) -> (<a>(a) -> (() -> ())... *TRUNCATED*",
        to_string_type_id_to_string_options(
          fixture.require_type_string(&String::from("f3")),
          &mut opts
        )
      );
    } else {
      let mut opts = ToStringOptions {
        exhaustive: true,
        max_type_length: 40,
        ..Default::default()
      };
      assert_eq!(
        "() -> ()",
        to_string_type_id_to_string_options(
          fixture.require_type_string(&String::from("f0")),
          &mut opts
        )
      );
      assert_eq!(
        "(() -> ()) -> () -> ()",
        to_string_type_id_to_string_options(
          fixture.require_type_string(&String::from("f1")),
          &mut opts
        )
      );
      assert_eq!(
        "((() -> ()) -> () -> ()) -> (() -> ()) -> ... *TRUNCATED*",
        to_string_type_id_to_string_options(
          fixture.require_type_string(&String::from("f2")),
          &mut opts
        )
      );
      assert_eq!(
        "(((() -> ()) -> () -> ()) -> (() -> ()) -> ... *TRUNCATED*",
        to_string_type_id_to_string_options(
          fixture.require_type_string(&String::from("f3")),
          &mut opts
        )
      );
    }
  }
}

mod to_string_table_respects_use_line_break {
  //! Ported from `tests/ToString.test.cpp`.

  #[cfg(test)]
  #[test]
  fn to_string_table_respects_use_line_break() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::to_string_to_string_alt_m::to_string_type_id_to_string_options,
      records::to_string_options::ToStringOptions,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local a: { prop: string, anotherProp: number, thirdProp: boolean }
    "#,
      ),
      None,
    );
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let mut opts = ToStringOptions {
      use_line_breaks: true,
      ..Default::default()
    };
    let a = fixture.require_type_string(&String::from("a"));

    assert_eq!(
      "{\n    anotherProp: number,\n    prop: string,\n    thirdProp: boolean\n}",
      to_string_type_id_to_string_options(a, &mut opts)
    );
  }
}

mod to_string_the_empty_type_pack_should_be_parenthesized {
  //! Ported from `tests/ToString.test.cpp`.

  #[cfg(test)]
  #[test]
  fn to_string_the_empty_type_pack_should_be_parenthesized() {
    use alloc::vec;

    use ulua_analysis::{
      functions::{
        to_string_to_string_alt_f::to_string_type_item,
        to_string_to_string_alt_g::to_string_type_pack_var,
      },
      records::{
        function_type::FunctionType, r#type::Type, type_arena::TypeArena, type_pack::TypePack,
        type_pack_var::TypePackVar,
      },
    };

    let empty_type_pack = TypePackVar::from(TypePack::new(vec![], None));
    assert_eq!("()", to_string_type_pack_var(&empty_type_pack));

    let mut arena = TypeArena::default();
    let empty_arg_pack = arena.add_type_pack_initializer_list_type_id(&[]);
    let empty_ret_pack = arena.add_type_pack_initializer_list_type_id(&[]);
    let unit_to_unit = Type::from(FunctionType::function_type_new(
      empty_arg_pack,
      empty_ret_pack,
      None,
      false,
    ));
    assert_eq!("() -> ()", to_string_type_item(&unit_to_unit));
  }
}

mod to_string_to_string_detailed {
  //! Ported from `tests/ToString.test.cpp`.

  #[cfg(test)]
  #[test]
  fn to_string_to_string_detailed() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::{
        flatten_type_pack::flatten_type_pack_id, follow_type::follow_type_id,
        get_type_alt_j::get_type_id,
        to_string_detailed_to_string::to_string_detailed_type_id_to_string_options,
        to_string_to_string_alt_m::to_string_type_id_to_string_options,
      },
      records::{function_type::FunctionType, to_string_options::ToStringOptions},
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function id3(a, b, c)
            return a, b, c
        end
    "#,
      ),
      None,
    );
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let mut opts = ToStringOptions::default();
    let id3_type = fixture.require_type_string(&String::from("id3"));
    let name_data = to_string_detailed_type_id_to_string_options(id3_type, &mut opts);

    assert_eq!(3, opts.name_map.types.size());
    assert_eq!("<a, b, c>(a, b, c) -> (a, b, c)", name_data.name);

    let ftv = get_type_id::<FunctionType>(follow_type_id(id3_type))
      .expect("expected id3 to be a function type");
    let (params, _) = flatten_type_pack_id(ftv.arg_types());
    assert_eq!(3, params.len());

    assert_eq!(
      "a",
      to_string_type_id_to_string_options(params[0], &mut opts)
    );
    assert_eq!(
      "b",
      to_string_type_id_to_string_options(params[1], &mut opts)
    );
    assert_eq!(
      "c",
      to_string_type_id_to_string_options(params[2], &mut opts)
    );
  }
}

mod to_string_to_string_error_pack {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/ToString.test.cpp:584:to_string_to_string_error_pack`
  //! Source: `tests/ToString.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/ToString.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/Scope.h
  //!   - includes -> source_file Analysis/include/Luau/ToString.h
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
  //!   - includes -> source_file Analysis/include/Luau/TypePack.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/ToString.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> type_alias type (Common/include/Luau/Variant.h)
  //!   - translates_to -> rust_item to_string_to_string_error_pack

  #[cfg(test)]
  #[test]
  fn to_string_to_string_error_pack() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_common::FFlag;
    use ulua_unit_test::records::fixture::Fixture;

    if !FFlag::DebugLuauForceOldSolver.get() {
      return;
    }

    let mut fixture = Fixture::fixture_bool(false);

    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
local function target(callback: nil) return callback(4, "hello") end
    "#,
      ),
      None,
    );

    assert!(!result.errors.is_empty(), "{:?}", result.errors);
    assert_eq!(
      "(nil) -> (*error-type*)",
      to_string_type_id(fixture.require_type_string(&String::from("target")))
    );
  }
}

mod to_string_to_string_generic_pack {
  //! Ported from `tests/ToString.test.cpp`.

  #[cfg(test)]
  #[test]
  fn to_string_to_string_generic_pack() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
function foo(a, b) return a(b) end
    "#,
      ),
      None,
    );
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "<a, b...>((a) -> (b...), a) -> (b...)",
      to_string_type_id(fixture.require_type_string(&String::from("foo")))
    );
  }
}

mod to_string_to_string_named_function_generic_pack {
  //! Ported from `tests/ToString.test.cpp`.

  #[cfg(test)]
  #[test]
  fn to_string_to_string_named_function_generic_pack() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::{
        follow_type::follow_type_id, get_type_alt_j::get_type_id,
        to_string_named_function_to_string::to_string_named_function_string_function_type,
      },
      records::function_type::FunctionType,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let _result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f(a: number, b: string) end
        local function test<T..., U...>(...: T...): U...
            f(...)
            return 1, 2, 3
        end
    "#,
      ),
      None,
    );

    let ty = fixture.require_type_string(&String::from("test"));
    let ftv =
      get_type_id::<FunctionType>(follow_type_id(ty)).expect("expected test to be a function type");
    assert_eq!(
      "test<T..., U...>(...: T...): U...",
      to_string_named_function_string_function_type("test", ftv)
    );
  }
}

mod to_string_to_string_named_function_hide_type_params {
  //! Ported from `tests/ToString.test.cpp`.

  #[cfg(test)]
  #[test]
  fn to_string_to_string_named_function_hide_type_params() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::{
        follow_type::follow_type_id, get_type_alt_j::get_type_id,
        to_string_named_function_to_string_alt_b::to_string_named_function_string_function_type_to_string_options,
      },
      records::{function_type::FunctionType, to_string_options::ToStringOptions},
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let _result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f<T>(x: T, g: <U>(T) -> U)): ()
        end
    "#,
      ),
      None,
    );

    let ty = fixture.require_type_string(&String::from("f"));
    let ftv =
      get_type_id::<FunctionType>(follow_type_id(ty)).expect("expected f to be a function type");
    let mut opts = ToStringOptions {
      hide_named_function_type_parameters: true,
      ..Default::default()
    };
    assert_eq!(
      "f(x: T, g: <U>(T) -> U): ()",
      to_string_named_function_string_function_type_to_string_options("f", ftv, &mut opts)
    );
  }
}

mod to_string_to_string_named_function_id {
  //! Ported from `tests/ToString.test.cpp`.

  #[cfg(test)]
  #[test]
  fn to_string_to_string_named_function_id() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::{
        follow_type::follow_type_id, get_type_alt_j::get_type_id,
        to_string_named_function_to_string::to_string_named_function_string_function_type,
      },
      records::function_type::FunctionType,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function id(x) return x end
    "#,
      ),
      None,
    );
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let ty = fixture.require_type_string(&String::from("id"));
    let ftv =
      get_type_id::<FunctionType>(follow_type_id(ty)).expect("expected id to be a function type");
    assert_eq!(
      "id<a>(x: a): a",
      to_string_named_function_string_function_type("id", ftv)
    );
  }
}

mod to_string_to_string_named_function_map {
  //! Ported from `tests/ToString.test.cpp`.

  #[cfg(test)]
  #[test]
  fn to_string_to_string_named_function_map() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::{
        follow_type::follow_type_id, get_type_alt_j::get_type_id,
        to_string_named_function_to_string::to_string_named_function_string_function_type,
      },
      records::function_type::FunctionType,
    };
    use ulua_common::FFlag;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function map(arr, fn)
            local t = {}
            for i = 0, #arr do
                t[i] = fn(arr[i])
            end
            return t
        end
    "#,
      ),
      None,
    );
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let ty = fixture.require_type_string(&String::from("map"));
    let ftv =
      get_type_id::<FunctionType>(follow_type_id(ty)).expect("expected map to be a function type");
    if !FFlag::DebugLuauForceOldSolver.get() {
      assert_eq!(
        "map<a, b>(arr: {a}, fn: (a) -> (b, ...unknown)): {b}",
        to_string_named_function_string_function_type("map", ftv)
      );
    } else {
      assert_eq!(
        "map<a, b>(arr: {a}, fn: (a) -> b): {b}",
        to_string_named_function_string_function_type("map", ftv)
      );
    }
  }
}

mod to_string_to_string_named_function_overrides_param_names {
  //! Ported from `tests/ToString.test.cpp`.

  #[cfg(test)]
  #[test]
  fn to_string_to_string_named_function_overrides_param_names() {
    use alloc::{string::String, vec};

    use ulua_analysis::{
      functions::{
        follow_type::follow_type_id, get_type_alt_j::get_type_id,
        to_string_named_function_to_string_alt_b::to_string_named_function_string_function_type_to_string_options,
      },
      records::{function_type::FunctionType, to_string_options::ToStringOptions},
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function test(a, b : string, ... : number) return a end
    "#,
      ),
      None,
    );
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let ty = fixture.require_type_string(&String::from("test"));
    let ftv =
      get_type_id::<FunctionType>(follow_type_id(ty)).expect("expected test to be a function type");
    let mut opts = ToStringOptions {
      named_function_override_arg_names: vec![
        String::from("first"),
        String::from("second"),
        String::from("third"),
      ],
      ..Default::default()
    };
    assert_eq!(
      "test<a>(first: a, second: string, ...: number): a",
      to_string_named_function_string_function_type_to_string_options("test", ftv, &mut opts)
    );
  }
}

mod to_string_to_string_named_function_type_annotation_has_partial_argnames {
  //! Ported from `tests/ToString.test.cpp`.

  #[cfg(test)]
  #[test]
  fn to_string_to_string_named_function_type_annotation_has_partial_argnames() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::{
        follow_type::follow_type_id, get_type_alt_j::get_type_id,
        to_string_named_function_to_string::to_string_named_function_string_function_type,
      },
      records::function_type::FunctionType,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local f: (number, y: number) -> number
    "#,
      ),
      None,
    );
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let ty = fixture.require_type_string(&String::from("f"));
    let ftv =
      get_type_id::<FunctionType>(follow_type_id(ty)).expect("expected f to be a function type");
    assert_eq!(
      "f(_: number, y: number): number",
      to_string_named_function_string_function_type("f", ftv)
    );
  }
}

mod to_string_to_string_named_function_unit_f {
  //! Ported from `tests/ToString.test.cpp`.

  #[cfg(test)]
  #[test]
  fn to_string_to_string_named_function_unit_f() {
    use ulua_analysis::{
      functions::to_string_named_function_to_string::to_string_named_function_string_function_type,
      records::{function_type::FunctionType, type_arena::TypeArena},
    };

    let mut arena = TypeArena::default();
    let empty_args = arena.add_type_pack_initializer_list_type_id(&[]);
    let empty_rets = arena.add_type_pack_initializer_list_type_id(&[]);
    let ftv = FunctionType::function_type_new(empty_args, empty_rets, None, false);

    assert_eq!(
      "f(): ()",
      to_string_named_function_string_function_type("f", &ftv)
    );
  }
}

mod to_string_to_string_named_function_variadics {
  //! Ported from `tests/ToString.test.cpp`.

  #[cfg(test)]
  #[test]
  fn to_string_to_string_named_function_variadics() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::{
        follow_type::follow_type_id, get_type_alt_j::get_type_id,
        to_string_named_function_to_string::to_string_named_function_string_function_type,
      },
      records::function_type::FunctionType,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f<a, b...>(x: a, ...): (a, a, b...)
            return x, x, ...
        end
    "#,
      ),
      None,
    );
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let ty = fixture.require_type_string(&String::from("f"));
    let ftv =
      get_type_id::<FunctionType>(follow_type_id(ty)).expect("expected f to be a function type");
    assert_eq!(
      "f<a, b...>(x: a, ...: any): (a, a, b...)",
      to_string_named_function_string_function_type("f", ftv)
    );
  }
}

mod to_string_to_string_named_function_variadics_2 {
  //! Ported from `tests/ToString.test.cpp`.

  #[cfg(test)]
  #[test]
  fn to_string_to_string_named_function_variadics2() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::{
        follow_type::follow_type_id, get_type_alt_j::get_type_id,
        to_string_named_function_to_string::to_string_named_function_string_function_type,
      },
      records::function_type::FunctionType,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f(): ...number
            return 1, 2, 3
        end
    "#,
      ),
      None,
    );
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let ty = fixture.require_type_string(&String::from("f"));
    let ftv =
      get_type_id::<FunctionType>(follow_type_id(ty)).expect("expected f to be a function type");
    assert_eq!(
      "f(): ...number",
      to_string_named_function_string_function_type("f", ftv)
    );
  }
}

mod to_string_to_string_named_function_variadics_3 {
  //! Ported from `tests/ToString.test.cpp`.

  #[cfg(test)]
  #[test]
  fn to_string_to_string_named_function_variadics3() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::{
        follow_type::follow_type_id, get_type_alt_j::get_type_id,
        to_string_named_function_to_string::to_string_named_function_string_function_type,
      },
      records::function_type::FunctionType,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f(): (string, ...number)
            return 'a', 1, 2, 3
        end
    "#,
      ),
      None,
    );
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let ty = fixture.require_type_string(&String::from("f"));
    let ftv =
      get_type_id::<FunctionType>(follow_type_id(ty)).expect("expected f to be a function type");
    assert_eq!(
      "f(): (string, ...number)",
      to_string_named_function_string_function_type("f", ftv)
    );
  }
}

mod to_string_to_string_the_bound_to_table_type_contained_within_a_type_pack {
  //! Ported from `tests/ToString.test.cpp`.

  #[cfg(test)]
  #[test]
  fn to_string_to_string_the_bound_to_table_type_contained_within_a_type_pack() {
    use alloc::string::String;

    use ulua_analysis::{
      enums::table_state::TableState,
      functions::{
        get_mutable_type::get_mutable_type_id, to_string_to_string_alt_d::to_string_type_pack_id,
      },
      records::{property_type::Property, table_type::TableType, type_arena::TypeArena},
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let number_type = fixture.get_builtins().number_type;
    let mut arena = TypeArena::default();

    let mut table = TableType::new();
    table.state = TableState::Sealed;
    table
      .props
      .insert(String::from("hello"), Property::rw_type_id(number_type));
    table
      .props
      .insert(String::from("world"), Property::rw_type_id(number_type));
    let tv1 = arena.add_type(table);
    let tpv1 = arena.add_type_pack_initializer_list_type_id(&[tv1]);

    let mut bound_table = TableType::new();
    bound_table.state = TableState::Free;
    bound_table
      .props
      .insert(String::from("hello"), Property::rw_type_id(number_type));
    let tv2 = arena.add_type(bound_table);
    let bttv = get_mutable_type_id::<TableType>(tv2).expect("expected table type");
    bttv.bound_to = Some(tv1);
    let tpv2 = arena.add_type_pack_initializer_list_type_id(&[tv2]);

    assert_eq!(
      "{ hello: number, world: number }",
      to_string_type_pack_id(tpv1)
    );
    assert_eq!(
      "{ hello: number, world: number }",
      to_string_type_pack_id(tpv2)
    );
  }
}

mod to_string_tostring_error_mismatch {
  //! Ported from `tests/ToString.test.cpp`.

  #[cfg(test)]
  #[test]
  fn to_string_tostring_error_mismatch() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_common::FFlag;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        --!strict
        function f1(t: {a : number, b: string, c: {d: string}}) : {a : number, b : string, c : { d : number}}
            return t
        end
    "#,
        ),
        None,
    );

    let expected = if !FFlag::DebugLuauForceOldSolver.get() {
      "Expected this to be\n\t'{ a: number, b: string, c: { d: number } }'\nbut got\n\t'{ a: number, b: string, c: { d: string } }'; \naccessing `c.d` results in `string` in the latter type and `number` in the former type, and `string` is not exactly `number`"
    } else {
      "Expected this to be exactly\n\t'{ a: number, b: string, c: { d: number } }'\nbut got\n\t'{ a: number, b: string, c: { d: string } }'\ncaused by:\n  Property 'c' is not compatible.\nExpected this to be exactly\n\t'{ d: number }'\nbut got\n\t'{ d: string }'\ncaused by:\n  Property 'd' is not compatible.\nExpected this to be exactly 'number', but got 'string'"
    };

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert_eq!(expected, to_string_type_error(&result.errors[0]));
  }
}

mod to_string_tostring_unsee_ttv_if_array {
  //! Ported from `tests/ToString.test.cpp`.

  #[cfg(test)]
  #[test]
  fn to_string_tostring_unsee_ttv_if_array() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local x: {string}
        -- This code is constructed very specifically to use the same (by pointer
        -- identity) type in the function twice.
        local y: (typeof(x), typeof(x)) -> ()
    "#,
      ),
      None,
    );
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    assert_eq!(
      "({string}, {string}) -> ()",
      to_string_type_id(fixture.require_type_string(&String::from("y")))
    );
  }
}

mod to_string_union_parenthesized_only_if_needed {
  //! Ported from `tests/ToString.test.cpp`.

  #[cfg(test)]
  #[test]
  fn to_string_union_parenthesized_only_if_needed() {
    use alloc::vec;

    use ulua_analysis::{
      functions::to_string_to_string_alt_f::to_string_type_item,
      records::{intersection_type::IntersectionType, r#type::Type, union_type::UnionType},
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let builtins = fixture.get_builtins();
    let itv = Type::from(IntersectionType {
      parts: vec![builtins.number_type, builtins.string_type],
    });
    let utv = Type::from(UnionType {
      options: vec![&itv as *const _, builtins.boolean_type],
    });

    assert_eq!("(number & string) | boolean", to_string_type_item(&utv));
  }
}
