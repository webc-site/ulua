extern crate alloc;

mod type_infer_intersection_types_argument_is_intersection {
  //! Ported from `tests/TypeInfer.intersectionTypes.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_intersection_types_argument_is_intersection() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type A = (number | boolean) -> number

        local function foo(f: A)
            f(5)
            f(true)
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_intersection_types_bounds_propagate_into_free_intersection_bounds {
  //! Ported from `tests/TypeInfer.intersectionTypes.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_intersection_types_bounds_propagate_into_free_intersection_bounds() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _sff = ScopedFastFlag::new(
      &FFlag::LuauPropagateFreeTypesIntoUnionAndIntersectionBounds,
      true,
    );
    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f<T>(a: T & string): T
            return a
        end

        local b = f("hello")
        local c = f(("world" :: string))
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "string",
      to_string_type_id(fixture.require_type_string(&String::from("b")))
    );
    assert_eq!(
      "string",
      to_string_type_id(fixture.require_type_string(&String::from("c")))
    );
  }
}

mod type_infer_intersection_types_cli_44817 {
  //! Ported from `tests/TypeInfer.intersectionTypes.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_intersection_types_cli_44817() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type X = {x: number}
        type Y = {y: number}
        type Z = {z: number}

        type XY = {x: number, y: number}
        type XYZ = {x:number, y: number, z: number}

        function f(xy: XY, xyz: XYZ): (X&Y, X&Y&Z)
            return xy, xyz
        end

        local xNy, xNyNz = f({x = 0, y = 0}, {x = 0, y = 0, z = 0})

        local t1: XY = xNy -- Type 'X & Y' could not be converted into 'XY'
        local t2: XY = xNyNz -- Type 'X & Y & Z' could not be converted into 'XY'
        local t3: XYZ = xNyNz -- Type 'X & Y & Z' could not be converted into 'XYZ'
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_intersection_types_cli_80596_simplify_degenerate_intersections {
  //! Ported from `tests/TypeInfer.intersectionTypes.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_intersection_types_cli_80596_simplify_degenerate_intersections() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _dcr = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type A = {
            x: number?,
        }

        type B = {
            x: number?,
        }

        type C = A & B

        function f(obj: C): number
            return obj.x or 3
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_intersection_types_cli_80596_simplify_more_realistic_intersections {
  //! Ported from `tests/TypeInfer.intersectionTypes.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_intersection_types_cli_80596_simplify_more_realistic_intersections() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _dcr = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type A = {
            x: number?,
            y: string?,
        }

        type B = {
            x: number?,
            z: string?,
        }

        type C = A & B

        function f(obj: C): number
            return obj.x or 3
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_intersection_types_error_detailed_intersection_all {
  //! Ported from `tests/TypeInfer.intersectionTypes.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_intersection_types_error_detailed_intersection_all() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_common::FFlag;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
type X = { x: number }
type Y = { y: number }
type Z = { z: number }
type XYZ = X & Y & Z

function f(a: XYZ): number
    return a
end
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);

    let expected = if !FFlag::DebugLuauForceOldSolver.get() {
      concat!(
        "Expected this to be 'number', but got 'X & Y & Z'; \n",
        "this is because \n\t",
        " * the 1st component of the intersection is `X`, which is not a subtype of `number`\n\t",
        " * the 2nd component of the intersection is `Y`, which is not a subtype of `number`\n\t",
        " * the 3rd component of the intersection is `Z`, which is not a subtype of `number`"
      )
    } else {
      "Expected this to be 'number', but got 'X & Y & Z'; none of the intersection parts are compatible"
    };

    assert_eq!(expected, to_string_type_error(&result.errors[0]));
  }
}

mod type_infer_intersection_types_error_detailed_intersection_part {
  //! Ported from `tests/TypeInfer.intersectionTypes.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_intersection_types_error_detailed_intersection_part() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_common::FFlag;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
type X = { x: number }
type Y = { y: number }
type Z = { z: number }
type XYZ = X & Y & Z
local a: XYZ = 3
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);

    let expected = if !FFlag::DebugLuauForceOldSolver.get() {
      concat!(
        "Expected this to be 'X & Y & Z', but got 'number'; \n",
        "this is because \n\t",
        " * the 1st component of the intersection is `X`, and `number` is not a subtype of `X`\n\t",
        " * the 2nd component of the intersection is `Y`, and `number` is not a subtype of `Y`\n\t",
        " * the 3rd component of the intersection is `Z`, and `number` is not a subtype of `Z`"
      )
    } else {
      r#"Expected this to be 'X & Y & Z', but got 'number'
caused by:
  Not all intersection parts are compatible.
Expected this to be 'X', but got 'number'"#
    };

    assert_eq!(expected, to_string_type_error(&result.errors[0]));
  }
}

mod type_infer_intersection_types_fx_intersection_as_argument {
  //! Ported from `tests/TypeInfer.intersectionTypes.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_intersection_types_fx_intersection_as_argument() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type A = (number) -> (string)
        type B = (string) -> (number)
        type C = (A) -> (number)

        local function foo(f: A & B, g: C)
            return g(f)
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_intersection_types_fx_union_as_argument_fails {
  //! Ported from `tests/TypeInfer.intersectionTypes.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_intersection_types_fx_union_as_argument_fails() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type A = (number) -> (string)
        type B = (string) -> (number)
        type C = (A) -> (number)

        local function foo(f: A | B, g: C)
            return g(f)
        end
    "#,
      ),
      None,
    );

    assert!(!result.errors.is_empty());
  }
}

mod type_infer_intersection_types_impossible_type {
  //! Ported from `tests/TypeInfer.intersectionTypes.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_intersection_types_impossible_type() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local c:number&string = 10
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_intersection_types_index_on_an_intersection_type_with_all_parts_missing_the_property {
  //! Ported from `tests/TypeInfer.intersectionTypes.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_intersection_types_index_on_an_intersection_type_with_all_parts_missing_the_property()
   {
    use alloc::string::String;

    use ulua_analysis::{
      functions::get_error::get_type_error, records::unknown_property::UnknownProperty,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type A = {}
        type B = {}

        local function f(t: A & B)
            local x = t.x
        end
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    let up =
      get_type_error::<UnknownProperty>(&result.errors[0]).expect("expected UnknownProperty");
    assert_eq!("x", up.key());
  }
}

mod type_infer_intersection_types_index_on_an_intersection_type_with_mixed_types {
  //! Ported from `tests/TypeInfer.intersectionTypes.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_intersection_types_index_on_an_intersection_type_with_mixed_types() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_common::FFlag;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type A = {x: number}
        type B = {x: string}

        local function f(t: A & B)
            return t.x
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    let expected = if !FFlag::DebugLuauForceOldSolver.get() {
      "(A & B) -> never"
    } else {
      "(A & B) -> number & string"
    };
    assert_eq!(
      expected,
      to_string_type_id(fixture.require_type_string(&String::from("f")))
    );
  }
}

mod type_infer_intersection_types_index_on_an_intersection_type_with_one_part_missing_the_property {
  //! Ported from `tests/TypeInfer.intersectionTypes.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_intersection_types_index_on_an_intersection_type_with_one_part_missing_the_property()
   {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type A = {x: number}
        type B = {}

        local function f(t: A & B)
            return t.x
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "(A & B) -> number",
      to_string_type_id(fixture.require_type_string(&String::from("f")))
    );
  }
}

mod type_infer_intersection_types_index_on_an_intersection_type_with_one_property_of_type_any {
  //! Ported from `tests/TypeInfer.intersectionTypes.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_intersection_types_index_on_an_intersection_type_with_one_property_of_type_any() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type A = {y: number}
        type B = {x: any}

