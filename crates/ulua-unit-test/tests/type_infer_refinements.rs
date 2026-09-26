extern crate alloc;

// 406 处 test fn 体内逐例重复的 use 统一上提至此（借 tst-r16 pretty_printer/fragment_autocomplete 上提先例；源头 cpp 侧即为整文件共享的 using 声明）。
use alloc::string::String;

use ulua_analysis::{
  enums::solver_mode::SolverMode,
  functions::{
    to_string_error::to_string_type_error,
    to_string_to_string::{to_string_type_id, to_string_type_id_to_string_options},
  },
  records::{
    arena_handle::Handle, internal_error_reporter::InternalErrorReporter, normalizer::Normalizer,
    optional_value_access::OptionalValueAccess, to_string_options::ToStringOptions,
    type_arena::TypeArena, type_mismatch::TypeMismatch, unifier_shared_state::UnifierSharedState,
    unknown_property::UnknownProperty,
  },
};
use ulua_ast::{
  enums::mode::Mode,
  records::{location::Location, position::Position},
};
use ulua_common::fflag;
use ulua_unit_test::{
  functions::type_error_data_ref::type_error_data_ref,
  records::{
    builtins_fixture::BuiltinsFixture, fixture::Fixture,
    refinement_extern_type_fixture::RefinementExternTypeFixture,
  },
  type_aliases::scoped_fast_flag::ScopedFastFlag,
};

// 样板收口助手：原逐例重复的 BuiltinsFixture/Fixture 构造+get_frontend+
// 默认选项检查语句收口为宏，行为与原语句逐字一致（借 tst-r16 ir_lowering 助手先例）。
macro_rules! bs_check {
  ($src:expr) => {{
    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture
      .base
      .check_string_optional_frontend_options($src, None);
    (fixture, result)
  }};
}

macro_rules! fx_check {
  ($src:expr) => {{
    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options($src, None);
    (fixture, result)
  }};
}

