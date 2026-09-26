use ulua_analysis::type_aliases::module_name_type::ModuleName;

extern crate alloc;

// Source: `tests/TypeInfer.classes.test.cpp`
#[test]
fn type_infer_classes_box_point_no_eq() {
  use ulua_analysis::type_aliases::type_error_data::TypeErrorData;
  use ulua_unit_test::records::classes_fixture::ClassesFixture;

  let mut fixture = ClassesFixture::default();
  fixture.get_frontend();

  let result = fixture.base.check_string_optional_frontend_options(
    r#"
class Point
    public x
    public y
end


class Box
    public x
end

local p1 = Point.new { x = 1, y = 2 }
local p2 = Box.new { x = 1 }
local _ = p1 == p1
-- This one too
local _ = p1 ~= p2
local _ = Box == Box
-- This line should error...
local _ = Point ~= Box
"#,
    None,
  );

  assert_eq!(2, result.errors.len(), "{:?}", result.errors);
  assert!(
    matches!(
      &result.errors[0].data,
      TypeErrorData::CannotCompareUnrelatedTypes(_)
    ),
    "{:?}",
    result.errors[0]
  );
  assert!(
    matches!(
      &result.errors[1].data,
      TypeErrorData::CannotCompareUnrelatedTypes(_)
    ),
    "{:?}",
    result.errors[1]
  );
  assert_eq!(15, result.errors[0].location.begin.line);
  assert_eq!(18, result.errors[1].location.begin.line);
}