        local function f(t: A & B)
            return t.x
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "(A & B) -> any",
      to_string_type_id(fixture.require_type_string(&String::from("f")))
    );
  }
}

mod type_infer_intersection_types_index_on_an_intersection_type_with_property_guaranteed_to_exist {
  //! Ported from `tests/TypeInfer.intersectionTypes.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_intersection_types_index_on_an_intersection_type_with_property_guaranteed_to_exist()
  {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_common::FFlag;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type A = {x: {y: number}}
        type B = {x: {y: number}}

        local function f(t: A & B)
            return t.x
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    let expected = if !FFlag::DebugLuauForceOldSolver.get() {
      "(A & B) -> { y: number }"
    } else {
      "(A & B) -> { y: number } & { y: number }"
    };
    assert_eq!(
      expected,
      to_string_type_id(fixture.require_type_string(&String::from("f")))
    );
  }
}

mod type_infer_intersection_types_index_on_an_intersection_type_works_at_arbitrary_depth {
  //! Ported from `tests/TypeInfer.intersectionTypes.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_intersection_types_index_on_an_intersection_type_works_at_arbitrary_depth() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_common::FFlag;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type A = {x: {y: {z: {thing: string}}}}
        type B = {x: {y: {z: {thing: string}}}}

        local function f(t: A & B)
            return t.x.y.z.thing
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    let expected = if !FFlag::DebugLuauForceOldSolver.get() {
      "(A & B) -> string"
    } else {
      "(A & B) -> string & string"
    };
    assert_eq!(
      expected,
      to_string_type_id(fixture.require_type_string(&String::from("f")))
    );
  }
}

mod type_infer_intersection_types_index_property_table_intersection_1 {
  //! Ported from `tests/TypeInfer.intersectionTypes.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_intersection_types_index_property_table_intersection_1() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
type Foo = {
	Bar: string,
} & { Baz: number }

function f(x: Foo)
    return x.Bar
end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_intersection_types_index_property_table_intersection_2 {
  //! Ported from `tests/TypeInfer.intersectionTypes.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_intersection_types_index_property_table_intersection_2() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        type Foo = {
            Bar: string,
        } & { Baz: number }

        function f(x: Foo)
            return x["Bar"]
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_intersection_types_intersect_bool_and_false {
  //! Ported from `tests/TypeInfer.intersectionTypes.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_intersection_types_intersect_bool_and_false() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_common::FFlag;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function f(x: boolean & false)
            local y : false = x -- OK
            local z : true = x  -- Not OK
        end
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);

    let expected = if !FFlag::DebugLuauForceOldSolver.get() {
      concat!(
        "Expected this to be 'true', but got 'boolean & false'; \n",
        "this is because \n\t",
        " * the 1st component of the intersection is `boolean`, which is not a subtype of `true`\n\t",
        " * the 2nd component of the intersection is `false`, which is not a subtype of `true`"
      )
    } else {
      "Expected this to be 'true', but got 'boolean & false'; none of the intersection parts are compatible"
    };

    assert_eq!(expected, to_string_type_error(&result.errors[0]));
  }
}

mod type_infer_intersection_types_intersect_false_and_bool_and_false {
  //! Ported from `tests/TypeInfer.intersectionTypes.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_intersection_types_intersect_false_and_bool_and_false() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_common::FFlag;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function f(x: false & (boolean & false))
            local y : false = x -- OK
            local z : true = x  -- Not OK
        end
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);

    let expected = if !FFlag::DebugLuauForceOldSolver.get() {
      concat!(
        "Expected this to be 'true', but got 'boolean & false & false'; \n",
        "this is because \n\t",
        " * the 1st component of the intersection is `false`, which is not a subtype of `true`\n\t",
        " * the 2nd component of the intersection is `boolean`, which is not a subtype of `true`\n\t",
        " * the 3rd component of the intersection is `false`, which is not a subtype of `true`"
      )
    } else {
      "Expected this to be 'true', but got 'boolean & false & false'; none of the intersection parts are compatible"
    };

    assert_eq!(expected, to_string_type_error(&result.errors[0]));
  }
}

mod type_infer_intersection_types_intersect_metatable_subtypes {
  //! Ported from `tests/TypeInfer.intersectionTypes.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_intersection_types_intersect_metatable_subtypes() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local x = setmetatable({ a = 5 }, { p = 5 })
        local y = setmetatable({ b = "hi" }, { p = 5, q = "hi" })
        local z = setmetatable({ a = 5, b = "hi" }, { p = 5, q = "hi" })

        type X = typeof(x)
        type Y = typeof(y)
        type Z = typeof(z)

        function f(xy: X&Y, yx: Y&X): (Z, Z)
            return xy, yx
        end

        f(z, z)
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_intersection_types_intersect_metatable_with_table {
  //! Ported from `tests/TypeInfer.intersectionTypes.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_intersection_types_intersect_metatable_with_table() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let source = if !FFlag::DebugLuauForceOldSolver.get() {
      r#"
            local x = setmetatable({ a = 5 }, { p = 5 })
            local z = setmetatable({ a = 5, b = "hi" }, { p = 5 })

            type X = typeof(x)
            type Y = { b : string }
            type Z = typeof(z)

            function f(xy: X&Y, yx: Y&X): (Z, Z)
                return xy, yx
            end

            f(z, z)
        "#
    } else {
      r#"
            local x = setmetatable({ a = 5 }, { p = 5 });
            local z = setmetatable({ a = 5, b = "hi" }, { p = 5 });

            type X = typeof(x)
            type Y = { b : string }
            type Z = typeof(z)

            -- TODO: once we have shape types, we should be able to initialize these with z
            local xy : X&Y;
            local yx : Y&X;
            z = xy;
            z = yx;
        "#
    };

    let result = fixture
      .base
      .check_string_optional_frontend_options(&String::from(source), None);
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_intersection_types_intersect_metatables {
  //! Ported from `tests/TypeInfer.intersectionTypes.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_intersection_types_intersect_metatables() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    if !FFlag::DebugLuauForceOldSolver.get() {
      return;
    }

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    if !FFlag::DebugLuauForceOldSolver.get() {
      let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
          r#"
            function f(a: string?, b: string?)
                local x = setmetatable({}, { p = 5, q = a })
                local y = setmetatable({}, { q = b, r = "hi" })
                local z = setmetatable({}, { p = 5, q = nil, r = "hi" })

                type X = typeof(x)
                type Y = typeof(y)
                type Z = typeof(z)

                function g(xy: X&Y, yx: Y&X): (Z, Z)
                    return xy, yx
                end

                g(z, z)
            end
        "#,
        ),
        None,
      );

      assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    } else {
      let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
          r#"
            local a : string? = nil
            local b : number? = nil

            local x = setmetatable({}, { p = 5, q = a });
            local y = setmetatable({}, { q = b, r = "hi" });
            local z = setmetatable({}, { p = 5, q = nil, r = "hi" });

            type X = typeof(x)
            type Y = typeof(y)
            type Z = typeof(z)

            local xy : X&Y = z;
            local yx : Y&X = z;
            z = xy;
            z = yx;
        "#,
        ),
        None,
      );

      assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    }
  }
}

mod type_infer_intersection_types_intersect_metatables_with_properties {
  //! Ported from `tests/TypeInfer.intersectionTypes.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_intersection_types_intersect_metatables_with_properties() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local x = setmetatable({ a = 5 }, { p = 5 })
        local y = setmetatable({ b = "hi" }, { q = "hi" })
        local z = setmetatable({ a = 5, b = "hi" }, { p = 5, q = "hi" })