// Source: `tests/TypeInfer.refinements.test.cpp`
#[test]
fn type_infer_refinements_a_and_b_or_a_and_c() {
  let (mut fixture, result) = fx_check!(
    r#"
        function f(a: string?, b: number?, c: boolean)
            if (a and b) or (a and c) then
                local foo = a
                local bar = b
                local baz = c
            else
                local foo = a
                local bar = b
                local baz = c
            end
        end
    "#
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  assert_eq!(
    "string",
    to_string_type_id(fixture.require_type_at_position_position(Position::new(3, 28)))
  );
  assert_eq!(
    "number?",
    to_string_type_id(fixture.require_type_at_position_position(Position::new(4, 28)))
  );
  if !fflag::DebugLuauForceOldSolver.get() {
    assert_eq!(
      "boolean",
      to_string_type_id(fixture.require_type_at_position_position(Position::new(5, 28)))
    );
  } else {
    assert_eq!(
      "true",
      to_string_type_id(fixture.require_type_at_position_position(Position::new(5, 28)))
    );
  }
  assert_eq!(
    "string?",
    to_string_type_id(fixture.require_type_at_position_position(Position::new(7, 28)))
  );
  assert_eq!(
    "number?",
    to_string_type_id(fixture.require_type_at_position_position(Position::new(8, 28)))
  );
  assert_eq!(
    "boolean",
    to_string_type_id(fixture.require_type_at_position_position(Position::new(9, 28)))
  );
}

// Source: `tests/TypeInfer.refinements.test.cpp`
#[test]
fn type_infer_refinements_and_constraint() {
  let (mut fixture, result) = fx_check!(
    r#"
        function f(a: string?, b: number?)
            if a and b then
                local x = a
                local y = b
            else
                local x = a
                local y = b
            end
        end
    "#
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  assert_eq!(
    "string",
    to_string_type_id(fixture.require_type_at_position_position(Position::new(3, 26)))
  );
  assert_eq!(
    "number",
    to_string_type_id(fixture.require_type_at_position_position(Position::new(4, 26)))
  );
  assert_eq!(
    "string?",
    to_string_type_id(fixture.require_type_at_position_position(Position::new(6, 26)))
  );
  assert_eq!(
    "number?",
    to_string_type_id(fixture.require_type_at_position_position(Position::new(7, 26)))
  );
}

// Source: `tests/TypeInfer.refinements.test.cpp`
#[test]
fn type_infer_refinements_and_or_peephole_refinement() {
  let (_fixture, result) = fx_check!(
    r#"
        local function len(a: {any})
            return a and #a or nil
        end
    "#
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.refinements.test.cpp`
#[test]
fn type_infer_refinements_apply_refinements_on_astexprindexexpr_whose_subscript_expr_is_constant_string()
 {
  let (_fixture, result) = fx_check!(
    r#"
        type T = { [string]: { prop: number }? }
        local t: T = {}

        if t["hello"] then
            local foo = t["hello"].prop
        end
    "#
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.refinements.test.cpp`
#[test]
fn type_infer_refinements_assert_a_to_be_truthy_then_assert_a_to_be_number() {
  let (mut fixture, result) = bs_check!(
    r#"
        local a: (number | string)?
        assert(a)
        local b = a
        assert(type(a) == "number")
        local c = a
    "#
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);

  assert_eq!(
    "number | string",
    to_string_type_id(
      fixture
        .base
        .require_type_at_position_position(Position::new(3, 18))
    )
  );
  assert_eq!(
    "number",
    to_string_type_id(
      fixture
        .base
        .require_type_at_position_position(Position::new(5, 18))
    )
  );
}

// Source: `tests/TypeInfer.refinements.test.cpp`
#[test]
fn type_infer_refinements_assert_and_typeof_refinement_context() {
  let _new_solver = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);

  let (_fixture, result) = bs_check!(
    r#"
        --!strict

        local x = {} :: unknown

        if typeof(x) == "table" then
            assert(typeof(x.transform) == "function")
        end
    "#
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.refinements.test.cpp`
#[test]
fn type_infer_refinements_assert_call_should_not_refine_despite_typeof() {
  let _new_solver = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);

  let (_fixture, result) = bs_check!(
    r#"
        --!strict
        local function foo(_: any)
            return true
        end

        local function f(x: unknown)
            if typeof(x) == "table" then
                assert(foo(typeof(x.bar)))
            end
        end
    "#
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  assert_eq!(
    "Type 'table' does not have key 'bar'",
    to_string_type_error(&result.errors[0])
  );
}

// Source: `tests/TypeInfer.refinements.test.cpp`
#[test]
fn type_infer_refinements_assert_non_binary_expressions_actually_resolve_constraints() {
  let (_fixture, result) = bs_check!(
    r#"
        local foo: string? = "hello"
        assert(foo)
        local bar: string = foo
    "#
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.refinements.test.cpp`
#[test]
fn type_infer_refinements_asserting_non_existent_properties_should_not_refine_extern_types_to_never()
 {
  let mut fixture = RefinementExternTypeFixture {
    base: BuiltinsFixture::default(),
  };
  fixture.get_frontend();
  let result = fixture.base.base.check_string_optional_frontend_options(
    r#"
        local weld: WeldConstraint = nil :: any
        assert(weld.Part8)
        print(weld)
        assert(weld.Part8.Name == "RootPart")
        local part8 = assert(weld.Part8)
        local pos = part8.Position
    "#,
    None,
  );

  assert!(!result.errors.is_empty(), "{:?}", result.errors);
  assert_eq!(
    to_string_type_error(&result.errors[0]),
    "Key 'Part8' not found in external type 'WeldConstraint'"
  );

  assert_eq!(
    "WeldConstraint",
    to_string_type_id(
      fixture
        .base
        .base
        .require_type_at_position_position(Position::new(3, 15))
    )
  );
  if !fflag::DebugLuauForceOldSolver.get() {
    assert_eq!(
      "any",
      to_string_type_id(
        fixture
          .base
          .base
          .require_type_at_position_position(Position::new(6, 29))
      )
    );
  } else {
    assert_eq!(
      "*error-type*",
      to_string_type_id(
        fixture
          .base
          .base
          .require_type_at_position_position(Position::new(6, 29))
      )
    );
  }
}

// Source: `tests/TypeInfer.refinements.test.cpp`
#[test]
fn type_infer_refinements_asserting_optional_properties_should_not_refine_extern_types_to_never() {
  let mut fixture = RefinementExternTypeFixture {
    base: BuiltinsFixture::default(),
  };
  fixture.get_frontend();
  let result = fixture.base.base.check_string_optional_frontend_options(
    r#"
        local weld: WeldConstraint = nil :: any
        assert(weld.Part1)
        print(weld) -- hover type incorrectly becomes `never`
        assert(weld.Part1.Name == "RootPart")
        local part1 = assert(weld.Part1)
        local pos = part1.Position
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);

  if !fflag::DebugLuauForceOldSolver.get() && fflag::LuauExternTypesNormalizeWithShapes.get() {
    assert_eq!(
      "WeldConstraint & { read Part1: ~(false?) }",
      to_string_type_id(
        fixture
          .base
          .base
          .require_type_at_position_position(Position::new(3, 15))
      )
    );
  } else {
    assert_eq!(
      "WeldConstraint",
      to_string_type_id(
        fixture
          .base
          .base
          .require_type_at_position_position(Position::new(3, 15))
      )
    );
  }
  assert_eq!(
    "Vector3",
    to_string_type_id(
      fixture
        .base
        .base
        .require_type_at_position_position(Position::new(6, 29))
    )
  );
}

// Source: `tests/TypeInfer.refinements.test.cpp`
#[test]
fn type_infer_refinements_call_an_incompatible_function_after_using_typeguard() {
  let (_fixture, result) = bs_check!(
    r#"
        local function f(x: number)
            return x
        end

        local function g(x: unknown)
            if type(x) == "string" then
                f(x)
            end
        end

        local function h(x: any)
            if type(x) == "string" then
                f(x)
            end
        end
    "#
  );

  if !fflag::DebugLuauForceOldSolver.get() {
    assert_eq!(1, result.errors.len(), "{:?}", result.errors);

    assert_eq!(
      "Expected this to be 'number', but got 'string'",
      to_string_type_error(&result.errors[0])
    );
    assert_eq!(
      Location::new(Position::new(7, 18), Position::new(7, 19)),
      result.errors[0].location
    );
  } else {
    assert_eq!(2, result.errors.len(), "{:?}", result.errors);

    assert_eq!(
      "Expected this to be 'number', but got 'string'",
      to_string_type_error(&result.errors[0])
    );
    assert_eq!(
      Location::new(Position::new(7, 18), Position::new(7, 19)),
      result.errors[0].location
    );

    assert_eq!(
      "Expected this to be 'number', but got 'string'",
      to_string_type_error(&result.errors[1])
    );
    assert_eq!(
      Location::new(Position::new(13, 18), Position::new(13, 19)),
      result.errors[1].location
    );
  }
}

// Source: `tests/TypeInfer.refinements.test.cpp`
#[test]
fn type_infer_refinements_call_to_undefined_method_is_not_a_refinement() {
  let _old_solver = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);
  let (_fixture, result) = bs_check!(
    r#"
        local function f(x: unknown)
            if typeof(x) == "table" then
                if x.foo() then
                end
            end
            return (nil :: never)
        end
    "#
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);

  let unknown_prop = type_error_data_ref::<UnknownProperty>(&result.errors[0])
    .unwrap_or_else(|| panic!("expected UnknownProperty, got {:?}", result.errors[0]));
  assert_eq!("foo", unknown_prop.key());
  assert_eq!("table", to_string_type_id(unknown_prop.table()));

  assert_eq!(
    Location::new(Position::new(3, 19), Position::new(3, 24)),
    result.errors[0].location
  );
}

// Source: `tests/TypeInfer.refinements.test.cpp`
#[test]
fn type_infer_refinements_cannot_call_a_function_single() {
  let _sff = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);

  let mut fixture = RefinementExternTypeFixture {
    base: BuiltinsFixture::default(),
  };
  fixture.get_frontend();
  let result = fixture.base.base.check_string_optional_frontend_options(
    r#"
        local function invokeDisconnect(d: unknown)
            if type(d) == "function" then
                d()
            end
        end
    "#,
    None,
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  assert_eq!(
    "The type function is not precise enough for us to determine the appropriate result type of this call.",
    to_string_type_error(&result.errors[0])
  );
}

// Source: `tests/TypeInfer.refinements.test.cpp`
#[test]
fn type_infer_refinements_cannot_call_a_function_union() {
  let _sff = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);

  let mut fixture = RefinementExternTypeFixture {
    base: BuiltinsFixture::default(),
  };
  fixture.get_frontend();
  let result = fixture.base.base.check_string_optional_frontend_options(
    r#"
        type Disconnectable = {
            Disconnect: (self: Disconnectable) -> (...any);
        } | {
            disconnect: (self: Disconnectable) -> (...any)
        } | ExternScriptConnection

        local x: Disconnectable = workspace.ChildAdded:Connect(function()
            print("child added")
        end)

        if type(x.Disconnect) == "function" then
            x:Disconnect()
        end
    "#,
    None,
  );

  assert_eq!(2, result.errors.len(), "{:?}", result.errors);

  // FIXME CLI-157125: It's a bit clowny that we return a union of
  // functions containing `function` here, but it looks like a side
  // effect of how we execute `hasProp`.
  let expected_error = String::from("Cannot call a value of type function in union:\n")
    + "  ((ExternScriptConnection) -> ()) | function | t2 where t1 = ExternScriptConnection | { Disconnect: t2 } | { "
    + "disconnect: (t1) -> (...any) } ; t2 = (t1) -> (...any)";

  assert_eq!(to_string_type_error(&result.errors[1]), expected_error);
}

// Source: `tests/TypeInfer.refinements.test.cpp`
#[test]
fn type_infer_refinements_cat_or_dog_through_a_local() {
  let (mut fixture, result) = fx_check!(
    r#"
        type Cat = { tag: "cat", catfood: string }
        type Dog = { tag: "dog", dogfood: string }
        type Animal = Cat | Dog

        local function f(animal: Animal)
            local tag = animal.tag
            if tag == "dog" then
                local dog = animal
            elseif tag == "cat" then
                local cat = animal
            end
        end
    "#
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);

  assert_eq!(
    "Cat | Dog",
    to_string_type_id(fixture.require_type_at_position_position(Position::new(8, 28)))
  );
  assert_eq!(
    "Cat | Dog",
    to_string_type_id(fixture.require_type_at_position_position(Position::new(10, 28)))
  );
}

// Source: `tests/TypeInfer.refinements.test.cpp`
#[test]
fn type_infer_refinements_check_refinement_to_primitive_and_compare() {
  let _new_solver = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);
  let (mut fixture, result) = bs_check!(
    r#"
        local function comesAfterLuau(word)
            return type(word) == "string" and word > "luau"
        end
    "#
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  assert_eq!(
    "(unknown) -> boolean",
    to_string_type_id(fixture.base.require_type_string("comesAfterLuau"))
  );
}

// Source: `tests/TypeInfer.refinements.test.cpp`
#[test]
fn type_infer_refinements_cli_120460_table_access_on_phi_node() {
  let (_fixture, result) = fx_check!(
    r#"
        --!strict
        local function foo(bar: string): string
            local baz: boolean = true
            if baz then
                local _ = (bar:sub(1))
            else
                local _ = (bar:sub(1))
            end
            return bar:sub(2) -- previously this would be `...never`
        end
    "#
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.refinements.test.cpp`
#[test]
fn type_infer_refinements_cli_140033_refine_union_of_extern_types() {
  let mut fixture = RefinementExternTypeFixture {
    base: BuiltinsFixture::default(),
  };
  fixture.get_frontend();
  let result = fixture.base.base.check_string_optional_frontend_options(
    r#"
        --!strict
        local function getImageLabel(vars: { Instance }): Folder | Part | nil
            for _, item in vars do
                if item:IsA("Folder") or item:IsA("Part") then
                    return item
                end
            end
            return nil
        end
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);

  assert_eq!(
    "Folder | Part",
    to_string_type_id(
      fixture
        .base
        .base
        .require_type_at_position_position(Position::new(5, 28))
    )
  );
}

// Source: `tests/TypeInfer.refinements.test.cpp`
#[test]
fn type_infer_refinements_cli_181100_fast_track_refinement_against_unknown() {
  let _new_solver = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);
  let _assert_on_forced_constraint =
    ScopedFastFlag::new(&fflag::DebugLuauAssertOnForcedConstraint, true);

  let (mut fixture, result) = bs_check!(
    r#"
        --!strict

        local Class = {}
        Class.__index = Class

        type Class = setmetatable<{ A: number }, typeof(Class)>

        function Class.Foo(x: Class, y: Class, z: Class)
            if y == z then
                return
            end
            local bar = y.A
            print(bar)
        end
    "#
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  assert_eq!(
    "number",
    to_string_type_id(
      fixture
        .base
        .require_type_at_position_position(Position::new(13, 19))
    )
  );
}

// Source: `tests/TypeInfer.refinements.test.cpp`
#[test]
fn type_infer_refinements_cli_181549_refined_string_should_be_subtype_of_string() {
  let _new_solver = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);

  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();
  let result = fixture.base.check_mode_string_optional_frontend_options(
    Mode::Nonstrict,
    r#"
      local hello : string = "world"

      if hello == "" then
          return
      end

      string.find(hello, "bye")
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.refinements.test.cpp`
#[test]
fn type_infer_refinements_cli_184413_refinement_of_union_of_read_types_is_read_type() {
  let (_fixture, result) = fx_check!(
    r#"
        export type States = "Closed" | "Closing" | "Opening" | "Open"
        export type MyType<A = any> = {
            State: States,
            IsOpen: boolean,
            Open: (self: MyType<A>) -> (),
        }

        local value = {} :: MyType

        function value:Open()
            if self.IsOpen == true then
            elseif self.State == "Closing" or self.State == "Opening" then
                -- Prior, this line errored as we were erroneously refining
                -- `self` with `{ State: "Closing" | "Opening" }` rather
                -- than `{ read State: "Closing" | "Opening" }
                self:Open()
            end
        end
    "#
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.refinements.test.cpp`
#[test]
fn type_infer_refinements_conditional_refinement_should_stay_error_suppressing() {
  let _new_solver = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);
  let (_fixture, result) = bs_check!(
    r#"
        local function test(element: any?)
            if element then
                local owner = element._owner
            end
        end
    "#
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.refinements.test.cpp`
#[test]
fn type_infer_refinements_correctly_lookup_a_shadowed_local_that_which_was_previously_refined() {
  let (_fixture, result) = bs_check!(
    r#"
        local foo: string? = "hi"
        assert(foo)
        local foo: number = 5
        print(foo:sub(1, 1))
    "#
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);

  assert_eq!(
    "Type 'number' does not have key 'sub'",
    to_string_type_error(&result.errors[0])
  );
}

// Source: `tests/TypeInfer.refinements.test.cpp`
#[test]
fn type_infer_refinements_correctly_lookup_property_whose_base_was_previously_refined() {
  let (mut fixture, result) = bs_check!(
    r#"
        type T = {x: string | number}
        local t: T? = {x = "hi"}
        if t then
            if type(t.x) == "string" then
                local foo = t.x
            end
        end
    "#
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);

  assert_eq!(
    "string",
    to_string_type_id(
      fixture
        .base
        .require_type_at_position_position(Position::new(5, 30))
    )
  );
}

// Source: `tests/TypeInfer.refinements.test.cpp`
#[test]
fn type_infer_refinements_correctly_lookup_property_whose_base_was_previously_refined_2() {
  let (mut fixture, result) = fx_check!(
    r#"
        type T = { x: { y: number }? }

        local function f(t: T?)
            if t and t.x then
                local foo = t.x.y
            end
        end
    "#
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);

  assert_eq!(
    "number",
    to_string_type_id(fixture.require_type_at_position_position(Position::new(5, 32)))
  );
}

// Source: `tests/TypeInfer.refinements.test.cpp`
#[test]
fn type_infer_refinements_dataflow_analysis_can_tell_refinements_when_its_appropriate_to_refine_into_nil_or_never()
 {
  let (mut fixture, result) = bs_check!(
    r#"
        local function f(t: {string}, s: string)
            local v1 = t[5]
            local v2 = v1

            if typeof(v1) == "nil" then
                local foo = v1
            else
                local foo = v1
            end

            if typeof(v2) == "nil" then
                local foo = v2
            else
                local foo = v2
            end

            if typeof(s) == "nil" then
                local foo = s -- line 18
            else
                local foo = s -- line 20
            end
        end
    "#
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);

  assert_eq!(
    "nil",
    to_string_type_id(
      fixture
        .base
        .require_type_at_position_position(Position::new(6, 28))
    )
  );
  assert_eq!(
    "string",
    to_string_type_id(
      fixture
        .base
        .require_type_at_position_position(Position::new(8, 28))
    )
  );

  assert_eq!(
    "nil",
    to_string_type_id(
      fixture
        .base
        .require_type_at_position_position(Position::new(12, 28))
    )
  );
  assert_eq!(
    "string",
    to_string_type_id(
      fixture
        .base
        .require_type_at_position_position(Position::new(14, 28))
    )
  );

  if !fflag::DebugLuauForceOldSolver.get() {
    assert_eq!(
      "nil & string",
      to_string_type_id(
        fixture
          .base
          .require_type_at_position_position(Position::new(18, 28))
      )
    );
    assert_eq!(
      "string",
      to_string_type_id(
        fixture
          .base
          .require_type_at_position_position(Position::new(20, 28))
      )
    );
  } else {
    assert_eq!(
      "nil",
      to_string_type_id(
        fixture
          .base
          .require_type_at_position_position(Position::new(18, 28))
      )
    );
    assert_eq!(
      "string",
      to_string_type_id(
        fixture
          .base
          .require_type_at_position_position(Position::new(20, 28))
      )
    );
  }
}

// Source: `tests/TypeInfer.refinements.test.cpp`
#[test]
fn type_infer_refinements_discriminate_from_isa_of_x() {
  let mut fixture = RefinementExternTypeFixture {
    base: BuiltinsFixture::default(),
  };
  fixture.get_frontend();
  let result = fixture.base.base.check_string_optional_frontend_options(
    r#"
        type T = {tag: "Part", x: Part} | {tag: "Folder", x: Folder}

        local function f(t: T)
            if t.x:IsA("Part") then
                local foo = t
            else
                local bar = t
            end
        end
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);

  assert_eq!(
    r#"{ tag: "Part", x: Part }"#,
    to_string_type_id(
      fixture
        .base
        .base
        .require_type_at_position_position(Position::new(5, 28))
    )
  );
  assert_eq!(
    r#"{ tag: "Folder", x: Folder }"#,
    to_string_type_id(
      fixture
        .base
        .base
        .require_type_at_position_position(Position::new(7, 28))
    )
  );
}

// Source: `tests/TypeInfer.refinements.test.cpp`
#[test]
fn type_infer_refinements_discriminate_from_truthiness_of_x() {
  let (mut fixture, result) = fx_check!(
    r#"
        type T = {tag: "missing", x: nil} | {tag: "exists", x: string}

        local function f(t: T)
            if t.x then
                local foo = t
            else
                local bar = t
            end
        end
    "#
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);

  if !fflag::DebugLuauForceOldSolver.get() {
    assert_eq!(
      r#"{ tag: "exists", x: string }"#,
      to_string_type_id(fixture.require_type_at_position_position(Position::new(5, 28)))
    );
    assert_eq!(
      r#"{ tag: "missing", x: nil }"#,
      to_string_type_id(fixture.require_type_at_position_position(Position::new(7, 28)))
    );
  } else {
    assert_eq!(
      r#"{ tag: "exists", x: string }"#,
      to_string_type_id(fixture.require_type_at_position_position(Position::new(5, 28)))
    );
    assert_eq!(
      r#"{ tag: "exists", x: string } | { tag: "missing", x: nil }"#,
      to_string_type_id(fixture.require_type_at_position_position(Position::new(7, 28)))
    );
  }
}

// Source: `tests/TypeInfer.refinements.test.cpp`
#[test]
fn type_infer_refinements_discriminate_on_properties_of_disjoint_tables_where_that_property_is_true_or_false()
 {
  let (_fixture, result) = fx_check!(
    r#"
        type Ok<T> = { ok: true, value: T }
        type Err<E> = { ok: false, error: E }
        type Result<T, E> = Ok<T> | Err<E>

        local function apply<T, E>(t: Result<T, E>, f: (T) -> (), g: (E) -> ())
            if t.ok then
                f(t.value)
            else
                g(t.error)
            end
        end
    "#
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.refinements.test.cpp`
#[test]
fn type_infer_refinements_discriminate_tag() {
  let (mut fixture, result) = fx_check!(
    r#"
        type Cat = {tag: "Cat", name: string, catfood: string}
        type Dog = {tag: "Dog", name: string, dogfood: string}
        type Animal = Cat | Dog

        local function f(animal: Animal)
            if animal.tag == "Cat" then
                local cat = animal
            elseif animal.tag == "Dog" then
                local dog = animal
            end
        end
    "#
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);

  assert_eq!(
    "Cat",
    to_string_type_id(fixture.require_type_at_position_position(Position::new(7, 33)))
  );
  assert_eq!(
    "Dog",
    to_string_type_id(fixture.require_type_at_position_position(Position::new(9, 33)))
  );
}

// Source: `tests/TypeInfer.refinements.test.cpp`
#[test]
fn type_infer_refinements_discriminate_tag_with_implicit_else() {
  let (mut fixture, result) = fx_check!(
    r#"
        type Cat = {tag: "Cat", name: string, catfood: string}
        type Dog = {tag: "Dog", name: string, dogfood: string}
        type Animal = Cat | Dog

        local function f(animal: Animal)
            if animal.tag == "Cat" then
                local cat = animal
            else
                local dog = animal
            end
        end
    "#
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);

  assert_eq!(
    "Cat",
    to_string_type_id(fixture.require_type_at_position_position(Position::new(7, 33)))
  );
  assert_eq!(
    "Dog",
    to_string_type_id(fixture.require_type_at_position_position(Position::new(9, 33)))
  );
}

// Source: `tests/TypeInfer.refinements.test.cpp`
#[test]
fn type_infer_refinements_either_number_or_string() {
  let (mut fixture, result) = bs_check!(
    r#"
        local function f(x: any, y: unknown)
            if type(x) == "number" or type(x) == "string" then
                local foo = x
            end
            if type(y) == "number" or type(y) == "string" then
                local foo = y
            end
        end
    "#
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);

  if !fflag::DebugLuauForceOldSolver.get() {
    assert_eq!(
      "*error-type* | number | string",
      to_string_type_id(
        fixture
          .base
          .require_type_at_position_position(Position::new(3, 28))
      )
    );
  } else {
    assert_eq!(
      "number | string",
      to_string_type_id(
        fixture
          .base
          .require_type_at_position_position(Position::new(3, 28))
      )
    );
  }
  assert_eq!(
    "number | string",
    to_string_type_id(
      fixture
        .base
        .require_type_at_position_position(Position::new(6, 28))
    )
  );
}

// Source: `tests/TypeInfer.refinements.test.cpp`
#[test]
fn type_infer_refinements_eliminate_subclasses_of_instance() {
  let mut fixture = RefinementExternTypeFixture {
    base: BuiltinsFixture::default(),
  };
  fixture.get_frontend();
  let result = fixture.base.base.check_string_optional_frontend_options(
    r#"
        local function f(x: Part | Folder | string)
            if typeof(x) == "Instance" then
                local foo = x
            else
                local foo = x
            end
        end
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);

  assert_eq!(
    "Folder | Part",
    to_string_type_id(
      fixture
        .base
        .base
        .require_type_at_position_position(Position::new(3, 28))
    )
  );
  assert_eq!(
    "string",
    to_string_type_id(
      fixture
        .base
        .base
        .require_type_at_position_position(Position::new(5, 28))
    )
  );
}

// Source: `tests/TypeInfer.refinements.test.cpp`
#[test]
fn type_infer_refinements_else_with_no_explicit_expression_should_also_refine_the_tagged_union() {
  let (_fixture, result) = fx_check!(
    r#"
        type Ok<T> = { tag: "ok", value: T }
        type Err<E> = { tag: "err", err: E }
        type Result<T, E> = Ok<T> | Err<E>

        function and_then<T, U, E>(r: Result<T, E>, f: (T) -> U): Result<U, E>
            if r.tag == "ok" then
                return { tag = "ok", value = f(r.value) }
            else
                return r
            end
        end
    "#
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.refinements.test.cpp`
#[test]
fn type_infer_refinements_ensure_t_after_return_references_all_reachable_points() {
  let (mut fixture, result) = bs_check!(
    r#"
        local t = {}

        local function f(k: string)
            if t[k] ~= nil then
                return
            end

            t[k] = 5
        end
    "#
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);

  let mut opts = ToStringOptions::new(true);
  let ty = fixture
    .base
    .require_type_at_position_position(Position::new(8, 12));
  if !fflag::DebugLuauForceOldSolver.get() {
    assert_eq!(
      "{ [string]: number }",
      to_string_type_id_to_string_options(ty, &mut opts)
    );
  } else {
    assert_eq!(
      "{| [string]: number |}",
      to_string_type_id_to_string_options(ty, &mut opts)
    );
  }
}

// Source: `tests/TypeInfer.refinements.test.cpp`
#[test]
fn type_infer_refinements_ex() {
  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();
  let _result = fixture.base.check_string_optional_frontend_options(
    r#"
local function f(x: string | number)
    if typeof((x)) == "string" then
        local y = x
    end
end
"#,
    None,
  );

  let t = fixture
    .base
    .require_type_at_position_position(Position::new(3, 18));
  assert_eq!("string", to_string_type_id(t));
}

// Source: `tests/TypeInfer.refinements.test.cpp`
#[test]
fn type_infer_refinements_fail_to_refine_a_property_of_subscript_expression() {
  let (mut fixture, result) = fx_check!(
    r#"
        type Foo = { foo: number? }
        local function f(t: {Foo})
            if t[1].foo then
                local foo = t[1].foo
            end
        end
    "#
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  assert_eq!(
    "number?",
    to_string_type_id(fixture.require_type_at_position_position(Position::new(4, 34)))
  );
}

// Source: `tests/TypeInfer.refinements.test.cpp`
#[test]
fn type_infer_refinements_falsiness_of_truthy_predicate_narrows_into_nil() {
  let (mut fixture, result) = bs_check!(
    r#"
        local function f(t: {number})
            local x = t[1]
            if not x then
                local foo = x
            else
                local bar = x
            end
        end
    "#
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);

  assert_eq!(
    "nil",
    to_string_type_id(
      fixture
        .base
        .require_type_at_position_position(Position::new(4, 28))
    )
  );
  assert_eq!(
    "number",
    to_string_type_id(
      fixture
        .base
        .require_type_at_position_position(Position::new(6, 28))
    )
  );
}

// Source: `tests/TypeInfer.refinements.test.cpp`
#[test]
fn type_infer_refinements_foo_call_should_not_refine() {
  let _new_solver = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);

  let (_fixture, result) = bs_check!(
    r#"
        --!strict

        local x = {} :: unknown
        local function foo(_: boolean) end

        if typeof(x) == "table" then
            foo(typeof(x.transform) == "function")
        end
    "#
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  assert_eq!(
    "Type 'table' does not have key 'transform'",
    to_string_type_error(&result.errors[0])
  );
}

// Source: `tests/TypeInfer.refinements.test.cpp`
#[test]
fn type_infer_refinements_force_simplify_constraint_doesnt_drop_blocked_type() {
  let _new_solver = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);

  let mut fixture = Fixture::fixture_bool(false);
  let results = fixture.check_string_optional_frontend_options(

          r#"
        local function track(instance): boolean
            local isBasePart = instance:IsA("BasePart")
            local isCharacter = false
            if not isBasePart then
                isCharacter = instance:FindFirstChildOfClass("Humanoid") and instance:FindFirstChild("HumanoidRootPart")
            end
            return isCharacter
        end
    "#
,
      None,
  );

  assert_eq!(1, results.errors.len(), "{:?}", results.errors);
  type_error_data_ref::<TypeMismatch>(&results.errors[0]).expect("expected TypeMismatch");
}

// Source: `tests/TypeInfer.refinements.test.cpp`
#[test]
fn type_infer_refinements_free_type_is_equal_to_an_lvalue() {
  let (mut fixture, result) = fx_check!(
    r#"
        local function f(a, b: string?)
            if a == b then
                local foo, bar = a, b
            end
        end
    "#
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);

  if !fflag::DebugLuauForceOldSolver.get() {
    assert_eq!(
      "unknown",
      to_string_type_id(fixture.require_type_at_position_position(Position::new(3, 33)))
    );

    let (builtin_types, ice_handler) = {
      let frontend = fixture.get_frontend();
      (
        frontend.builtin_types_handle().as_ptr(),
        &mut frontend.ice_handler as *mut InternalErrorReporter,
      )
    };
    let mut arena = TypeArena::default();
    let mut state = UnifierSharedState::new(ice_handler);
    let mut normalizer = Normalizer::new(
      Some(Handle::from_mut(&mut arena)),
      Handle::from_ptr(builtin_types),
      Some(Handle::from_mut(&mut state)),
      SolverMode::New,
      false,
    );
    let ty = fixture.require_type_at_position_position(Position::new(3, 36));
    let normalized = normalizer
      .try_normalize(ty)
      .expect("normalization must succeed");
    assert_eq!(
      "string?",
      to_string_type_id(normalizer.type_from_normal(normalized.as_ref()))
    );
  } else {
    assert_eq!(
      "a",
      to_string_type_id(fixture.require_type_at_position_position(Position::new(3, 33)))
    );
    assert_eq!(
      "string?",
      to_string_type_id(fixture.require_type_at_position_position(Position::new(3, 36)))
    );
  }
}

// Source: `tests/TypeInfer.refinements.test.cpp`
#[test]
fn type_infer_refinements_function_call_with_colon_after_refining_not_to_be_nil() {
  let _new_solver = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);
  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();
  let result = fixture.base.check_string_optional_frontend_options(

          r#"
        --!strict
        export type Observer<T> = {
            read complete: ((self: Observer<T>) -> ())?,
        }

        local function _f(handler: Observer<any>)
            assert(handler.complete ~= nil)
            handler:complete() -- incorrectly gives Value of type '((Observer<any>) -> ())?' could be nil
            handler.complete(handler) -- works fine, both forms should avoid the error
        end
    "#
,
      None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.refinements.test.cpp`
#[test]
fn type_infer_refinements_function_calls_are_not_nillable() {
  let (_fixture, result) = bs_check!(
    r#"
        local BEFORE_SLASH_PATTERN = "^(.*)[\\/]"
        function operateOnPath(path: string): string?
            local fileName = string.gsub(path, BEFORE_SLASH_PATTERN, "")
            if string.match(fileName, "^init%.") then
                return "path=" .. fileName
            end
            return nil
        end
    "#
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.refinements.test.cpp`
#[test]
fn type_infer_refinements_fuzz_filtered_refined_types_are_followed() {
  let (_fixture, result) = fx_check!(
    r#"
local _
do
local _ = _ ~= _ or _ or _
end
    "#
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.refinements.test.cpp`
#[test]
fn type_infer_refinements_globals_can_be_narrowed_too() {
  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();
  let _result = fixture.base.check_string_optional_frontend_options(
    r#"
        if typeof(string) == 'string' then
            local foo = string
        end
    "#,
    None,
  );

  if !fflag::DebugLuauForceOldSolver.get() {
    assert_eq!(
      "string & typeof(string)",
      to_string_type_id(
        fixture
          .base
          .require_type_at_position_position(Position::new(2, 24))
      )
    );
  } else {
    assert_eq!(
      "never",
      to_string_type_id(
        fixture
          .base
          .require_type_at_position_position(Position::new(2, 24))
      )
    );
  }
}

// Source: `tests/TypeInfer.refinements.test.cpp`
#[test]
fn type_infer_refinements_impossible_type_narrow_is_not_an_error() {
  let (_fixture, result) = bs_check!(
    r#"
        local t: {string} = {"a", "b", "c"}
        local v = t[4]
        if not v then
            t[4] = "d"
        else
            print(v)
        end
    "#
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.refinements.test.cpp`
#[test]
fn type_infer_refinements_index_on_a_refined_property() {
  let (_fixture, result) = bs_check!(
    r#"
        local t: {x: {y: string}?} = {x = {y = "hello!"}}

        if t.x then
            print(t.x.y)
        end
    "#
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.refinements.test.cpp`
#[test]
fn type_infer_refinements_inline_if_conditional_context() {
  let (_fixture, result) = bs_check!(
    r#"
        --!strict

        type Value<T> = {
            kind: "value",
            value: T
        }

        local function peek<T>(state: Value<T> | T): T
            return if typeof(state) == "table" and state.kind == "value"
                then (state :: Value<T>).value :: T
                else state :: T
        end
    "#
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.refinements.test.cpp`
#[test]
fn type_infer_refinements_invert_is_truthy_constraint() {
  let (mut fixture, result) = fx_check!(
    r#"
        function f(v: string?)
            if not v then
                local s = v
            else
                local s = v
            end
        end
    "#
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  assert_eq!(
    "nil",
    to_string_type_id(fixture.require_type_at_position_position(Position::new(3, 26)))
  );
  assert_eq!(
    "string",
    to_string_type_id(fixture.require_type_at_position_position(Position::new(5, 26)))
  );
}

// Source: `tests/TypeInfer.refinements.test.cpp`
#[test]
fn type_infer_refinements_invert_is_truthy_constraint_ifelse_expression() {
  let (mut fixture, result) = bs_check!(
    r#"
        function f(v:string?)
            return if not v then tostring(v) else v
        end
    "#
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);

  assert_eq!(
    "nil",
    to_string_type_id(
      fixture
        .base
        .require_type_at_position_position(Position::new(2, 42))
    )
  );
  assert_eq!(
    "string",
    to_string_type_id(
      fixture
        .base
        .require_type_at_position_position(Position::new(2, 50))
    )
  );
}

// Source: `tests/TypeInfer.refinements.test.cpp`
#[test]
fn type_infer_refinements_invert_is_truthy_constraint_while_expression() {
  let (mut fixture, result) = bs_check!(
    r#"
        function f(v:string?)
            while not v do
                local foo = v
            end
        end
    "#
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);

  assert_eq!(
    "nil",
    to_string_type_id(
      fixture
        .base
        .require_type_at_position_position(Position::new(3, 28))
    )
  );
}

// Source: `tests/TypeInfer.refinements.test.cpp`
#[test]
fn type_infer_refinements_is_truthy_constraint() {
  let (mut fixture, result) = fx_check!(
    r#"
        function f(v: string?)
            if v then
                local s = v
            else
                local s = v
            end
        end
    "#
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  assert_eq!(
    "string",
    to_string_type_id(fixture.require_type_at_position_position(Position::new(3, 26)))
  );
  assert_eq!(
    "nil",
    to_string_type_id(fixture.require_type_at_position_position(Position::new(5, 26)))
  );
}

// Source: `tests/TypeInfer.refinements.test.cpp`
#[test]
fn type_infer_refinements_is_truthy_constraint_ifelse_expression() {
  let (mut fixture, result) = bs_check!(
    r#"
        function f(v:string?)
            return if v then v else tostring(v)
        end
    "#
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);

  assert_eq!(
    "string",
    to_string_type_id(
      fixture
        .base
        .require_type_at_position_position(Position::new(2, 29))
    )
  );
  assert_eq!(
    "nil",
    to_string_type_id(
      fixture
        .base
        .require_type_at_position_position(Position::new(2, 45))
    )
  );
}

// Source: `tests/TypeInfer.refinements.test.cpp`
#[test]
fn type_infer_refinements_is_truthy_constraint_while_expression() {
  let (mut fixture, result) = bs_check!(
    r#"
        function f(v:string?)
            while v do
                local foo = v
            end
        end
    "#
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);

  assert_eq!(
    "string",
    to_string_type_id(
      fixture
        .base
        .require_type_at_position_position(Position::new(3, 28))
    )
  );
}

// Source: `tests/TypeInfer.refinements.test.cpp`
#[test]
fn type_infer_refinements_isa_type_refinement_must_be_known_ahead_of_time() {
  let mut fixture = RefinementExternTypeFixture {
    base: BuiltinsFixture::default(),
  };
  fixture.get_frontend();
  let result = fixture.base.base.check_string_optional_frontend_options(
    r#"
        local function f(x): Instance
            if x:IsA("Folder") then
                local foo = x
            else
                local foo = x
            end

            return x
        end
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);

  if !fflag::DebugLuauForceOldSolver.get() {
    assert_eq!(
      "t1 where t1 = Instance & { read IsA: (t1, string) -> (unknown, ...unknown) }",
      to_string_type_id(
        fixture
          .base
          .base
          .require_type_at_position_position(Position::new(3, 28))
      )
    );
    assert_eq!(
      "t1 where t1 = Instance & { read IsA: (t1, string) -> (unknown, ...unknown) }",
      to_string_type_id(
        fixture
          .base
          .base
          .require_type_at_position_position(Position::new(5, 28))
      )
    );
  } else {
    assert_eq!(
      "Instance",
      to_string_type_id(
        fixture
          .base
          .base
          .require_type_at_position_position(Position::new(3, 28))
      )
    );
    assert_eq!(
      "Instance",
      to_string_type_id(
        fixture
          .base
          .base
          .require_type_at_position_position(Position::new(5, 28))
      )
    );
  }
}

// Source: `tests/TypeInfer.refinements.test.cpp`
#[test]
fn type_infer_refinements_len_operator_in_if_is_just_a_proposition() {
  let _new_solver = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);

  let mut fixture = Fixture::fixture_bool(false);
  let _result = fixture.check_string_optional_frontend_options(
    r#"
type Pool = { x : number }
local pool = p :: Pool
if #pool then
    local y = pool
end
"#,
    None,
  );

  let ty = fixture.require_type_at_position_position(Position::new(4, 14));
  assert_ne!("never", to_string_type_id(ty));
}

// Source: `tests/TypeInfer.refinements.test.cpp`
#[test]
fn type_infer_refinements_limit_complexity_of_arithmetic_type_functions() {
  let _new_solver = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);

  let (_fixture, result) = fx_check!(
    r#"
        local Hermite = {}

        function Hermite:__init(p0, p1, m0, m1)
            self[1] = {
                p0.x;
                p0.y;
                p0.z;
            }
            self[2] = {
                m0.x;
                m0.y;
                m0.z;
            }
            self[3] = {
                3*(p1.x - p0.x) - 2*m0.x - m1.x;
                3*(p1.y - p0.y) - 2*m0.y - m1.y;
                3*(p1.z - p0.z) - 2*m0.z - m1.z;
            }
        end

        return Hermite
    "#
  );

  assert!(!result.errors.is_empty());
}

// Source: `tests/TypeInfer.refinements.test.cpp`
#[test]
fn type_infer_refinements_long_disjunction_of_refinements_should_not_trip_recursion_counter() {
  let mut fixture = Fixture::fixture_bool(false);
  let _result = fixture.check_string_optional_frontend_options(
    r#"
function(obj)
    if script.Parent.SeatNumber.Value == "1D" or
    script.Parent.SeatNumber.Value == "2D" or
    script.Parent.SeatNumber.Value == "3D" or
    script.Parent.SeatNumber.Value == "4D" or
    script.Parent.SeatNumber.Value == "5D" or
    script.Parent.SeatNumber.Value == "6D" or
    script.Parent.SeatNumber.Value == "7D" or
    script.Parent.SeatNumber.Value == "8D" or
    script.Parent.SeatNumber.Value == "9D" or
    script.Parent.SeatNumber.Value == "10D" or
    script.Parent.SeatNumber.Value == "11D" or
    script.Parent.SeatNumber.Value == "12D" or
    script.Parent.SeatNumber.Value == "13D" or
    script.Parent.SeatNumber.Value == "14D" or
    script.Parent.SeatNumber.Value == "15D" or
    script.Parent.SeatNumber.Value == "16D" or
    script.Parent.SeatNumber.Value == "1C" or
    script.Parent.SeatNumber.Value == "2C" or
    script.Parent.SeatNumber.Value == "3C" or
    script.Parent.SeatNumber.Value == "4C" or
    script.Parent.SeatNumber.Value == "5C" or
    script.Parent.SeatNumber.Value == "6C" or
    script.Parent.SeatNumber.Value == "7C" or
    script.Parent.SeatNumber.Value == "8C" or
    script.Parent.SeatNumber.Value == "9C" or
    script.Parent.SeatNumber.Value == "10C" or
    script.Parent.SeatNumber.Value == "11C" or
    script.Parent.SeatNumber.Value == "12C" or
    script.Parent.SeatNumber.Value == "13C" or
    script.Parent.SeatNumber.Value == "14C" or
    script.Parent.SeatNumber.Value == "15C" or
    script.Parent.SeatNumber.Value == "16C" then
end
"#,
    None,
  );
}

// Source: `tests/TypeInfer.refinements.test.cpp`
#[test]
fn type_infer_refinements_luau_polyfill_isindexkey_refine_conjunction() {
  let _new_solver = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);
  let (_fixture, result) = bs_check!(
    r#"
        local function isIndexKey(k, contiguousLength)
            return type(k) == "number"
                and k <= contiguousLength -- nothing out of bounds
                and 1 <= k -- nothing illegal for array indices
                and math.floor(k) == k -- no float keys
        end
    "#
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.refinements.test.cpp`
#[test]
fn type_infer_refinements_luau_polyfill_isindexkey_refine_conjunction_variant() {
  let (_fixture, result) = bs_check!(
    r#"
        local function isIndexKey(k, contiguousLength: number)
            return type(k) == "number"
                and k <= contiguousLength -- nothing out of bounds
                and 1 <= k -- nothing illegal for array indices
                and math.floor(k) == k -- no float keys
        end
    "#
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.refinements.test.cpp`
#[test]
fn type_infer_refinements_lvalue_is_equal_to_a_term() {
  let (mut fixture, result) = fx_check!(
    r#"
        local function f(a: (string | number)?)
            if a == 1 then
                local foo = a
            else
                local foo = a
            end
        end
    "#
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);

  assert_eq!(
    "(number | string)?",
    to_string_type_id(fixture.require_type_at_position_position(Position::new(3, 28)))
  );
  assert_eq!(
    "(number | string)?",
    to_string_type_id(fixture.require_type_at_position_position(Position::new(5, 28)))
  );
}

// Source: `tests/TypeInfer.refinements.test.cpp`
#[test]
fn type_infer_refinements_lvalue_is_equal_to_another_lvalue() {
  let (mut fixture, result) = fx_check!(
    r#"
        local function f(a: (string | number)?, b: boolean?)
            if a == b then
                local foo, bar = a, b
            else
                local foo, bar = a, b
            end
        end
    "#
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);

  assert_eq!(
    "(number | string)?",
    to_string_type_id(fixture.require_type_at_position_position(Position::new(3, 33)))
  );
  assert_eq!(
    "boolean?",
    to_string_type_id(fixture.require_type_at_position_position(Position::new(3, 36)))
  );

  assert_eq!(
    "(number | string)?",
    to_string_type_id(fixture.require_type_at_position_position(Position::new(5, 33)))
  );
  assert_eq!(
    "boolean?",
    to_string_type_id(fixture.require_type_at_position_position(Position::new(5, 36)))
  );
}

// Source: `tests/TypeInfer.refinements.test.cpp`
#[test]
fn type_infer_refinements_lvalue_is_not_nil() {
  let (mut fixture, result) = fx_check!(
    r#"
        local function f(a: (string | number)?)
            if a ~= nil then
                local foo = a
            else
                local foo = a
            end
        end
    "#
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);

  assert_eq!(
    "number | string",
    to_string_type_id(fixture.require_type_at_position_position(Position::new(3, 28)))
  );
  if !fflag::DebugLuauForceOldSolver.get() {
    assert_eq!(
      "nil",
      to_string_type_id(fixture.require_type_at_position_position(Position::new(5, 28)))
    );
  } else {
    assert_eq!(
      "(number | string)?",
      to_string_type_id(fixture.require_type_at_position_position(Position::new(5, 28)))
    );
  }
}

// Source: `tests/TypeInfer.refinements.test.cpp`
#[test]
fn type_infer_refinements_many_refinements_on_val() {
  let (mut fixture, result) = bs_check!(
    r#"
        local function is_nan(val: any): boolean
            return type(val) == "number" and val ~= val
        end

        local function is_js_boolean(val: any): boolean
            return not not val and val ~= 0 and val ~= "" and not is_nan(val)
        end
    "#
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);

  assert_eq!(
    "(any) -> boolean",
    to_string_type_id(fixture.base.require_type_string("is_nan"))
  );
  assert_eq!(
    "(any) -> boolean",
    to_string_type_id(fixture.base.require_type_string("is_js_boolean"))
  );
}

// Source: `tests/TypeInfer.refinements.test.cpp`
#[test]
fn type_infer_refinements_merge_should_be_fully_agnostic_of_hashmap_ordering() {
  let (mut fixture, result) = bs_check!(
    r#"
        local function f(b: string | { x: string }, a)
            assert(type(a) == "string")
            assert(type(b) == "string" or type(b) == "table")

            if type(b) == "string" then
                local foo = b
            end
        end
    "#
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);

  assert_eq!(
    "string",
    to_string_type_id(
      fixture
        .base
        .require_type_at_position_position(Position::new(6, 28))
    )
  );
}

// Source: `tests/TypeInfer.refinements.test.cpp`
#[test]
fn type_infer_refinements_more_complex_long_disjunction_of_refinements_shouldnt_trip_ice() {
  let mut fixture = Fixture::fixture_bool(false);
  let _result = fixture.check_string_optional_frontend_options(
    r#"
script:connect(function(obj)
	if script.Parent.SeatNumber.Value == "1D" or
    script.Parent.SeatNumber.Value == "2D" or
    script.Parent.SeatNumber.Value == "3D" or
    script.Parent.SeatNumber.Value == "4D" or
    script.Parent.SeatNumber.Value == "5D" or
    script.Parent.SeatNumber.Value == "6D" or
    script.Parent.SeatNumber.Value == "7D" or
    script.Parent.SeatNumber.Value == "8D" or
    script.Parent.SeatNumber.Value == "9D" or
    script.Parent.SeatNumber.Value == "10D" or
    script.Parent.SeatNumber.Value == "11D" or
    script.Parent.SeatNumber.Value == "12D" or
    script.Parent.SeatNumber.Value == "13D" or
    script.Parent.SeatNumber.Value == "14D" or
    script.Parent.SeatNumber.Value == "15D" or
    script.Parent.SeatNumber.Value == "16D" or
    script.Parent.SeatNumber.Value == "1C" or
    script.Parent.SeatNumber.Value == "2C" or
    script.Parent.SeatNumber.Value == "3C" or
    script.Parent.SeatNumber.Value == "4C" or
    script.Parent.SeatNumber.Value == "5C" or
    script.Parent.SeatNumber.Value == "6C" or
    script.Parent.SeatNumber.Value == "7C" or
    script.Parent.SeatNumber.Value == "8C" or
    script.Parent.SeatNumber.Value == "9C" or
    script.Parent.SeatNumber.Value == "10C" or
    script.Parent.SeatNumber.Value == "11C" or
    script.Parent.SeatNumber.Value == "12C" or
    script.Parent.SeatNumber.Value == "13C" or
    script.Parent.SeatNumber.Value == "14C" or
    script.Parent.SeatNumber.Value == "15C" or
    script.Parent.SeatNumber.Value == "16C" then
    end)
"#,
    None,
  );
}

// Source: `tests/TypeInfer.refinements.test.cpp`
#[test]
fn type_infer_refinements_mutate_prop_of_some_refined_symbol() {
  let mut fixture = RefinementExternTypeFixture {
    base: BuiltinsFixture::default(),
  };
  fixture.get_frontend();
  let result = fixture.base.base.check_string_optional_frontend_options(
    r#"
        local function instances(): {Instance} error("") end
        local function vec3(x, y, z): Vector3 error("") end

        for _, object in ipairs(instances()) do
            if object:IsA("Part") then
                object.Position = vec3(1, 2, 3)
            end
        end
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.refinements.test.cpp`
#[test]
fn type_infer_refinements_mutate_prop_of_some_refined_symbol_2() {
  let mut fixture = RefinementExternTypeFixture {
    base: BuiltinsFixture::default(),
  };
  fixture.get_frontend();
  let result = fixture.base.base.check_string_optional_frontend_options(
    r#"
        type Result<T, E> = never
            | { tag: "ok", value: T }
            | { tag: "err", error: E }

        local function results(): {Result<number, string>} error("") end

        for _, res in ipairs(results()) do
            if res.tag == "ok" then
                res.value = 7
            end
        end
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.refinements.test.cpp`
#[test]
fn type_infer_refinements_narrow_boolean_to_true_or_false() {
  let (mut fixture, result) = fx_check!(
    r#"
        local function f(x: boolean)
            if x then
                local foo = x
            else
                local foo = x
            end
        end
    "#
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);

  assert_eq!(
    "true",
    to_string_type_id(fixture.require_type_at_position_position(Position::new(3, 28)))
  );
  assert_eq!(
    "false",
    to_string_type_id(fixture.require_type_at_position_position(Position::new(5, 28)))
  );
}

// Source: `tests/TypeInfer.refinements.test.cpp`
#[test]
fn type_infer_refinements_narrow_from_subclasses_of_instance_or_string_or_vector_3() {
  let mut fixture = RefinementExternTypeFixture {
    base: BuiltinsFixture::default(),
  };
  fixture.get_frontend();
  let result = fixture.base.base.check_string_optional_frontend_options(
    r#"
        local function f(x: Part | Folder | string | Vector3)
            if typeof(x) == "Instance" then
                local foo = x
            else
                local foo = x
            end
        end
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);

  assert_eq!(
    "Folder | Part",
    to_string_type_id(
      fixture
        .base
        .base
        .require_type_at_position_position(Position::new(3, 28))
    )
  );
  assert_eq!(
    "Vector3 | string",
    to_string_type_id(
      fixture
        .base
        .base
        .require_type_at_position_position(Position::new(5, 28))
    )
  );
}

// Source: `tests/TypeInfer.refinements.test.cpp`
#[test]
fn type_infer_refinements_narrow_property_of_a_bounded_variable() {
  let (_fixture, result) = fx_check!(
    r#"
        local t
        local u: {x: number?} = {x = nil}
        t = u

        if t.x then
            local foo: number = t.x
        end
    "#
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.refinements.test.cpp`
#[test]
fn type_infer_refinements_non_conditional_context_in_if_should_not_refine() {
  let _new_solver = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);

  let (_fixture, result) = bs_check!(
    r#"
        local function bing(_: any) end
        local function foobar(x: unknown)
            assert(typeof(x) == "table")
            if bing(x.foo) then
            end
        end
    "#
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  assert_eq!(
    "Type 'table' does not have key 'foo'",
    to_string_type_error(&result.errors[0])
  );
}

// Source: `tests/TypeInfer.refinements.test.cpp`
#[test]
fn type_infer_refinements_nonnil_refinement_on_generic() {
  let (mut fixture, result) = bs_check!(
    r#"
        local function printOptional<T>(item: T?, printer: (T) -> string): string
            if item ~= nil then
                return printer(item)
            else
                return ""
            end
        end
    "#
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  if !fflag::DebugLuauForceOldSolver.get() {
    assert_eq!(
      "T & ~nil",
      to_string_type_id(
        fixture
          .base
          .require_type_at_position_position(Position::new(3, 31))
      )
    );
  } else {
    assert_eq!(
      "T",
      to_string_type_id(
        fixture
          .base
          .require_type_at_position_position(Position::new(3, 31))
      )
    );
  }
}

// Source: `tests/TypeInfer.refinements.test.cpp`
#[test]
fn type_infer_refinements_nonoptional_type_can_narrow_to_nil_if_sense_is_true() {
  let _assert_on_forced_constraint =
    ScopedFastFlag::new(&fflag::DebugLuauAssertOnForcedConstraint, true);
  let (mut fixture, result) = bs_check!(
    r#"
        local t = {"hello"}
        local v = t[2]
        if type(v) == "nil" then
            local foo = v
        else
            local foo = v
        end

        if not (type(v) ~= "nil") then
            local foo = v
        else
            local foo = v
        end
    "#
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);

  if !fflag::DebugLuauForceOldSolver.get() {
    assert_eq!(
      "nil & string",
      to_string_type_id(
        fixture
          .base
          .require_type_at_position_position(Position::new(4, 24))
      )
    );
    assert_eq!(
      "string & ~nil",
      to_string_type_id(
        fixture
          .base
          .require_type_at_position_position(Position::new(6, 24))
      )
    );

    assert_eq!(
      "nil & string",
      to_string_type_id(
        fixture
          .base
          .require_type_at_position_position(Position::new(10, 24))
      )
    );
    assert_eq!(
      "string & ~nil",
      to_string_type_id(
        fixture
          .base
          .require_type_at_position_position(Position::new(12, 24))
      )
    );
  } else {
    assert_eq!(
      "nil",
      to_string_type_id(
        fixture
          .base
          .require_type_at_position_position(Position::new(4, 24))
      )
    );
    assert_eq!(
      "string",
      to_string_type_id(
        fixture
          .base
          .require_type_at_position_position(Position::new(6, 24))
      )
    );

    assert_eq!(
      "nil",
      to_string_type_id(
        fixture
          .base
          .require_type_at_position_position(Position::new(10, 24))
      )
    );
    assert_eq!(
      "string",
      to_string_type_id(
        fixture
          .base
          .require_type_at_position_position(Position::new(12, 24))
      )
    );
  }
}

// Source: `tests/TypeInfer.refinements.test.cpp`
#[test]
fn type_infer_refinements_not_a_and_not_b() {
  let (mut fixture, result) = fx_check!(
    r#"
        local function f(a: number?, b: number?)
            if (not a) and (not b) then
                local foo = a
                local bar = b
            end
        end
    "#
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);

  assert_eq!(
    "nil",
    to_string_type_id(fixture.require_type_at_position_position(Position::new(3, 28)))
  );
  assert_eq!(
    "nil",
    to_string_type_id(fixture.require_type_at_position_position(Position::new(4, 28)))
  );
}

// Source: `tests/TypeInfer.refinements.test.cpp`
#[test]
fn type_infer_refinements_not_a_and_not_b_2() {
  let (mut fixture, result) = fx_check!(
    r#"
        local function f(a: number?, b: number?)
            if not (a or b) then
                local foo = a
                local bar = b
            end
        end
    "#
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);

  assert_eq!(
    "nil",
    to_string_type_id(fixture.require_type_at_position_position(Position::new(3, 28)))
  );
  assert_eq!(
    "nil",
    to_string_type_id(fixture.require_type_at_position_position(Position::new(4, 28)))
  );
}

// Source: `tests/TypeInfer.refinements.test.cpp`
#[test]
fn type_infer_refinements_not_a_or_not_b() {
  let (mut fixture, result) = fx_check!(
    r#"
        local function f(a: number?, b: number?)
            if (not a) or (not b) then
                local foo = a
                local bar = b
            end
        end
    "#
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);

  assert_eq!(
    "number?",
    to_string_type_id(fixture.require_type_at_position_position(Position::new(3, 28)))
  );
  assert_eq!(
    "number?",
    to_string_type_id(fixture.require_type_at_position_position(Position::new(4, 28)))
  );
}

// Source: `tests/TypeInfer.refinements.test.cpp`
#[test]
fn type_infer_refinements_not_a_or_not_b_2() {
  let (mut fixture, result) = fx_check!(
    r#"
        local function f(a: number?, b: number?)
            if not (a and b) then
                local foo = a
                local bar = b
            end
        end
    "#
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);

  assert_eq!(
    "number?",
    to_string_type_id(fixture.require_type_at_position_position(Position::new(3, 28)))
  );
  assert_eq!(
    "number?",
    to_string_type_id(fixture.require_type_at_position_position(Position::new(4, 28)))
  );
}

// Source: `tests/TypeInfer.refinements.test.cpp`
#[test]
fn type_infer_refinements_not_and_constraint() {
  let (mut fixture, result) = fx_check!(
    r#"
        function f(a: string?, b: number?)
            if not (a and b) then
                local x = a
                local y = b
            else
                local x = a
                local y = b
            end
        end
    "#
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  assert_eq!(
    "string?",
    to_string_type_id(fixture.require_type_at_position_position(Position::new(3, 26)))
  );
  assert_eq!(
    "number?",
    to_string_type_id(fixture.require_type_at_position_position(Position::new(4, 26)))
  );
  assert_eq!(
    "string",
    to_string_type_id(fixture.require_type_at_position_position(Position::new(6, 26)))
  );
  assert_eq!(
    "number",
    to_string_type_id(fixture.require_type_at_position_position(Position::new(7, 26)))
  );
}

// Source: `tests/TypeInfer.refinements.test.cpp`
#[test]
fn type_infer_refinements_not_t_or_some_prop_of_t() {
  let (mut fixture, result) = fx_check!(
    r#"
        local function f(t: {x: boolean}?)
            if not t or t.x then
                local foo = t
            end
        end
    "#
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);

  if !fflag::DebugLuauForceOldSolver.get() {
    assert_eq!(
      "({ read x: ~(false?) } & { x: boolean })?",
      to_string_type_id(fixture.require_type_at_position_position(Position::new(3, 28)))
    );
  } else {
    assert_eq!(
      "{ x: boolean }?",
      to_string_type_id(fixture.require_type_at_position_position(Position::new(3, 28)))
    );
  }
}

// Source: `tests/TypeInfer.refinements.test.cpp`
#[test]
fn type_infer_refinements_or_predicate_with_truthy_predicates() {
  let (mut fixture, result) = fx_check!(
    r#"
        function f(a: string?, b: number?)
            if a or b then
                local x = a
                local y = b
            else
                local x = a
                local y = b
            end
        end
    "#
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  assert_eq!(
    "string?",
    to_string_type_id(fixture.require_type_at_position_position(Position::new(3, 26)))
  );
  assert_eq!(
    "number?",
    to_string_type_id(fixture.require_type_at_position_position(Position::new(4, 26)))
  );
  assert_eq!(
    "nil",
    to_string_type_id(fixture.require_type_at_position_position(Position::new(6, 26)))
  );
  assert_eq!(
    "nil",
    to_string_type_id(fixture.require_type_at_position_position(Position::new(7, 26)))
  );
}

// Source: `tests/TypeInfer.refinements.test.cpp`
#[test]
fn type_infer_refinements_oss_1451() {
  let _new_solver = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);

  let (_fixture, result) = bs_check!(
    r#"
        type Part = {
            HasTag: (Part, string) -> boolean,
            Name: string,
        }
        local myList = {} :: {Part}
        local nextPart = (table.remove(myList)) :: Part

        if nextPart:HasTag("foo") then
          return
        end

        print(nextPart.Name)

    "#
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.refinements.test.cpp`
#[test]
fn type_infer_refinements_oss_1517_equality_doesnt_add_nil() {
  let (_fixture, result) = fx_check!(
    r#"
        type MyType = {
            data: any
        }

        local function createMyType(): MyType
            local obj = { data = {} }
            return obj
        end

        local function testTypeInference()
            local a: MyType = createMyType()
            local b: MyType = createMyType()

            if a == b then
                local c: MyType = b
                local value = b.data
            end
        end
    "#
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.refinements.test.cpp`
#[test]
fn type_infer_refinements_oss_1528_method_calls_are_not_nillable() {
  let (_fixture, result) = bs_check!(
    r#"
        type RunService = {
            IsRunning: (RunService) -> boolean
        }
        type Game = {
            GetRunService: (Game) -> RunService
        }
        local function getServices(g: Game): RunService
            local service = g:GetRunService()
            if service:IsRunning() then
                return service
            end
            error("Oh no! The service isn't running!")
        end
    "#
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.refinements.test.cpp`
#[test]
fn type_infer_refinements_oss_1687_equality_shouldnt_leak_nil() {
  let (_fixture, result) = fx_check!(
    r#"
        --!strict
        function returns_two(): number
            return 2
        end

        function is_two(num: number): boolean
            return num==2
        end

        local my_number = returns_two()

        if my_number == 2 then
            is_two(my_number) --type error, my_number: number?
        end
    "#
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.refinements.test.cpp`
#[test]
fn type_infer_refinements_oss_1835() {
  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();

  let result = fixture.base.check_string_optional_frontend_options(
    r#"
        --!strict
        local t: {name: string}? = nil

        function f()
            local name = if t then t.name else "name"
        end
    "#,
    None,
  );
  assert_eq!(0, result.errors.len(), "{:?}", result.errors);

  let result = fixture.base.check_string_optional_frontend_options(
    r#"
        --!strict
        local t: {name: string}? = nil

        function f()
            if t then end
            local name = if t then t.name else "name"
        end
    "#,
    None,
  );
  assert_eq!(0, result.errors.len(), "{:?}", result.errors);

  let result = fixture.base.check_string_optional_frontend_options(
    r#"
        local t: {name: string}? = nil
        if t then end
        print(t.name)
        local name = if t then t.name else "name"
    "#,
    None,
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  type_error_data_ref::<OptionalValueAccess>(&result.errors[0])
    .expect("expected OptionalValueAccess");
}

// Source: `tests/TypeInfer.refinements.test.cpp`
#[test]
fn type_infer_refinements_parenthesized_expressions_are_followed_through() {
  let (mut fixture, result) = fx_check!(
    r#"
        function f(v: string?)
            if (not v) then
                local s = v
            else
                local s = v
            end
        end
    "#
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  assert_eq!(
    "nil",
    to_string_type_id(fixture.require_type_at_position_position(Position::new(3, 26)))
  );
  assert_eq!(
    "string",
    to_string_type_id(fixture.require_type_at_position_position(Position::new(5, 26)))
  );
}

// Source: `tests/TypeInfer.refinements.test.cpp`
#[test]
fn type_infer_refinements_prove_that_dataflow_analysis_isnt_doing_alias_tracking_yet() {
  let (mut fixture, result) = fx_check!(
    r#"
        local function f(tag: "cat" | "dog")
            local tag2 = tag

            if tag2 == "cat" then
                local foo = tag
            else
                local foo = tag
            end
        end
    "#
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);

  assert_eq!(
    r#""cat" | "dog""#,
    to_string_type_id(fixture.require_type_at_position_position(Position::new(5, 28)))
  );
  assert_eq!(
    r#""cat" | "dog""#,
    to_string_type_id(fixture.require_type_at_position_position(Position::new(7, 28)))
  );
}

// Source: `tests/TypeInfer.refinements.test.cpp`
#[test]
fn type_infer_refinements_refine_a_param_that_got_resolved_during_constraint_solving_stage() {
  let mut fixture = RefinementExternTypeFixture {
    base: BuiltinsFixture::default(),
  };
  fixture.get_frontend();
  let result = fixture.base.base.check_string_optional_frontend_options(
    r#"
        type Id<T> = T

        local function f(x: Id<Id<Part | Folder> | Id<string>>)
            if typeof(x) ~= "string" and x:IsA("Part") then
                local foo = x
            else
                local foo = x
            end
        end
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  assert_eq!(
    "Part",
    to_string_type_id(
      fixture
        .base
        .base
        .require_type_at_position_position(Position::new(5, 28))
    )
  );
  assert_eq!(
    "Folder | string",
    to_string_type_id(
      fixture
        .base
        .base
        .require_type_at_position_position(Position::new(7, 28))
    )
  );
}

// Source: `tests/TypeInfer.refinements.test.cpp`
#[test]
fn type_infer_refinements_refine_a_param_that_got_resolved_during_constraint_solving_stage_2() {
  let mut fixture = RefinementExternTypeFixture {
    base: BuiltinsFixture::default(),
  };
  fixture.get_frontend();
  let result = fixture.base.base.check_string_optional_frontend_options(
    r#"
        local function hof(f: (Instance) -> ()) end

        hof(function(inst)
            if inst:IsA("Part") then
                local foo = inst
            else
                local foo = inst
            end
        end)
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  assert_eq!(
    "Part",
    to_string_type_id(
      fixture
        .base
        .base
        .require_type_at_position_position(Position::new(5, 28))
    )
  );
  if !fflag::DebugLuauForceOldSolver.get() {
    assert_eq!(
      "Instance & ~Part",
      to_string_type_id(
        fixture
          .base
          .base
          .require_type_at_position_position(Position::new(7, 28))
      )
    );
  } else {
    assert_eq!(
      "Instance",
      to_string_type_id(
        fixture
          .base
          .base
          .require_type_at_position_position(Position::new(7, 28))
      )
    );
  }
}

// Source: `tests/TypeInfer.refinements.test.cpp`
#[test]
fn type_infer_refinements_refine_a_property_not_to_be_nil_through_an_intersection_table() {
  let (_fixture, result) = fx_check!(
    r#"
        type T = {} & {f: ((string) -> string)?}
        local function f(t: T, x)
            if t.f then
                t.f(x)
            end
        end
    "#
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.refinements.test.cpp`
#[test]
fn type_infer_refinements_refine_a_property_of_some_global() {
  let (mut fixture, result) = fx_check!(
    r#"
        foo = { bar = 5 :: number? }

        if foo.bar then
            local bar = foo.bar
        end
    "#
  );

  if !fflag::DebugLuauForceOldSolver.get() {
    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "number",
      to_string_type_id(fixture.require_type_at_position_position(Position::new(4, 30)))
    );
  }
}

// Source: `tests/TypeInfer.refinements.test.cpp`
#[test]
fn type_infer_refinements_refine_any_and_unknown_should_still_be_any() {
  let (_fixture, result) = bs_check!(
    r#"
        local REACT_FRAGMENT_TYPE = (nil :: any)
        local function typeOf(object: any)
            local __type = object.type

            if __type == REACT_FRAGMENT_TYPE then
                return __type
            else
                return __type
                    and typeof(__type) == "table"
                    and __type["$$typeof"]
            end
        end
    "#
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.refinements.test.cpp`
#[test]
fn type_infer_refinements_refine_boolean() {
  let (mut fixture, result) = bs_check!(
    r#"
        local function f(x: number | boolean)
            if typeof(x) == "boolean" then
                local foo = x
            else
                local foo = x
            end
        end
    "#
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  assert_eq!(
    "boolean",
    to_string_type_id(
      fixture
        .base
        .require_type_at_position_position(Position::new(3, 28))
    )
  );
  assert_eq!(
    "number",
    to_string_type_id(
      fixture
        .base
        .require_type_at_position_position(Position::new(5, 28))
    )
  );
}

// Source: `tests/TypeInfer.refinements.test.cpp`
#[test]
fn type_infer_refinements_refine_buffer() {
  let (mut fixture, result) = bs_check!(
    r#"
        local function f(x: number | buffer)
            if typeof(x) == "buffer" then
                local foo = x
            else
                local foo = x
            end
        end
    "#
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  assert_eq!(
    "buffer",
    to_string_type_id(
      fixture
        .base
        .require_type_at_position_position(Position::new(3, 28))
    )
  );
  assert_eq!(
    "number",
    to_string_type_id(
      fixture
        .base
        .require_type_at_position_position(Position::new(5, 28))
    )
  );
}

// Source: `tests/TypeInfer.refinements.test.cpp`
#[test]
fn type_infer_refinements_refine_by_no_refine_should_always_reduce() {
  let _new_solver = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);

  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();
  let result = fixture.base.check_string_optional_frontend_options(

          r#"
        function foo(t): boolean return true end

        function select<K, V>(t: { [K]: V }, columns: { K }): { [K]: V }
            local result = {}
            if foo(t) then
                for k, v in t do
                    if table.find(columns, k) then
                        result[k] = v -- was TypeError: Type function instance refine<intersect<K, ~nil>, *no-refine*> is uninhabited
                    end
                end
            else
                for k, v in pairs(t) do
                    if table.find(columns, k) then
                        result[k] = v
                    end
                end
            end
            return result
        end
    "#
,
      None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.refinements.test.cpp`
#[test]
fn type_infer_refinements_refine_param_of_type_folder_or_part_without_using_typeof() {
  let mut fixture = RefinementExternTypeFixture {
    base: BuiltinsFixture::default(),
  };
  fixture.get_frontend();
  let result = fixture.base.base.check_string_optional_frontend_options(
    r#"
        local function f(x: Part | Folder)
            if x:IsA("Folder") then
                local foo = x
            else
                local foo = x
            end
        end
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);

  assert_eq!(
    "Folder",
    to_string_type_id(
      fixture
        .base
        .base
        .require_type_at_position_position(Position::new(3, 28))
    )
  );
  assert_eq!(
    "Part",
    to_string_type_id(
      fixture
        .base
        .base
        .require_type_at_position_position(Position::new(5, 28))
    )
  );
}

// Source: `tests/TypeInfer.refinements.test.cpp`
#[test]
fn type_infer_refinements_refine_param_of_type_instance_without_using_typeof() {
  let mut fixture = RefinementExternTypeFixture {
    base: BuiltinsFixture::default(),
  };
  fixture.get_frontend();
  let result = fixture.base.base.check_string_optional_frontend_options(
    r#"
        local function f(x: Instance)
            if x:IsA("Folder") then
                local foo = x
            elseif typeof(x) == "table" then
                local foo = x
            end
        end
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);

  assert_eq!(
    "Folder",
    to_string_type_id(
      fixture
        .base
        .base
        .require_type_at_position_position(Position::new(3, 28))
    )
  );
  assert_eq!(
    "never",
    to_string_type_id(
      fixture
        .base
        .base
        .require_type_at_position_position(Position::new(5, 28))
    )
  );
}

// Source: `tests/TypeInfer.refinements.test.cpp`
#[test]
fn type_infer_refinements_refine_the_correct_types_opposite_of_when_a_is_not_number_or_string() {
  let (mut fixture, result) = bs_check!(
    r#"
        local function f(a: string | number | boolean)
            if type(a) ~= "number" and type(a) ~= "string" then
                local foo = a
            else
                local foo = a
            end
        end
    "#
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);

  assert_eq!(
    "boolean",
    to_string_type_id(
      fixture
        .base
        .require_type_at_position_position(Position::new(3, 28))
    )
  );
  assert_eq!(
    "number | string",
    to_string_type_id(
      fixture
        .base
        .require_type_at_position_position(Position::new(5, 28))
    )
  );
}

// Source: `tests/TypeInfer.refinements.test.cpp`
#[test]
fn type_infer_refinements_refine_the_correct_types_opposite_of_while_a_is_not_number_or_string() {
  let (mut fixture, result) = bs_check!(
    r#"
        local function f(a: string | number | boolean)
            while type(a) ~= "number" and type(a) ~= "string" do
                local foo = a
            end
        end
    "#
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);

  assert_eq!(
    "boolean",
    to_string_type_id(
      fixture
        .base
        .require_type_at_position_position(Position::new(3, 28))
    )
  );
}

// Source: `tests/TypeInfer.refinements.test.cpp`
#[test]
fn type_infer_refinements_refine_thread() {
  let (mut fixture, result) = bs_check!(
    r#"
        local function f(x: number | thread)
            if typeof(x) == "thread" then
                local foo = x
            else
                local foo = x
            end
        end
    "#
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  assert_eq!(
    "thread",
    to_string_type_id(
      fixture
        .base
        .require_type_at_position_position(Position::new(3, 28))
    )
  );
  assert_eq!(
    "number",
    to_string_type_id(
      fixture
        .base
        .require_type_at_position_position(Position::new(5, 28))
    )
  );
}

// Source: `tests/TypeInfer.refinements.test.cpp`
#[test]
fn type_infer_refinements_refine_unknown_to_table() {
  let _new_solver = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);
  let (mut fixture, result) = bs_check!(
    r#"
        local function f(a: unknown)
            if typeof(a) == "table" then
                for i, v in a do
                    return i, v
                end
            end

            error("")
        end
    "#
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);

  assert_eq!(
    "(unknown) -> (~nil, unknown)",
    to_string_type_id(fixture.base.require_type_string("f"))
  );
}

// Source: `tests/TypeInfer.refinements.test.cpp`
#[test]
fn type_infer_refinements_refine_unknown_to_table_then_clone_it() {
  let (_fixture, result) = bs_check!(
    r#"
        local function f(x: unknown)
            if typeof(x) == "table" then
                local cloned: {} = table.clone(x)
            end
        end
    "#
  );

  if !fflag::DebugLuauForceOldSolver.get() {
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  } else {
    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  }
}

// Source: `tests/TypeInfer.refinements.test.cpp`
#[test]
fn type_infer_refinements_refine_unknown_to_table_then_take_the_length() {
  let (mut fixture, result) = bs_check!(
    r#"
        local function f(x: unknown)
            if typeof(x) == "table" then
                local len = #x
            end
        end
    "#
  );

  if !fflag::DebugLuauForceOldSolver.get() {
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "table",
      to_string_type_id(
        fixture
          .base
          .require_type_at_position_position(Position::new(3, 29))
      )
    );
  } else {
    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
      "unknown",
      to_string_type_id(
        fixture
          .base
          .require_type_at_position_position(Position::new(3, 29))
      )
    );
  }
}

// Source: `tests/TypeInfer.refinements.test.cpp`
#[test]
fn type_infer_refinements_refine_unknown_to_table_then_test_a_nested_prop() {
  let (_fixture, result) = bs_check!(
    r#"
        local function f(x: unknown): string?
            if typeof(x) == "table" then
                -- this should error, `x.foo` is an unknown property
                if typeof(x.foo.bar) == "string" then
                    return x.foo.bar
                end
            end

            return nil
        end
    "#
  );

  if !fflag::DebugLuauForceOldSolver.get() {
    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    let up =
      type_error_data_ref::<UnknownProperty>(&result.errors[0]).expect("expected UnknownProperty");
    assert_eq!("bar", up.key());
    assert_eq!("unknown", to_string_type_id(up.table()));
  } else {
    assert_eq!(2, result.errors.len(), "{:?}", result.errors);

    for error in &result.errors {
      let up = type_error_data_ref::<UnknownProperty>(error)
        .unwrap_or_else(|| panic!("expected UnknownProperty, got {error:?}"));
      assert_eq!("foo", up.key());
      assert_eq!("unknown", to_string_type_id(up.table()));
    }
  }
}

// Source: `tests/TypeInfer.refinements.test.cpp`
#[test]
fn type_infer_refinements_refine_unknown_to_table_then_test_a_prop() {
  let (_fixture, result) = bs_check!(
    r#"
        local function f(x: unknown): string?
            if typeof(x) == "table" then
                if typeof(x.foo) == "string" then
                    return x.foo
                end
            end

            return nil
        end
    "#
  );

  if !fflag::DebugLuauForceOldSolver.get() {
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  } else {
    assert_eq!(2, result.errors.len(), "{:?}", result.errors);

    for error in &result.errors {
      let up = type_error_data_ref::<UnknownProperty>(error)
        .unwrap_or_else(|| panic!("expected UnknownProperty, got {error:?}"));
      assert_eq!("foo", up.key());
      assert_eq!("unknown", to_string_type_id(up.table()));
    }
  }
}

// Source: `tests/TypeInfer.refinements.test.cpp`
#[test]
fn type_infer_refinements_refine_unknown_to_table_then_test_a_tested_nested_prop() {
  let (_fixture, result) = bs_check!(
    r#"
        local function f(x: unknown): string?
            if typeof(x) == "table" then
                if typeof(x.foo) == "table" and typeof(x.foo.bar) == "string" then
                    return x.foo.bar
                end
            end

            return nil
        end
    "#
  );

  if !fflag::DebugLuauForceOldSolver.get() {
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  } else {
    assert_eq!(3, result.errors.len(), "{:?}", result.errors);

    for error in &result.errors {
      let up = type_error_data_ref::<UnknownProperty>(error)
        .unwrap_or_else(|| panic!("expected UnknownProperty, got {error:?}"));
      assert_eq!("foo", up.key());
      assert_eq!("unknown", to_string_type_id(up.table()));
    }
  }
}

// Source: `tests/TypeInfer.refinements.test.cpp`
#[test]
fn type_infer_refinements_refine_unknowns() {
  let (mut fixture, result) = bs_check!(
    r#"
        local function f(x: unknown)
            if type(x) == "string" then
                local foo = x
            else
                local bar = x
            end
        end
    "#
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);

  if !fflag::DebugLuauForceOldSolver.get() {
    assert_eq!(
      "string",
      to_string_type_id(
        fixture
          .base
          .require_type_at_position_position(Position::new(3, 28))
      )
    );
    assert_eq!(
      "~string",
      to_string_type_id(
        fixture
          .base
          .require_type_at_position_position(Position::new(5, 28))
      )
    );
  } else {
    assert_eq!(
      "string",
      to_string_type_id(
        fixture
          .base
          .require_type_at_position_position(Position::new(3, 28))
      )
    );
    assert_eq!(
      "unknown",
      to_string_type_id(
        fixture
          .base
          .require_type_at_position_position(Position::new(5, 28))
      )
    );
  }
}

// Source: `tests/TypeInfer.refinements.test.cpp`
#[test]
fn type_infer_refinements_refinements_from_and_should_not_refine_to_never() {
  let _new_solver = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);

  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();
  fixture.base.load_definition(
    r#"
        declare extern type Config with
            KeyboardEnabled: boolean
            MouseEnabled: boolean
        end
    "#,
    false,
  );

  let results = fixture.base.check_string_optional_frontend_options(
    r#"
        local config: Config
        local function serialize()
            if config.KeyboardEnabled and config.MouseEnabled then
                return 0
            else
                print(config)
                return 1
            end
        end
    "#,
    None,
  );

  assert_eq!(0, results.errors.len(), "{:?}", results.errors);

  let expected = if fflag::LuauExternTypesNormalizeWithShapes.get() {
    "(Config & { read KeyboardEnabled: false? }) | (Config & { read MouseEnabled: false? })"
  } else {
    "Config"
  };
  assert_eq!(
    expected,
    to_string_type_id(
      fixture
        .base
        .require_type_at_position_position(Position::new(6, 24))
    )
  );
}

// Source: `tests/TypeInfer.refinements.test.cpp`
#[test]
fn type_infer_refinements_refinements_should_avoid_building_up_big_intersect_families() {
  let mut fixture = Fixture::fixture_bool(false);
  let _result = fixture.check_string_optional_frontend_options(

          r#"
script:connect(function(obj)
	if script.Parent.SeatNumber.Value == "1D" or script.Parent.SeatNumber.Value == "2D" or script.Parent.SeatNumber.Value == "3D" or script.Parent.SeatNumber.Value == "4D" or script.Parent.SeatNumber.Value == "5D" or script.Parent.SeatNumber.Value == "6D" or script.Parent.SeatNumber.Value == "7D" or script.Parent.SeatNumber.Value == "8D" or script.Parent.SeatNumber.Value == "9D" or script.Parent.SeatNumber.Value == "10D" or script.Parent.SeatNumber.Value == "11D" or script.Parent.SeatNumber.Value == "12D" or script.Parent.SeatNumber.Value == "13D" or script.Parent.SeatNumber.Value == "14D" or script.Parent.SeatNumber.Value == "15D" or script.Parent.SeatNumber.Value == "16D" or script.Parent.SeatNumber.Value == "1C" or script.Parent.SeatNumber.Value == "2C" or script.Parent.SeatNumber.Value == "3C" or script.Parent.SeatNumber.Value == "4C" or script.Parent.SeatNumber.Value == "5C" or script.Parent.SeatNumber.Value == "6C" or script.Parent.SeatNumber.Value == "7C" or script.Parent.SeatNumber.Value == "8C" or script.Parent.SeatNumber.Value == "9C" or script.Parent.SeatNumber.Value == "10C" or script.Parent.SeatNumber.Value == "11C" or script.Parent.SeatNumber.Value == "12C" or script.Parent.SeatNumber.Value == "13C" or script.Parent.SeatNumber.Value == "14C" or script.Parent.SeatNumber.Value == "15C" or script.Parent.SeatNumber.Value == "16C" then
		if p.Name == script.Parent.Parent.Parent.Parent.Parent.Parent.MainParts.CD.SurfaceGui[script.Parent.SeatNumber.Value].Player.Value or script.Parent.Parent.Parent.Parent.Parent.Parent.MainParts.CD.SurfaceGui[script.Parent.SeatNumber.Value].Player.Value == "" then
		else
			if script.Parent:FindFirstChild("SeatWeld") then
			end
		end
	else
		if p.Name == script.Parent.Parent.Parent.Parent.Parent.Parent.MainParts.AB.SurfaceGui[script.Parent.SeatNumber.Value].Player.Value or script.Parent.Parent.Parent.Parent.Parent.Parent.MainParts.AB.SurfaceGui[script.Parent.SeatNumber.Value].Player.Value == "" then
			print("Allowed")
		else
			if script.Parent:FindFirstChild("SeatWeld") then
			end
		end
	end
end)
"#
,
      None,
  );
}

// Source: `tests/TypeInfer.refinements.test.cpp`
#[test]
fn type_infer_refinements_refinements_should_not_affect_assignment() {
  let (_fixture, result) = fx_check!(
    r#"
        local a: unknown = true
        if a == true then
            a = 'not even remotely similar to a boolean'
        end
    "#
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.refinements.test.cpp`
#[test]
fn type_infer_refinements_refinements_should_preserve_error_suppression() {
  let (_fixture, result) = bs_check!(
    r#"
        local a: any = {}
        local b
        if typeof(a) == "table" then
           b = a.field
        end
    "#
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.refinements.test.cpp`
#[test]
fn type_infer_refinements_refinements_table_intersection_limits() {
  let mut fixture = Fixture::fixture_bool(false);
  let _result = fixture.check_string_optional_frontend_options(
    r#"
--!strict
type Dir = {
    a: number?, b: number?, c: number?, d: number?, e: number?, f: number?,
    g: number?, h: number?, i: number?, j: number?, k: number?, l: number?,
    m: number?, n: number?, o: number?, p: number?, q: number?, r: number?,
}

local function test(dirs: {Dir})
    for k, dir in dirs
        local success, message = pcall(function()
            assert(dir.a == nil or type(dir.a) == "number")
            assert(dir.b == nil or type(dir.b) == "number")
            assert(dir.c == nil or type(dir.c) == "number")
            assert(dir.d == nil or type(dir.d) == "number")
            assert(dir.e == nil or type(dir.e) == "number")
            assert(dir.f == nil or type(dir.f) == "number")
            assert(dir.g == nil or type(dir.g) == "number")
            assert(dir.h == nil or type(dir.h) == "number")
            assert(dir.i == nil or type(dir.i) == "number")
            assert(dir.j == nil or type(dir.j) == "number")
            assert(dir.k == nil or type(dir.k) == "number")
            assert(dir.l == nil or type(dir.l) == "number")
            assert(dir.m == nil or type(dir.m) == "number")
            assert(dir.n == nil or type(dir.n) == "number")
            assert(dir.o == nil or type(dir.o) == "number")
            assert(dir.p == nil or type(dir.p) == "number")
            assert(dir.q == nil or type(dir.q) == "number")
            assert(dir.r == nil or type(dir.r) == "number")
            assert(dir.t == nil or type(dir.t) == "number")
            assert(dir.u == nil or type(dir.u) == "number")
            assert(dir.v == nil or type(dir.v) == "number")
            local checkpoint = dir

            checkpoint.w = 1
        end)
        assert(success)
    end
end
    "#,
    None,
  );
}

// Source: `tests/TypeInfer.refinements.test.cpp`
#[test]
fn type_infer_refinements_string_not_equal_to_string_or_nil() {
  let (mut fixture, result) = fx_check!(
    r#"
        local t: {string} = {"hello"}

        local a: string = t[1]
        local b: string? = nil
        if a ~= b then
            local foo, bar = a, b
        else
            local foo, bar = a, b
        end
    "#
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);

  assert_eq!(
    "string",
    to_string_type_id(fixture.require_type_at_position_position(Position::new(6, 29)))
  );
  assert_eq!(
    "string?",
    to_string_type_id(fixture.require_type_at_position_position(Position::new(6, 32)))
  );

  assert_eq!(
    "string",
    to_string_type_id(fixture.require_type_at_position_position(Position::new(8, 29)))
  );
  assert_eq!(
    "string?",
    to_string_type_id(fixture.require_type_at_position_position(Position::new(8, 32)))
  );
}

// Source: `tests/TypeInfer.refinements.test.cpp`
#[test]
fn type_infer_refinements_table_name_index_without_prior_assignment_from_branch() {
  let _new_solver = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);

  let mut fixture = Fixture::fixture_bool(false);
  let results = fixture.check_string_optional_frontend_options(
    r#"
        local GetDictionary : (unknown, boolean) -> { Player: {} }? = nil :: any

        local CharEntry = GetDictionary(nil, false)
        if not CharEntry then
            CharEntry = GetDictionary(nil, true)
        end

        local x = CharEntry.Player
    "#,
    None,
  );

  assert_eq!(1, results.errors.len(), "{:?}", results.errors);
  type_error_data_ref::<OptionalValueAccess>(&results.errors[0])
    .expect("expected OptionalValueAccess");
  assert_eq!("{  }", to_string_type_id(fixture.require_type_string("x")));
}

// Source: `tests/TypeInfer.refinements.test.cpp`
#[test]
fn type_infer_refinements_term_is_equal_to_an_lvalue() {
  let (mut fixture, result) = fx_check!(
    r#"
        local function f(a: (string | number)?)
            if "hello" == a then
                local foo = a
            else
                local foo = a
            end
        end
    "#
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);

  if !fflag::DebugLuauForceOldSolver.get() {
    assert_eq!(
      r#""hello""#,
      to_string_type_id(fixture.require_type_at_position_position(Position::new(3, 28)))
    );
    assert_eq!(
      r#"((string & ~"hello") | number)?"#,
      to_string_type_id(fixture.require_type_at_position_position(Position::new(5, 28)))
    );
  } else {
    assert_eq!(
      r#""hello""#,
      to_string_type_id(fixture.require_type_at_position_position(Position::new(3, 28)))
    );
    assert_eq!(
      "(number | string)?",
      to_string_type_id(fixture.require_type_at_position_position(Position::new(5, 28)))
    );
  }
}

// Source: `tests/TypeInfer.refinements.test.cpp`
#[test]
fn type_infer_refinements_truthy_call_of_function_with_table_value_as_argument_should_not_refine_value_as_never()
 {
  let (mut fixture, result) = fx_check!(
    r#"
        type Item = {}

        local function predicate(value: Item): boolean
            return true
        end

        local function checkValue(value: Item)
            if predicate(value) then
                local _ = value
            end
        end
    "#
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  assert_eq!(
    "Item",
    to_string_type_id(fixture.require_type_at_position_position(Position::new(8, 27)))
  );
  assert_eq!(
    "Item",
    to_string_type_id(fixture.require_type_at_position_position(Position::new(9, 28)))
  );
}

// Source: `tests/TypeInfer.refinements.test.cpp`
#[test]
fn type_infer_refinements_truthy_constraint_on_properties() {
  let (mut fixture, result) = fx_check!(
    r#"
        local t: {x: number?} = {x = 1}

        if t.x then
            local t2 = t
            local foo = t.x
        end

        local bar = t.x
    "#
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);

  if !fflag::DebugLuauForceOldSolver.get() {
    assert_eq!(
      "{ read x: number, write x: number? }",
      to_string_type_id(fixture.require_type_at_position_position(Position::new(4, 23)))
    );
    assert_eq!(
      "number",
      to_string_type_id(fixture.require_type_at_position_position(Position::new(5, 26)))
    );
  }

  assert_eq!(
    "number?",
    to_string_type_id(fixture.require_type_string("bar"))
  );
}

// Source: `tests/TypeInfer.refinements.test.cpp`
#[test]
fn type_infer_refinements_truthy_refinement_on_generic() {
  let (mut fixture, result) = bs_check!(
    r#"
        local function printOptional<T>(item: T?, printer: (T) -> string): string
            if item then
                return printer(item)
            else
                return ""
            end
        end
    "#
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  if !fflag::DebugLuauForceOldSolver.get() {
    assert_eq!(
      "T & ~(false?)",
      to_string_type_id(
        fixture
          .base
          .require_type_at_position_position(Position::new(3, 31))
      )
    );
  } else {
    assert_eq!(
      "T",
      to_string_type_id(
        fixture
          .base
          .require_type_at_position_position(Position::new(3, 31))
      )
    );
  }
}

// Source: `tests/TypeInfer.refinements.test.cpp`
#[test]
fn type_infer_refinements_type_annotations_arent_relevant_when_doing_dataflow_analysis() {
  let (mut fixture, result) = bs_check!(
    r#"
        local function s() return "hello" end

        local function f(t: {string})
            local s1: string = t[5]
            local s2: string = s()

            if typeof(s1) == "nil" and typeof(s2) == "nil" then
                local foo = s1
                local bar = s2
            end
        end
    "#
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);

  assert_eq!(
    "nil",
    to_string_type_id(
      fixture
        .base
        .require_type_at_position_position(Position::new(8, 28))
    )
  );
  assert_eq!(
    "nil",
    to_string_type_id(
      fixture
        .base
        .require_type_at_position_position(Position::new(9, 28))
    )
  );
}

// Source: `tests/TypeInfer.refinements.test.cpp`
#[test]
fn type_infer_refinements_type_assertion_expr_carry_its_constraints() {
  let (mut fixture, result) = fx_check!(
    r#"
        function g(a: number?, b: string?)
            if (a :: any) and (b :: any) then
                local x = a
                local y = b
            end
        end
    "#
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  if !fflag::DebugLuauForceOldSolver.get() {
    assert_eq!(
      "number?",
      to_string_type_id(fixture.require_type_at_position_position(Position::new(3, 26)))
    );
    assert_eq!(
      "string?",
      to_string_type_id(fixture.require_type_at_position_position(Position::new(4, 26)))
    );
  } else {
    assert_eq!(
      "number",
      to_string_type_id(fixture.require_type_at_position_position(Position::new(3, 26)))
    );
    assert_eq!(
      "string",
      to_string_type_id(fixture.require_type_at_position_position(Position::new(4, 26)))
    );
  }
}

// Source: `tests/TypeInfer.refinements.test.cpp`
#[test]
fn type_infer_refinements_type_comparison_ifelse_expression() {
  let (mut fixture, result) = bs_check!(
    r#"
        function returnOne(x)
            return 1
        end

        function f(v:any)
            return if typeof(v) == "number" then v else returnOne(v)
        end

        function g(v:unknown)
            return if typeof(v) == "number" then v else returnOne(v)
        end
    "#
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);

  if !fflag::DebugLuauForceOldSolver.get() {
    assert_eq!(
      "*error-type* | number",
      to_string_type_id(
        fixture
          .base
          .require_type_at_position_position(Position::new(6, 49))
      )
    );
    assert_eq!(
      "*error-type* | ~number",
      to_string_type_id(
        fixture
          .base
          .require_type_at_position_position(Position::new(6, 66))
      )
    );
  } else {
    assert_eq!(
      "number",
      to_string_type_id(
        fixture
          .base
          .require_type_at_position_position(Position::new(6, 49))
      )
    );
    assert_eq!(
      "any",
      to_string_type_id(
        fixture
          .base
          .require_type_at_position_position(Position::new(6, 66))
      )
    );
  }

  assert_eq!(
    "number",
    to_string_type_id(
      fixture
        .base
        .require_type_at_position_position(Position::new(10, 49))
    )
  );
  if !fflag::DebugLuauForceOldSolver.get() {
    assert_eq!(
      "~number",
      to_string_type_id(
        fixture
          .base
          .require_type_at_position_position(Position::new(10, 66))
      )
    );
  } else {
    assert_eq!(
      "unknown",
      to_string_type_id(
        fixture
          .base
          .require_type_at_position_position(Position::new(10, 66))
      )
    );
  }
}

// Source: `tests/TypeInfer.refinements.test.cpp`
#[test]
fn type_infer_refinements_type_function_reduction_with_union_type_application() {
  let _new_solver = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);
  let _assert_on_forced_constraint =
    ScopedFastFlag::new(&fflag::DebugLuauAssertOnForcedConstraint, true);

  let (_fixture, result) = fx_check!(
    r#"
        local lastTick = 0
        local jumpAnimTime = 0
        local toolAnimTime = 0

        function move(time, tool, animStringValueObject)
            local deltaTime = time - lastTick
            lastTick = time

            if jumpAnimTime > 0 then
                jumpAnimTime = jumpAnimTime - deltaTime
            end

            if animStringValueObject then
                toolAnimTime = time + .3
            end

            if time > toolAnimTime then
            end
        end
    "#
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.refinements.test.cpp`
#[test]
fn type_infer_refinements_type_guard_can_filter_for_intersection_of_tables() {
  let mut fixture = BuiltinsFixture::default();
  fixture.get_frontend();
  let _result = fixture.base.check_string_optional_frontend_options(
    r#"
        type XYCoord = {x: number} & {y: number}
        local function f(t: XYCoord?)
            if type(t) == "table" then
                local foo = t
            else
                local foo = t
            end
        end
    "#,
    None,
  );

  let mut opts = ToStringOptions {
    exhaustive: true,
    ..Default::default()
  };
  let table_ty = fixture
    .base
    .require_type_at_position_position(Position::new(4, 28));
  assert_eq!(
    "{ x: number } & { y: number }",
    to_string_type_id_to_string_options(table_ty, &mut opts)
  );
  assert_eq!(
    "nil",
    to_string_type_id(
      fixture
        .base
        .require_type_at_position_position(Position::new(6, 28))
    )
  );
}

// Source: `tests/TypeInfer.refinements.test.cpp`
#[test]
fn type_infer_refinements_type_guard_can_filter_for_overloaded_function() {
  let (mut fixture, result) = bs_check!(
    r#"
        type SomeOverloadedFunction = ((number) -> string) & ((string) -> number)
        local function f(g: SomeOverloadedFunction?)
            if type(g) == "function" then
                local foo = g
            else
                local foo = g
            end
        end
    "#
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);

  assert_eq!(
    "((number) -> string) & ((string) -> number)",
    to_string_type_id(
      fixture
        .base
        .require_type_at_position_position(Position::new(4, 28))
    )
  );
  assert_eq!(
    "nil",
    to_string_type_id(
      fixture
        .base
        .require_type_at_position_position(Position::new(6, 28))
    )
  );
}

// Source: `tests/TypeInfer.refinements.test.cpp`
#[test]
fn type_infer_refinements_type_guard_narrowed_into_nothingness() {
  let (mut fixture, result) = bs_check!(
    r#"
        local function f(t: {x: number})
            if type(t) ~= "table" then
                local foo = t
                error(("Expected a table, got %s"):format(type(t)))
            end

            return t.x + 1
        end
    "#
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);

  if !fflag::DebugLuauForceOldSolver.get() {
    assert_eq!(
      "{ x: number } & ~table",
      to_string_type_id(
        fixture
          .base
          .require_type_at_position_position(Position::new(3, 28))
      )
    );
  } else {
    assert_eq!(
      "never",
      to_string_type_id(
        fixture
          .base
          .require_type_at_position_position(Position::new(3, 28))
      )
    );
  }
}

// Source: `tests/TypeInfer.refinements.test.cpp`
#[test]
fn type_infer_refinements_type_narrow_but_the_discriminant_type_isnt_a_class() {
  let mut fixture = RefinementExternTypeFixture {
    base: BuiltinsFixture::default(),
  };
  fixture.get_frontend();
  let result = fixture.base.base.check_string_optional_frontend_options(
    r#"
        local function f(x: string | number | Instance | Vector3)
            if type(x) == "any" then
                local foo = x
            else
                local foo = x
            end
        end
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);

  if !fflag::DebugLuauForceOldSolver.get() {
    assert_eq!(
      "never",
      to_string_type_id(
        fixture
          .base
          .base
          .require_type_at_position_position(Position::new(3, 28))
      )
    );
    assert_eq!(
      "Instance | Vector3 | number | string",
      to_string_type_id(
        fixture
          .base
          .base
          .require_type_at_position_position(Position::new(5, 28))
      )
    );
  } else {
    assert_eq!(
      "*error-type*",
      to_string_type_id(
        fixture
          .base
          .base
          .require_type_at_position_position(Position::new(3, 28))
      )
    );
    assert_eq!(
      "*error-type*",
      to_string_type_id(
        fixture
          .base
          .base
          .require_type_at_position_position(Position::new(5, 28))
      )
    );
  }
}

// Source: `tests/TypeInfer.refinements.test.cpp`
#[test]
fn type_infer_refinements_type_narrow_for_all_the_userdata() {
  let mut fixture = RefinementExternTypeFixture {
    base: BuiltinsFixture::default(),
  };
  fixture.get_frontend();
  let result = fixture.base.base.check_string_optional_frontend_options(
    r#"
        local function f(x: string | number | Instance | Vector3)
            if type(x) == "userdata" then
                local foo = x
            else
                local foo = x
            end
        end
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);

  assert_eq!(
    "Instance | Vector3",
    to_string_type_id(
      fixture
        .base
        .base
        .require_type_at_position_position(Position::new(3, 28))
    )
  );
  assert_eq!(
    "number | string",
    to_string_type_id(
      fixture
        .base
        .base
        .require_type_at_position_position(Position::new(5, 28))
    )
  );
}

// Source: `tests/TypeInfer.refinements.test.cpp`
#[test]
fn type_infer_refinements_type_narrow_to_vector() {
  let (mut fixture, result) = bs_check!(
    r#"
        local function f(x)
            if type(x) == "vector" then
                local foo = x
            end
        end
    "#
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);

  if !fflag::DebugLuauForceOldSolver.get() {
    assert_eq!(
      "unknown & vector",
      to_string_type_id(
        fixture
          .base
          .require_type_at_position_position(Position::new(3, 28))
      )
    );
  } else {
    assert_eq!(
      "*error-type*",
      to_string_type_id(
        fixture
          .base
          .require_type_at_position_position(Position::new(3, 28))
      )
    );
  }
}

// Source: `tests/TypeInfer.refinements.test.cpp`
#[test]
fn type_infer_refinements_type_vector_refine() {
  let _new_solver = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);

  let (_fixture, result) = bs_check!(
    r#"
        function foo(x: unknown)
            if type(x) == "vector" then
                local y = x.y
                local z = y.bad
            end
        end
    "#
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  type_error_data_ref::<UnknownProperty>(&result.errors[0]).expect("expected UnknownProperty");
}

// Source: `tests/TypeInfer.refinements.test.cpp`
#[test]
fn type_infer_refinements_typeguard_cast_free_table_to_vector() {
  // CLI-115286 - Refining via type(x) == 'vector' does not work in the new solver
  ulua_unit_test::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

  let mut fixture = RefinementExternTypeFixture {
    base: BuiltinsFixture::default(),
  };
  fixture
    .get_frontend()
    .set_luau_solver_mode(if !fflag::DebugLuauForceOldSolver.get() {
      SolverMode::New
    } else {
      SolverMode::Old
    });
  let result = fixture.base.base.check_string_optional_frontend_options(
    r#"
        local function f(vec)
            local X, Y, Z = vec.X, vec.Y, vec.Z

            if type(vec) == "vector" then
                local foo = vec
            elseif typeof(vec) == "Instance" then
                local foo = vec
            else
                local foo = vec
            end
        end
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);

  // type(vec) == "vector"
  assert_eq!(
    "Vector3",
    to_string_type_id(
      fixture
        .base
        .base
        .require_type_at_position_position(Position::new(5, 28))
    )
  );

  // typeof(vec) == "Instance"
  assert_eq!(
    "never",
    to_string_type_id(
      fixture
        .base
        .base
        .require_type_at_position_position(Position::new(7, 28))
    )
  );

  // type(vec) ~= "vector" and typeof(vec) ~= "Instance"
  assert_eq!(
    "{+ X: a, Y: b, Z: c +}",
    to_string_type_id(
      fixture
        .base
        .base
        .require_type_at_position_position(Position::new(9, 28))
    )
  );
}

// Source: `tests/TypeInfer.refinements.test.cpp`
#[test]
fn type_infer_refinements_typeguard_cast_instance_or_vector_3_to_vector() {
  let mut fixture = RefinementExternTypeFixture {
    base: BuiltinsFixture::default(),
  };
  fixture.get_frontend();
  let result = fixture.base.base.check_string_optional_frontend_options(
    r#"
        local function f(x: Instance | Vector3)
            if typeof(x) == "Vector3" then
                local foo = x
            else
                local foo = x
            end
        end
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);

  assert_eq!(
    "Vector3",
    to_string_type_id(
      fixture
        .base
        .base
        .require_type_at_position_position(Position::new(3, 28))
    )
  );
  assert_eq!(
    "Instance",
    to_string_type_id(
      fixture
        .base
        .base
        .require_type_at_position_position(Position::new(5, 28))
    )
  );
}

// Source: `tests/TypeInfer.refinements.test.cpp`
#[test]
fn type_infer_refinements_typeguard_doesnt_leak_to_elseif() {
  let (_fixture, result) = bs_check!(
    r#"
        function f(a)
           if type(a) == "boolean" then
                local a1 = a
            elseif a.fn() then
                local a2 = a
            else
                local a3 = a
            end
        end
    "#
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.refinements.test.cpp`
#[test]
fn type_infer_refinements_typeguard_in_assert_position() {
  let (mut fixture, result) = bs_check!(
    r#"
        function f(a)
            assert(type(a) == "number")
            local b = a
            return b
        end
    "#
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  if !fflag::DebugLuauForceOldSolver.get() {
    assert_eq!(
      "<a>(a) -> a & number",
      to_string_type_id(fixture.base.require_type_string("f"))
    );
  } else {
    assert_eq!(
      "<a>(a) -> number",
      to_string_type_id(fixture.base.require_type_string("f"))
    );
  }
}

// Source: `tests/TypeInfer.refinements.test.cpp`
#[test]
fn type_infer_refinements_typeguard_in_if_condition_position() {
  let (mut fixture, result) = bs_check!(
    r#"
        function f(s: any, t: unknown)
            if type(s) == "number" then
                local n = s
            end
            if type(t) == "number" then
                local n = t
            end
        end
    "#
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  if !fflag::DebugLuauForceOldSolver.get() {
    assert_eq!(
      "*error-type* | number",
      to_string_type_id(
        fixture
          .base
          .require_type_at_position_position(Position::new(3, 26))
      )
    );
  } else {
    assert_eq!(
      "number",
      to_string_type_id(
        fixture
          .base
          .require_type_at_position_position(Position::new(3, 26))
      )
    );
  }
  assert_eq!(
    "number",
    to_string_type_id(
      fixture
        .base
        .require_type_at_position_position(Position::new(6, 26))
    )
  );
}

// Source: `tests/TypeInfer.refinements.test.cpp`
#[test]
fn type_infer_refinements_typeguard_narrows_for_functions() {
  let (mut fixture, result) = bs_check!(
    r#"
        local function weird(x: string | ((number) -> string))
            if type(x) == "function" then
                local foo = x
            else
                local foo = x
            end
        end
    "#
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);

  assert_eq!(
    "(number) -> string",
    to_string_type_id(
      fixture
        .base
        .require_type_at_position_position(Position::new(3, 28))
    )
  );
  assert_eq!(
    "string",
    to_string_type_id(
      fixture
        .base
        .require_type_at_position_position(Position::new(5, 28))
    )
  );
}

// Source: `tests/TypeInfer.refinements.test.cpp`
#[test]
fn type_infer_refinements_typeguard_narrows_for_table() {
  let (mut fixture, result) = bs_check!(
    r#"
        local function f(x: string | {x: number} | {y: boolean})
            if type(x) == "table" then
                local foo = x
            else
                local foo = x
            end
        end
    "#
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);

  assert_eq!(
    "{ x: number } | { y: boolean }",
    to_string_type_id(
      fixture
        .base
        .require_type_at_position_position(Position::new(3, 28))
    )
  );
  assert_eq!(
    "string",
    to_string_type_id(
      fixture
        .base
        .require_type_at_position_position(Position::new(5, 28))
    )
  );
}

// Source: `tests/TypeInfer.refinements.test.cpp`
#[test]
fn type_infer_refinements_typeguard_not_to_be_string() {
  let (mut fixture, result) = bs_check!(
    r#"
        local function f(x: string | number | boolean)
            if type(x) ~= "string" then
                local foo = x
            else
                local foo = x
            end
        end
    "#
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);

  assert_eq!(
    "boolean | number",
    to_string_type_id(
      fixture
        .base
        .require_type_at_position_position(Position::new(3, 28))
    )
  );
  assert_eq!(
    "string",
    to_string_type_id(
      fixture
        .base
        .require_type_at_position_position(Position::new(5, 28))
    )
  );
}

// Source: `tests/TypeInfer.refinements.test.cpp`
#[test]
fn type_infer_refinements_typeof_instance_error() {
  let mut fixture = RefinementExternTypeFixture {
    base: BuiltinsFixture::default(),
  };
  fixture.get_frontend();
  let result = fixture.base.base.check_string_optional_frontend_options(
    r#"
        local function f(x: Part)
            if typeof(x) == "Instance" then
                local foo : Folder = x
            end
        end
    "#,
    None,
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.refinements.test.cpp`
#[test]
fn type_infer_refinements_typeof_instance_isa_refinement() {
  let mut fixture = RefinementExternTypeFixture {
    base: BuiltinsFixture::default(),
  };
  fixture.get_frontend();
  let result = fixture.base.base.check_string_optional_frontend_options(
    r#"
        local function f(x: Part | Folder | string)
            if typeof(x) == "Instance" then
                local foo = x
                if foo:IsA("Folder") then
                    local bar = foo
                end
            else
                local foo = x
            end
        end
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);

  assert_eq!(
    "Folder | Part",
    to_string_type_id(
      fixture
        .base
        .base
        .require_type_at_position_position(Position::new(3, 28))
    )
  );
  assert_eq!(
    "Folder",
    to_string_type_id(
      fixture
        .base
        .base
        .require_type_at_position_position(Position::new(5, 32))
    )
  );
  assert_eq!(
    "string",
    to_string_type_id(
      fixture
        .base
        .base
        .require_type_at_position_position(Position::new(8, 28))
    )
  );
}

// Source: `tests/TypeInfer.refinements.test.cpp`
#[test]
fn type_infer_refinements_typeof_instance_refinement() {
  let mut fixture = RefinementExternTypeFixture {
    base: BuiltinsFixture::default(),
  };
  fixture.get_frontend();
  let result = fixture.base.base.check_string_optional_frontend_options(
    r#"
        local function f(x: Instance | Vector3)
            if typeof(x) == "Instance" then
                local foo = x
            else
                local foo = x
            end
        end
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);

  assert_eq!(
    "Instance",
    to_string_type_id(
      fixture
        .base
        .base
        .require_type_at_position_position(Position::new(3, 28))
    )
  );
  assert_eq!(
    "Vector3",
    to_string_type_id(
      fixture
        .base
        .base
        .require_type_at_position_position(Position::new(5, 28))
    )
  );
}

// Source: `tests/TypeInfer.refinements.test.cpp`
#[test]
fn type_infer_refinements_typeof_refinement_context() {
  let _new_solver = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);

  let (_fixture, result) = bs_check!(
    r#"
        --!strict

        local x = {} :: unknown

        if typeof(x) == "table" then
            if typeof(x.transform) == "function" then
            	local y = x.transform
            end
        end
    "#
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.refinements.test.cpp`
#[test]
fn type_infer_refinements_unknown_lvalue_is_not_synonymous_with_other_on_not_equal() {
  let (mut fixture, result) = fx_check!(
    r#"
        local function f(a: any, b: {x: number}?)
            if a ~= b then
                local foo, bar = a, b
            end
        end
    "#
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);

  assert_eq!(
    "any",
    to_string_type_id(fixture.require_type_at_position_position(Position::new(3, 33)))
  );
  assert_eq!(
    "{ x: number }?",
    to_string_type_id(fixture.require_type_at_position_position(Position::new(3, 36)))
  );
}

// Source: `tests/TypeInfer.refinements.test.cpp`
#[test]
fn type_infer_refinements_unm_operator_is_just_a_proposition() {
  let _new_solver = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);

  let mut fixture = Fixture::fixture_bool(false);
  let _result = fixture.check_string_optional_frontend_options(
    r#"
type Pool = { x : number }
local pool = p :: Pool
if -pool then
    local y = pool
end
"#,
    None,
  );

  let ty = fixture.require_type_at_position_position(Position::new(4, 14));
  assert_ne!("never", to_string_type_id(ty));
}

// Source: `tests/TypeInfer.refinements.test.cpp`
#[test]
fn type_infer_refinements_what_nonsensical_condition() {
  let (mut fixture, result) = bs_check!(
    r#"
        local function f(x)
            if type(x) == "string" and type(x) == "number" then
                local foo = x
            end
        end
    "#
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);

  assert_eq!(
    "never",
    to_string_type_id(
      fixture
        .base
        .require_type_at_position_position(Position::new(3, 28))
    )
  );
}

// Source: `tests/TypeInfer.refinements.test.cpp`
#[test]
fn type_infer_refinements_x_as_any_if_x_is_instance_elseif_x_is_table() {
  // CLI-117136 - this code doesn't finish constraint solving and has blocked types in the output
  if !fflag::DebugLuauForceOldSolver.get() {
    return;
  }

  let mut fixture = RefinementExternTypeFixture {
    base: BuiltinsFixture::default(),
  };
  fixture.get_frontend();
  let result = fixture.base.base.check_string_optional_frontend_options(
    r#"
        --!nonstrict

        local function f(x)
            if typeof(x) == "Instance" and x:IsA("Folder") then
                local foo = x
            elseif typeof(x) == "table" then
                local foo = x
            end
        end
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);

  if !fflag::DebugLuauForceOldSolver.get() {
    assert_eq!(
      "Folder & Instance & {-  -}",
      to_string_type_id(
        fixture
          .base
          .base
          .require_type_at_position_position(Position::new(5, 28))
      )
    );
    assert_eq!(
      "(~Folder | ~Instance) & {-  -} & never",
      to_string_type_id(
        fixture
          .base
          .base
          .require_type_at_position_position(Position::new(7, 28))
      )
    );
  } else {
    assert_eq!(
      "Folder",
      to_string_type_id(
        fixture
          .base
          .base
          .require_type_at_position_position(Position::new(5, 28))
      )
    );
    assert_eq!(
      "any",
      to_string_type_id(
        fixture
          .base
          .base
          .require_type_at_position_position(Position::new(7, 28))
      )
    );
  }
}

// Source: `tests/TypeInfer.refinements.test.cpp`
#[test]
fn type_infer_refinements_x_is_not_instance_or_else_not_part() {
  let mut fixture = RefinementExternTypeFixture {
    base: BuiltinsFixture::default(),
  };
  fixture.get_frontend();
  let result = fixture.base.base.check_string_optional_frontend_options(
    r#"
        local function f(x: Part | Folder | string)
            if typeof(x) ~= "Instance" or not x:IsA("Part") then
                local foo = x
            else
                local foo = x
            end
        end
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);

  assert_eq!(
    "Folder | string",
    to_string_type_id(
      fixture
        .base
        .base
        .require_type_at_position_position(Position::new(3, 28))
    )
  );
  assert_eq!(
    "Part",
    to_string_type_id(
      fixture
        .base
        .base
        .require_type_at_position_position(Position::new(5, 28))
    )
  );
}

// 缺口（未移植，对照 `tests/TypeInfer.refinements.test.cpp`，逐名清点后 34 候选）：
// - if_local_* / if_const_* / elseif_local_expression_* 共 24 例（:3386–:3823）——
//   `if local`/`if const` 绑定语法受上游 FFlag DebugLuauIfLocalSyntax /
//   DebugLuauIfLocalAnalysis 门控，本仓 parser 未接入（见
//   crates/ulua-ast/src/records/ast_expr_if_else.rs:23 与
//   parser_parse_if_else_expr.rs:59 注记），非纯测试缺口。
// - indexing_unknown_refined_to_table_reports_missing_indexer（:408）——依赖 FFlag
//   `LuauCannotAddIndexerToTablePrimitive`，本移植未定义该 flag。
// - indexing_into_error_gives_error（:3323）——实测本移植对
//   `typeof(item)=="table" and item.key~=nil` 合取中的 `~= nil` 收窄不传递，
//   报 2 枚 TypeMismatch（negation `nil` not subtype of `string`），属求解器
//   refinement 收窄行为缺口（上游修复未 sync）。
// - cli_181894_refinement_cancelled_by_for_loop（:3338）——实测在
//   `if closestChanger == nil then return end` 之后、for-in 循环后再取
//   `.Instances` 报 OptionalValueAccess，收窄在 return 分支的取消/保留行为未 sync。
// - unification_with_refinements_doesnt_impact_freevars（:3361）——实测 sorter
//   类型串与 cpp 期望 `(unknown, unknown) -> boolean` 不一致（freevars 在
//   收窄+统一路径上的泛型保持未 sync）。
// 证伪注：逐名清点的 34 候选中另有 6 例（not_a_and_not_b2/not_a_or_not_b2/
// correctly_lookup_property_whose_base_was_previously_refined2/
// narrow_from_subclasses_of_instance_or_string_or_vector3/
// typeguard_cast_instance_or_vector3_to_vector/falsiness_of_TruthyPredicate_narrows_into_nil）
// 系 cpp 复名与 Rust 归一化重命名（尾缀 _2/_3、蛇形化）的同名已覆盖，非真缺。