// Source: `tests/TypeInfer.classes.test.cpp`
#[test]
fn type_infer_classes_class_mm() {
  use ulua_unit_test::records::classes_fixture::ClassesFixture;

  let mut fixture = ClassesFixture::default();
  fixture.get_frontend();

  let result = fixture.base.check_string_optional_frontend_options(
    r#"
class Point
    function __add(self, other: unknown)
    end
end

local p = Point.new {}
p:__add()
"#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.classes.test.cpp`
#[test]
fn type_infer_classes_class_structure() {
  use ulua_analysis::{functions::get_type, records::extern_type::ExternType};
  use ulua_unit_test::records::classes_fixture::ClassesFixture;

  let mut fixture = ClassesFixture::default();
  fixture.get_frontend();

  let result = fixture.base.check_string_optional_frontend_options(
    r#"
class Point
    public x
    public y

    function magnitude(self): number
        return sqrt(self.x * self.x + self.y * self.y)
    end

    function zero(): Point
        return Point.new { x = 0, y = 0 }
    end

    function __tostring(self): string
        return `Point(x={self.x}, y={self.y})`
    end

end

local p = Point
"#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);

  let t = fixture.base.require_type_string("p");
  let et = get_type::get::<ExternType>(t).expect("expected ExternType");
  assert_eq!(
    Some(unsafe { (*fixture.base.builtin_types).class_type }),
    et.parent
  );
  assert!(et.props.contains_key("zero"));
  assert!(et.props.contains_key("new"));
}

// Source: `tests/TypeInfer.classes.test.cpp`
#[test]
fn type_infer_classes_isinstance_refines_imported_class() {
  use ulua_analysis::functions::to_string_to_string::to_string_type_id;
  use ulua_ast::records::position::Position;
  use ulua_common::fflag;
  use ulua_unit_test::{
    records::classes_fixture::ClassesFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
  };

  let _flags = [
    ScopedFastFlag::new(&fflag::LuauExportValueSyntax, true),
    ScopedFastFlag::new(&fflag::LuauExportValueTypecheck, true),
  ];

  let mut fixture = ClassesFixture::default();
  fixture.base.file_resolver.source.insert(
    String::from("game/A"),
    String::from(
      r#"
        export class Point
            public x: number
        end
    "#,
    ),
  );
  fixture.base.file_resolver.source.insert(
    String::from("game/B"),
    String::from(
      r#"
        local A = require(game.A)

        local x : unknown = (A.Point.new { x = 0 } ) :: any
        if class.isinstance(x, A.Point) then
            local y = x
        end
    "#,
    ),
  );

  fixture.get_frontend();
  let module_b = fixture
    .get_frontend()
    .check_module_name_optional_frontend_options(&ModuleName::from("game/B"), None);

  assert_eq!(0, module_b.errors.len(), "{:?}", module_b.errors);
  assert_eq!(
    "Point",
    to_string_type_id(fixture.base.require_type_at_position_module_name_position(
      "game/B",
      Position {
        line: 5,
        column: 22
      }
    ))
  );
}

// Source: `tests/TypeInfer.classes.test.cpp`
#[test]
fn type_infer_classes_isinstance_refines_imported_class_but_not_a_class() {
  use ulua_analysis::{
    functions::to_string_to_string::to_string_type_id, type_aliases::type_error_data::TypeErrorData,
  };
  use ulua_common::fflag;
  use ulua_unit_test::{
    records::classes_fixture::ClassesFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
  };

  let _flags = [
    ScopedFastFlag::new(&fflag::LuauExportValueSyntax, true),
    ScopedFastFlag::new(&fflag::LuauExportValueTypecheck, true),
  ];

  let mut fixture = ClassesFixture::default();
  fixture.base.file_resolver.source.insert(
    String::from("game/A"),
    String::from(
      r#"
        export class Point
            public x: number
        end

        export const notAPoint = nil
    "#,
    ),
  );
  fixture.base.file_resolver.source.insert(
    String::from("game/B"),
    String::from(
      r#"
        local A = require(game.A)

        local x : unknown = (A.Point.new { x = 0 } ) :: any
        if class.isinstance(x, A.notAPoint) then
            local y = x
        end
    "#,
    ),
  );

  fixture.get_frontend();
  let module_a = fixture
    .get_frontend()
    .check_module_name_optional_frontend_options(&ModuleName::from("game/A"), None);
  let module_b = fixture
    .get_frontend()
    .check_module_name_optional_frontend_options(&ModuleName::from("game/B"), None);

  assert_eq!(0, module_a.errors.len(), "{:?}", module_a.errors);
  assert_eq!(1, module_b.errors.len(), "{:?}", module_b.errors);
  let TypeErrorData::TypeMismatch(err) = &module_b.errors[0].data else {
    panic!("expected TypeMismatch, got {:?}", module_b.errors[0]);
  };
  assert_eq!("class", to_string_type_id(err.wanted_type));
  assert_eq!("nil", to_string_type_id(err.given_type));
}

// Source: `tests/TypeInfer.classes.test.cpp`
#[test]
fn type_infer_classes_isinstance_refines_optional_property() {
  use ulua_analysis::functions::to_string_to_string::to_string_type_id;
  use ulua_ast::records::position::Position;
  use ulua_unit_test::records::classes_fixture::ClassesFixture;

  let mut fixture = ClassesFixture::default();
  fixture.get_frontend();

  let result = fixture.base.check_string_optional_frontend_options(
    r#"
class Point
    public x
end

local function f(t: { x: Point? })
    if t.x and class.isinstance(t.x, Point) then
        local s = t.x
    end
end
"#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  assert_eq!(
    "Point",
    to_string_type_id(fixture.base.require_type_at_position_position(Position {
      line: 7,
      column: 20
    }))
  );
}

// Source: `tests/TypeInfer.classes.test.cpp`
#[test]
fn type_infer_classes_isinstance_refines_property_already_typed() {
  use ulua_analysis::functions::to_string_to_string::to_string_type_id;
  use ulua_ast::records::position::Position;
  use ulua_unit_test::records::classes_fixture::ClassesFixture;

  let mut fixture = ClassesFixture::default();
  fixture.get_frontend();

  let result = fixture.base.check_string_optional_frontend_options(
    r#"
class Point
    public x
end

local function f(t: { x: Point })
    if class.isinstance(t.x, Point) then
        local s = t.x
    end
end
"#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  assert_eq!(
    "Point",
    to_string_type_id(fixture.base.require_type_at_position_position(Position {
      line: 7,
      column: 20
    }))
  );
}

// Source: `tests/TypeInfer.classes.test.cpp`
#[test]
fn type_infer_classes_isinstance_refines_union_value() {
  use ulua_analysis::functions::to_string_to_string::to_string_type_id;
  use ulua_ast::records::position::Position;
  use ulua_unit_test::records::classes_fixture::ClassesFixture;

  let mut fixture = ClassesFixture::default();
  fixture.get_frontend();

  let result = fixture.base.check_string_optional_frontend_options(
    r#"
class Point
    public x
end

local function f(v: Point | string)
    if class.isinstance(v, Point) then
        local s = v
    else
        local s = v
    end
end
"#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  assert_eq!(
    "Point",
    to_string_type_id(fixture.base.require_type_at_position_position(Position {
      line: 7,
      column: 18
    }))
  );
  assert_eq!(
    "string",
    to_string_type_id(fixture.base.require_type_at_position_position(Position {
      line: 9,
      column: 18
    }))
  );
}

// Source: `tests/TypeInfer.classes.test.cpp`
#[test]
fn type_infer_classes_isinstance_refines_unknown_value() {
  use ulua_analysis::functions::to_string_to_string::to_string_type_id;
  use ulua_ast::records::position::Position;
  use ulua_common::fflag;
  use ulua_unit_test::{
    records::classes_fixture::ClassesFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
  };

  let _sff = ScopedFastFlag::new(&fflag::LuauIntegerType2, true);
  let mut fixture = ClassesFixture::default();
  fixture.get_frontend();

  let result = fixture.base.check_string_optional_frontend_options(
    r#"
class Point
    public x
end

local function f(v: unknown)
    if class.isinstance(v, Point) then
        local s = v
    else
        local s = v
    end
end
"#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  assert_eq!(
    "Point",
    to_string_type_id(fixture.base.require_type_at_position_position(Position {
      line: 7,
      column: 18
    }))
  );
  assert_eq!(
    "((userdata & ~Point) | boolean | buffer | function | integer | number | string | table | thread)?",
    to_string_type_id(fixture.base.require_type_at_position_position(Position {
      line: 9,
      column: 18
    }))
  );
}

// Source: `tests/TypeInfer.classes.test.cpp`
#[test]
fn type_infer_classes_not_isinstance_refines_union() {
  use ulua_analysis::functions::to_string_to_string::to_string_type_id;
  use ulua_ast::records::position::Position;
  use ulua_unit_test::records::classes_fixture::ClassesFixture;

  let mut fixture = ClassesFixture::default();
  fixture.get_frontend();

  let result = fixture.base.check_string_optional_frontend_options(
    r#"
class Point
    public x
end

local function f(v: Point | string)
    if not class.isinstance(v, Point) then
        local s = v
    else
        local s = v
    end
end
"#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  assert_eq!(
    "string",
    to_string_type_id(fixture.base.require_type_at_position_position(Position {
      line: 7,
      column: 18
    }))
  );
  assert_eq!(
    "Point",
    to_string_type_id(fixture.base.require_type_at_position_position(Position {
      line: 9,
      column: 18
    }))
  );
}

// Source: `tests/TypeInfer.classes.test.cpp`
#[test]
fn type_infer_classes_not_isinstance_refines_unknown() {
  use ulua_analysis::functions::to_string_to_string::to_string_type_id;
  use ulua_ast::records::position::Position;
  use ulua_unit_test::records::classes_fixture::ClassesFixture;

  let mut fixture = ClassesFixture::default();
  fixture.get_frontend();

  let result = fixture.base.check_string_optional_frontend_options(
    r#"
class Point
    public x
end

local function f(v: unknown)
    if not class.isinstance(v, Point) then
        local s = v
    else
        local s = v
    end
end
"#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  assert_eq!(
    "Point",
    to_string_type_id(fixture.base.require_type_at_position_position(Position {
      line: 9,
      column: 18
    }))
  );
}

// Source: `tests/TypeInfer.classes.test.cpp`
#[test]
fn type_infer_classes_point_eq_mm() {
  use ulua_unit_test::records::classes_fixture::ClassesFixture;

  let mut fixture = ClassesFixture::default();
  fixture.get_frontend();

  let result = fixture.base.check_string_optional_frontend_options(
    r#"
class Point
    public x
    public y

    function __eq(self, other: Point): boolean
        return self.x == other.x and self.y == other.y
    end
    function zero(): Point
        return Point.new { x = 0, y = 0 }
    end
end

local p1 = Point.new { x = 1, y = 2 }
local p2 = Point.new { x = 1, y = 2 }
local _ = p1 == p2
local _ = p1 ~= Point.zero()
"#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.classes.test.cpp`
#[test]
fn type_infer_classes_point_tostring() {
  use ulua_common::fflag;
  use ulua_unit_test::{
    records::classes_fixture::ClassesFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
  };

  let _sff = ScopedFastFlag::new(&fflag::DebugLuauUserDefinedClasses, true);
  let mut fixture = ClassesFixture::default();
  fixture.get_frontend();

  let result = fixture.base.check_string_optional_frontend_options(
    r#"
class Point
    public x
    public y
    function __tostring(self): string
        return `Point(x={self.x}, y={self.y})`
    end
end

local p = Point.new { x = 1, y = 2 }
local _ = tostring(p)
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}
// Source: `tests/TypeInfer.classes.test.cpp`
#[test]
fn type_infer_classes_accept_read_only_tables() {
  use ulua_analysis::functions::to_string_to_string::to_string_type_id;
  use ulua_unit_test::records::classes_fixture::ClassesFixture;

  let mut fixture = ClassesFixture::default();
  fixture.get_frontend();

  let result = fixture.base.check_string_optional_frontend_options(
    r#"
        class Foo
            public bar: number | string
        end

        local function ofnumbertbl(tbl: { bar: number })
            return Foo.new(tbl)
        end

        local function inference(tbl)
            return Foo.new(tbl)
        end
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  assert_eq!(
    "({ bar: number }) -> Foo",
    to_string_type_id(fixture.base.require_type_string("ofnumbertbl"))
  );
  assert_eq!(
    "({ read bar: number | string }) -> Foo",
    to_string_type_id(fixture.base.require_type_string("inference"))
  );
}

// Source: `tests/TypeInfer.classes.test.cpp`
#[test]
fn type_infer_classes_all_fields_initialized_before_use() {
  use ulua_unit_test::records::classes_fixture::ClassesFixture;

  let mut fixture = ClassesFixture::default();
  fixture.get_frontend();

  let result = fixture.base.check_string_optional_frontend_options(
    r#"
        local something

        class Foo
            public x: number
            function __init(self)
                self.x = 5
                something = self.x
            end
        end
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.classes.test.cpp`
#[test]
fn type_infer_classes_class_refers_to_later_type_alias() {
  use ulua_analysis::functions::to_string_to_string::to_string_type_id;
  use ulua_unit_test::records::classes_fixture::ClassesFixture;

  let mut fixture = ClassesFixture::default();
  fixture.get_frontend();

  let result = fixture.base.check_string_optional_frontend_options(
    r#"
        class Foo
            public bar: BarType
        end

        type BarType = number | string

        local function getbar(f: Foo)
            return f.bar
        end
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  assert_eq!(
    "(Foo) -> number | string",
    to_string_type_id(fixture.base.require_type_string("getbar"))
  );
}

// Source: `tests/TypeInfer.classes.test.cpp`
#[test]
fn type_infer_classes_conditional_assignment_is_not_yet_allowed() {
  // It would be nice to afford this someday.
  use ulua_analysis::{
    functions::get_error::get_type_error,
    records::uninitialized_field_access::UninitializedFieldAccess,
  };
  use ulua_unit_test::records::classes_fixture::ClassesFixture;

  let mut fixture = ClassesFixture::default();
  fixture.get_frontend();

  let result = fixture.base.check_string_optional_frontend_options(
    r#"
        class Foo
            public x: number
            public y: number
            function __init(self, b: boolean)
                if b then
                    self.x = 0
                else
                    self.x = 2
                end
                self.y = self.x
            end
        end
    "#,
    None,
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  let e = get_type_error::<UninitializedFieldAccess>(&result.errors[0])
    .expect("expected UninitializedFieldAccess");
  assert_eq!(Some(String::from("x")), e.field_name);
}

// Source: `tests/TypeInfer.classes.test.cpp`
#[test]
fn type_infer_classes_constructors_must_accept_self() {
  use ulua_analysis::{functions::get_error::get_type_error, records::syntax_error::SyntaxError};
  use ulua_unit_test::records::classes_fixture::ClassesFixture;

  let mut fixture = ClassesFixture::default();
  fixture.get_frontend();

  let result = fixture.base.check_string_optional_frontend_options(
    r#"
        class Point2
            public x: number
            public y: number

            function __init(x: number, y: number) end
        end

        class Point3
            function __init() end
        end
    "#,
    None,
  );

  assert_eq!(2, result.errors.len(), "{:?}", result.errors);
  let e1 = get_type_error::<SyntaxError>(&result.errors[0]).expect("expected SyntaxError");
  assert_eq!(
    "__init's first parameter must be named 'self'.",
    e1.message()
  );
  let e2 = get_type_error::<SyntaxError>(&result.errors[1]).expect("expected SyntaxError");
  assert_eq!("__init must have at least one parameter.", e2.message());
}

// Source: `tests/TypeInfer.classes.test.cpp`
#[test]
fn type_infer_classes_field_read_inside_closure() {
  // This is technically safe, maybe in the future we have more sophisticated logic to allow this
  use ulua_analysis::{
    functions::get_error::get_type_error,
    records::uninitialized_field_access::UninitializedFieldAccess,
  };
  use ulua_unit_test::records::classes_fixture::ClassesFixture;

  let mut fixture = ClassesFixture::default();
  fixture.get_frontend();

  let result = fixture.base.check_string_optional_frontend_options(
    r#"
        class Foo
            public x: number
            function __init(self)
                local f = function() return self.x end
                self.x = 0
            end
        end
    "#,
    None,
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  let e = get_type_error::<UninitializedFieldAccess>(&result.errors[0])
    .expect("expected UninitializedFieldAccess");
  assert_eq!(Some(String::from("x")), e.field_name);
}

// Source: `tests/TypeInfer.classes.test.cpp`
#[test]
fn type_infer_classes_fuzzy_classes_crash() {
  // TODO CLI-201171: This should be an error, but at least it doesn't crash.
  use ulua_unit_test::records::classes_fixture::ClassesFixture;

  let mut fixture = ClassesFixture::default();
  fixture.get_frontend();

  let result = fixture.base.check_string_optional_frontend_options(
    r#"
        class sqrt extends sqrt
        end
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.classes.test.cpp`
#[test]
fn type_infer_classes_method_calls_on_fully_initialized_instances_are_ok() {
  use ulua_unit_test::records::classes_fixture::ClassesFixture;

  let mut fixture = ClassesFixture::default();
  fixture.get_frontend();

  let result = fixture.base.check_string_optional_frontend_options(
    r#"
        class Foo
            public x: number
            function __init(self)
                self.x = 0
                self:increment()
            end

            function increment(self)
                self.x += 1
            end
        end
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.classes.test.cpp`
#[test]
fn type_infer_classes_method_calls_require_full_initialization() {
  use ulua_analysis::{
    functions::get_error::get_type_error,
    records::uninitialized_field_access::UninitializedFieldAccess,
  };
  use ulua_unit_test::records::classes_fixture::ClassesFixture;

  let mut fixture = ClassesFixture::default();
  fixture.get_frontend();

  let result = fixture.base.check_string_optional_frontend_options(
    r#"
        class Foo
            public x: number
            function __init(self)
                self:increment()
            end

            function increment(self)
                self.x += 1
            end
        end
    "#,
    None,
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  let e = get_type_error::<UninitializedFieldAccess>(&result.errors[0])
    .expect("expected UninitializedFieldAccess");
  assert!(e.field_name.is_none(), "{:?}", e);
}

// Source: `tests/TypeInfer.classes.test.cpp`
#[test]
fn type_infer_classes_nilable_fields_dont_need_initialization() {
  use ulua_analysis::{
    functions::get_error::get_type_error,
    records::uninitialized_field_access::UninitializedFieldAccess,
  };
  use ulua_unit_test::records::classes_fixture::ClassesFixture;

  let mut fixture = ClassesFixture::default();
  fixture.get_frontend();

  let result = fixture.base.check_string_optional_frontend_options(
    r#"
        local function doSomething(...) end

        class Foo
            public x: number
            public y: number?
            public z: any
            public w: unknown
            function __init(self)
                doSomething(self.x, self.y, self.z, self.w)
            end
        end
    "#,
    None,
  );

  // Access to x is bad.  y, z, and w are all fine.
  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  let e = get_type_error::<UninitializedFieldAccess>(&result.errors[0])
    .expect("expected UninitializedFieldAccess");
  assert_eq!(Some(String::from("x")), e.field_name);
}

// Source: `tests/TypeInfer.classes.test.cpp`
#[test]
fn type_infer_classes_no_fields_no_errors() {
  use ulua_unit_test::records::classes_fixture::ClassesFixture;

  let mut fixture = ClassesFixture::default();
  fixture.get_frontend();

  let result = fixture.base.check_string_optional_frontend_options(
    r#"
        local function doSomething(x) end

        class Foo
            function __init(self)
                doSomething(self)
            end
        end
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.classes.test.cpp`
#[test]
fn type_infer_classes_ok_conditional_assignment() {
  // It would be nice to afford this someday.
  use ulua_unit_test::records::classes_fixture::ClassesFixture;

  let mut fixture = ClassesFixture::default();
  fixture.get_frontend();

  let result = fixture.base.check_string_optional_frontend_options(
    r#"
        class Foo
            public x: number
            public y: number
            function __init(self, b: boolean)
                self.x = if b then 0 else 2
                self.y = self.x
            end
        end
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.classes.test.cpp`
#[test]
fn type_infer_classes_partial_initialization_order() {
  use ulua_analysis::{
    functions::get_error::get_type_error,
    records::uninitialized_field_access::UninitializedFieldAccess,
  };
  use ulua_unit_test::records::classes_fixture::ClassesFixture;

  let mut fixture = ClassesFixture::default();
  fixture.get_frontend();

  let result = fixture.base.check_string_optional_frontend_options(
    r#"
        local something

        class Foo
            public x: number
            public y: number
            function __init(self)
                self.x = 0
                something = self.x
                something = self.y
                self.y = 1
            end
        end
    "#,
    None,
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  let e = get_type_error::<UninitializedFieldAccess>(&result.errors[0])
    .expect("expected UninitializedFieldAccess");
  assert_eq!(Some(String::from("y")), e.field_name);
}

// Source: `tests/TypeInfer.classes.test.cpp`
#[test]
fn type_infer_classes_pass_self_after_initialization() {
  use ulua_unit_test::records::classes_fixture::ClassesFixture;

  let mut fixture = ClassesFixture::default();
  fixture.get_frontend();

  let result = fixture.base.check_string_optional_frontend_options(
    r#"
        local function doSomething(x) end

        class Foo
            public x: number
            function __init(self)
                self.x = 0
                doSomething(self)
            end
        end
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.classes.test.cpp`
#[test]
fn type_infer_classes_pass_self_before_initialization() {
  use ulua_analysis::{
    functions::get_error::get_type_error,
    records::uninitialized_field_access::UninitializedFieldAccess,
  };
  use ulua_unit_test::records::classes_fixture::ClassesFixture;

  let mut fixture = ClassesFixture::default();
  fixture.get_frontend();

  let result = fixture.base.check_string_optional_frontend_options(
    r#"
        local function doSomething(x) end

        class Foo
            public x: number
            function __init(self)
                doSomething(self)
                self.x = 0
            end
        end
    "#,
    None,
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  let e = get_type_error::<UninitializedFieldAccess>(&result.errors[0])
    .expect("expected UninitializedFieldAccess");
  assert!(e.field_name.is_none(), "{:?}", e);
}

// Source: `tests/TypeInfer.classes.test.cpp`
#[test]
fn type_infer_classes_pass_self_with_nilable_fields_unassigned() {
  use ulua_unit_test::records::classes_fixture::ClassesFixture;

  let mut fixture = ClassesFixture::default();
  fixture.get_frontend();

  let result = fixture.base.check_string_optional_frontend_options(
    r#"
        local function doSomething(x) end

        class Foo
            public x: number
            public y: string?
            function __init(self)
                self.x = 0
                doSomething(self)
            end
        end
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.classes.test.cpp`
#[test]
fn type_infer_classes_read_error_suppressing_field_before_assign() {
  // TODO: CLI-222651: This shouldn't error because the annotation on x is error suppressing
  use ulua_unit_test::records::classes_fixture::ClassesFixture;

  let mut fixture = ClassesFixture::default();
  fixture.get_frontend();

  let result = fixture.base.check_string_optional_frontend_options(
    r#"
        local something

        class Foo
            public x: string & any
            function __init(self)
                something = self.x
            end
        end
    "#,
    None,
  );

  assert!(!result.errors.is_empty(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.classes.test.cpp`
#[test]
fn type_infer_classes_read_nested_field_of_uninitialized() {
  use ulua_analysis::{
    functions::get_error::get_type_error,
    records::uninitialized_field_access::UninitializedFieldAccess,
  };
  use ulua_unit_test::records::classes_fixture::ClassesFixture;

  let mut fixture = ClassesFixture::default();
  fixture.get_frontend();

  let result = fixture.base.check_string_optional_frontend_options(
    r#"
        local something

        class Foo
            public x: {y: number}
            function __init(self)
                something = self.x.y
            end
        end
    "#,
    None,
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  let e = get_type_error::<UninitializedFieldAccess>(&result.errors[0])
    .expect("expected UninitializedFieldAccess");
  assert_eq!(Some(String::from("x")), e.field_name);
}

// Source: `tests/TypeInfer.classes.test.cpp`
#[test]
fn type_infer_classes_read_nilable_field_before_assign() {
  use ulua_unit_test::records::classes_fixture::ClassesFixture;

  let mut fixture = ClassesFixture::default();
  fixture.get_frontend();

  let result = fixture.base.check_string_optional_frontend_options(
    r#"
        local something

        class Foo
            public x: number?
            function __init(self)
                something = self.x
            end
        end
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.classes.test.cpp`
#[test]
fn type_infer_classes_refer_to_uninitialized_field() {
  use ulua_analysis::{
    functions::get_error::get_type_error,
    records::uninitialized_field_access::UninitializedFieldAccess,
  };
  use ulua_unit_test::records::classes_fixture::ClassesFixture;

  let mut fixture = ClassesFixture::default();
  fixture.get_frontend();

  let result = fixture.base.check_string_optional_frontend_options(
    r#"
        local something

        class Foo
            public x: number
            function __init(self)
                something = self.x
            end
        end
    "#,
    None,
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  let e = get_type_error::<UninitializedFieldAccess>(&result.errors[0])
    .expect("expected UninitializedFieldAccess");
  assert_eq!(Some(String::from("x")), e.field_name);
}

// Source: `tests/TypeInfer.classes.test.cpp`
#[test]
fn type_infer_classes_refer_to_uninitialized_field_index_computed_index() {
  use ulua_analysis::{
    functions::{get_error::get_type_error, to_string_error::to_string_type_error},
    records::uninitialized_field_access::UninitializedFieldAccess,
  };
  use ulua_unit_test::records::classes_fixture::ClassesFixture;

  let mut fixture = ClassesFixture::default();
  fixture.get_frontend();

  let result = fixture.base.check_string_optional_frontend_options(
    r#"
        local something

        class Foo
            public xy: number
            function __init(self)
                something = self["x" .. "y"]
            end
        end
    "#,
    None,
  );

  assert_eq!(2, result.errors.len(), "{:?}", result.errors);
  // 对照 cpp `toString(e1->ty) == "Foo"`：动态属性访问错误的消息内嵌类型名。
  assert_eq!(
    "Attempting a dynamic property access on type 'Foo' is unsafe and may cause exceptions at runtime",
    to_string_type_error(&result.errors[0])
  );
  let e2 = get_type_error::<UninitializedFieldAccess>(&result.errors[1])
    .expect("expected UninitializedFieldAccess");
  // The type checker only reports specific field errors for constant strings, so we just report the error on self in this case
  assert!(e2.field_name.is_none(), "{:?}", e2);
}

// Source: `tests/TypeInfer.classes.test.cpp`
#[test]
fn type_infer_classes_refer_to_uninitialized_field_index_string_expr() {
  use ulua_analysis::{
    functions::get_error::get_type_error,
    records::uninitialized_field_access::UninitializedFieldAccess,
  };
  use ulua_unit_test::records::classes_fixture::ClassesFixture;

  let mut fixture = ClassesFixture::default();
  fixture.get_frontend();

  let result = fixture.base.check_string_optional_frontend_options(
    r#"
        local something

        class Foo
            public x: number
            function __init(self)
                something = self["x"]
            end
        end
    "#,
    None,
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  let e = get_type_error::<UninitializedFieldAccess>(&result.errors[0])
    .expect("expected UninitializedFieldAccess");
  assert_eq!(Some(String::from("x")), e.field_name);
}

// Source: `tests/TypeInfer.classes.test.cpp`
#[test]
fn type_infer_classes_reference_to_shadowed_self_is_absurd_but_ok() {
  use ulua_unit_test::records::classes_fixture::ClassesFixture;

  let mut fixture = ClassesFixture::default();
  fixture.get_frontend();

  let result = fixture.base.check_string_optional_frontend_options(
    r#"
        local something

        class Foo
            function __init(self)
                local self = {}
                something = self
            end
        end
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.classes.test.cpp`
#[test]
fn type_infer_classes_self_referential_assign() {
  use ulua_analysis::{
    functions::get_error::get_type_error,
    records::uninitialized_field_access::UninitializedFieldAccess,
  };
  use ulua_unit_test::records::classes_fixture::ClassesFixture;

  let mut fixture = ClassesFixture::default();
  fixture.get_frontend();

  let result = fixture.base.check_string_optional_frontend_options(
    r#"
        class Foo
            public x: number
            public y: number
            function __init(self)
                self.x, self.y = self.y, self.x
            end
        end
    "#,
    None,
  );

  assert_eq!(2, result.errors.len(), "{:?}", result.errors);
  let e0 = get_type_error::<UninitializedFieldAccess>(&result.errors[0])
    .expect("expected UninitializedFieldAccess");
  let e1 = get_type_error::<UninitializedFieldAccess>(&result.errors[1])
    .expect("expected UninitializedFieldAccess");
  // Both `self.x` and `self.y` are read before either is initialized; the
  // two errors are collected from a hash map, so their order isn't fixed.
  let mut names = [e0.field_name.as_deref(), e1.field_name.as_deref()];
  names.sort_unstable();
  assert_eq!([Some("x"), Some("y")], names);
}

// Source: `tests/TypeInfer.classes.test.cpp`
#[test]
fn type_infer_classes_shadowing_self_via_closure() {
  use ulua_unit_test::records::classes_fixture::ClassesFixture;

  let mut fixture = ClassesFixture::default();
  fixture.get_frontend();

  let result = fixture.base.check_string_optional_frontend_options(
    r#"
        class Foo
            public x: number
            function __init(self)
                local f = function(self) return self.x end
                self.x = 0
            end
        end
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.classes.test.cpp`
#[test]
fn type_infer_classes_type_assertion_loophole() {
  use ulua_unit_test::records::classes_fixture::ClassesFixture;

  let mut fixture = ClassesFixture::default();
  fixture.get_frontend();

  let result = fixture.base.check_string_optional_frontend_options(
    r#"
        local something: any

        class Foo
            public x: number
            function __init(self)
                something = self :: Foo
                something = (self :: Foo).x
            end
        end
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.classes.test.cpp`
#[test]
fn type_infer_classes_typed_self_parameter_after_class_declaration() {
  // Annotations on the self parameter are forbidden, but we still have to
  // parse this without crashing.
  use ulua_analysis::{
    functions::{get_error::get_type_error, to_string_to_string::to_string_type_id},
    records::{syntax_error::SyntaxError, type_mismatch::TypeMismatch},
  };
  use ulua_unit_test::records::classes_fixture::ClassesFixture;

  let mut fixture = ClassesFixture::default();
  fixture.get_frontend();

  let result = fixture.base.check_string_optional_frontend_options(
    r#"
        class Q
            function f(self: number) end
        end
    "#,
    None,
  );

  assert_eq!(2, result.errors.len(), "{:?}", result.errors);
  let e0 = get_type_error::<SyntaxError>(&result.errors[0]).expect("expected SyntaxError");
  assert_eq!(
    "The 'self' parameter cannot have a type annotation",
    e0.message()
  );

  let e1 = get_type_error::<TypeMismatch>(&result.errors[1]).expect("expected TypeMismatch");
  assert_eq!("number", to_string_type_id(e1.wanted_type));
  assert_eq!("Q", to_string_type_id(e1.given_type));
}

// Source: `tests/TypeInfer.classes.test.cpp`
#[test]
fn type_infer_classes_typeof_class_prop_ice() {
  use ulua_unit_test::records::classes_fixture::ClassesFixture;

  let mut fixture = ClassesFixture::default();
  fixture.get_frontend();

  let result = fixture.base.check_string_optional_frontend_options(
    r#"
        local x = 1
        class Foo
            public bar: typeof(x)
        end
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.classes.test.cpp`
#[test]
fn type_infer_classes_typeof_indexing_ice_in_class_prop_typeof() {
  use ulua_analysis::{
    functions::get_error::get_type_error, records::unknown_property::UnknownProperty,
  };
  use ulua_unit_test::records::classes_fixture::ClassesFixture;

  let mut fixture = ClassesFixture::default();
  fixture.get_frontend();

  let result = fixture.base.check_string_optional_frontend_options(
    r#"
local A = ""
class B
    public C: { _: typeof(A.D) }
end
    "#,
    None,
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  let err = get_type_error::<UnknownProperty>(&result.errors[0]).expect("expected UnknownProperty");
  assert_eq!("D", err.key());
}

// Source: `tests/TypeInfer.classes.test.cpp`
#[test]
fn type_infer_classes_unannotated_field_doesnt_need_initialization() {
  use ulua_unit_test::records::classes_fixture::ClassesFixture;

  let mut fixture = ClassesFixture::default();
  fixture.get_frontend();

  let result = fixture.base.check_string_optional_frontend_options(
    r#"
        class Foo
            public x
            public y: number
            function __init(self)
                self.y = 0
            end
        end
    "#,
    None,
  );

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/TypeInfer.classes.test.cpp`
#[test]
fn type_infer_classes_variadic_constructor() {
  use ulua_unit_test::records::classes_fixture::ClassesFixture;

  let mut fixture = ClassesFixture::default();
  fixture.get_frontend();

  let result = fixture.base.check_string_optional_frontend_options(
    r#"
        class Foo
            public values: {number}
            function __init(self, ...: number)
                self.values = {...}
            end
        end

        local f = Foo.new(3, 4, 5) -- OK
        local g = Foo.new(3, 4, 5, "six") -- Error
    "#,
    None,
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  assert_eq!(9, result.errors[0].location.begin.line);
}

// Source: `tests/TypeInfer.classes.test.cpp`
#[test]
fn type_infer_classes_variadic_constructor_with_leading_positional_arguments() {
  use ulua_unit_test::records::classes_fixture::ClassesFixture;

  let mut fixture = ClassesFixture::default();
  fixture.get_frontend();

  let result = fixture.base.check_string_optional_frontend_options(
    r#"
        class Foo
            public x: number
            public y: string
            public values: {number}
            function __init(self, x: number, y: string, ...: number)
                self.x = x
                self.y = y
                self.values = {...}
            end
        end

        local f = Foo.new(3, "four", 5) -- OK
        local g = Foo.new(3, "four", 5, "six") -- Error
    "#,
    None,
  );

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  assert_eq!(13, result.errors[0].location.begin.line);
}

// Source: `tests/TypeInfer.classes.test.cpp:1070-1099`
// （§8 缺口登记解锁：`TypeAnnotationRequired` 变体已随本诊断移植落地。）
#[test]
fn type_infer_classes_class_methods_should_be_annotated_except_for_self() {
  use ulua_analysis::{
    functions::get_error::get_type_error,
    records::{
      type_annotation_required::TypeAnnotationRequired,
      uninhabited_type_function::UninhabitedTypeFunction,
    },
  };
  use ulua_unit_test::records::classes_fixture::ClassesFixture;

  let mut fixture = ClassesFixture::default();
  fixture.get_frontend();

  let result = fixture.base.check_string_optional_frontend_options(
    r#"
class Foo
    public x: number

    function __init(self, x)
        self.x = x
    end

    function double(this)
        return Foo.new(this.x * 2)
    end

    function double2(self): Foo
        return Foo.new(self.x * 2)
    end
end
"#,
    None,
  );

  assert_eq!(3, result.errors.len(), "{:?}", result.errors);
  get_type_error::<TypeAnnotationRequired>(&result.errors[0])
    .expect("errors[0] 应为 TypeAnnotationRequired");
  get_type_error::<UninhabitedTypeFunction>(&result.errors[1])
    .expect("errors[1] 应为 UninhabitedTypeFunction");
  get_type_error::<TypeAnnotationRequired>(&result.errors[2])
    .expect("errors[2] 应为 TypeAnnotationRequired");

  assert_eq!(4, result.errors[0].location.begin.line);
  assert_eq!(9, result.errors[1].location.begin.line);
  assert_eq!(8, result.errors[2].location.begin.line);
}