        type X = typeof(x)
        type Y = typeof(y)
        type Z = typeof(z)

        function f(xy: X&Y): Z
            return xy
        end

        f(z)
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_intersection_types_intersect_saturate_overloaded_functions {
  //! Ported from `tests/TypeInfer.intersectionTypes.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_intersection_types_intersect_saturate_overloaded_functions() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_common::FFlag;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function foo(x: ((number?) -> number?) & ((string?) -> string?))
            local y : (nil) -> nil = x -- Not OK (fixed in DCR)
            local z : (number) -> number = x -- Not OK
        end
    "#,
      ),
      None,
    );

    if !FFlag::DebugLuauForceOldSolver.get() {
      assert!(result.errors.len() >= 2, "{:?}", result.errors);
      let expected1 = concat!(
        "Expected this to be\n",
        "\t'(nil) -> nil'\n",
        "but got\n",
        "\t'((number?) -> number?) & ((string?) -> string?)'; \n",
        "this is because \n",
        "\t * in the 1st component of the intersection, the function returns the 1st entry in the type pack which has the 1st component of the union as `number` and it returns the 1st entry in the type pack is `nil`, and `number` is not a subtype of `nil`\n",
        "\t * in the 2nd component of the intersection, the function returns the 1st entry in the type pack which has the 1st component of the union as `string` and it returns the 1st entry in the type pack is `nil`, and `string` is not a subtype of `nil`"
      );
      let expected2 = concat!(
        "Expected this to be\n",
        "\t'(number) -> number'\n",
        "but got\n",
        "\t'((number?) -> number?) & ((string?) -> string?)';\n",
        "this is because\n",
        "\t * in the 1st component of the intersection, the function returns the 1st entry in the type pack which has the 2nd component of the union as `nil` and it returns the 1st entry in the type pack is `number`, and `nil` is not a subtype of `number`\n",
        "\t * in the 2nd component of the intersection, the function returns the 1st entry in the type pack which has the 1st component of the union as `string` and it returns the 1st entry in the type pack is `number`, and `string` is not a subtype of `number`\n",
        "\t * in the 2nd component of the intersection, the function returns the 1st entry in the type pack which has the 2nd component of the union as `nil` and it returns the 1st entry in the type pack is `number`, and `nil` is not a subtype of `number`\n",
        "\t * in the 2nd component of the intersection, the function takes the 1st entry in the type pack which has the 1st component of the union as `string` and it takes the 1st entry in the type pack is `number`, and `string` is not a supertype of `number`\n",
        "\t * in the 2nd component of the intersection, the function takes the 1st entry in the type pack which has the 2nd component of the union as `nil` and it takes the 1st entry in the type pack is `number`, and `nil` is not a supertype of `number`\n"
      );

      ulua_unit_test::CHECK_LONG_STRINGS_EQ!(expected1, to_string_type_error(&result.errors[0]));
      ulua_unit_test::CHECK_LONG_STRINGS_EQ!(expected2, to_string_type_error(&result.errors[1]));
    } else {
      assert_eq!(1, result.errors.len(), "{:?}", result.errors);
      let expected = r#"Expected this to be
	'(number) -> number'
but got
	'((number?) -> number?) & ((string?) -> string?)'; none of the intersection parts are compatible"#;
      assert_eq!(expected, to_string_type_error(&result.errors[0]));
    }
  }
}

mod type_infer_intersection_types_intersection_of_tables {
  //! Ported from `tests/TypeInfer.intersectionTypes.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_intersection_types_intersection_of_tables() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_common::FFlag;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function f(x: { p : number?, q : string? } & { p : number?, q : number?, r : number? })
            local y : { p : number?, q : nil, r : number? } = x -- OK
            local z : { p : nil } = x -- Not OK
        end
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);

    let expected = if !FFlag::DebugLuauForceOldSolver.get() {
      concat!(
        "Expected this to be '{ p: nil }', but got '{ p: number?, q: number?, r: number? } & { p: number?, q: string? }",
        "'; \nthis is because \n\t",
        " * in the 1st component of the intersection, accessing `p` has the 1st component of the union as `number` and ",
        "accessing `p` results in `nil`, and `number` is not exactly `nil`\n\t",
        " * in the 2nd component of the intersection, accessing `p` has the 1st component of the union as `number` and ",
        "accessing `p` results in `nil`, and `number` is not exactly `nil`"
      )
    } else {
      "Expected this to be '{ p: nil }', but got '{ p: number?, q: number?, r: number? } & { p: number?, q: string? }'; none of the intersection parts are compatible"
    };

    assert_eq!(expected, to_string_type_error(&result.errors[0]));
  }
}

mod type_infer_intersection_types_intersection_of_tables_with_never_properties {
  //! Ported from `tests/TypeInfer.intersectionTypes.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_intersection_types_intersection_of_tables_with_never_properties() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function f(x : { p : number?, q : never } & { p : never, q : string? })
            local y : { p : never, q : never } = x -- OK
            local z : never = x -- OK
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_intersection_types_intersection_of_tables_with_top_properties {
  //! Ported from `tests/TypeInfer.intersectionTypes.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_intersection_types_intersection_of_tables_with_top_properties() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_common::FFlag;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function f(x : { p : number?, q : any } & { p : unknown, q : string? })
            local y : { p : number?, q : string? } = x -- OK
            local z : { p : string?, q : number? } = x -- Not OK
        end
    "#,
      ),
      None,
    );

    let expected = if !FFlag::DebugLuauForceOldSolver.get() {
      concat!(
        "Expected this to be\n",
        "\t'{ p: string?, q: number? }'\n",
        "but got\n",
        "\t'{ p: number?, q: any } & { p: unknown, q: string? }'; \n",
        "this is because \n",
        "\t * in the 1st component of the intersection, accessing `p` has the 1st component of the union as `number` and accessing `p` has the 1st component of the union as `string`, and `number` is not exactly `string`\n",
        "\t * in the 1st component of the intersection, accessing `p` has the 1st component of the union as `number` and accessing `p` has the 2nd component of the union as `nil`, and `number` is not exactly `nil`\n",
        "\t * in the 1st component of the intersection, accessing `p` has the 2nd component of the union as `nil` and accessing `p` has the 1st component of the union as `string`, and `nil` is not exactly `string`\n",
        "\t * in the 1st component of the intersection, accessing `q` results in `any` and accessing `q` has the 1st component of the union as `number`, and `any` is not exactly `number`\n",
        "\t * in the 1st component of the intersection, accessing `q` results in `any` and accessing `q` has the 2nd component of the union as `nil`, and `any` is not exactly `nil`\n",
        "\t * in the 2nd component of the intersection, accessing `p` results in `unknown` and accessing `p` has the 1st component of the union as `string`, and `unknown` is not exactly `string`\n",
        "\t * in the 2nd component of the intersection, accessing `p` results in `unknown` and accessing `p` has the 2nd component of the union as `nil`, and `unknown` is not exactly `nil`\n",
        "\t * in the 2nd component of the intersection, accessing `q` has the 1st component of the union as `string` and accessing `q` has the 1st component of the union as `number`, and `string` is not exactly `number`\n",
        "\t * in the 2nd component of the intersection, accessing `q` has the 1st component of the union as `string` and accessing `q` has the 2nd component of the union as `nil`, and `string` is not exactly `nil`\n",
        "\t * in the 2nd component of the intersection, accessing `q` has the 2nd component of the union as `nil` and accessing `q` has the 1st component of the union as `number`, and `nil` is not exactly `number`"
      )
    } else {
      r#"Expected this to be
	'{ p: string?, q: number? }'
but got
	'{ p: number?, q: any } & { p: unknown, q: string? }'; none of the intersection parts are compatible"#
    };

    if FFlag::DebugLuauForceOldSolver.get() {
      assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    }
    assert_eq!(expected, to_string_type_error(&result.errors[0]));
  }
}

