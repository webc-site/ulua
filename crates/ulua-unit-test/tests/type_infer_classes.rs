extern crate alloc;

mod type_infer_classes_box_point_no_eq {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.classes.test.cpp:106:type_infer_classes_box_point_no_eq`
  //! Source: `tests/TypeInfer.classes.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.classes.test.cpp
  //! - source_includes:
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.classes.test.cpp
  //! - outgoing:
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record CannotCompareUnrelatedTypes (Analysis/include/Luau/Error.h)
  //!   - translates_to -> rust_item type_infer_classes_box_point_no_eq

  #[cfg(test)]
  #[test]
  fn type_infer_classes_box_point_no_eq() {
    use ulua_analysis::type_aliases::type_error_data::TypeErrorData;
    use ulua_unit_test::records::classes_fixture::ClassesFixture;

    let mut fixture = ClassesFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
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
      ),
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
}

mod type_infer_classes_class_mm {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.classes.test.cpp:139:type_infer_classes_class_mm`
  //! Source: `tests/TypeInfer.classes.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.classes.test.cpp
  //! - source_includes:
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.classes.test.cpp
  //! - outgoing:
  //!   - translates_to -> rust_item type_infer_classes_class_mm

  #[cfg(test)]
  #[test]
  fn type_infer_classes_class_mm() {
    use ulua_unit_test::records::classes_fixture::ClassesFixture;

    let mut fixture = ClassesFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
class Point
    function __add(self, other)
    end
end

local p = Point.new {}
p:__add()
"#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_classes_class_structure {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.classes.test.cpp:153:type_infer_classes_class_structure`
  //! Source: `tests/TypeInfer.classes.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.classes.test.cpp
  //! - source_includes:
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.classes.test.cpp
  //! - outgoing:
  //!   - calls -> function get (tests/Fixture.h)
  //!   - type_ref -> record ExternType (Analysis/include/Luau/Type.h)
  //!   - type_ref -> record TableType (Analysis/include/Luau/Type.h)
  //!   - translates_to -> rust_item type_infer_classes_class_structure

  #[cfg(test)]
  #[test]
  fn type_infer_classes_class_structure() {
    use ulua_analysis::{functions::get_type_alt_j::get_type_id, records::extern_type::ExternType};
    use ulua_unit_test::records::classes_fixture::ClassesFixture;

    let mut fixture = ClassesFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
class Point
    public x
    public y

    function magnitude(self)
        return sqrt(self.x * self.x + self.y * self.y)
    end

    function zero()
        return Point.new { x = 0, y = 0 }
    end

    function __tostring(self)
        return `Point(x={self.x}, y={self.y})`
    end

end

local p = Point
"#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let t = fixture.base.require_type_string(&String::from("p"));
    let et = get_type_id::<ExternType>(t).expect("expected ExternType");
    assert_eq!(
      Some(unsafe { (*fixture.base.builtin_types).class_type }),
      et.parent
    );
    assert!(et.props.contains_key("zero"));
    assert!(et.props.contains_key("new"));
  }
}

mod type_infer_classes_isinstance_refines_imported_class {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.classes.test.cpp:314:type_infer_classes_isinstance_refines_imported_class`
  //! Source: `tests/TypeInfer.classes.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.classes.test.cpp
  //! - source_includes:
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.classes.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method ClassesFixture::getFrontend (tests/TypeInfer.classes.test.cpp)
  //!   - translates_to -> rust_item type_infer_classes_isinstance_refines_imported_class