mod type_infer_intersection_types_less_greedy_unification_with_intersection_types {
  //! Ported from `tests/TypeInfer.intersectionTypes.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_intersection_types_less_greedy_unification_with_intersection_types() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_common::FFlag;
    use ulua_unit_test::records::fixture::Fixture;

    if FFlag::DebugLuauForceOldSolver.get() {
      return;
    }

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f(t): { x: number } & { x: string }
            local x = t.x
            return t
        end
    "#,
      ),
      None,
    );

    assert_eq!(3, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "(never) -> { x: number } & { x: string }",
      to_string_type_id(fixture.require_type_string(&String::from("f")))
    );
  }
}

mod type_infer_intersection_types_less_greedy_unification_with_intersection_types_2 {
  //! Ported from `tests/TypeInfer.intersectionTypes.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_intersection_types_less_greedy_unification_with_intersection_types_2() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_common::FFlag;
    use ulua_unit_test::records::fixture::Fixture;

    if FFlag::DebugLuauForceOldSolver.get() {
      return;
    }

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function f(t: { x: number } & { x: string })
            return t.x
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "({ x: number } & { x: string }) -> never",
      to_string_type_id(fixture.require_type_string(&String::from("f")))
    );
  }
}

mod type_infer_intersection_types_narrow_intersection_nevers {
  //! Ported from `tests/TypeInfer.intersectionTypes.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_intersection_types_narrow_intersection_nevers() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::records::position::Position;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _sffs = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    fixture.base.load_definition(
      &String::from(
        r#"
        declare extern type Player with
            Character: unknown
        end
    "#,
      ),
      false,
    );

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function foo(player: Player?)
            if player and player.Character then
                print(player.Character)
            end
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "Player & { read Character: ~(false?) }",
      to_string_type_id(fixture.base.require_type_at_position_position(Position {
        line: 3,
        column: 23,
      }))
    );
  }
}

mod type_infer_intersection_types_no_stack_overflow_from_flattenintersection {
  //! Ported from `tests/TypeInfer.intersectionTypes.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_intersection_types_no_stack_overflow_from_flattenintersection() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local l0,l0
        repeat
        type t0 = ((any)|((any)&((any)|((any)&((any)|(any))))))&(t0)
        function _(l0):(t0)&(t0)
            while nil do
            end
        end
        until _(_)(_)._
    "#,
      ),
      None,
    );

    assert!(!result.errors.is_empty());
  }
}

mod type_infer_intersection_types_overload_is_not_a_function {
  //! Ported from `tests/TypeInfer.intersectionTypes.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_intersection_types_overload_is_not_a_function() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
--!nonstrict
function _(...):((typeof(not _))&(typeof(not _)))&((typeof(not _))&(typeof(not _)))
_(...)(setfenv,_,not _,"")[_] = nil
end
do end
_(...)(...,setfenv,_):_G()
"#,
      ),
      None,
    );
  }
}

mod type_infer_intersection_types_overloaded_functions_mentioning_generic {
  //! Ported from `tests/TypeInfer.intersectionTypes.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_intersection_types_overloaded_functions_mentioning_generic() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_common::FFlag;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function f<a>()
            function g(x : ((number?) -> (a | number)) & ((string?) -> (a | string)))
                local y : (nil) -> a = x -- OK
                local z : (number?) -> a = x -- Not OK
            end
        end
    "#,
      ),
      None,
    );

    if !FFlag::DebugLuauForceOldSolver.get() {
      assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    } else {
      assert_eq!(1, result.errors.len(), "{:?}", result.errors);
      let expected = r#"Expected this to be
	'(number?) -> a'
but got
	'((number?) -> a | number) & ((string?) -> a | string)'; none of the intersection parts are compatible"#;
      assert_eq!(expected, to_string_type_error(&result.errors[0]));
    }
  }
}

mod type_infer_intersection_types_overloaded_functions_mentioning_generic_packs {
  //! Ported from `tests/TypeInfer.intersectionTypes.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_intersection_types_overloaded_functions_mentioning_generic_packs() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::{
        get_error::get_type_error, to_string_error::to_string_type_error,
        to_string_to_string_alt_c::to_string_type_id,
      },
      records::type_mismatch::TypeMismatch,
    };
    use ulua_common::FFlag;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        function f<a...,b...>()
            function g(x : ((number?, a...) -> (number?, b...)) & ((string?, a...) -> (string?, b...)))
                local y : ((nil, a...) -> (nil, b...)) = x -- OK in the old solver, not OK in the new
                local z : ((nil, b...) -> (nil, a...)) = x -- Not OK
                local w : ((number?, a...) -> (number?, b...)) = x -- OK in both solvers
            end
        end
    "#,
        ),
        None,
    );

    if !FFlag::DebugLuauForceOldSolver.get() {
      assert_eq!(2, result.errors.len(), "{:?}", result.errors);
      let tm1 = get_type_error::<TypeMismatch>(&result.errors[0]).expect("expected TypeMismatch");
      assert_eq!(
        "(nil, a...) -> (nil, b...)",
        to_string_type_id(tm1.wanted_type)
      );
      assert_eq!(
        "((number?, a...) -> (number?, b...)) & ((string?, a...) -> (string?, b...))",
        to_string_type_id(tm1.given_type)
      );
      let tm2 = get_type_error::<TypeMismatch>(&result.errors[1]).expect("expected TypeMismatch");
      assert_eq!(
        "(nil, b...) -> (nil, a...)",
        to_string_type_id(tm2.wanted_type)
      );
      assert_eq!(
        "((number?, a...) -> (number?, b...)) & ((string?, a...) -> (string?, b...))",
        to_string_type_id(tm2.given_type)
      );

      let expected1 = concat!(
        "Expected this to be\n\t",
        "'(nil, a...) -> (nil, b...)'",
        "\nbut got\n\t",
        "'((number?, a...) -> (number?, b...)) & ((string?, a...) -> (string?, b...))'",
        "; \nthis is because \n\t",
        " * in the 1st component of the intersection, the function returns the 1st entry in the type pack which has the 1st component of ",
        "the ",
        "union as `number` and it returns the 1st entry in the type pack is `nil`, and `number` is not a subtype of `nil`\n\t",
        " * in the 2nd component of the intersection, the function returns the 1st entry in the type pack which has the 1st component of ",
        "the ",
        "union as `string` and it returns the 1st entry in the type pack is `nil`, and `string` is not a subtype of `nil`"
      );
      let expected2 = concat!(
        "Expected this to be\n\t",
        "'(nil, b...) -> (nil, a...)'",
        "\nbut got\n\t",
        "'((number?, a...) -> (number?, b...)) & ((string?, a...) -> (string?, b...))'",
        "; \nthis is because \n\t",
        " * in the 1st component of the intersection, the function returns a tail of `b...` and it returns a tail of `a...`, and `b...` is ",
        "not a ",
        "subtype of `a...`\n\t",
        " * in the 1st component of the intersection, the function returns the 1st entry in the type pack which has the 1st component of ",
        "the ",
        "union as `number` and it returns the 1st entry in the type pack is `nil`, and `number` is not a subtype of `nil`\n\t",
        " * in the 1st component of the intersection, the function takes a tail of `a...` and it takes a tail of `b...`, and `a...` is not ",
        "a ",
        "supertype of `b...`\n\t",
        " * in the 2nd component of the intersection, the function returns a tail of `b...` and it returns a tail of `a...`, and `b...` is ",
        "not a ",
        "subtype of `a...`\n\t",
        " * in the 2nd component of the intersection, the function returns the 1st entry in the type pack which has the 1st component of ",
        "the ",
        "union as `string` and it returns the 1st entry in the type pack is `nil`, and `string` is not a subtype of `nil`\n\t",
        " * in the 2nd component of the intersection, the function takes a tail of `a...` and it takes a tail of `b...`, and `a...` is not ",
        "a ",
        "supertype of `b...`"
      );

      assert_eq!(expected1, to_string_type_error(&result.errors[0]));
      assert_eq!(expected2, to_string_type_error(&result.errors[1]));
    } else {
      assert_eq!(1, result.errors.len(), "{:?}", result.errors);
      let expected = r#"Expected this to be
	'(nil, b...) -> (nil, a...)'
but got
	'((number?, a...) -> (number?, b...)) & ((string?, a...) -> (string?, b...))'; none of the intersection parts are compatible"#;
      assert_eq!(expected, to_string_type_error(&result.errors[0]));
    }
  }
}

mod type_infer_intersection_types_overloaded_functions_mentioning_generics {
  //! Ported from `tests/TypeInfer.intersectionTypes.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_intersection_types_overloaded_functions_mentioning_generics() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_common::FFlag;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function f<a,b,c>()
            function g(x : ((a?) -> (a | b)) & ((c?) -> (b | c)))
                local y : (nil) -> ((a & c) | b) = x -- OK
                local z : (a?) -> ((a & c) | b) = x -- Not OK
            end
        end
    "#,
      ),
      None,
    );

    if !FFlag::DebugLuauForceOldSolver.get() {
      assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    } else {
      assert_eq!(1, result.errors.len(), "{:?}", result.errors);
      let expected = r#"Expected this to be
	'(a?) -> (a & c) | b'
but got
	'((a?) -> a | b) & ((c?) -> b | c)'; none of the intersection parts are compatible"#;
      assert_eq!(expected, to_string_type_error(&result.errors[0]));
    }
  }
}

mod type_infer_intersection_types_overloaded_functions_returning_intersections {
  //! Ported from `tests/TypeInfer.intersectionTypes.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_intersection_types_overloaded_functions_returning_intersections() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_common::FFlag;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        function f(x : ((number?) -> ({ p : number } & { q : number })) & ((string?) -> ({ p : number } & { r : number })))
            local y : (nil) -> { p : number, q : number, r : number} = x -- OK
            local z : (number?) -> { p : number, q : number, r : number} = x -- Not OK
        end
    "#,
        ),
        None,
    );

    if !FFlag::DebugLuauForceOldSolver.get() {
      assert_eq!(2, result.errors.len(), "{:?}", result.errors);
      let expected1 = concat!(
        "Expected this to be\n",
        "\t'(nil) -> { p: number, q: number, r: number }'\n",
        "but got\n",
        "\t'((number?) -> { p: number } & { q: number }) & ((string?) -> { p: number } & { r: number })'; \n",
        "this is because \n",
        "\t * in the 1st component of the intersection, the function returns the 1st entry in the type pack which has the 1st component of the intersection as `{ p: number }` and it returns the 1st entry in the type pack is `{ p: number, q: number, r: number }`, and `{ p: number }` is not a subtype of `{ p: number, q: number, r: number }`\n",
        "\t * in the 1st component of the intersection, the function returns the 1st entry in the type pack which has the 2nd component of the intersection as `{ q: number }` and it returns the 1st entry in the type pack is `{ p: number, q: number, r: number }`, and `{ q: number }` is not a subtype of `{ p: number, q: number, r: number }`\n",
        "\t * in the 2nd component of the intersection, the function returns the 1st entry in the type pack which has the 1st component of the intersection as `{ p: number }` and it returns the 1st entry in the type pack is `{ p: number, q: number, r: number }`, and `{ p: number }` is not a subtype of `{ p: number, q: number, r: number }`\n",
        "\t * in the 2nd component of the intersection, the function returns the 1st entry in the type pack which has the 2nd component of the intersection as `{ r: number }` and it returns the 1st entry in the type pack is `{ p: number, q: number, r: number }`, and `{ r: number }` is not a subtype of `{ p: number, q: number, r: number }`"
      );
      let expected2 = concat!(
        "Expected this to be\n",
        "\t'(number?) -> { p: number, q: number, r: number }'\n",
        "but got\n",
        "\t'((number?) -> { p: number } & { q: number }) & ((string?) -> { p: number } & { r: number })'; \n",
        "this is because \n",
        "\t * in the 1st component of the intersection, the function returns the 1st entry in the type pack which has the 1st component of the intersection as `{ p: number }` and it returns the 1st entry in the type pack is `{ p: number, q: number, r: number }`, and `{ p: number }` is not a subtype of `{ p: number, q: number, r: number }`\n",
        "\t * in the 1st component of the intersection, the function returns the 1st entry in the type pack which has the 2nd component of the intersection as `{ q: number }` and it returns the 1st entry in the type pack is `{ p: number, q: number, r: number }`, and `{ q: number }` is not a subtype of `{ p: number, q: number, r: number }`\n",
        "\t * in the 2nd component of the intersection, the function returns the 1st entry in the type pack which has the 1st component of the intersection as `{ p: number }` and it returns the 1st entry in the type pack is `{ p: number, q: number, r: number }`, and `{ p: number }` is not a subtype of `{ p: number, q: number, r: number }`\n",
        "\t * in the 2nd component of the intersection, the function returns the 1st entry in the type pack which has the 2nd component of the intersection as `{ r: number }` and it returns the 1st entry in the type pack is `{ p: number, q: number, r: number }`, and `{ r: number }` is not a subtype of `{ p: number, q: number, r: number }`\n",
        "\t * in the 2nd component of the intersection, the function takes the 1st entry in the type pack which has the 1st component of the union as `string` and it takes the 1st entry in the type pack has the 1st component of the union as `number`, and `string` is not a supertype of `number`\n",
        "\t * in the 2nd component of the intersection, the function takes the 1st entry in the type pack which has the 2nd component of the union as `nil` and it takes the 1st entry in the type pack has the 1st component of the union as `number`, and `nil` is not a supertype of `number`"
      );

      assert_eq!(expected1, to_string_type_error(&result.errors[0]));
      assert_eq!(expected2, to_string_type_error(&result.errors[1]));
    } else {
      assert_eq!(1, result.errors.len(), "{:?}", result.errors);
      let expected = r#"Expected this to be
	'(number?) -> { p: number, q: number, r: number }'
but got
	'((number?) -> { p: number } & { q: number }) & ((string?) -> { p: number } & { r: number })'; none of the intersection parts are compatible"#;
      assert_eq!(expected, to_string_type_error(&result.errors[0]));
    }
  }
}