  #[cfg(test)]
  #[test]
  fn type_infer_classes_isinstance_refines_imported_class() {
    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::records::position::Position;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::classes_fixture::ClassesFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _flags = [
      ScopedFastFlag::new(&FFlag::LuauExportValueSyntax, true),
      ScopedFastFlag::new(&FFlag::LuauExportValueTypecheck, true),
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
      .check_module_name_optional_frontend_options(&String::from("game/B"), None);

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
}

mod type_infer_classes_isinstance_refines_imported_class_but_not_a_class {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.classes.test.cpp:337:type_infer_classes_isinstance_refines_imported_class_but_not_a_class`
  //! Source: `tests/TypeInfer.classes.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.classes.test.cpp
  //! - source_includes:
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.classes.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method ClassesFixture::getFrontend (tests/TypeInfer.classes.test.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - translates_to -> rust_item type_infer_classes_isinstance_refines_imported_class_but_not_a_class

  #[cfg(test)]
  #[test]
  fn type_infer_classes_isinstance_refines_imported_class_but_not_a_class() {
    use ulua_analysis::{
      functions::to_string_to_string_alt_c::to_string_type_id,
      type_aliases::type_error_data::TypeErrorData,
    };
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::classes_fixture::ClassesFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _flags = [
      ScopedFastFlag::new(&FFlag::LuauExportValueSyntax, true),
      ScopedFastFlag::new(&FFlag::LuauExportValueTypecheck, true),
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
      .check_module_name_optional_frontend_options(&String::from("game/A"), None);
    let module_b = fixture
      .get_frontend()
      .check_module_name_optional_frontend_options(&String::from("game/B"), None);

    assert_eq!(0, module_a.errors.len(), "{:?}", module_a.errors);
    assert_eq!(1, module_b.errors.len(), "{:?}", module_b.errors);
    let TypeErrorData::TypeMismatch(err) = &module_b.errors[0].data else {
      panic!("expected TypeMismatch, got {:?}", module_b.errors[0]);
    };
    assert_eq!("class", to_string_type_id(err.wanted_type));
    assert_eq!("nil", to_string_type_id(err.given_type));
  }
}

mod type_infer_classes_isinstance_refines_optional_property {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.classes.test.cpp:278:type_infer_classes_isinstance_refines_optional_property`
  //! Source: `tests/TypeInfer.classes.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.classes.test.cpp
  //! - source_includes:
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.classes.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_classes_isinstance_refines_optional_property

  #[cfg(test)]
  #[test]
  fn type_infer_classes_isinstance_refines_optional_property() {
    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::records::position::Position;
    use ulua_unit_test::records::classes_fixture::ClassesFixture;

    let mut fixture = ClassesFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
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
      ),
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
}

mod type_infer_classes_isinstance_refines_property_already_typed {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.classes.test.cpp:296:type_infer_classes_isinstance_refines_property_already_typed`
  //! Source: `tests/TypeInfer.classes.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.classes.test.cpp
  //! - source_includes:
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.classes.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_classes_isinstance_refines_property_already_typed

  #[cfg(test)]
  #[test]
  fn type_infer_classes_isinstance_refines_property_already_typed() {
    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::records::position::Position;
    use ulua_unit_test::records::classes_fixture::ClassesFixture;

    let mut fixture = ClassesFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
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
      ),
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
}

mod type_infer_classes_isinstance_refines_union_value {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.classes.test.cpp:216:type_infer_classes_isinstance_refines_union_value`
  //! Source: `tests/TypeInfer.classes.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.classes.test.cpp
  //! - source_includes:
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.classes.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_classes_isinstance_refines_union_value

  #[cfg(test)]
  #[test]
  fn type_infer_classes_isinstance_refines_union_value() {
    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::records::position::Position;
    use ulua_unit_test::records::classes_fixture::ClassesFixture;

    let mut fixture = ClassesFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
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
      ),
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
}

mod type_infer_classes_isinstance_refines_unknown_value {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.classes.test.cpp:192:type_infer_classes_isinstance_refines_unknown_value`
  //! Source: `tests/TypeInfer.classes.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.classes.test.cpp
  //! - source_includes:
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.classes.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_classes_isinstance_refines_unknown_value

  #[cfg(test)]
  #[test]
  fn type_infer_classes_isinstance_refines_unknown_value() {
    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::records::position::Position;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::classes_fixture::ClassesFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _sff = ScopedFastFlag::new(&FFlag::LuauIntegerType2, true);
    let mut fixture = ClassesFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
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
      ),
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
}

mod type_infer_classes_not_isinstance_refines_union {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.classes.test.cpp:237:type_infer_classes_not_isinstance_refines_union`
  //! Source: `tests/TypeInfer.classes.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.classes.test.cpp
  //! - source_includes:
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.classes.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - translates_to -> rust_item type_infer_classes_not_isinstance_refines_union

  #[cfg(test)]
  #[test]
  fn type_infer_classes_not_isinstance_refines_union() {
    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::records::position::Position;
    use ulua_unit_test::records::classes_fixture::ClassesFixture;

    let mut fixture = ClassesFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
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
      ),
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
}

mod type_infer_classes_not_isinstance_refines_unknown {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.classes.test.cpp:258:type_infer_classes_not_isinstance_refines_unknown`
  //! Source: `tests/TypeInfer.classes.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.classes.test.cpp
  //! - source_includes:
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.classes.test.cpp
  //! - outgoing:
  //!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
  //!   - translates_to -> rust_item type_infer_classes_not_isinstance_refines_unknown

  #[cfg(test)]
  #[test]
  fn type_infer_classes_not_isinstance_refines_unknown() {
    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_ast::records::position::Position;
    use ulua_unit_test::records::classes_fixture::ClassesFixture;

    let mut fixture = ClassesFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
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
      ),
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
}

mod type_infer_classes_point_eq_mm {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.classes.test.cpp:82:type_infer_classes_point_eq_mm`
  //! Source: `tests/TypeInfer.classes.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.classes.test.cpp
  //! - source_includes:
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.classes.test.cpp
  //! - outgoing:
  //!   - translates_to -> rust_item type_infer_classes_point_eq_mm

  #[cfg(test)]
  #[test]
  fn type_infer_classes_point_eq_mm() {
    use ulua_unit_test::records::classes_fixture::ClassesFixture;

    let mut fixture = ClassesFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
class Point
    public x
    public y

    function __eq(self, other)
        return self.x == other.x and self.y == other.y
    end
    function zero()
        return Point.new { x = 0, y = 0 }
    end
end

local p1 = Point.new { x = 1, y = 2 }
local p2 = Point.new { x = 1, y = 2 }
local _ = p1 == p2
local _ = p1 ~= Point.zero()
"#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}

mod type_infer_classes_point_tostring {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.classes.test.cpp:63:type_infer_classes_point_tostring`
  //! Source: `tests/TypeInfer.classes.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/TypeInfer.classes.test.cpp
  //! - source_includes:
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //!   - includes -> source_file tests/ScopedFlags.h
  //! - incoming:
  //!   - declares <- source_file tests/TypeInfer.classes.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - calls -> macro tostring (VM/src/lvm.h)
  //!   - translates_to -> rust_item type_infer_classes_point_tostring

  #[cfg(test)]
  #[test]
  fn type_infer_classes_point_tostring() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::classes_fixture::ClassesFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _sff = ScopedFastFlag::new(&FFlag::DebugLuauUserDefinedClasses, true);
    let mut fixture = ClassesFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
class Point
    public x
    public y
    function __tostring(self)
        return `Point(x={self.x}, y={self.y})`
    end
end

local p = Point.new { x = 1, y = 2 }
local _ = tostring(p)
    "#,
      ),
      None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
  }
}