mod type_infer_intersection_types_overloadeded_functions_with_never_arguments {
  //! Ported from `tests/TypeInfer.intersectionTypes.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_intersection_types_overloadeded_functions_with_never_arguments() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_common::FFlag;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function f<a...,b...>()
            function g(x : ((number) -> number?) & ((never) -> string?))
                local y : (never) -> nil = x -- OK
                local z : (number?) -> nil = x -- Not OK
            end
        end
    "#,
      ),
      None,
    );

    if !FFlag::DebugLuauForceOldSolver.get() {
      assert!(result.errors.len() >= 2, "{:?}", result.errors);
      let expected1 = concat!(
        "Expected this to be\n\t",
        "'(never) -> nil'",
        "\nbut got\n\t",
        "'((never) -> string?) & ((number) -> number?)'",
        "; \nthis is because \n\t",
        " * in the 1st component of the intersection, the function returns the 1st entry in the type pack which has the 1st component of ",
        "the ",
        "union as `number` and it returns the 1st entry in the type pack is `nil`, and `number` is not a subtype of `nil`\n\t",
        " * in the 2nd component of the intersection, the function returns the 1st entry in the type pack which has the 1st component of ",
        "the ",
        "union as `string` and it returns the 1st entry in the type pack is `nil`, and `string` is not a subtype of `nil`"
      );
      let expected2 = concat!(
        "Expected this to be\n\t",
        "'(number?) -> nil'",
        "\nbut got\n\t",
        "'((never) -> string?) & ((number) -> number?)'",
        "; \nthis is because \n\t",
        " * in the 1st component of the intersection, the function returns the 1st entry in the type pack which has the 1st component of ",
        "the ",
        "union as `number` and it returns the 1st entry in the type pack is `nil`, and `number` is not a subtype of `nil`\n\t",
        " * in the 1st component of the intersection, the function takes the 1st entry in the type pack which is `number` and it takes the ",
        "1st ",
        "entry in the type pack has the 2nd component of the union as `nil`, and `number` is not a supertype of `nil`\n\t",
        " * in the 2nd component of the intersection, the function returns the 1st entry in the type pack which has the 1st component of ",
        "the ",
        "union as `string` and it returns the 1st entry in the type pack is `nil`, and `string` is not a subtype of `nil`\n\t",
        " * in the 2nd component of the intersection, the function takes the 1st entry in the type pack which is `never` and it takes the ",
        "1st ",
        "entry in the type pack has the 1st component of the union as `number`, and `never` is not a supertype of `number`\n\t",
        " * in the 2nd component of the intersection, the function takes the 1st entry in the type pack which is `never` and it takes the ",
        "1st ",
        "entry in the type pack has the 2nd component of the union as `nil`, and `never` is not a supertype of `nil`"
      );

      assert_eq!(expected1, to_string_type_error(&result.errors[0]));
      assert_eq!(expected2, to_string_type_error(&result.errors[1]));
    } else {
      assert_eq!(1, result.errors.len(), "{:?}", result.errors);
      let expected = r#"Expected this to be
	'(number?) -> nil'
but got
	'((never) -> string?) & ((number) -> number?)'; none of the intersection parts are compatible"#;
      assert_eq!(expected, to_string_type_error(&result.errors[0]));
    }
  }
}

mod type_infer_intersection_types_overloadeded_functions_with_never_result {
  //! Ported from `tests/TypeInfer.intersectionTypes.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_intersection_types_overloadeded_functions_with_never_result() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_common::FFlag;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
    function f<a...,b...>()
        function g(x : ((number) -> number) & ((nil) -> never))
            local y : (number?) -> number = x -- OK
            local z : (number?) -> never = x -- Not OK
        end
    end
    "#,
      ),
      None,
    );

    if !FFlag::DebugLuauForceOldSolver.get() {
      assert!(result.errors.len() >= 2, "{:?}", result.errors);
      let expected1 = concat!(
        "Expected this to be\n\t",
        "'(number?) -> number'",
        "\nbut got\n\t",
        "'((nil) -> never) & ((number) -> number)'",
        "; \nthis is because \n\t",
        " * in the 1st component of the intersection, the function takes the 1st entry in the type pack which is `number` and it takes the ",
        "1st ",
        "entry in the type pack has the 2nd component of the union as `nil`, and `number` is not a supertype of `nil`\n\t",
        " * in the 2nd component of the intersection, the function takes the 1st entry in the type pack which is `nil` and it takes the ",
        "1st ",
        "entry in the type pack has the 1st component of the union as `number`, and `nil` is not a supertype of `number`"
      );
      let expected2 = concat!(
        "Expected this to be\n\t",
        "'(number?) -> never'",
        "\nbut got\n\t",
        "'((nil) -> never) & ((number) -> number)'",
        "; \nthis is because \n\t",
        " * in the 1st component of the intersection, the function returns the 1st entry in the type pack which is `number` and it returns ",
        "the ",
        "1st entry in the type pack is `never`, and `number` is not a subtype of `never`\n\t",
        " * in the 1st component of the intersection, the function takes the 1st entry in the type pack which is `number` and it takes the ",
        "1st ",
        "entry in the type pack has the 2nd component of the union as `nil`, and `number` is not a supertype of `nil`\n\t",
        " * in the 2nd component of the intersection, the function takes the 1st entry in the type pack which is `nil` and it takes the ",
        "1st ",
        "entry in the type pack has the 1st component of the union as `number`, and `nil` is not a supertype of `number`"
      );

      assert_eq!(expected1, to_string_type_error(&result.errors[0]));
      assert_eq!(expected2, to_string_type_error(&result.errors[1]));
    } else {
      assert_eq!(1, result.errors.len(), "{:?}", result.errors);
      let expected = r#"Expected this to be
	'(number?) -> never'
but got
	'((nil) -> never) & ((number) -> number)'; none of the intersection parts are compatible"#;
      assert_eq!(expected, to_string_type_error(&result.errors[0]));
    }
  }
}

mod type_infer_intersection_types_overloadeded_functions_with_overlapping_results_and_variadics {
  //! Ported from `tests/TypeInfer.intersectionTypes.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_intersection_types_overloadeded_functions_with_overlapping_results_and_variadics() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_unit_test::records::fixture::Fixture;

    ulua_unit_test::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function f(x : ((string?) -> (string | number)) & ((number?) -> ...number))
            local y : ((nil) -> (number, number?)) = x -- OK
            local z : ((string | number) -> (number, number?)) = x -- Not OK
        end
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    let expected = concat!(
      "Expected this to be\n\t",
      "'(number | string) -> (number, number?)'",
      "\nbut got\n\t",
      "'((number?) -> (...number)) & ((string?) -> number | string)'",
      "; none of the intersection parts are compatible"
    );
    assert_eq!(expected, to_string_type_error(&result.errors[0]));
  }
}

mod type_infer_intersection_types_overloadeded_functions_with_unknown_arguments {
  //! Ported from `tests/TypeInfer.intersectionTypes.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_intersection_types_overloadeded_functions_with_unknown_arguments() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_unit_test::records::fixture::Fixture;

    ulua_unit_test::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function f<a...,b...>()
            function g(x : ((number) -> number?) & ((unknown) -> string?))
                local y : (number) -> nil = x -- OK
                local z : (number?) -> nil = x -- Not OK
            end
        end
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    let expected = concat!(
      "Expected this to be\n\t",
      "'(number?) -> nil'",
      "\nbut got\n\t",
      "'((number) -> number?) & ((unknown) -> string?)'",
      "; none of the intersection parts are compatible"
    );
    assert_eq!(expected, to_string_type_error(&result.errors[0]));
  }
}

mod type_infer_intersection_types_overloadeded_functions_with_unknown_result {
  //! Ported from `tests/TypeInfer.intersectionTypes.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_intersection_types_overloadeded_functions_with_unknown_result() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_unit_test::records::fixture::Fixture;

    ulua_unit_test::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function f<a...,b...>()
            function g(x : ((number) -> number) & ((nil) -> unknown))
                local y : (number?) -> unknown = x -- OK
                local z : (number?) -> number? = x -- Not OK
            end
        end
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    let expected = concat!(
      "Expected this to be\n\t",
      "'(number?) -> number?'",
      "\nbut got\n\t",
      "'((nil) -> unknown) & ((number) -> number)'",
      "; none of the intersection parts are compatible"
    );
    assert_eq!(expected, to_string_type_error(&result.errors[0]));
  }
}

mod type_infer_intersection_types_overloadeded_functions_with_weird_typepacks_1 {
  //! Ported from `tests/TypeInfer.intersectionTypes.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_intersection_types_overloadeded_functions_with_weird_typepacks_1() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_common::FFlag;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function f<a...,b...>()
            function g(x : (() -> a...) & (() -> b...))
                local y : (() -> b...) & (() -> a...) = x -- OK
                local z : () -> () = x -- Not OK
            end
        end
    "#,
      ),
      None,
    );

    if !FFlag::DebugLuauForceOldSolver.get() {
      assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    } else {
      assert_eq!(1, result.errors.len(), "{:?}", result.errors);
      assert_eq!(
        "Expected this to be '() -> ()', but got '(() -> (a...)) & (() -> (b...))'; none of the intersection parts are compatible",
        to_string_type_error(&result.errors[0])
      );
    }
  }
}

mod type_infer_intersection_types_overloadeded_functions_with_weird_typepacks_2 {
  //! Ported from `tests/TypeInfer.intersectionTypes.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_intersection_types_overloadeded_functions_with_weird_typepacks_2() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::{
        get_error::get_type_error, to_string_error::to_string_type_error,
        to_string_to_string_alt_c::to_string_type_id,
      },
      records::type_mismatch::TypeMismatch,
    };
    use ulua_common::FFlag;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function f<a...,b...>()
            function g(x : ((a...) -> ()) & ((b...) -> ()))
                local y : ((b...) -> ()) & ((a...) -> ()) = x -- OK
                local z : () -> () = x -- Not OK
            end
        end
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);

    if !FFlag::DebugLuauForceOldSolver.get() {
      let err = get_type_error::<TypeMismatch>(&result.errors[0]).expect("expected TypeMismatch");
      assert_eq!("() -> ()", to_string_type_id(err.wanted_type));
      assert_eq!(
        "((a...) -> ()) & ((b...) -> ())",
        to_string_type_id(err.given_type)
      );
    } else {
      assert_eq!(
        "Expected this to be '() -> ()', but got '((a...) -> ()) & ((b...) -> ())'; none of the intersection parts are compatible",
        to_string_type_error(&result.errors[0])
      );
    }
  }
}

mod type_infer_intersection_types_overloadeded_functions_with_weird_typepacks_3 {
  //! Ported from `tests/TypeInfer.intersectionTypes.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_intersection_types_overloadeded_functions_with_weird_typepacks_3() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::{
        get_error::get_type_error, to_string_error::to_string_type_error,
        to_string_to_string_alt_c::to_string_type_id,
      },
      records::type_mismatch::TypeMismatch,
    };
    use ulua_common::FFlag;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function f<a...>()
            function g(x : (() -> a...) & (() -> (number?,a...)))
                local y : (() -> (number?,a...)) & (() -> a...) = x -- OK
                local z : () -> (number) = x -- Not OK
            end
        end
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);

    if !FFlag::DebugLuauForceOldSolver.get() {
      let err = get_type_error::<TypeMismatch>(&result.errors[0]).expect("expected TypeMismatch");
      assert_eq!("() -> number", to_string_type_id(err.wanted_type));
      assert_eq!(
        "(() -> (a...)) & (() -> (number?, a...))",
        to_string_type_id(err.given_type)
      );
    } else {
      let expected = r#"Expected this to be
	'() -> number'
but got
	'(() -> (a...)) & (() -> (number?, a...))'; none of the intersection parts are compatible"#;
      assert_eq!(expected, to_string_type_error(&result.errors[0]));
    }
  }
}

mod type_infer_intersection_types_overloadeded_functions_with_weird_typepacks_4 {
  //! Ported from `tests/TypeInfer.intersectionTypes.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_intersection_types_overloadeded_functions_with_weird_typepacks_4() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::{
        get_error::get_type_error, to_string_error::to_string_type_error,
        to_string_to_string_alt_c::to_string_type_id,
      },
      records::type_mismatch::TypeMismatch,
    };
    use ulua_common::FFlag;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function f<a...>()
            function g(x : ((a...) -> ()) & ((number,a...) -> number))
                local y : ((number,a...) -> number) & ((a...) -> ()) = x -- OK
                local z : (number?) -> () = x -- Not OK
            end
        end
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);

    if !FFlag::DebugLuauForceOldSolver.get() {
      let err = get_type_error::<TypeMismatch>(&result.errors[0]).expect("expected TypeMismatch");
      assert_eq!("(number?) -> ()", to_string_type_id(err.wanted_type));
      assert_eq!(
        "((a...) -> ()) & ((number, a...) -> number)",
        to_string_type_id(err.given_type)
      );
      let expected = concat!(
        "Expected this to be\n\t",
        "'(number?) -> ()'",
        "\nbut got\n\t",
        "'((a...) -> ()) & ((number, a...) -> number)'",
        "; \nthis is because \n\t",
        " * in the 1st component of the intersection, the function takes a tail of `a...` and it takes the portion of the type pack starting at ",
        "index 0 to the end`number?`, and `a...` is not a supertype of `number?`\n\t",
        " * in the 2nd component of the intersection, the function returns is `number` and it returns `()`, and `number` is not a subtype of ",
        "`()`\n\t",
        " * in the 2nd component of the intersection, the function takes a tail of `a...` and it takes `number?`, and `a...` is not a ",
        "supertype ",
        "of `number?`\n\t",
        " * in the 2nd component of the intersection, the function takes the 1st entry in the type pack which is `number` and it takes the ",
        "1st ",
        "entry in the type pack has the 2nd component of the union as `nil`, and `number` is not a supertype of `nil`"
      );
      assert_eq!(expected, to_string_type_error(&result.errors[0]));
    } else {
      let expected = r#"Expected this to be
	'(number?) -> ()'
but got
	'((a...) -> ()) & ((number, a...) -> number)'; none of the intersection parts are compatible"#;
      assert_eq!(expected, to_string_type_error(&result.errors[0]));
    }
  }
}

mod type_infer_intersection_types_propagates_name {
  //! Ported from `tests/TypeInfer.intersectionTypes.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_intersection_types_propagates_name() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let code = String::from(
      r#"
        type A={a:number}
        type B={b:string}

        local function f(t: A & B)
            return t
        end
    "#,
    );

    let expected = String::from(
      r#"
        type A={a:number}
        type B={b:string}

        local function f(t: A & B): A&B
            return t
        end
    "#,
    );

    assert_eq!(expected, fixture.decorate_with_types(&code));
  }
}

mod type_infer_intersection_types_select_correct_union_fn {
  //! Ported from `tests/TypeInfer.intersectionTypes.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_intersection_types_select_correct_union_fn() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type A = (number) -> (string)
        type B = (string) -> (number)

        local function foo(f: A & B)
            return f(10), f("a")
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "(((number) -> string) & ((string) -> number)) -> (string, number)",
      to_string_type_id(fixture.require_type_string(&String::from("foo")))
    );
  }
}

mod type_infer_intersection_types_should_still_pick_an_overload_whose_arguments_are_unions {
  //! Ported from `tests/TypeInfer.intersectionTypes.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_intersection_types_should_still_pick_an_overload_whose_arguments_are_unions() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type A = (number) -> string
        type B = (string) -> number

        local function foo(f: A & B)
            return f(1), f("five")
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "(((number) -> string) & ((string) -> number)) -> (string, number)",
      to_string_type_id(fixture.require_type_string(&String::from("foo")))
    );
  }
}

mod type_infer_intersection_types_table_combines {
  //! Ported from `tests/TypeInfer.intersectionTypes.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_intersection_types_table_combines() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type A={a:number}
        type B={b:string}

        local c:A & B = {a=10, b="s"}
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_intersection_types_table_combines_missing {
  //! Ported from `tests/TypeInfer.intersectionTypes.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_intersection_types_table_combines_missing() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type A={a:number}
        type B={b:string}

        local c:A & B = {a=10}
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_intersection_types_table_extra_ok {
  //! Ported from `tests/TypeInfer.intersectionTypes.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_intersection_types_table_extra_ok() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type A={a:number}
        type B={b:string}

        local function f(t: A & B): A
            return t
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_intersection_types_table_intersection_setmetatable {
  //! Ported from `tests/TypeInfer.intersectionTypes.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_intersection_types_table_intersection_setmetatable() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        function f(t: {} & {})
            setmetatable(t, {})
        end
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_intersection_types_table_intersection_write {
  //! Ported from `tests/TypeInfer.intersectionTypes.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_intersection_types_table_intersection_write() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);

    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type X = { x: number }
        type XY = X & { y: number }

        function f(t: XY)
            t.x = 10
        end
    "#,
      ),
      None,
    );
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type X = {}
        type XY = X & { x: number, y: number }

        function f(t: XY)
            t.x = 10
        end
    "#,
      ),
      None,
    );
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type X = { x: number }
        type Y = { y: number }
        type XY = X & Y

        function f(t: XY)
            t.x = 10
        end
    "#,
      ),
      None,
    );
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type A = { x: {y: number} }
        type B = { x: {y: number} }

        function f(t: A & B)
            t.x = { y = 4 }
            t.x.y = 40
        end
    "#,
      ),
      None,
    );
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_intersection_types_table_intersection_write_sealed {
  //! Ported from `tests/TypeInfer.intersectionTypes.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_intersection_types_table_intersection_write_sealed() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type X = { x: number }
        type Y = { y: number }
        type XY = X & Y

        function f(t: XY)
            t.z = 10
        end
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "Cannot add property 'z' to table 'X & Y'",
      to_string_type_error(&result.errors[0])
    );
  }
}

mod type_infer_intersection_types_table_intersection_write_sealed_indirect {
  //! Ported from `tests/TypeInfer.intersectionTypes.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_intersection_types_table_intersection_write_sealed_indirect() {
    use alloc::string::String;

    use ulua_analysis::{
      functions::{
        get_error::get_type_error, to_string_error::to_string_type_error,
        to_string_to_string_alt_c::to_string_type_id,
      },
      records::type_mismatch::TypeMismatch,
    };
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _sff = ScopedFastFlag::new(&FFlag::LuauCheckFunctionStatementTypes, true);

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type X = { x: (number) -> number }
        type Y = { y: (string) -> string }

        type XY = X & Y

        function f(t: XY)
            function t.z(a:number) return a * 10 end
            function t:y(a:number) return a * 10 end
            function t:w(a:number) return a * 10 end
        end
    "#,
      ),
      None,
    );

    assert_eq!(4, result.errors.len(), "{:?}", result.errors);
    if !FFlag::DebugLuauForceOldSolver.get() {
      assert_eq!(
        "Cannot add property 'z' to table 'X & Y'",
        to_string_type_error(&result.errors[0])
      );
      let err1 = get_type_error::<TypeMismatch>(&result.errors[1]).expect("expected TypeMismatch");
      assert_eq!("number", to_string_type_id(err1.given_type));
      assert_eq!("string", to_string_type_id(err1.wanted_type));
      let err2 = get_type_error::<TypeMismatch>(&result.errors[2]).expect("expected TypeMismatch");
      assert_eq!(
        "(string, number) -> string",
        to_string_type_id(err2.given_type)
      );
      assert_eq!("(string) -> string", to_string_type_id(err2.wanted_type));
      assert_eq!(
        "Cannot add property 'w' to table 'X & Y'",
        to_string_type_error(&result.errors[3])
      );
    } else {
      let expected = concat!(
        "Expected this to be\n\t",
        "'(string) -> string'",
        "\nbut got\n\t",
        "'(string, number) -> string'",
        "\ncaused by:\n",
        "  Argument count mismatch. Function expects 2 arguments, but only 1 is specified"
      );

      assert_eq!(expected, to_string_type_error(&result.errors[0]));
      assert_eq!(
        "Cannot add property 'z' to table 'X & Y'",
        to_string_type_error(&result.errors[1])
      );
      assert_eq!(
        "Expected this to be 'string', but got 'number'",
        to_string_type_error(&result.errors[2])
      );
      assert_eq!(
        "Cannot add property 'w' to table 'X & Y'",
        to_string_type_error(&result.errors[3])
      );
    }
  }
}

mod type_infer_intersection_types_table_write_sealed_indirect {
  //! Ported from `tests/TypeInfer.intersectionTypes.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_intersection_types_table_write_sealed_indirect() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_unit_test::records::fixture::Fixture;

    ulua_unit_test::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
    type XY = { x: (number) -> number, y: (string) -> string }

    local xy : XY = {
        x = function(a: number) return -a end,
        y = function(a: string) return a .. "b" end
    }
    function xy.z(a:number) return a * 10 end
    function xy:y(a:number) return a * 10 end
    function xy:w(a:number) return a * 10 end
    "#,
      ),
      None,
    );

    assert_eq!(4, result.errors.len(), "{:?}", result.errors);
    let expected = concat!(
      "Expected this to be\n\t",
      "'(string) -> string'",
      "\nbut got\n\t",
      "'(string, number) -> string'",
      "\ncaused by:\n",
      "  Argument count mismatch. Function expects 2 arguments, but only 1 is specified"
    );
    assert_eq!(expected, to_string_type_error(&result.errors[0]));
    assert_eq!(
      "Cannot add property 'z' to table 'XY'",
      to_string_type_error(&result.errors[1])
    );
    assert_eq!(
      "Expected this to be 'string', but got 'number'",
      to_string_type_error(&result.errors[2])
    );
    assert_eq!(
      "Cannot add property 'w' to table 'XY'",
      to_string_type_error(&result.errors[3])
    );
  }
}

mod type_infer_intersection_types_union_saturate_overloaded_functions {
  //! Ported from `tests/TypeInfer.intersectionTypes.test.cpp`.

  #[cfg(test)]
  #[test]
  fn type_infer_intersection_types_union_saturate_overloaded_functions() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_error::to_string_type_error;
    use ulua_unit_test::records::fixture::Fixture;

    ulua_unit_test::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        function f(x: ((number) -> number) & ((string) -> string))
            local y : ((number | string) -> (number | string)) = x -- OK
            local z : ((number | boolean) -> (number | boolean)) = x -- Not OK
        end
    "#,
      ),
      None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    let expected = concat!(
      "Expected this to be\n\t",
      "'(boolean | number) -> boolean | number'",
      "\nbut got\n\t",
      "'((number) -> number) & ((string) -> string)'",
      "; none of the intersection parts are compatible"
    );
    assert_eq!(expected, to_string_type_error(&result.errors[0]));
  }
}
