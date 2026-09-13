use alloc::sync::Arc;
use core::{ffi, ffi::CStr, mem::align_of, ptr::null_mut, slice::from_raw_parts, str::from_utf8};
use std::panic::{AssertUnwindSafe, catch_unwind};

use ulua_ast::{
  records::{
    ast_attr::AstAttrType as UluaAstAstAttrType, ast_expr, ast_expr_binary::AstExprBinaryOp,
    ast_expr_call, ast_expr_constant_integer, ast_expr_constant_number, ast_expr_function,
    ast_expr_interp_string, ast_node, ast_stat_assign, ast_stat_block, ast_stat_class,
    ast_stat_compound_assign, ast_stat_continue::AstStatContinue, ast_stat_declare_extern_type,
    ast_stat_declare_function, ast_stat_declare_global, ast_stat_expr, ast_stat_function,
    ast_stat_local, ast_stat_local_function, ast_stat_return, ast_stat_type_alias,
    ast_stat_while::AstStatWhile, ast_type::AstType, ast_type_function, ast_type_group,
    ast_type_intersection, ast_type_pack_explicit, ast_type_pack_generic::AstTypePackGeneric,
    ast_type_pack_variadic::AstTypePackVariadic, ast_type_reference, ast_type_table,
    ast_type_union, parse_result, position::Position as UluaAstPosition,
  },
  rtti,
};
extern crate alloc;

// Port of `cpp/tests/Parser.test.cpp`.
// Automatically aggregated test suite.

mod parser_aligns_things {
  use super::*;
  #[cfg(test)]
  #[test]
  fn parser_aligns_things() {
    use ulua_ast::records::allocator::Allocator;

    let mut alloc = Allocator::new();
    let _one = alloc.alloc(0u8);
    let two = alloc.alloc(0.0_f64);
    let align_mask = align_of::<f64>() - 1;
    let two_addr = two as usize;
    assert_eq!(0, two_addr & align_mask);
  }
}

mod parser_all_disallowed_metamethods {

  #[cfg(test)]
  #[test]
  fn parser_all_disallowed_metamethods() {
    use ulua_ast::records::parse_options::ParseOptions;
    use ulua_common::FFlag::DebugLuauUserDefinedClasses;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _flag = ScopedFastFlag::new(&DebugLuauUserDefinedClasses, true);

    let mut fix = Fixture::default();
    let code = r#"class Foo
      function __index() end
      function __newindex() end
      function __mode() end
      function __metatable() end
      function __type() end
      function __doesnotexist() end
  end"#;

    let result = fix.try_parse(code, &ParseOptions::default());

    assert_eq!(result.errors.len(), 6);
    assert_eq!(
      result.errors[0].get_message(),
      "Classes cannot define '__index' as a metamethod"
    );
    assert_eq!(
      result.errors[1].get_message(),
      "Classes cannot define '__newindex' as a metamethod"
    );
    assert_eq!(
      result.errors[2].get_message(),
      "Classes cannot define '__mode' as a metamethod"
    );
    assert_eq!(
      result.errors[3].get_message(),
      "Classes cannot define '__metatable' as a metamethod"
    );
    assert_eq!(
      result.errors[4].get_message(),
      "Classes cannot define '__type' as a metamethod"
    );
    assert_eq!(
      result.errors[5].get_message(),
      "Cannot use '__doesnotexist' as a method name: names starting with '__' are reserved"
    );
  }
}

mod parser_allocator_can_be_moved {
  use super::*;
  #[cfg(test)]
  #[test]
  fn parser_allocator_can_be_moved() {
    use ulua_ast::records::allocator::Allocator;
    use ulua_unit_test::records::counter::Counter;

    let mut c: *mut Counter = null_mut();

    let mut inner = || {
      let mut allocator = Allocator::new();
      c = allocator.alloc(Counter::counter_counter());

      Allocator::allocator_allocator(&mut allocator)
    };

    Counter::reset_instance_count();
    let _a = Allocator::allocator_allocator(&mut inner());

    assert_eq!(1, unsafe { (*c).id });
  }
}

mod parser_allow_unicode_in_string {

  #[cfg(test)]
  #[test]
  fn parser_allow_unicode_in_string() {
    use ulua_ast::records::parse_options::ParseOptions;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    // C++ source is `local snowman = "☃"` (U+2603); the ported literal was
    // double-encoded UTF-8 mojibake. Use a \u escape (encoding-safe).
    let source = String::from("local snowman = \"\u{2603}\"");
    let options = ParseOptions::new();
    let result = fixture.parse_ex(&source, &options);
    assert!(result.errors.is_empty());
  }
}

mod parser_allowed_metamethods_still_work {
  use ulua_unit_test::type_aliases::scoped_fast_flag::ScopedFastFlag;

  #[cfg(test)]
  #[test]
  fn parser_allowed_metamethods_still_work() {
    use ulua_ast::records::parse_options::ParseOptions;
    use ulua_common::FFlag::DebugLuauUserDefinedClasses;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fix = Fixture::default();
    let code = r"class Foo
      function __tostring(self) end
      function __add(self, other) end
      function __eq(self, other) end
      -- Silly, but allowed.
      function _(self) end
  end";

    let _flag = ScopedFastFlag::new(&DebugLuauUserDefinedClasses, true);
    let result = fix.try_parse(&String::from(code), &ParseOptions::new());

    assert_eq!(result.errors.len(), 0);
  }
}

mod parser_annotations_can_be_tables {

  #[cfg(test)]
  #[test]
  fn parser_annotations_can_be_tables() {
    use ulua_ast::records::parse_options::ParseOptions;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    let stat = fixture.parse(
      "local zero: number\n\
           local one: {x: number, y: string}",
      &ParseOptions::default(),
    );
    let root = unsafe { &*stat };
    assert!(!root.body.data.is_null());
  }
}

mod parser_ast_name_comparison {
  use super::*;
  #[cfg(test)]
  #[test]
  fn parser_ast_name_comparison() {
    use ulua_ast::records::ast_name::AstName;

    let empty1 = AstName::new();
    let empty2 = AstName::new();
    assert!(!empty1.operator_lt(&empty2));

    let one = AstName::ast_name_c_char(c"one".as_ptr() as *const ffi::c_char);
    let two = AstName::ast_name_c_char(c"two".as_ptr() as *const ffi::c_char);

    let one_lt_two = one.operator_lt(&two);
    let two_lt_one = two.operator_lt(&one);
    assert_ne!(one_lt_two, two_lt_one);
  }
}

mod parser_attributes_cannot_be_duplicated {

  use ulua_ast::records::parse_options::ParseOptions;
  #[cfg(test)]
  #[test]
  fn parser_attributes_cannot_be_duplicated() {
    use ulua_ast::records::{location::Location, position::Position};
    use ulua_unit_test::{
      functions::check_first_error_for_attributes::check_first_error_for_attributes,
      records::fixture::Fixture,
    };

    let mut fix = Fixture::default();
    let code = "\n@checked\n    @checked\nfunction hello(x, y)\n    return x + y\nend";

    let result = fix.try_parse(code, &ParseOptions::default());

    let expected_location = Location::new(Position::new(2, 4), Position::new(2, 12));
    let expected_message = "Cannot duplicate attribute '@checked'";

    check_first_error_for_attributes(&result.errors, 1, expected_location, expected_message);
  }
}

mod parser_basic_less_than_check_no_explicit_type_instantiaton {

  use ulua_ast::records::parse_options::ParseOptions;
  #[cfg(test)]
  #[test]
  fn parser_basic_less_than_check_no_explicit_type_instantiaton() {
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    let _stat = fixture.parse(r#"local a = b.c < d"#, &ParseOptions::default());
    assert!(!_stat.is_null());
  }
}

mod parser_basic_parse {

  #[cfg(test)]
  #[test]
  fn parser_basic_parse() {
    use ulua_ast::records::parse_options::ParseOptions;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let _stat = fixture.parse(r#"print("Hello World!")"#, &ParseOptions::new());
    assert!(!_stat.is_null());
  }
}

mod parser_break_return_not_last_error {

  #[cfg(test)]
  #[test]
  fn parser_break_return_not_last_error() {
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    fixture.match_parse_error(
      &String::from("return 0 print(5)"),
      &String::from("Expected <eof>, got 'print'"),
      None,
    );
    fixture.match_parse_error(
      &String::from("while true do break print(5) end"),
      &String::from("Expected 'end' (to close 'do' at column 12), got 'print'"),
      None,
    );
  }
}

mod parser_can_haz_annotations {

  #[cfg(test)]
  #[test]
  fn parser_can_haz_annotations() {
    use ulua_ast::records::parse_options::ParseOptions;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    let block = fixture.parse(
      "local foo: string = \"Hello Types!\"",
      &ParseOptions::default(),
    );
    assert!(!block.is_null());
  }
}

mod parser_can_parse_complex_unions_successfully {

  use ulua_ast::records::parse_options::ParseOptions;
  #[cfg(test)]
  #[test]
  fn parser_can_parse_complex_unions_successfully() {
    use ulua_common::FInt;
    use ulua_unit_test::{records::fixture::Fixture, type_aliases::scoped_fast_int::ScopedFastInt};

    let mut fixture = Fixture::fixture_bool(false);

    // Set recursion limit and type length limit to 10 for all tests in this fixture
    let _sfis = [
      ScopedFastInt::new(&FInt::LuauRecursionLimit, 10),
      ScopedFastInt::new(&FInt::LuauTypeLengthLimit, 10),
    ];

    // Test 1: Complex union with multiple function types, table types, parenthesized types, and intersection types
    fixture.parse(
      r#"local f:
  () -> ()
  |
  () -> ()
  |
  {a: number}
  |
  {b: number}
  |
  ((number))
  |
  ((number))
  |
  (a & (b & nil))
  |
  (a & (b & nil))
  "#,
      &ParseOptions::default(),
    );

    // Test 2: Union of nullable types
    fixture.parse(
      r#"local f: a? | b? | c? | d? | e? | f? | g? | h?
  "#,
      &ParseOptions::default(),
    );

    // Test 3: Exceeded type length limit
    fixture.match_parse_error(
      "local t: a & b & c & d & e & f & g & h & i & j & nil",
      "Exceeded allowed type length; simplify your type annotation to make the code compile",
      None,
    );
  }
}

mod parser_can_parse_leading_ampersand_intersections_successfully {

  use ulua_ast::records::parse_options::ParseOptions;
  #[cfg(test)]
  #[test]
  fn parser_can_parse_leading_ampersand_intersections_successfully() {
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    fixture.parse("type A = & { string } & { number }", &ParseOptions::new());
  }
}

mod parser_can_parse_leading_bar_unions_successfully {

  #[cfg(test)]
  #[test]
  fn parser_can_parse_leading_bar_unions_successfully() {
    use ulua_ast::records::parse_options::ParseOptions;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let source = String::from("type A = | \"Hello\" | \"World\"");
    let options = ParseOptions::new();
    let _result = fixture.parse_ex(&source, &options);
  }
}

mod parser_cannot_use_as_variable_name {

  #[cfg(test)]
  #[test]
  fn parser_cannot_use_as_variable_name() {
    use ulua_ast::records::parse_options::ParseOptions;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let source = String::from(
      "local @blah = 3
  ",
    );
    let mut opts = ParseOptions::new();
    opts.allow_declaration_syntax = true;

    let result = fixture.try_parse(&source, &opts);

    assert!(!result.errors.is_empty());
  }
}

mod parser_cannot_write_multiple_values_in_type_groups {

  #[cfg(test)]
  #[test]
  fn parser_cannot_write_multiple_values_in_type_groups() {
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    fixture.match_parse_error(
      &String::from("type F = ((string, number))"),
      &String::from("Expected '->' when parsing function type, got ')'"),
      None,
    );
    fixture.match_parse_error(
      &String::from("type F = () -> ((string, number))"),
      &String::from("Expected '->' when parsing function type, got ')'"),
      None,
    );
  }
}

mod parser_capture_broken_comment {

  #[cfg(test)]
  #[test]
  fn parser_capture_broken_comment() {
    use ulua_ast::records::{
      location::Location, parse_options::ParseOptions, parse_result::ParseResult,
      position::Position,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    let source = String::from("\n        local a = \"test\"\n\n        --[[broken!\n    ");
    let mut options = ParseOptions::new();
    options.capture_comments = true;

    let result: ParseResult = fixture.try_parse(&source, &options);

    assert_eq!(result.comment_locations.len(), 1);
    let expected_location = Location::new(Position::new(3, 8), Position::new(4, 4));
    assert_eq!(result.comment_locations[0].location, expected_location);
  }
}

mod parser_capture_broken_comment_at_the_start_of_the_file {

  #[cfg(test)]
  #[test]
  fn parser_capture_broken_comment_at_the_start_of_the_file() {
    use ulua_ast::records::{location::Location, parse_options::ParseOptions, position::Position};
    use ulua_unit_test::records::fixture::Fixture;

    let mut fix = Fixture::default();
    let code = String::from("\n        --[[\n    ");
    let mut options = ParseOptions::new();
    options.capture_comments = true;

    let result = fix.try_parse(&code, &options);

    assert_eq!(1, result.comment_locations.len());
    let expected_location = Location::new(Position::new(1, 8), Position::new(2, 4));
    assert_eq!(expected_location, result.comment_locations[0].location);
  }
}

mod parser_capture_comments {

  #[cfg(test)]
  #[test]
  fn parser_capture_comments() {
    use ulua_ast::records::{location::Location, parse_options::ParseOptions, position::Position};
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let source = String::from(
      "\n        --!strict\n\n        local a = 5 -- comment one\n        local b = 8 -- comment two\n        --[[\n            Multi line comment\n        ]]\n        local c = 'see'\n    ",
    );
    let mut options = ParseOptions::new();
    options.capture_comments = true;

    let result = fixture.parse_ex(&source, &options);

    assert!(result.errors.is_empty());
    assert_eq!(result.comment_locations.len(), 4);

    let loc0 = Location::new(Position::new(1, 8), Position::new(1, 17));
    assert_eq!(result.comment_locations[0].location, loc0);

    let loc1 = Location::new(Position::new(3, 20), Position::new(3, 34));
    assert_eq!(result.comment_locations[1].location, loc1);

    let loc2 = Location::new(Position::new(4, 20), Position::new(4, 34));
    assert_eq!(result.comment_locations[2].location, loc2);

    let loc3 = Location::new(Position::new(5, 8), Position::new(7, 10));
    assert_eq!(result.comment_locations[3].location, loc3);
  }
}

mod parser_class_declaration {
  use ulua_common::FFlag;

  use super::*;
  #[cfg(test)]
  #[test]
  fn parser_class_declaration() {
    use ulua_ast::records::{
      ast_class_property::AstClassProperty, ast_expr_call::AstExprCall,
      ast_expr_local::AstExprLocal, ast_node::AstNode, ast_stat_class::AstStatClass,
      ast_stat_expr::AstStatExpr, parse_options::ParseOptions,
    };
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _g = ScopedFastFlag::new(&FFlag::DebugLuauUserDefinedClasses, true);

    let mut fix = Fixture::default();
    let src = String::from(
      "\n        class Point2\n            public x: number\n            public y: number\n        end\n        print(Point2)\n    ",
    );
    let res = fix.try_parse(&src, &ParseOptions::default());
    assert!(res.errors.is_empty());

    assert_eq!(unsafe { (*res.root).body.size }, 2);

    let s0 = unsafe { *(*res.root).body.data.add(0) };
    let first = unsafe { rtti::ast_node_as::<AstStatClass>(s0 as *mut AstNode) };
    assert!(!first.is_null());
    let first = unsafe { &*first };
    assert!(unsafe { (*first.name).name }.operator_eq_c_char(c"Point2"));

    assert_eq!(first.members.size, 2);

    let m1 = unsafe { &*first.members.data.add(0) }.get_if::<AstClassProperty>();
    assert!(m1.is_some());
    assert!(m1.unwrap().name.operator_eq_c_char(c"x"));

    let m2 = unsafe { &*first.members.data.add(1) }.get_if::<AstClassProperty>();
    assert!(m2.is_some());
    assert!(m2.unwrap().name.operator_eq_c_char(c"y"));

    let s1 = unsafe { *(*res.root).body.data.add(1) };
    let second = unsafe { rtti::ast_node_as::<AstStatExpr>(s1 as *mut AstNode) };
    assert!(!second.is_null());
    let second = unsafe { &*second };

    let call = unsafe { rtti::ast_node_as::<AstExprCall>(second.expr as *mut AstNode) };
    assert!(!call.is_null());
    let call = unsafe { &*call };

    assert_eq!(call.args.size, 1);
    let a0 = unsafe { *call.args.data.add(0) };
    let local = unsafe { rtti::ast_node_as::<AstExprLocal>(a0 as *mut AstNode) };
    assert!(!local.is_null());
    let local = unsafe { &*local };
    assert!(local.local == first.name);
  }
}

mod parser_class_indexer {
  use super::*;
  #[cfg(test)]
  #[test]
  fn parser_class_indexer() {
    use ulua_ast::{
      records::{
        ast_stat_declare_extern_type::AstStatDeclareExternType,
        ast_type_reference::AstTypeReference, parse_options::ParseOptions,
      },
      rtti::{ast_node_as, ast_node_is},
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    let parse_result = fixture.parse_ex(
      &String::from(
        "declare extern type Foo with\n\
               prop: boolean\n\
               [string]: number\n\
               end",
      ),
      &ParseOptions::default(),
    );
    let root = unsafe { &*parse_result.root };
    assert_eq!(root.body.size, 1);

    let stat_declare_extern_type = unsafe {
      let stat_ptr = *root.body.data.add(0);
      ast_node_as::<AstStatDeclareExternType>(stat_ptr as *mut ast_node::AstNode)
    };
    assert!(!stat_declare_extern_type.is_null());
    let stat_declare_extern_type = unsafe { &*stat_declare_extern_type };
    assert!(!stat_declare_extern_type.indexer.is_null());
    let indexer = unsafe { &*stat_declare_extern_type.indexer };

    assert!(ast_node_is::<AstTypeReference>(
      indexer.index_type as *mut ast_node::AstNode
    ));
    let index_type_ref =
      unsafe { &*ast_node_as::<AstTypeReference>(indexer.index_type as *mut ast_node::AstNode) };
    assert_eq!(
      unsafe { CStr::from_ptr(index_type_ref.name.value) },
      c"string"
    );

    assert!(ast_node_is::<AstTypeReference>(
      indexer.result_type as *mut ast_node::AstNode
    ));
    let result_type_ref =
      unsafe { &*ast_node_as::<AstTypeReference>(indexer.result_type as *mut ast_node::AstNode) };
    assert_eq!(
      unsafe { CStr::from_ptr(result_type_ref.name.value) },
      c"number"
    );

    let error_parse_result = fixture.match_parse_error(
      &String::from(
        "declare extern type Foo with\n\
               [string]: number\n\
               -- can only have one indexer\n\
               [number]: number\n\
               end",
      ),
      &String::from("Cannot have more than one indexer on an extern type"),
      None,
    );
    let error_root = unsafe { &*error_parse_result.root };
    assert_eq!(error_root.body.size, 1);

    let error_stat_declare_extern_type = unsafe {
      let stat_ptr = *error_root.body.data.add(0);
      ast_node_as::<AstStatDeclareExternType>(stat_ptr as *mut ast_node::AstNode)
    };
    assert!(!error_stat_declare_extern_type.is_null());
    let error_stat_declare_extern_type = unsafe { &*error_stat_declare_extern_type };
    assert!(!error_stat_declare_extern_type.indexer.is_null());
  }
}

mod parser_class_is_still_contextual {
  use ulua_ast::records::parse_options::ParseOptions;

  use super::*;
  #[cfg(test)]
  #[test]
  fn parser_class_is_still_contextual() {
    use ulua_common::FFlag::DebugLuauUserDefinedClasses;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _scoped_flag = ScopedFastFlag::new(&DebugLuauUserDefinedClasses, true);

    let mut fixture = Fixture::default();
    // The C++ R"(...)" DELIMITER parens leaked into the raw-string content here
    // (stray leading '(' / trailing ')' = invalid Lua). Drop them.
    let source = "
          local class = 42
          print(class)
      ";

    let result = fixture.try_parse(source, &ParseOptions::default());

    assert!(result.errors.is_empty());
    assert_eq!(unsafe { (*result.root).body.size }, 2);
    let locals = unsafe {
      rtti::ast_node_as::<ast_stat_local::AstStatLocal>(
        *(*result.root).body.data.add(0) as *mut ast_node::AstNode
      )
    };
    assert!(!locals.is_null());
    assert_eq!(unsafe { (*locals).vars.size }, 1);
    let var_name = unsafe { (*locals).vars.data.add(0).read() };
    assert_eq!(unsafe { CStr::from_ptr((*var_name).name.value) }, c"class");
  }
}

mod parser_class_method_missing_end_error {

  #[cfg(test)]
  #[test]
  fn parser_class_method_missing_end_error() {
    use ulua_common::FFlag::DebugLuauUserDefinedClasses;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _flag = ScopedFastFlag::new(&DebugLuauUserDefinedClasses, true);

    let mut fixture = Fixture::default();
    fixture.match_parse_error(
      &String::from(
        "\n        class Foo\n            function bar()\n                local x = 1\n    ",
      ),
      &String::from("Expected 'end' (to close 'function' at line 3), got <eof>"),
      None,
    );
  }
}

mod parser_class_method_properties {
  use super::*;
  #[cfg(test)]
  #[test]
  fn parser_class_method_properties() {
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    let result1 = fixture.match_parse_error(
      &String::from(
        "\n        declare extern type Foo with\n            -- method's first parameter must be 'self'\n            function method(foo: number)\n            function method2(self)\n        end\n        ",
      ),
      &String::from("'self' must be present as the unannotated first parameter"),
      None,
    );

    assert_eq!(1, unsafe { (*result1.root).body.size });

    let klass = unsafe {
      let node = (*result1.root).body.data.add(0);
      rtti::ast_node_as::<ast_stat_declare_extern_type::AstStatDeclareExternType>(
        *node as *mut ast_node::AstNode,
      )
    };
    assert!(!klass.is_null());
    assert_eq!(2, unsafe { (*klass).props.size });

    let mut fixture2 = Fixture::default();
    let result2 = fixture2.match_parse_error(
      &String::from(
        "\n        declare extern type Foo with\n            function method(self, foo)\n            function method2()\n        end\n        ",
      ),
      &String::from("All declaration parameters aside from 'self' must be annotated"),
      None,
    );

    assert_eq!(1, unsafe { (*result2.root).body.size });

    let klass2 = unsafe {
      let node = (*result2.root).body.data.add(0);
      rtti::ast_node_as::<ast_stat_declare_extern_type::AstStatDeclareExternType>(
        *node as *mut ast_node::AstNode,
      )
    };
    assert!(!klass2.is_null());
    assert_eq!(2, unsafe { (*klass2).props.size });
  }
}

mod parser_class_parse_errors {

  use ulua_common::FFlag;
  #[cfg(test)]
  #[test]
  fn parser_class_parse_errors() {
    use ulua_ast::records::parse_options::ParseOptions;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _g = ScopedFastFlag::new(&FFlag::DebugLuauUserDefinedClasses, true);

    let mut fix = Fixture::default();

    // C++ only checks that the parser does not crash on these malformed inputs.
    let opts = ParseOptions::default();
    fix.try_parse(&String::from(" class Hello "), &opts);
    fix.try_parse(&String::from(" class Hello public "), &opts);
    fix.try_parse(&String::from(" class Hello public x "), &opts);
    fix.try_parse(&String::from(" class Hello public x: "), &opts);
    fix.try_parse(&String::from(" class Hello public x: number "), &opts);
    fix.try_parse(&String::from(" class Hello end "), &opts);
    fix.try_parse(&String::from(" class Hello public end "), &opts);
    fix.try_parse(&String::from(" class Hello private end "), &opts);
    fix.try_parse(&String::from(" class Hello public x end "), &opts);
    fix.try_parse(&String::from(" class Hello public x: end "), &opts);
    fix.try_parse(&String::from(" class Hello public x: number end "), &opts);
    fix.try_parse(
      &String::from(" class Hello public x: number public x: string end "),
      &opts,
    );
    fix.try_parse(
      &String::from(" class Hello public x: number function x() end end "),
      &opts,
    );
  }
}

mod parser_class_public_function {

  use ulua_common::FFlag;
  #[cfg(test)]
  #[test]
  fn parser_class_public_function() {
    use ulua_ast::records::parse_options::ParseOptions;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _g = ScopedFastFlag::new(&FFlag::DebugLuauUserDefinedClasses, true);

    let mut fix = Fixture::default();
    let src =
      String::from("\n        class Foo\n            public function bar() end\n        end\n    ");
    let result = fix.try_parse(&src, &ParseOptions::default());
    assert!(result.errors.is_empty());
  }
}

mod parser_class_recovery_error_in_property_type {
  use ulua_common::FFlag;

  use super::*;
  #[cfg(test)]
  #[test]
  fn parser_class_recovery_error_in_property_type() {
    use ulua_ast::records::{
      ast_class_method::AstClassMethod, ast_class_property::AstClassProperty, ast_node::AstNode,
      ast_stat_class::AstStatClass, parse_options::ParseOptions,
    };
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _g = ScopedFastFlag::new(&FFlag::DebugLuauUserDefinedClasses, true);

    let mut fix = Fixture::default();
    let src = String::from(
      "\nclass Foo\n    public x: { a: number\n    public y: number\n    function bar() end\nend\n    ",
    );
    let result = fix.try_parse(&src, &ParseOptions::default());
    assert!(!result.errors.is_empty());

    assert_eq!(unsafe { (*result.root).body.size }, 1);
    let s0 = unsafe { *(*result.root).body.data.add(0) };
    let cls = unsafe { rtti::ast_node_as::<AstStatClass>(s0 as *mut AstNode) };
    assert!(!cls.is_null());
    let cls = unsafe { &*cls };
    assert_eq!(cls.members.size, 3);

    let m1 = unsafe { &*cls.members.data.add(0) }.get_if::<AstClassProperty>();
    assert!(m1.is_some());
    assert!(m1.unwrap().name.operator_eq_c_char(c"x"));

    let m2 = unsafe { &*cls.members.data.add(1) }.get_if::<AstClassProperty>();
    assert!(m2.is_some());
    assert!(m2.unwrap().name.operator_eq_c_char(c"y"));

    let m3 = unsafe { &*cls.members.data.add(2) }.get_if::<AstClassMethod>();
    assert!(m3.is_some());
    assert!(m3.unwrap().function_name.operator_eq_c_char(c"bar"));
  }
}

mod parser_class_recovery_invalid_body_token {
  use ulua_common::FFlag;

  use super::*;
  #[cfg(test)]
  #[test]
  fn parser_class_recovery_invalid_body_token() {
    use ulua_ast::records::{
      ast_class_method::AstClassMethod, ast_class_property::AstClassProperty, ast_node::AstNode,
      ast_stat_class::AstStatClass, parse_options::ParseOptions,
    };
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _g = ScopedFastFlag::new(&FFlag::DebugLuauUserDefinedClasses, true);

    let mut fix = Fixture::default();
    let src = String::from(
      "\nclass Foo\n    public x: number\n    blah\n    function bar() end\nend\n    ",
    );
    let result = fix.try_parse(&src, &ParseOptions::default());
    assert!(!result.errors.is_empty());

    assert_eq!(unsafe { (*result.root).body.size }, 1);
    let s0 = unsafe { *(*result.root).body.data.add(0) };
    let cls = unsafe { rtti::ast_node_as::<AstStatClass>(s0 as *mut AstNode) };
    assert!(!cls.is_null());
    let cls = unsafe { &*cls };
    assert_eq!(cls.members.size, 2);

    let m1 = unsafe { &*cls.members.data.add(0) }.get_if::<AstClassProperty>();
    assert!(m1.is_some());
    assert!(m1.unwrap().name.operator_eq_c_char(c"x"));

    let m2 = unsafe { &*cls.members.data.add(1) }.get_if::<AstClassMethod>();
    assert!(m2.is_some());
    assert!(m2.unwrap().function_name.operator_eq_c_char(c"bar"));
  }
}

mod parser_class_recovery_public_no_name_and_invalid_body_token {
  use ulua_common::FFlag;

  use super::*;
  #[cfg(test)]
  #[test]
  fn parser_class_recovery_public_no_name_and_invalid_body_token() {
    use ulua_ast::records::{
      ast_class_method::AstClassMethod, ast_class_property::AstClassProperty, ast_node::AstNode,
      ast_stat_class::AstStatClass, parse_options::ParseOptions,
    };
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _g = ScopedFastFlag::new(&FFlag::DebugLuauUserDefinedClasses, true);

    let mut fix = Fixture::default();
    let src = String::from(
      "\nclass Foo\n    public propone\n    function methodone()\n    end\n\n    function methodtwo()\n        blah\n\n    public proptwo\n\n    function methodthree()\n    end\nend\n    ",
    );
    let result = fix.try_parse(&src, &ParseOptions::default());
    assert!(!result.errors.is_empty());

    assert_eq!(unsafe { (*result.root).body.size }, 1);
    let s0 = unsafe { *(*result.root).body.data.add(0) };
    let cls = unsafe { rtti::ast_node_as::<AstStatClass>(s0 as *mut AstNode) };
    assert!(!cls.is_null());
    let cls = unsafe { &*cls };
    assert!(unsafe { (*cls.name).name }.operator_eq_c_char(c"Foo"));

    assert_eq!(cls.members.size, 3);

    let m1 = unsafe { &*cls.members.data.add(0) }.get_if::<AstClassProperty>();
    assert!(m1.is_some());
    assert!(m1.unwrap().name.operator_eq_c_char(c"propone"));

    let m2 = unsafe { &*cls.members.data.add(1) }.get_if::<AstClassMethod>();
    assert!(m2.is_some());
    assert!(m2.unwrap().function_name.operator_eq_c_char(c"methodone"));

    let m3 = unsafe { &*cls.members.data.add(2) }.get_if::<AstClassMethod>();
    assert!(m3.is_some());
    assert!(m3.unwrap().function_name.operator_eq_c_char(c"methodtwo"));
  }
}

mod parser_class_self_cannot_be_annotated {

  use ulua_common::FFlag;
  #[cfg(test)]
  #[test]
  fn parser_class_self_cannot_be_annotated() {
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _g = ScopedFastFlag::new(&FFlag::DebugLuauUserDefinedClasses, true);

    let mut fix = Fixture::default();
    fix.match_parse_error(
      &String::from(
        "\n        class Foobar\n            function baz(self: number, foobar)\n        end\n    ",
      ),
      &String::from("The 'self' parameter cannot have a type annotation"),
      None,
    );
  }
}

mod parser_classes_can_be_shadowed_by_locals {

  use ulua_common::FFlag;
  #[cfg(test)]
  #[test]
  fn parser_classes_can_be_shadowed_by_locals() {
    use ulua_ast::records::parse_options::ParseOptions;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _g = ScopedFastFlag::new(&FFlag::DebugLuauUserDefinedClasses, true);

    let mut fix = Fixture::default();
    // This is legal: exactly one class with a given name, but it may be shadowed by a local.
    let src = String::from(
      "\n        class Foobar\n        end\n\n        -- This is legal: the rule is that there is exactly one class with a\n        -- given name, but we can shadow it with a local.\n        local Foobar\n    ",
    );
    let result = fix.try_parse(&src, &ParseOptions::default());
    assert!(result.errors.is_empty());
  }
}

mod parser_classes_can_have_members_named_public {

  use ulua_common::FFlag;
  #[cfg(test)]
  #[test]
  fn parser_classes_can_have_members_named_public() {
    use ulua_ast::records::parse_options::ParseOptions;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _g = ScopedFastFlag::new(&FFlag::DebugLuauUserDefinedClasses, true);

    let mut fix = Fixture::default();
    let src = String::from(
      "\n        class Foobar\n            function public() end\n        end\n\n        class Barbaz\n            public public\n        end\n    ",
    );
    let result = fix.try_parse(&src, &ParseOptions::default());
    assert!(result.errors.is_empty());
  }
}

mod parser_classes_can_interleave_methods_and_properties {
  use ulua_common::FFlag;

  use super::*;
  #[cfg(test)]
  #[test]
  fn parser_classes_can_interleave_methods_and_properties() {
    use ulua_ast::records::{
      ast_class_method::AstClassMethod, ast_class_property::AstClassProperty, ast_node::AstNode,
      ast_stat_class::AstStatClass, parse_options::ParseOptions,
    };
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _g = ScopedFastFlag::new(&FFlag::DebugLuauUserDefinedClasses, true);

    let mut fix = Fixture::default();
    let src = String::from(
      "\n        class Student\n            public name: string\n\n            function getname(self): string\n                return self.name:upper()\n            end\n\n            public year: number\n\n            function getyear(self): number\n                assert(self.year >= 1900 and self.year < 2100)\n                return self.year\n            end\n        end\n    ",
    );
    let res = fix.try_parse(&src, &ParseOptions::default());
    assert!(res.errors.is_empty());

    assert_eq!(unsafe { (*res.root).body.size }, 1);
    let s0 = unsafe { *(*res.root).body.data.add(0) };
    let cls = unsafe { rtti::ast_node_as::<AstStatClass>(s0 as *mut AstNode) };
    assert!(!cls.is_null());
    let cls = unsafe { &*cls };
    assert!(unsafe { (*cls.name).name }.operator_eq_c_char(c"Student"));

    assert_eq!(cls.members.size, 4);

    let m1 = unsafe { &*cls.members.data.add(0) }.get_if::<AstClassProperty>();
    assert!(m1.is_some());
    assert!(m1.unwrap().name.operator_eq_c_char(c"name"));

    let m2 = unsafe { &*cls.members.data.add(1) }.get_if::<AstClassMethod>();
    assert!(m2.is_some());
    assert!(m2.unwrap().function_name.operator_eq_c_char(c"getname"));

    let m3 = unsafe { &*cls.members.data.add(2) }.get_if::<AstClassProperty>();
    assert!(m3.is_some());
    assert!(m3.unwrap().name.operator_eq_c_char(c"year"));

    let m4 = unsafe { &*cls.members.data.add(3) }.get_if::<AstClassMethod>();
    assert!(m4.is_some());
    assert!(m4.unwrap().function_name.operator_eq_c_char(c"getyear"));
  }
}

mod parser_classes_can_only_have_functions_and_properties {

  #[cfg(test)]
  #[test]
  fn parser_classes_can_only_have_functions_and_properties() {
    use ulua_common::FFlag::DebugLuauUserDefinedClasses;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _flag = ScopedFastFlag::new(&DebugLuauUserDefinedClasses, true);

    let source = r#"
          class Bicycle
              while true do
                  cycle()
              end
          end
      "#;

    let expected_message = "Only class properties and functions can be declared within a class";

    let mut fixture = Fixture::fixture_bool(false);
    let _result = fixture.match_parse_error(source, expected_message, None);
  }
}

mod parser_classes_cannot_be_shadowed_by_classes {

  use ulua_common::FFlag;
  #[cfg(test)]
  #[test]
  fn parser_classes_cannot_be_shadowed_by_classes() {
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _g = ScopedFastFlag::new(&FFlag::DebugLuauUserDefinedClasses, true);

    let mut fix = Fixture::default();
    fix.match_parse_error(
      &String::from(
        "\n        class Foobar\n        end\n\n        class Foobar\n        end\n    ",
      ),
      &String::from("A class named 'Foobar' has already been declared in this module"),
      None,
    );
  }
}

mod parser_classes_cannot_be_shadowed_by_classes_with_local_between {

  use ulua_common::FFlag;
  #[cfg(test)]
  #[test]
  fn parser_classes_cannot_be_shadowed_by_classes_with_local_between() {
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _g = ScopedFastFlag::new(&FFlag::DebugLuauUserDefinedClasses, true);

    let mut fix = Fixture::default();
    fix.match_parse_error(
          &String::from(
              "\n        class Foobar\n        end\n\n        local Foobar\n\n        class Foobar\n        end\n    ",
          ),
          &String::from(
              "A class named 'Foobar' has already been declared in this module",
          ),
          None,
      );
  }
}

mod parser_classes_nested_and_repeated {

  #[cfg(test)]
  #[test]
  fn parser_classes_nested_and_repeated() {
    use ulua_ast::records::parse_options::ParseOptions;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _debug_luau_user_defined_classes =
      ScopedFastFlag::new(&FFlag::DebugLuauUserDefinedClasses, true);

    let mut fix = Fixture::default();
    let code = r#"
          class Foo
          end
          if true then
              class Foo
              end
          end
      "#;

    let result = fix.try_parse(code, &ParseOptions::default());

    assert_eq!(result.errors.len(), 1);
    assert_eq!(
      result.errors[0].get_message(),
      "Cannot declare class 'Foo' inside another statement or expression"
    );
  }
}

mod parser_classes_only_work_at_top_level {

  use ulua_common::FFlag;
  #[cfg(test)]
  #[test]
  fn parser_classes_only_work_at_top_level() {
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _g = ScopedFastFlag::new(&FFlag::DebugLuauUserDefinedClasses, true);

    let mut fix = Fixture::default();
    fix.match_parse_error(
          &String::from(
              "\n            return function ()\n                class DynamicPlayer\n                    public level: number\n                end\n                return DynamicPlayer\n            end\n        ",
          ),
          &String::from(
              "Cannot declare class 'DynamicPlayer' inside another statement or expression",
          ),
          None,
      );

    fix.match_parse_error(
          &String::from(
              "\n            if math.random() > 0.5 then\n                class DynamicPlayer\n                    public level: number\n                end\n            end\n        ",
          ),
          &String::from(
              "Cannot declare class 'DynamicPlayer' inside another statement or expression",
          ),
          None,
      );
  }
}

mod parser_classes_work_after_other_statements {
  use ulua_common::FFlag;

  use super::*;
  #[cfg(test)]
  #[test]
  fn parser_classes_work_after_other_statements() {
    use ulua_ast::records::parse_options::ParseOptions;
    use ulua_unit_test::records::fixture::Fixture;

    FFlag::DebugLuauUserDefinedClasses.set(true);

    let mut fixture = Fixture::default();
    let source = r#"
          if math.random() > 0.5 then
              print("I am a test case!")
          end
  
          class Player
              public health: number
          end
      "#;

    let result = fixture.try_parse(source, &ParseOptions::new());

    assert_eq!(result.errors.len(), 0);

    assert_eq!(2, unsafe { (*result.root).body.size });
    let cls = unsafe {
      rtti::ast_node_as::<ast_stat_class::AstStatClass>(
        *(*result.root).body.data.add(1) as *mut ast_node::AstNode
      )
    };
    assert!(!cls.is_null());
    // C++ `CHECK_EQ(cls->name->name, "Player")` compares AstName content (strcmp),
    // not the interned pointer. The port compared `name.value` against the literal's
    // pointer, which never matches. Compare NUL-terminated content instead.
    assert_eq!(
      unsafe { CStr::from_ptr((*cls).name.as_ref().unwrap().name.value) },
      c"Player"
    );
  }
}

mod parser_complex_union_in_generic_ty {
  use super::*;
  #[cfg(test)]
  #[test]
  fn parser_complex_union_in_generic_ty() {
    use ulua_ast::records::parse_options::ParseOptions;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fix = Fixture::default();
    let code = String::from(
      "type X<T> = T\n\
           local x: X<\n\
               | number\n\
               | boolean\n\
               | string\n\
           >",
    );

    let result = fix.try_parse(&code, &ParseOptions::new());

    assert_eq!(result.errors.len(), 0);

    let block = unsafe { &*result.root };
    assert_eq!(block.body.size, 2);

    let stat_1 = unsafe { *block.body.data.add(1) };
    let assignment = unsafe {
      rtti::ast_node_as::<ast_stat_local::AstStatLocal>(stat_1 as *mut ast_node::AstNode)
    };
    assert!(!assignment.is_null());

    let assignment = unsafe { &*assignment };
    assert_eq!(assignment.vars.size, 1);
    assert_eq!(assignment.values.size, 0);

    let var_0 = unsafe { &**assignment.vars.data.add(0) };
    assert_eq!(unsafe { CStr::from_ptr(var_0.name.value) }, c"x");

    let annotation = unsafe { &*var_0.annotation };
    let generic_ty = unsafe {
      rtti::ast_node_as::<ast_type_reference::AstTypeReference>(
        annotation as *const _ as *mut ast_node::AstNode,
      )
    };
    assert!(!generic_ty.is_null());

    let generic_ty = unsafe { &*generic_ty };
    assert_eq!(generic_ty.parameters.size, 1);

    let param_0 = unsafe { *generic_ty.parameters.data.add(0) };
    let param_ty = unsafe { &*param_0.r#type };
    let union_ty = unsafe {
      rtti::ast_node_as::<ast_type_union::AstTypeUnion>(
        param_ty as *const _ as *mut ast_node::AstNode,
      )
    };
    assert!(!union_ty.is_null());

    let union_ty = unsafe { &*union_ty };
    assert_eq!(union_ty.types.size, 3);

    let expected_types: [&CStr; 3] = [c"number", c"boolean", c"string"];
    for (i, expected) in expected_types.iter().enumerate() {
      let ty = unsafe { &*(*union_ty.types.data.add(i)) };
      let ty_ref = unsafe {
        rtti::ast_node_as::<ast_type_reference::AstTypeReference>(
          ty as *const _ as *mut ast_node::AstNode,
        )
      };
      assert!(!ty_ref.is_null());
      assert_eq!(unsafe { CStr::from_ptr((*ty_ref).name.value) }, expected);
    }
  }
}

mod parser_const_shadow {

  use ulua_ast::records::parse_options::ParseOptions;
  #[cfg(test)]
  #[test]
  fn parser_const_shadow() {
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);

    let source = String::from(
      "const a = 42\n\
           const a = 43\n\
           \n\
           do\n\
               const a = 44\n\
               do\n\
                   local a = 44.1\n\
                   do\n\
                       const a = 44.2\n\
                   end\n\
                   a = 44.3\n\
               end\n\
           end\n\
           \n\
           function f()\n\
               const a = 45\n\
               local a = 46\n\
               return function(x) a = x end\n\
           end",
    );
    let _stat = fixture.parse(&source, &ParseOptions::default());
    assert!(!_stat.is_null());
  }
}

mod parser_continue_not_last_error {

  #[cfg(test)]
  #[test]
  fn parser_continue_not_last_error() {
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    fixture.match_parse_error(
      &String::from("while true do continue print(5) end"),
      &String::from("Expected 'end' (to close 'do' at column 12), got 'print'"),
      None,
    );
  }
}

mod parser_debugnoinline_not_allowed_without_flag {

  #[cfg(test)]
  #[test]
  fn parser_debugnoinline_not_allowed_without_flag() {
    use ulua_ast::records::{location::Location, parse_options::ParseOptions, position::Position};
    use ulua_unit_test::{
      functions::check_first_error_for_attributes::check_first_error_for_attributes,
      records::fixture::Fixture,
    };

    let mut fix = Fixture::default();
    let result = fix.try_parse(
      &String::from("\n@debugnoinline\nlocal function hello(x, y)\n    return x + y\nend"),
      &ParseOptions::default(),
    );

    check_first_error_for_attributes(
      &result.errors,
      1,
      Location::new(Position::new(1, 0), Position::new(1, 14)),
      "Invalid attribute '@debugnoinline'",
    );
  }
}

mod parser_disallow_double_underscore_properties {

  use ulua_common::FFlag;
  #[cfg(test)]
  #[test]
  fn parser_disallow_double_underscore_properties() {
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _g = ScopedFastFlag::new(&FFlag::DebugLuauUserDefinedClasses, true);

    let mut fix = Fixture::default();
    fix.match_parse_error(
      &String::from("\n        class Foo\n            public __add: any\n        end\n    "),
      &String::from("Class properties cannot start with '__'"),
      None,
    );
  }
}

mod parser_do_block_end_location_is_after_end_token {
  use super::*;
  #[cfg(test)]
  #[test]
  fn parser_do_block_end_location_is_after_end_token() {
    use ulua_ast::records::{
      ast_stat_block::AstStatBlock, location::Location, parse_options::ParseOptions,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    let stat = fixture.parse(
      "\n        do\n            local x = 1\n        end\n    ",
      &ParseOptions::default(),
    );
    assert!(!stat.is_null());

    let root = unsafe { &*stat };
    assert_eq!(root.body.size, 1);

    let block =
      unsafe { rtti::ast_node_as::<AstStatBlock>(*root.body.data as *mut ast_node::AstNode) };
    assert!(!block.is_null());

    let expected_location = Location::new(UluaAstPosition::new(1, 8), UluaAstPosition::new(3, 11));
    assert_eq!(unsafe { (*block).base.base.location }, expected_location);
  }
}

mod parser_do_block_with_no_end {
  use super::*;
  #[cfg(test)]
  #[test]
  fn parser_do_block_with_no_end() {
    use ulua_ast::records::{
      ast_stat_block::AstStatBlock, parse_options::ParseOptions, parse_result::ParseResult,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let source = String::from("do\n");
    let options = ParseOptions::new();
    let result: ParseResult = fixture.try_parse(&source, &options);

    assert_eq!(1, result.errors.len());

    let stat0 = unsafe { (*result.root).body.data.add(0) };
    let stat0_block =
      unsafe { rtti::ast_node_as::<AstStatBlock>(*stat0 as *mut ast_node::AstNode) };
    assert!(!stat0_block.is_null());

    assert!(!unsafe { (*stat0_block).has_end });
  }
}

mod parser_do_end_block_with_cst {
  use super::*;
  #[cfg(test)]
  #[test]
  fn parser_do_end_block_with_cst() {
    use ulua_ast::records::{
      ast_node::AstNode, ast_stat_block::AstStatBlock, cst_stat_do::CstStatDo,
      parse_options::ParseOptions, parse_result::ParseResult, position::Position,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let source =
      String::from("\n        do\n            local hello = \"world\"\n        end\n    ");
    let mut parse_options = ParseOptions::new();
    parse_options.store_cst_data = true;

    let result: ParseResult = fixture.parse_ex(&source, &parse_options);

    assert!(!result.root.is_null());

    let module_cst_node = result.cst_node_map.find(&(result.root as *mut AstNode));
    assert!(module_cst_node.is_none());

    assert_eq!(1, unsafe { (*result.root).body.size });

    let do_block = unsafe {
      rtti::ast_node_as::<AstStatBlock>(*(*result.root).body.data.add(0) as *mut AstNode)
    };
    assert!(!do_block.is_null());

    let do_block_cst_node = result.cst_node_map.find(&(do_block as *mut AstNode));
    assert!(!do_block_cst_node.is_none());

    let do_block_cst = unsafe { rtti::cst_node_as::<CstStatDo>(*do_block_cst_node.unwrap()) };
    assert!(!do_block_cst.is_null());

    assert_eq!(Position::new(2, 12), unsafe {
      (*do_block_cst).stats_start_position
    });
    assert_eq!(Position::new(3, 8), unsafe { (*do_block_cst).end_position });
  }
}

mod parser_do_not_hang_on_incomplete_attribute_list {

  use ulua_ast::records::parse_options::ParseOptions;
  #[cfg(test)]
  #[test]
  fn parser_do_not_hang_on_incomplete_attribute_list() {
    use ulua_ast::records::{location::Location, position::Position};
    use ulua_unit_test::{
      functions::check_first_error_for_attributes::check_first_error_for_attributes,
      records::fixture::Fixture,
    };

    let mut fix = Fixture::default();

    let code1 = r#"
@[]
function hello(x, y)
    return x + y
end"#;
    let result1 = fix.try_parse(code1, &ParseOptions::default());
    let expected_location1 = Location::new(Position::new(1, 0), Position::new(1, 3));
    let expected_message1 = "Attribute list cannot be empty";
    check_first_error_for_attributes(&result1.errors, 1, expected_location1, expected_message1);

    let code2 = r"@[";
    let result2 = fix.try_parse(code2, &ParseOptions::default());
    let expected_location2 = Location::new(Position::new(0, 2), Position::new(0, 2));
    let expected_message2 = "Expected identifier when parsing attribute name, got <eof>";
    check_first_error_for_attributes(&result2.errors, 1, expected_location2, expected_message2);

    let code3 = "@[\n        function foo() end\n    ]";
    let result3 = fix.try_parse(code3, &ParseOptions::default());
    let expected_location3 = Location::new(Position::new(1, 8), Position::new(1, 16));
    let expected_message3 = "Expected identifier when parsing attribute name, got 'function'";
    check_first_error_for_attributes(&result3.errors, 1, expected_location3, expected_message3);

    let code4 = "@[deprecated\n        local function foo() end\n    ]";
    let result4 = fix.try_parse(code4, &ParseOptions::default());
    let expected_location4 = Location::new(Position::new(1, 8), Position::new(1, 13));
    let expected_message4 = "Expected ']' (to close '@[' at line 1), got 'local'";
    check_first_error_for_attributes(&result4.errors, 1, expected_location4, expected_message4);
  }
}

mod parser_dont_parse_attribute_on_argument_non_function {
  use ulua_unit_test::functions::check_first_error_for_attributes::check_first_error_for_attributes;

  #[cfg(test)]
  #[test]
  fn parser_dont_parse_attribute_on_argument_non_function() {
    use ulua_ast::records::{location::Location, parse_options::ParseOptions, position::Position};
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let source = String::from(
      "\nlocal function invoker(f, y)\n    return f(y)\nend\n\ninvoker(function(x) return (x + 2) end, @checked 1)\n",
    );
    let options = ParseOptions::new();
    let result = fixture.try_parse(&source, &options);

    let expected_location = Location::new(Position::new(5, 40), Position::new(5, 48));
    let expected_message = "Expected 'function' declaration after attribute, but got '1' instead";

    check_first_error_for_attributes(&result.errors, 1, expected_location, expected_message);
  }
}

mod parser_dont_parse_attributes_on_non_function_stat {

  #[cfg(test)]
  #[test]
  fn parser_dont_parse_attributes_on_non_function_stat() {
    use ulua_ast::records::{location::Location, parse_options::ParseOptions, position::Position};
    use ulua_common::FFlag;
    use ulua_unit_test::{
      functions::check_first_error_for_attributes::check_first_error_for_attributes,
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let mut fix = Fixture::default();

    let pr1 = fix.try_parse(
      &String::from("\n@checked\nif a<0 then a = 0 end"),
      &ParseOptions::default(),
    );
    let expected_location = Location::new(Position::new(2, 0), Position::new(2, 2));
    let expected_message = "Expected 'function', 'local function', 'const function', 'declare function' or a function type declaration after attribute, but got 'if' instead";
    check_first_error_for_attributes(&pr1.errors, 1, expected_location, expected_message);

    let pr2 = fix.try_parse(
      &String::from("\nlocal i = 1\n@checked\nwhile a[i] do\n    print(a[i])\n    i = i + 1\nend"),
      &ParseOptions::default(),
    );
    let expected_location = Location::new(Position::new(3, 0), Position::new(3, 5));
    let expected_message = "Expected 'function', 'local function', 'const function', 'declare function' or a function type declaration after attribute, but got 'while' instead";
    check_first_error_for_attributes(&pr2.errors, 1, expected_location, expected_message);

    let pr3 = fix.try_parse(
          &String::from("\n@checked\ndo\n    local a2 = 2*a\n    local d = sqrt(b^2 - 4*a*c)\n    x1 = (-b + d)/a2\n    x2 = (-b - d)/a2\nend"),
          &ParseOptions::default(),
      );
    let expected_location = Location::new(Position::new(2, 0), Position::new(2, 2));
    let expected_message = "Expected 'function', 'local function', 'const function', 'declare function' or a function type declaration after attribute, but got 'do' instead";
    check_first_error_for_attributes(&pr3.errors, 1, expected_location, expected_message);

    let pr4 = fix.try_parse(
      &String::from("\n@checked\nfor i=1,10 do print(i) end\n"),
      &ParseOptions::default(),
    );
    let expected_location = Location::new(Position::new(2, 0), Position::new(2, 3));
    let expected_message = "Expected 'function', 'local function', 'const function', 'declare function' or a function type declaration after attribute, but got 'for' instead";
    check_first_error_for_attributes(&pr4.errors, 1, expected_location, expected_message);

    let pr5 = fix.try_parse(
      &String::from("\n@checked\nrepeat\n    line = io.read()\nuntil line ~= \"\"\n"),
      &ParseOptions::default(),
    );
    let expected_location = Location::new(Position::new(2, 0), Position::new(2, 6));
    let expected_message = "Expected 'function', 'local function', 'const function', 'declare function' or a function type declaration after attribute, but got 'repeat' instead";
    check_first_error_for_attributes(&pr5.errors, 1, expected_location, expected_message);

    let pr6 = fix.try_parse(
      &String::from("\n@checked\nlocal x = 10\n"),
      &ParseOptions::default(),
    );
    let expected_location = Location::new(Position::new(2, 6), Position::new(2, 7));
    check_first_error_for_attributes(
      &pr6.errors,
      1,
      expected_location,
      "Expected 'function' after local declaration with attribute, but got 'x' instead",
    );

    // C++: ScopedFastFlag sffs[] = {{FFlag::LuauExportValueSyntax, true}};
    // These live through pr7..pr9; without LuauExportValueSyntax the `export local`
    // case below takes a different parse path and reports a different error.
    let _sff_export = ScopedFastFlag::new(&FFlag::LuauExportValueSyntax, true);

    let pr7 = fix.try_parse(
      &String::from("\n@checked\nexport local x = 10\n"),
      &ParseOptions::default(),
    );
    let expected_location = Location::new(Position::new(2, 7), Position::new(2, 12));
    check_first_error_for_attributes(
      &pr7.errors,
      1,
      expected_location,
      "Expected 'function' after export declaration with attribute, but got 'local' instead",
    );

    let pr8 = fix.try_parse(
          &String::from(
              "\nlocal i = 1\nwhile a[i] do\n    if a[i] == v then @checked break end\n    i = i + 1\nend\n",
          ),
          &ParseOptions::default(),
      );
    let expected_location = Location::new(Position::new(3, 31), Position::new(3, 36));
    let expected_message = "Expected 'function', 'local function', 'const function', 'declare function' or a function type declaration after attribute, but got 'break' instead";
    check_first_error_for_attributes(&pr8.errors, 1, expected_location, expected_message);

    let pr9 = fix.try_parse(
      &String::from("\nfunction foo1 () @checked return 'a' end\n"),
      &ParseOptions::default(),
    );
    let expected_location = Location::new(Position::new(1, 26), Position::new(1, 32));
    let expected_message = "Expected 'function', 'local function', 'const function', 'declare function' or a function type declaration after attribute, but got 'return' instead";
    check_first_error_for_attributes(&pr9.errors, 1, expected_location, expected_message);
  }
}

mod parser_dont_parse_attributes_on_non_function_type_declarations {
  use ulua_unit_test::functions::check_first_error_for_attributes::check_first_error_for_attributes;

  #[cfg(test)]
  #[test]
  fn parser_dont_parse_attributes_on_non_function_type_declarations() {
    use ulua_ast::records::{location::Location, parse_options::ParseOptions, position::Position};
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let mut opts = ParseOptions::new();
    opts.allow_declaration_syntax = true;

    let source1 = String::from("\n@checked declare foo: number\n");
    let result1 = fixture.try_parse(&source1, &opts);

    let expected_location1 = Location::new(Position::new(1, 17), Position::new(1, 20));
    let expected_message1 =
      "Expected a function type declaration after attribute, but got 'foo' instead";
    check_first_error_for_attributes(&result1.errors, 1, expected_location1, expected_message1);

    let source2 = String::from(
      "\n@checked declare extern type Foo with\n    prop: number\n    function method(self, foo: number): string\nend",
    );
    let result2 = fixture.try_parse(&source2, &opts);

    let expected_location2 = Location::new(Position::new(1, 17), Position::new(1, 23));
    let expected_message2 =
      "Expected a function type declaration after attribute, but got 'extern' instead";
    check_first_error_for_attributes(&result2.errors, 1, expected_location2, expected_message2);

    let source3 = String::from("\ndeclare bit32: {\n    band: @checked number\n})");
    let result3 = fixture.try_parse(&source3, &opts);

    let expected_location3 = Location::new(Position::new(2, 19), Position::new(2, 25));
    let expected_message3 = "Expected '(' when parsing function parameters, got 'number'";
    check_first_error_for_attributes(&result3.errors, 1, expected_location3, expected_message3);
  }
}

mod parser_duplicate_class_methods {

  #[cfg(test)]
  #[test]
  fn parser_duplicate_class_methods() {
    use ulua_common::FFlag::DebugLuauUserDefinedClasses;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _flag = ScopedFastFlag::new(&DebugLuauUserDefinedClasses, true);

    let mut fixture = Fixture::default();
    fixture.match_parse_error(
      &String::from("class Hello\n    function hi() end\n    function hi() end\nend"),
      &String::from("Duplicate class member 'hi'"),
      None,
    );
  }
}

mod parser_duplicate_unnamed_class_methods {

  use ulua_ast::records::parse_options::ParseOptions;
  use ulua_common::FFlag;
  #[cfg(test)]
  #[test]
  fn parser_duplicate_unnamed_class_methods() {
    use ulua_unit_test::records::fixture::Fixture;

    FFlag::DebugLuauUserDefinedClasses.set(true);

    let mut fixture = Fixture::default();
    let source = "\nclass Hello\n    function () end\n    function () end\nend\n        ";

    let result = fixture.try_parse(source, &ParseOptions::default());

    assert_eq!(result.errors.len(), 3);
    assert_eq!(
      result.errors[0].get_message(),
      "Expected identifier when parsing method name, got '('"
    );
    assert_eq!(
      result.errors[1].get_message(),
      "Expected identifier when parsing method name, got '('"
    );
    assert_eq!(
      result.errors[2].get_message(),
      "Duplicate class member '%error-id%'"
    );
  }
}

mod parser_empty_attribute_name_is_not_allowed {

  use ulua_ast::records::parse_options::ParseOptions;
  #[cfg(test)]
  #[test]
  fn parser_empty_attribute_name_is_not_allowed() {
    use ulua_ast::records::{location::Location, position::Position};
    use ulua_unit_test::{
      functions::check_first_error_for_attributes::check_first_error_for_attributes,
      records::fixture::Fixture,
    };

    let mut fix = Fixture::default();
    let code = "\n@\nfunction hello(x, y)\n    return x + y\nend";

    let result = fix.try_parse(code, &ParseOptions::default());

    let expected_location = Location::new(Position::new(1, 0), Position::new(1, 1));
    let expected_message = "Attribute name is missing";

    check_first_error_for_attributes(&result.errors, 1, expected_location, expected_message);
  }
}

mod parser_empty_function_type_error_recovery {

  #[cfg(test)]
  #[test]
  fn parser_empty_function_type_error_recovery() {
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::default();

    // C++ uses `parse(...)` (which throws ParseErrors) and checks the first error
    // message. The port called the unrelated `parseType` frontend stub and ignored
    // the result entirely. Use `match_parse_error`, which parses and asserts the
    // first error message.

    // Case 1: empty `()` in a union — special-cased error message.
    fixture.match_parse_error(
      &String::from("\ntype Fn = (\n    any,\n    string | number | ()\n) -> any\n"),
      &String::from("Expected '->' after '()' when parsing function type; did you mean 'nil'?"),
      None,
    );

    // Case 2: arguments present, no special case.
    fixture.match_parse_error(
      &String::from("type Fn = (any, string | number | (number, number)) -> any"),
      &String::from("Expected '->' when parsing function type, got ')'"),
      None,
    );

    // Case 3: generic arguments present, no special case.
    fixture.match_parse_error(
      &String::from("type Fn = (any, string | number | <a>()) -> any"),
      &String::from("Expected '->' when parsing function type, got ')'"),
      None,
    );

    // Case 4: variadic generic arguments present, no special case.
    fixture.match_parse_error(
      &String::from("type Fn = (any, string | number | <a...>()) -> any"),
      &String::from("Expected '->' when parsing function type, got ')'"),
      None,
    );
  }
}

mod parser_end_extent_doesnt_consume_comments {
  use super::*;
  #[cfg(test)]
  #[test]
  fn parser_end_extent_doesnt_consume_comments() {
    use ulua_ast::records::parse_options::ParseOptions;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    let source =
      String::from("\n        type F = number\n        --comment\n        print('hello')\n    ");
    let block = fixture.parse(&source, &ParseOptions::new());
    unsafe {
      let body = (*block).body;
      assert_eq!(2, body.size);
      let first_stat = *body.data.add(0);
      let first_loc = (*first_stat).base.location;
      assert_eq!(UluaAstPosition::new(1, 23), first_loc.end);
    }
  }
}

mod parser_end_extent_doesnt_consume_comments_even_with_capture {

  #[cfg(test)]
  #[test]
  fn parser_end_extent_doesnt_consume_comments_even_with_capture() {
    use ulua_ast::records::{ast_stat::AstStat, parse_options::ParseOptions, position::Position};
    use ulua_unit_test::records::fixture::Fixture;

    let mut fix = Fixture::default();
    let code =
      String::from("\n        type F = number\n        --comment\n        print('hello')\n    ");
    let mut opts = ParseOptions::new();
    opts.capture_comments = true;

    let block = fix.parse(&code, &opts);

    unsafe {
      assert_eq!(2, (*block).body.size);
      let first_stat: *mut AstStat = *((*block).body.data.add(0));
      let expected_end = Position::new(1, 23);
      assert_eq!(expected_end, (*first_stat).base.location.end);
    }
  }
}

mod parser_end_extent_of_functions_unions_and_intersections {
  use super::*;
  #[cfg(test)]
  #[test]
  fn parser_end_extent_of_functions_unions_and_intersections() {
    use ulua_ast::records::parse_options::ParseOptions;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    let source = String::from(
      "\n        type F = (string) -> string\n        type G = string | number | boolean\n        type H = string & number & boolean\n        print('hello')\n    ",
    );
    let result = fixture.try_parse(&source, &ParseOptions::new());
    assert_eq!(4, unsafe { (*result.root).body.size });
    assert_eq!(UluaAstPosition::new(1, 35), unsafe {
      (**(*result.root).body.data.add(0)).base.location.end
    });
    assert_eq!(UluaAstPosition::new(2, 42), unsafe {
      (**(*result.root).body.data.add(1)).base.location.end
    });
    assert_eq!(UluaAstPosition::new(3, 42), unsafe {
      (**(*result.root).body.data.add(2)).base.location.end
    });
  }
}

mod parser_error_const_function_reassignment {

  #[cfg(test)]
  #[test]
  fn parser_error_const_function_reassignment() {
    use ulua_common::FFlag::LuauExportValueSyntax;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let mut fixture = Fixture::fixture_bool(false);
    let _sff_luau_export_value_syntax = ScopedFastFlag::new(&LuauExportValueSyntax, true);

    let source = String::from("const function a() return 42 end; a = 43");
    let message = String::from("Variable 'a' is constant and may not be reassigned");

    let _result = fixture.match_parse_error(&source, &message, None);
  }
}

mod parser_error_const_not_initialized {

  #[cfg(test)]
  #[test]
  fn parser_error_const_not_initialized() {
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    fixture.match_parse_error(
      &String::from("const c"),
      &String::from("Missing initializer in const declaration"),
      None,
    );

    fixture.match_parse_error(
      &String::from("const a, b = nil"),
      &String::from("Missing initializer in const declaration"),
      None,
    );

    fixture.match_parse_error(
      &String::from("const a, b, c = f(), 42"),
      &String::from("Missing initializer in const declaration"),
      None,
    );

    fixture.match_parse_error(
      &String::from("const a, b, c = ..., 42"),
      &String::from("Missing initializer in const declaration"),
      None,
    );
  }
}

mod parser_error_const_reassignment {
  use super::*;
  #[cfg(test)]
  #[test]
  fn parser_error_const_reassignment() {
    use alloc::string::String;

    use ulua_common::FFlag::LuauExportValueSyntax;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let mut fixture = Fixture::fixture_bool(false);
    let _sff_luau_export_value_syntax = ScopedFastFlag::new(&LuauExportValueSyntax, true);

    fixture.match_parse_error(
      &String::from("const a = 42; a = 43"),
      &String::from("Variable 'a' is constant and may not be reassigned"),
      None,
    );

    fixture.match_parse_error(
      &String::from("local b; const a = 42; a, b = 43"),
      &String::from("Variable 'a' is constant and may not be reassigned"),
      None,
    );

    fixture.match_parse_error(
      &String::from("local b; const a = 42; b, a = 43"),
      &String::from("Variable 'a' is constant and may not be reassigned"),
      None,
    );

    fixture.match_parse_error(
      &String::from("local b; const a = 42; b, a = ..."),
      &String::from("Variable 'a' is constant and may not be reassigned"),
      None,
    );

    fixture.match_parse_error(
      &String::from("const a = 42; function a() end"),
      &String::from("Variable 'a' is constant and may not be reassigned"),
      None,
    );
  }
}

mod parser_error_message_for_using_function_as_type_annotation {

  #[cfg(test)]
  #[test]
  fn parser_error_message_for_using_function_as_type_annotation() {
    use ulua_ast::records::parse_options::ParseOptions;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fix = Fixture::default();
    let code = String::from("type Foo = function");
    let result = fix.try_parse(&code, &ParseOptions::new());

    assert_eq!(result.errors.len(), 1);
    assert_eq!(
      result.errors[0].get_message().as_str(),
      "Using 'function' as a type annotation is not supported, consider replacing with a function type annotation e.g. '(...any) -> ...any'"
    );
  }
}

mod parser_error_on_confusable {

  #[cfg(test)]
  #[test]
  fn parser_error_on_confusable() {
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    // C++ source is `local pi = 3․13` (U+2024 ONE DOT LEADER, a `.` confusable).
    // The ported literal was double-encoded UTF-8 mojibake; use a \u escape.
    fixture.match_parse_error(
          &String::from("\n        local pi = 3\u{2024}13\n    "),
          &String::from("Expected identifier when parsing expression, got Unicode character U+2024 (did you mean '.'?)"),
          None,
      );
  }
}

mod parser_error_on_non_utf_8_sequence {
  use super::*;
  #[cfg(test)]
  #[test]
  fn parser_error_on_non_utf_8_sequence() {
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    let expected =
      String::from("Expected identifier when parsing expression, got invalid UTF-8 sequence");

    // C++ source used raw bytes \xFF and \xE2 which are invalid UTF-8. A lossy
    // conversion would replace those bytes with U+FFFD (valid UTF-8), so the lexer
    // would never see an invalid sequence. The lexer reads the Buffer byte-by-byte
    // (via a raw *const c_char), so we build the String from the exact bytes with
    // `from_utf8_unchecked` to keep the raw 0xFF / 0xE2 intact, matching C++.
    let source1 = unsafe {
      String::from_utf8_unchecked(alloc::vec![
        b'l', b'o', b'c', b'a', b'l', b' ', b'p', b'i', b' ', b'=', b' ', 0xFF, b'!',
      ])
    };
    fixture.match_parse_error(&source1, &expected, None);
    let source2 = unsafe {
      String::from_utf8_unchecked(alloc::vec![
        b'l', b'o', b'c', b'a', b'l', b' ', b'p', b'i', b' ', b'=', b' ', 0xE2, b'!',
      ])
    };
    fixture.match_parse_error(&source2, &expected, None);
  }
}

mod parser_error_on_unicode {

  #[cfg(test)]
  #[test]
  fn parser_error_on_unicode() {
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    // C++ source is `local ☃ = 10` (U+2603 SNOWMAN). The ported literal was
    // double-encoded UTF-8 mojibake; use an explicit \u escape (encoding-safe).
    fixture.match_parse_error(
      &String::from("local \u{2603} = 10"),
      &String::from("Expected identifier when parsing variable name, got Unicode character U+2603"),
      None,
    );
  }
}

mod parser_explicit_type_instantiation_empty_list {

  #[cfg(test)]
  #[test]
  fn parser_explicit_type_instantiation_empty_list() {
    use ulua_ast::records::parse_options::ParseOptions;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    let stat = fixture.parse("f<<>>()", &ParseOptions::default());
    assert!(!stat.is_null());
  }
}

mod parser_explicit_type_instantiation_errors {

  #[cfg(test)]
  #[test]
  fn parser_explicit_type_instantiation_errors() {
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    fixture.match_parse_error(
      &String::from("local a = x:a<<T>>"),
      &String::from("Expected '(', '{' or <string> when parsing function call, got <eof>"),
      None,
    );
  }
}

mod parser_explicit_type_instantiation_expression {

  #[cfg(test)]
  #[test]
  fn parser_explicit_type_instantiation_expression() {
    use ulua_ast::records::parse_options::ParseOptions;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    let stat = fixture.parse("local x = f<<T, U>>", &ParseOptions::default());
    assert!(!stat.is_null());
  }
}

mod parser_explicit_type_instantiation_expression_call {
  use super::*;
  #[cfg(test)]
  #[test]
  fn parser_explicit_type_instantiation_expression_call() {
    use ulua_ast::records::{
      ast_expr_call::AstExprCall, ast_expr_instantiate::AstExprInstantiate,
      ast_stat_local::AstStatLocal, parse_options::ParseOptions, parse_result::ParseResult,
    };
    use ulua_unit_test::{
      functions::string_at_location::string_at_location, records::fixture::Fixture,
    };

    let mut fixture = Fixture::fixture_bool(false);
    let source = String::from("local x = f<<T, U>>()");
    let options = ParseOptions::new();
    let result: ParseResult = fixture.parse_ex(&source, &options);

    assert!(!result.root.is_null());

    let local = unsafe { (*result.root).body.data.add(0) };
    let local = unsafe { rtti::ast_node_as::<AstStatLocal>(*local as *mut ast_node::AstNode) };
    assert!(!local.is_null());

    assert_eq!(1, unsafe { (*local).vars.size });

    let expr = unsafe { (*local).values.data.add(0) };
    assert!(!expr.is_null());

    let call = unsafe { rtti::ast_node_as::<AstExprCall>(*expr as *mut ast_node::AstNode) };
    assert!(!call.is_null());

    let explicit_type_instantiation =
      unsafe { rtti::ast_node_as::<AstExprInstantiate>((*call).func as *mut ast_node::AstNode) };
    assert!(!explicit_type_instantiation.is_null());

    let location = unsafe { &(*explicit_type_instantiation).base.base.location };
    let expected = string_at_location(&source, location);
    assert_eq!("f<<T, U>>", expected);
  }
}

mod parser_explicit_type_instantiation_indexing {

  #[cfg(test)]
  #[test]
  fn parser_explicit_type_instantiation_indexing() {
    use ulua_ast::records::parse_options::ParseOptions;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    let stat = fixture.parse(
      r#"t.f<<T, U>>()
          t:f<<T, U>>()
          t["f"]<<T, U>>()"#,
      &ParseOptions::default(),
    );
    assert!(!stat.is_null());
  }
}

mod parser_explicit_type_instantiation_statement {

  #[cfg(test)]
  #[test]
  fn parser_explicit_type_instantiation_statement() {
    use ulua_ast::records::parse_options::ParseOptions;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    let stat = fixture.parse("f<<T, U>>()", &ParseOptions::default());
    assert!(!stat.is_null());
  }
}

mod parser_export_class {
  use ulua_ast::records::parse_options::ParseOptions;
  use ulua_common::FFlag;

  use super::*;
  #[cfg(test)]
  #[test]
  fn parser_export_class() {
    use ulua_unit_test::records::fixture::Fixture;

    FFlag::DebugLuauUserDefinedClasses.set(true);

    let mut fixture = Fixture::default();
    let source = "\n        export class Foo\n        end\n    ";

    let result = fixture.try_parse(&String::from(source), &ParseOptions::new());

    assert_eq!(result.errors.len(), 0);

    let block = unsafe { &*result.root };
    assert_eq!(block.body.size, 1);

    let stat_ptr = unsafe { *block.body.data.add(0) };
    let class_decl = unsafe {
      rtti::ast_node_as::<ast_stat_class::AstStatClass>(stat_ptr as *mut ast_node::AstNode)
    };
    assert!(!class_decl.is_null());
    let class_decl = unsafe { &*class_decl };
    assert!(class_decl.exported);
  }
}

mod parser_export_is_an_identifier_only_when_followed_by_type {

  use ulua_ast::records::parse_options::ParseOptions;
  #[cfg(test)]
  #[test]
  fn parser_export_is_an_identifier_only_when_followed_by_type() {
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let mut fixture = Fixture::default();
    let _sff = ScopedFastFlag::new(&FFlag::LuauExportValueSyntax, false);

    let result = fixture.try_parse(
      &String::from("export function a() end"),
      &ParseOptions::new(),
    );

    assert_eq!(result.errors.len(), 1);
    assert_eq!(
      result.errors.first().unwrap().get_message().as_str(),
      "Incomplete statement: expected assignment or a function call"
    );
  }
}

mod parser_export_value_parse_edge_cases {
  use ulua_ast::records::parse_options::ParseOptions;

  use super::*;
  #[cfg(test)]
  #[test]
  fn parser_export_value_parse_edge_cases() {
    use ulua_common::FFlag::LuauExportValueSyntax;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let mut fixture = Fixture::fixture_bool(false);
    let _sff_luau_export_value_syntax = ScopedFastFlag::new(&LuauExportValueSyntax, true);

    let source1 = String::from("export = 5\nexport += 1\nexport()");
    let parse_result1 = fixture.parse(&source1, &ParseOptions::default());
    let ast_block1 = unsafe { &*parse_result1 };
    assert_eq!(ast_block1.body.size, 3);
    assert!(unsafe {
      !rtti::ast_node_as::<ast_stat_assign::AstStatAssign>(
        *ast_block1.body.data.add(0) as *mut ast_node::AstNode
      )
      .is_null()
    });
    assert!(unsafe {
      !rtti::ast_node_as::<ast_stat_compound_assign::AstStatCompoundAssign>(
        *ast_block1.body.data.add(1) as *mut ast_node::AstNode,
      )
      .is_null()
    });
    assert!(unsafe {
      !rtti::ast_node_as::<ast_stat_expr::AstStatExpr>(
        *ast_block1.body.data.add(2) as *mut ast_node::AstNode
      )
      .is_null()
    });

    let source2 = String::from("export local x = 5");
    let _parse_result2 = fixture.parse(&source2, &ParseOptions::default());

    let source3 = String::from("export const x = 5");
    let _parse_result3 = fixture.parse(&source3, &ParseOptions::default());

    let source4 = String::from("export function foo()\nend");
    let _parse_result4 = fixture.parse(&source4, &ParseOptions::default());

    fixture.match_parse_error(
      &String::from("export 42"),
      &String::from("Incomplete statement: expected assignment or a function call"),
      None,
    );
    fixture.match_parse_error(
      &String::from("export if true then end"),
      &String::from("Incomplete statement: expected assignment or a function call"),
      None,
    );
    fixture.match_parse_error(
      &String::from("export"),
      &String::from("Incomplete statement: expected assignment or a function call"),
      None,
    );
  }
}

mod parser_export_value_parse_failures {
  use ulua_ast::records::parse_options::ParseOptions;

  use super::*;
  #[cfg(test)]
  #[test]
  fn parser_export_value_parse_failures() {
    use alloc::string::String;

    use ulua_common::FFlag::{DebugLuauUserDefinedClasses, LuauExportValueSyntax};
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let mut fixture = Fixture::fixture_bool(false);
    let _sff_luau_export_value_syntax = ScopedFastFlag::new(&LuauExportValueSyntax, true);
    let _sff_debug_luau_user_defined_classes =
      ScopedFastFlag::new(&DebugLuauUserDefinedClasses, true);

    let sources = [
      String::from("\nexport foo = 5\n    "),
      String::from("\nexport foo\n    "),
      String::from("\nfunction foo()\nend\nexport foo\n    "),
      String::from("\nexport local function foo()\nend\n    "),
    ];

    for source in sources.iter() {
      let _result = fixture.try_parse(source, &ParseOptions::default());
    }

    let duplicate_export = fixture.try_parse(
      &String::from("\nexport local foo = 1\nexport local foo = 2\n    "),
      &ParseOptions::default(),
    );
    let first_error_message = duplicate_export.errors.first().unwrap().get_message();
    assert!(first_error_message.contains("foo"));

    fixture.match_parse_error(
      &String::from("\nexport local answer = 42\nreturn {answer = answer}\n    "),
      &String::from(
        "Exporting values is not compatible with top-level return (export/return conflict)",
      ),
      None,
    );

    fixture.match_parse_error(
      &String::from("\nif skip then\n    return\nend\n\nexport local answer = 42\n    "),
      &String::from(
        "Exporting values is not compatible with top-level return (export/return conflict)",
      ),
      None,
    );

    fixture.match_parse_error(
          &String::from("\nexport class Player\n    public health: number\n    \n    function setHealth(self, health: number)\n        self.health = health\n        return self\n    end\n\n    function getHealth(self): number\n        return self.health\n    end\nend\n\nreturn Player {health = 100}\n    "),
          &String::from("Exporting values is not compatible with top-level return (export/return conflict)"),
          None,
      );

    let block_sources = [
      String::from("\nif true then\n    export local insideIf = 1\nend\n    "),
      String::from("\ndo\n    export const insideDo = 1\nend\n    "),
      String::from("\nwhile true do\n    export local insideWhile = 1\nend\n    "),
      String::from("\nrepeat\n    export local insideRepeat = 1\nuntil true\n    "),
      String::from("\nfor i = 1, 1 do\n    export local insideFor = i\nend\n    "),
      String::from("\nlocal function test()\n    export local insideFunction = 1\nend\n    "),
    ];

    for source in block_sources.iter() {
      fixture.match_parse_error(
        source,
        &String::from("'export' may only be applied to top-level statements"),
        None,
      );
    }
  }
}

mod parser_export_value_rfc {
  use ulua_ast::records::parse_options::ParseOptions;

  use super::*;
  #[cfg(test)]
  #[test]
  fn parser_export_value_rfc() {
    use ulua_common::FFlag::LuauExportValueSyntax;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let mut fixture = Fixture::fixture_bool(false);
    let _sff_luau_export_value_syntax = ScopedFastFlag::new(&LuauExportValueSyntax, true);

    let source = String::from(
      "export local version = \"1.0.0\"\n\
           export const TAU = math.pi * 2\n\
           export local settings: Settings = getSettings()\n\
           export local a, b, c = 1, 2, 3\n\
           export local d\n\
           \n\
           export function add(a: number, b: number): number\n\
               return a + b\n\
           end\n\
           \n\
           export local f, g\n\
           function f()\n\
               return g()\n\
           end\n\
           \n\
           function g()\n\
               return 42\n\
           end\n\
           \n\
           local function ret(): (string, number, boolean)\n\
               return \"heh\", 42, false\n\
           end\n\
           export local x, y, z = ret()\n",
    );

    let result = fixture.parse(&source, &ParseOptions::new());

    assert!(!result.is_null());
    assert_eq!(unsafe { (*result).body.size }, 11);

    let version = unsafe {
      rtti::ast_node_as::<ast_stat_local::AstStatLocal>(
        *(*result).body.data.add(0) as *mut ast_node::AstNode
      )
    };
    assert!(!version.is_null());
    assert!(unsafe { (*version).is_exported });
    assert!(!unsafe { (*version).is_const });
    assert_eq!(unsafe { (*version).vars.size }, 1);
    assert!(unsafe { (**(*version).vars.data.add(0)).is_exported });
    assert!(!unsafe { (**(*version).vars.data.add(0)).is_const });

    let tau = unsafe {
      rtti::ast_node_as::<ast_stat_local::AstStatLocal>(
        *(*result).body.data.add(1) as *mut ast_node::AstNode
      )
    };
    assert!(!tau.is_null());
    assert!(unsafe { (*tau).is_exported });
    assert!(unsafe { (*tau).is_const });
    assert_eq!(unsafe { (*tau).vars.size }, 1);
    assert!(unsafe { (**(*tau).vars.data.add(0)).is_exported });
    assert!(unsafe { (**(*tau).vars.data.add(0)).is_const });

    let settings = unsafe {
      rtti::ast_node_as::<ast_stat_local::AstStatLocal>(
        *(*result).body.data.add(2) as *mut ast_node::AstNode
      )
    };
    assert!(!settings.is_null());
    assert!(unsafe { (*settings).is_exported });
    assert!(!unsafe { (*settings).is_const });
    assert_eq!(unsafe { (*settings).vars.size }, 1);
    assert!(!unsafe { (**(*settings).vars.data.add(0)).annotation.is_null() });

    let abc = unsafe {
      rtti::ast_node_as::<ast_stat_local::AstStatLocal>(
        *(*result).body.data.add(3) as *mut ast_node::AstNode
      )
    };
    assert!(!abc.is_null());
    assert!(unsafe { (*abc).is_exported });
    assert!(!unsafe { (*abc).is_const });
    assert_eq!(unsafe { (*abc).vars.size }, 3);
    for i in 0..unsafe { (*abc).vars.size } as usize {
      assert!(unsafe { (**(*abc).vars.data.add(i)).is_exported });
      assert!(!unsafe { (**(*abc).vars.data.add(i)).is_const });
    }

    let d = unsafe {
      rtti::ast_node_as::<ast_stat_local::AstStatLocal>(
        *(*result).body.data.add(4) as *mut ast_node::AstNode
      )
    };
    assert!(!d.is_null());
    assert!(unsafe { (*d).is_exported });
    assert!(!unsafe { (*d).is_const });
    assert_eq!(unsafe { (*d).vars.size }, 1);
    assert_eq!(unsafe { (*d).values.size }, 0);
    assert!(unsafe { (**(*d).vars.data.add(0)).is_exported });

    let add = unsafe {
      rtti::ast_node_as::<ast_stat_local_function::AstStatLocalFunction>(
        *(*result).body.data.add(5) as *mut ast_node::AstNode,
      )
    };
    assert!(!add.is_null());
    assert!(unsafe { (*(*add).name).is_exported });
    assert!(unsafe { (*(*add).name).is_const });

    let forward_decls = unsafe {
      rtti::ast_node_as::<ast_stat_local::AstStatLocal>(
        *(*result).body.data.add(6) as *mut ast_node::AstNode
      )
    };
    assert!(!forward_decls.is_null());
    assert!(unsafe { (*forward_decls).is_exported });
    assert!(!unsafe { (*forward_decls).is_const });
    assert_eq!(unsafe { (*forward_decls).vars.size }, 2);
    assert_eq!(unsafe { (*forward_decls).values.size }, 0);
    for i in 0..unsafe { (*forward_decls).vars.size } as usize {
      assert!(unsafe { (**(*forward_decls).vars.data.add(i)).is_exported });
      assert!(!unsafe { (**(*forward_decls).vars.data.add(i)).is_const });
    }

    assert!(unsafe {
      rtti::ast_node_is::<ast_stat_function::AstStatFunction>(
        *(*result).body.data.add(7) as *mut ast_node::AstNode
      )
    });
    assert!(unsafe {
      rtti::ast_node_is::<ast_stat_function::AstStatFunction>(
        *(*result).body.data.add(8) as *mut ast_node::AstNode
      )
    });

    // C++ reads `xyz` from body.data[3] (re-checking the `export local a, b, c`
    // statement — the name is misleading but faithful to upstream).
    let xyz = unsafe {
      rtti::ast_node_as::<ast_stat_local::AstStatLocal>(
        *(*result).body.data.add(3) as *mut ast_node::AstNode
      )
    };
    assert!(!xyz.is_null());
    assert!(unsafe { (*xyz).is_exported });
    assert!(!unsafe { (*xyz).is_const });
    assert_eq!(unsafe { (*xyz).vars.size }, 3);
    for i in 0..unsafe { (*xyz).vars.size } as usize {
      assert!(unsafe { (**(*xyz).vars.data.add(i)).is_exported });
      assert!(!unsafe { (**(*xyz).vars.data.add(i)).is_const });
    }

    let source2 = String::from(
      "export type Config = {\n\
           debug: boolean,\n\
           timeout: number,\n\
           }\n\
           \n\
           return {\n\
           debug = false,\n\
           timeout = 5,\n\
           }\n",
    );

    let _result2 = fixture.parse(&source2, &ParseOptions::new());
  }
}

mod parser_expr_group_with_cst {
  use ulua_ast::records::ast_stat::AstStat;
  use ulua_common::FFlag;

  use super::*;
  #[cfg(test)]
  #[test]
  fn parser_expr_group_with_cst() {
    use ulua_ast::{
      records::{
        ast_expr_group::AstExprGroup, ast_node::AstNode, ast_stat_local::AstStatLocal,
        cst_expr_group::CstExprGroup, cst_node::CstNode, parse_options::ParseOptions,
        parse_result::ParseResult, position::Position,
      },
      rtti::{ast_node_as, cst_node_as},
    };
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _scoped_flag = ScopedFastFlag::new(&FFlag::LuauCstExprGroup, true);
    let mut fixture = Fixture::fixture_bool(false);
    let source = String::from("\n        local a = (1 + 2)\n    ");
    let mut parse_options = ParseOptions::new();
    parse_options.store_cst_data = true;

    let result: ParseResult = fixture.parse_ex(&source, &parse_options);

    assert!(!result.root.is_null());
    assert_eq!(1, unsafe { (*result.root).body.size });

    let local_stmt = unsafe {
      let stmt_ptr = (*result.root).body.data.add(0);
      let stmt_node = &**stmt_ptr;
      ast_node_as::<AstStatLocal>(stmt_node as *const AstStat as *mut AstNode)
    };
    assert!(!local_stmt.is_null());

    assert_eq!(1, unsafe { (*local_stmt).values.size });

    let group_expr = unsafe {
      let expr_ptr = (*local_stmt).values.data.add(0);
      let expr_node = &**expr_ptr;
      ast_node_as::<AstExprGroup>(expr_node as *const ast_expr::AstExpr as *mut AstNode)
    };
    assert!(!group_expr.is_null());

    let base_cst_node = result
      .cst_node_map
      .find(&(group_expr as *mut AstNode))
      .copied()
      .unwrap_or(null_mut());
    assert!(!base_cst_node.is_null());

    let cst_node = unsafe {
      let node_ref = &*base_cst_node;
      cst_node_as::<CstExprGroup>(node_ref as *const CstNode as *mut CstNode)
    };
    assert!(!cst_node.is_null());

    let expected_close_position = Position::new(1, 24);
    assert_eq!(expected_close_position, unsafe {
      (*cst_node).close_position
    });
  }
}

mod parser_extern_read_write_attributes {
  use super::*;
  #[cfg(test)]
  #[test]
  fn parser_extern_read_write_attributes() {
    use ulua_ast::{
      enums::ast_table_access::AstTableAccess,
      records::{
        ast_stat_block::AstStatBlock, ast_stat_declare_extern_type::AstStatDeclareExternType,
        parse_options::ParseOptions,
      },
    };
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _ = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fix = Fixture::default();
    let code = String::from(
      "\n        declare extern type Foo with\n            read ReadOnlyMember: string\n            write WriteOnlyMember: number\n            ReadWriteMember: vector\n            wRITE BadAttributeMember: buffer\n        end\n    ",
    );

    let result = fix.try_parse(&code, &ParseOptions::new());

    assert_eq!(result.errors.len(), 1);
    assert_eq!(result.errors[0].get_location().begin.line, 5);
    assert_eq!(
      *result.errors[0].get_message(),
      "Expected blank or 'read' or 'write' attribute, got 'wRITE'"
    );

    let stat: *mut AstStatBlock = result.root;
    assert_eq!(unsafe { (*stat).body.size }, 1);

    let declared_extern_type: *mut AstStatDeclareExternType = unsafe {
      rtti::ast_node_as::<AstStatDeclareExternType>(
        *(*stat).body.data.add(0) as *mut ast_node::AstNode
      )
    };
    assert!(!declared_extern_type.is_null());
    assert_eq!(unsafe { (*declared_extern_type).props.size }, 4);

    let props_ptr = unsafe { (*declared_extern_type).props.data };
    assert_eq!(unsafe { (*props_ptr.add(0)).access }, AstTableAccess::Read);
    assert_eq!(unsafe { (*props_ptr.add(1)).access }, AstTableAccess::Write);
    assert_eq!(
      unsafe { (*props_ptr.add(2)).access },
      AstTableAccess::ReadWrite
    );
    assert_eq!(
      unsafe { (*props_ptr.add(3)).access },
      AstTableAccess::ReadWrite
    );
  }
}

mod parser_extra_table_indexer_recovery {

  #[cfg(test)]
  #[test]
  fn parser_extra_table_indexer_recovery() {
    use ulua_ast::records::parse_options::ParseOptions;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    let parse_result = fixture.try_parse(
      &String::from("local a : { [string] : number, [number] : string, count: number }"),
      &ParseOptions::default(),
    );
    assert_eq!(parse_result.errors.len(), 1);
  }
}

mod parser_extra_token_in_consume {

  use ulua_ast::records::parse_options::ParseOptions;
  #[cfg(test)]
  #[test]
  fn parser_extra_token_in_consume() {
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    let source = "\nfunction test + (a, f) return a + f end\nreturn test(2, 3)\n";

    let result = fixture.try_parse(source, &ParseOptions::default());

    assert_eq!(result.errors.len(), 1);
    assert_eq!(
      result.errors[0].get_message(),
      "Expected '(' when parsing function, got '+'"
    );
  }
}

mod parser_extra_token_in_consume_match {

  use ulua_ast::records::parse_options::ParseOptions;
  #[cfg(test)]
  #[test]
  fn parser_extra_token_in_consume_match() {
    use ulua_unit_test::records::fixture::Fixture;

    let mut fix = Fixture::default();
    let code = String::from("function test(a, f+) return a + f end\nreturn test(2, 3)\n");

    let result = fix.try_parse(&code, &ParseOptions::new());

    assert_eq!(result.errors.len(), 1);
    assert_eq!(
      result.errors[0].get_message(),
      "Expected ')' (to close '(' at column 14), got '+'"
    );
  }
}

mod parser_extra_token_in_consume_match_end {

  use ulua_ast::records::parse_options::ParseOptions;
  #[cfg(test)]
  #[test]
  fn parser_extra_token_in_consume_match_end() {
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    let source = "\nif true then\n    return 12\nthen\nend\n";

    let result = fixture.try_parse(source, &ParseOptions::default());

    assert_eq!(result.errors.len(), 1);
    assert_eq!(
      result.errors[0].get_message(),
      "Expected 'end' (to close 'then' at line 2), got 'then'"
    );
  }
}

mod parser_for_loop_with_single_var_has_comma_positions_of_size_zero {
  use super::*;
  #[cfg(test)]
  #[test]
  fn parser_for_loop_with_single_var_has_comma_positions_of_size_zero() {
    use ulua_ast::{
      records::{
        ast_stat_for_in::AstStatForIn, cst_stat_for_in::CstStatForIn, parse_options::ParseOptions,
      },
      rtti::{ast_node_as, cst_node_as},
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let source = String::from("for value in tbl do\nend");
    let mut options = ParseOptions::new();
    options.store_cst_data = true;
    let result = fixture.parse_ex(&source, &options);

    assert!(!result.root.is_null());
    assert_eq!(1, unsafe { (*result.root).body.size });

    let for_loop = unsafe { *(*result.root).body.data.add(0) };
    let for_loop = unsafe { ast_node_as::<AstStatForIn>(for_loop as *mut ast_node::AstNode) };
    assert!(!for_loop.is_null());

    let base_cst_node = result
      .cst_node_map
      .find(&(for_loop as *mut ast_node::AstNode));
    assert!(base_cst_node.is_some());

    let cst_node = unsafe { cst_node_as::<CstStatForIn>(*base_cst_node.unwrap()) };
    assert!(!cst_node.is_null());

    assert_eq!(0, unsafe { (*cst_node).vars_comma_positions.size });
  }
}

mod parser_function_name_has_correct_start_location {
  use super::*;
  #[cfg(test)]
  #[test]
  fn parser_function_name_has_correct_start_location() {
    use ulua_ast::records::{parse_options::ParseOptions, position::Position};
    use ulua_unit_test::records::fixture::Fixture;

    let mut fix = Fixture::default();
    let code = String::from(
      "\n        function simple()\n        end\n\n        function T:complex()\n        end\n    ",
    );
    let opts = ParseOptions::new();
    let block = unsafe {
      rtti::ast_node_as::<ast_stat_block::AstStatBlock>(
        fix.parse(&code, &opts) as *mut ast_node::AstNode
      )
    };
    let body = unsafe { (*block).body };
    let size = body.size as usize;
    assert_eq!(size, 2);

    let first_stat = unsafe { *body.data.add(0) };
    let function1 = unsafe {
      rtti::ast_node_as::<ast_stat_function::AstStatFunction>(first_stat as *mut ast_node::AstNode)
    };
    assert!(!function1.is_null());
    let name_loc = unsafe { (*function1).name };
    let name_loc = unsafe { (*name_loc).base.location };
    assert_eq!(name_loc.begin, Position::new(1, 17));

    let second_stat = unsafe { *body.data.add(1) };
    let function2 = unsafe {
      rtti::ast_node_as::<ast_stat_function::AstStatFunction>(second_stat as *mut ast_node::AstNode)
    };
    assert!(!function2.is_null());
    let name_loc2 = unsafe { (*function2).name };
    let name_loc2 = unsafe { (*name_loc2).base.location };
    assert_eq!(name_loc2.begin, Position::new(4, 17));
  }
}

mod parser_function_return_type_should_disambiguate_from_function_type_and_multiple_returns {
  use super::*;
  #[cfg(test)]
  #[test]
  fn parser_function_return_type_should_disambiguate_from_function_type_and_multiple_returns() {
    use ulua_ast::records::parse_options::ParseOptions;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    let block = fixture.parse(
      "function f(): (number, string) return 1, \"foo\" end",
      &ParseOptions::default(),
    );
    assert!(!block.is_null());

    let root = unsafe { &*block };
    assert!(root.body.size > 0);

    let stat_func = unsafe {
      rtti::ast_node_as::<ast_stat_function::AstStatFunction>(
        (*root.body.data) as *mut ast_node::AstNode,
      )
    };
    assert!(!stat_func.is_null());

    let func_expr = unsafe { &*(*stat_func).func };
    assert!(!func_expr.return_annotation.is_null());

    let type_pack = unsafe {
      rtti::ast_node_as::<ast_type_pack_explicit::AstTypePackExplicit>(
        (*(*stat_func).func).return_annotation as *mut ast_node::AstNode,
      )
    };
    assert!(!type_pack.is_null());

    let type_list = unsafe { &(*type_pack).type_list };
    assert!(type_list.tail_type.is_null());

    let ret_types = &type_list.types;
    assert!(ret_types.size == 2);

    let ty0 = unsafe {
      rtti::ast_node_as::<ast_type_reference::AstTypeReference>(
        (*ret_types.data) as *mut ast_node::AstNode,
      )
    };
    assert!(!ty0.is_null());
    assert_eq!(unsafe { CStr::from_ptr((*ty0).name.value) }, c"number");

    let ty1 = unsafe {
      rtti::ast_node_as::<ast_type_reference::AstTypeReference>(
        (*ret_types.data.add(1)) as *mut ast_node::AstNode,
      )
    };
    assert!(!ty1.is_null());
    assert_eq!(unsafe { CStr::from_ptr((*ty1).name.value) }, c"string");
  }
}

mod parser_function_return_type_should_parse_as_function_type_annotation_with_no_args {
  use super::*;
  #[cfg(test)]
  #[test]
  fn parser_function_return_type_should_parse_as_function_type_annotation_with_no_args() {
    use ulua_ast::records::parse_options::ParseOptions;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    let block = fixture.parse(
      "function f(): () -> nil return nil end",
      &ParseOptions::default(),
    );
    assert!(!block.is_null());

    let root = unsafe { &*block };
    assert!(root.body.size > 0);

    let stat_func = unsafe {
      rtti::ast_node_as::<ast_stat_function::AstStatFunction>(
        *root.body.data.add(0) as *mut ast_node::AstNode
      )
    };
    assert!(!stat_func.is_null());

    let func = unsafe { &*stat_func }.func;
    assert!(!func.is_null());

    let return_annotation = unsafe { &*func }.return_annotation;
    assert!(!return_annotation.is_null());

    let type_pack = unsafe {
      rtti::ast_node_as::<ast_type_pack_explicit::AstTypePackExplicit>(
        return_annotation as *mut ast_node::AstNode,
      )
    };
    assert!(!type_pack.is_null());

    let type_list = unsafe { &*type_pack }.type_list;
    assert!(type_list.tail_type.is_null());

    let ret_types = type_list.types;
    assert_eq!(ret_types.size, 1);

    let fun_ty = unsafe {
      rtti::ast_node_as::<ast_type_function::AstTypeFunction>(
        *ret_types.data.add(0) as *mut ast_node::AstNode
      )
    };
    assert!(!fun_ty.is_null());

    let arg_types = unsafe { &*fun_ty }.arg_types;
    assert_eq!(arg_types.types.size, 0);
    assert!(arg_types.tail_type.is_null());

    let return_types = unsafe { &*fun_ty }.return_types;
    assert!(!return_types.is_null());

    let fun_return_pack = unsafe {
      rtti::ast_node_as::<ast_type_pack_explicit::AstTypePackExplicit>(
        return_types as *mut ast_node::AstNode,
      )
    };
    assert!(!fun_return_pack.is_null());
    assert!(unsafe { &*fun_return_pack }.type_list.tail_type.is_null());

    let ty = unsafe {
      rtti::ast_node_as::<ast_type_reference::AstTypeReference>(
        *(&*fun_return_pack).type_list.types.data.add(0) as *mut ast_node::AstNode,
      )
    };
    assert!(!ty.is_null());

    let name = unsafe { &*ty }.name;
    let name_str = unsafe { CStr::from_ptr(name.value).to_string_lossy() };
    assert_eq!(name_str, "nil");
  }
}

mod parser_function_start_locations_are_before_attributes {
  use ulua_ast::records::parse_options::ParseOptions;

  use super::*;
  #[cfg(test)]
  #[test]
  fn parser_function_start_locations_are_before_attributes() {
    use ulua_ast::records::{
      ast_expr_function::AstExprFunction, ast_stat_function::AstStatFunction,
      ast_stat_local::AstStatLocal, ast_stat_local_function::AstStatLocalFunction,
      location::Location, position::Position,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    let stat = fixture.parse(
          "\n        @native\n        function globalFunction()\n        end\n\n        @native\n        local function localFunction()\n        end\n\n        local _ = @native function()\n        end\n    ",
          &ParseOptions::default(),
      );
    let root = unsafe { &*stat };
    assert_eq!(3, root.body.size);

    let global_function = unsafe {
      rtti::ast_node_as::<AstStatFunction>(*root.body.data.add(0) as *mut ast_node::AstNode)
    };
    assert!(!global_function.is_null());
    assert_eq!(
      Location::new(Position::new(1, 8), Position::new(3, 11)),
      unsafe { (*global_function).base.base.location }
    );

    let local_function = unsafe {
      rtti::ast_node_as::<AstStatLocalFunction>(*root.body.data.add(1) as *mut ast_node::AstNode)
    };
    assert!(!local_function.is_null());
    assert_eq!(
      Location::new(Position::new(5, 8), Position::new(7, 11)),
      unsafe { (*local_function).base.base.location }
    );

    let local_variable = unsafe {
      rtti::ast_node_as::<AstStatLocal>(*root.body.data.add(2) as *mut ast_node::AstNode)
    };
    assert!(!local_variable.is_null());
    assert_eq!(1, unsafe { (*local_variable).values.size });

    let anonymous_function = unsafe {
      rtti::ast_node_as::<AstExprFunction>(
        *(*local_variable).values.data.add(0) as *mut ast_node::AstNode
      )
    };
    assert!(!anonymous_function.is_null());
    assert_eq!(
      Location::new(Position::new(9, 18), Position::new(10, 11)),
      unsafe { (*anonymous_function).base.base.location }
    );
  }
}

mod parser_function_type_annotation {

  #[cfg(test)]
  #[test]
  fn parser_function_type_annotation() {
    use ulua_ast::records::parse_options::ParseOptions;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    let stat = fixture.parse("local f: (number, string) -> nil", &ParseOptions::default());
    assert!(!stat.is_null());
  }
}

mod parser_function_type_matching_parenthesis {

  #[cfg(test)]
  #[test]
  fn parser_function_type_matching_parenthesis() {
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    fixture.match_parse_error(
      &String::from("local a: <T>(number -> string"),
      &String::from("Expected ')' (to close '(' at column 13), got '->'"),
      None,
    );
  }
}

mod parser_function_type_named_arguments {
  use ulua_ast::records::parse_options::ParseOptions;

  use super::*;
  #[cfg(test)]
  #[test]
  fn parser_function_type_named_arguments() {
    use ulua_unit_test::records::fixture::Fixture;

    {
      let mut fixture = Fixture::default();
      let result = fixture.parse_ex(
        &String::from("type MyFunc = (a: number, b: string, c: number) -> string"),
        &ParseOptions::default(),
      );
      let stat: *mut ast_stat_block::AstStatBlock = result.root;
      assert!(!stat.is_null());
      let decl = unsafe {
        rtti::ast_node_as::<ast_stat_type_alias::AstStatTypeAlias>(
          *(&*stat).body.data as *mut ast_node::AstNode,
        )
      };
      assert!(!decl.is_null());
      let func = unsafe {
        rtti::ast_node_as::<ast_type_function::AstTypeFunction>(
          (*decl).type_ptr as *mut ast_node::AstNode,
        )
      };
      assert!(!func.is_null());
      assert_eq!(unsafe { (*func).arg_types.types.size }, 3);
      assert_eq!(unsafe { (*func).arg_names.size }, 3);
      assert!(!unsafe { (*func).arg_names.data.add(2).read().is_none() });
      let arg_name_c = unsafe { (*func).arg_names.data.add(2).read().unwrap() };
      assert_eq!(
        unsafe { CStr::from_ptr(arg_name_c.0.value).to_string_lossy() },
        "c"
      );
    }

    {
      let mut fixture = Fixture::default();
      let result = fixture.parse_ex(
        &String::from("type MyFunc = (a: number, string, c: number) -> string"),
        &ParseOptions::default(),
      );
      let stat: *mut ast_stat_block::AstStatBlock = result.root;
      assert!(!stat.is_null());
      let decl = unsafe {
        rtti::ast_node_as::<ast_stat_type_alias::AstStatTypeAlias>(
          *(&*stat).body.data as *mut ast_node::AstNode,
        )
      };
      assert!(!decl.is_null());
      let func = unsafe {
        rtti::ast_node_as::<ast_type_function::AstTypeFunction>(
          (*decl).type_ptr as *mut ast_node::AstNode,
        )
      };
      assert!(!func.is_null());
      assert_eq!(unsafe { (*func).arg_types.types.size }, 3);
      assert_eq!(unsafe { (*func).arg_names.size }, 3);
      assert!(unsafe { (*func).arg_names.data.add(1).read().is_none() });
      assert!(!unsafe { (*func).arg_names.data.add(2).read().is_none() });
      let arg_name_c = unsafe { (*func).arg_names.data.add(2).read().unwrap() };
      assert_eq!(
        unsafe { CStr::from_ptr(arg_name_c.0.value).to_string_lossy() },
        "c"
      );
    }

    {
      let mut fixture = Fixture::default();
      let result = fixture.parse_ex(
        &String::from("type MyFunc = (a: number, string, number) -> string"),
        &ParseOptions::default(),
      );
      let stat: *mut ast_stat_block::AstStatBlock = result.root;
      assert!(!stat.is_null());
      let decl = unsafe {
        rtti::ast_node_as::<ast_stat_type_alias::AstStatTypeAlias>(
          *(&*stat).body.data as *mut ast_node::AstNode,
        )
      };
      assert!(!decl.is_null());
      let func = unsafe {
        rtti::ast_node_as::<ast_type_function::AstTypeFunction>(
          (*decl).type_ptr as *mut ast_node::AstNode,
        )
      };
      assert!(!func.is_null());
      assert_eq!(unsafe { (*func).arg_types.types.size }, 3);
      assert_eq!(unsafe { (*func).arg_names.size }, 3);
      assert!(unsafe { (*func).arg_names.data.add(1).read().is_none() });
      assert!(unsafe { (*func).arg_names.data.add(2).read().is_none() });
    }

    {
      let mut fixture = Fixture::default();
      let result = fixture.parse_ex(
              &String::from("type MyFunc = (a: number, b: string, c: number) -> (d: number, e: string, f: number) -> string"),
              &ParseOptions::default(),
          );
      let stat: *mut ast_stat_block::AstStatBlock = result.root;
      assert!(!stat.is_null());
      let decl = unsafe {
        rtti::ast_node_as::<ast_stat_type_alias::AstStatTypeAlias>(
          *(&*stat).body.data as *mut ast_node::AstNode,
        )
      };
      assert!(!decl.is_null());
      let func = unsafe {
        rtti::ast_node_as::<ast_type_function::AstTypeFunction>(
          (*decl).type_ptr as *mut ast_node::AstNode,
        )
      };
      assert!(!func.is_null());
      assert_eq!(unsafe { (*func).arg_types.types.size }, 3);
      assert_eq!(unsafe { (*func).arg_names.size }, 3);
      assert!(!unsafe { (*func).arg_names.data.add(2).read().is_none() });
      let arg_name_c = unsafe { (*func).arg_names.data.add(2).read().unwrap() };
      assert_eq!(
        unsafe { CStr::from_ptr(arg_name_c.0.value).to_string_lossy() },
        "c"
      );
      let explicit_pack = unsafe {
        rtti::ast_node_as::<ast_type_pack_explicit::AstTypePackExplicit>(
          (*func).return_types as *mut ast_node::AstNode,
        )
      };
      assert!(!explicit_pack.is_null());
      let func_ret = unsafe {
        rtti::ast_node_as::<ast_type_function::AstTypeFunction>(
          (*explicit_pack).type_list.types.data.add(0).read() as *mut ast_node::AstNode,
        )
      };
      assert!(!func_ret.is_null());
      assert_eq!(unsafe { (*func_ret).arg_types.types.size }, 3);
      assert_eq!(unsafe { (*func_ret).arg_names.size }, 3);
      assert!(!unsafe { (*func_ret).arg_names.data.add(2).read().is_none() });
      let arg_name_f = unsafe { (*func_ret).arg_names.data.add(2).read().unwrap() };
      assert_eq!(
        unsafe { CStr::from_ptr(arg_name_f.0.value).to_string_lossy() },
        "f"
      );
    }

    {
      let mut fixture = Fixture::default();
      fixture.match_parse_error(
        &String::from(
          "type MyFunc = (a: number, b: string, c: number) -> (d: number, e: string, f: number)",
        ),
        &String::from("Expected '->' when parsing function type, got <eof>"),
        None,
      );
    }

    {
      let mut fixture = Fixture::default();
      fixture.match_parse_error(
        &String::from("type MyFunc = (number) -> (d: number) <a, b, c> -> number"),
        &String::from("Expected '->' when parsing function type, got '<'"),
        None,
      );
    }
  }
}

mod parser_functions_can_have_0_arguments {

  #[cfg(test)]
  #[test]
  fn parser_functions_can_have_0_arguments() {
    use ulua_ast::records::parse_options::ParseOptions;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    let stat = fixture.parse("local f: () -> number", &ParseOptions::default());
    assert!(!stat.is_null());
  }
}

mod parser_functions_can_have_a_function_type_annotation {
  use super::*;
  #[cfg(test)]
  #[test]
  fn parser_functions_can_have_a_function_type_annotation() {
    use ulua_ast::records::{
      ast_stat_function::AstStatFunction, ast_type_function::AstTypeFunction,
      ast_type_pack_explicit::AstTypePackExplicit, parse_options::ParseOptions,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    let stat = fixture.parse(
      "function f(): (number) -> nil return nil end",
      &ParseOptions::default(),
    );
    let root = unsafe { &*stat };
    assert!(!root.body.data.is_null());

    let first_stat = unsafe { *root.body.data.add(0) };
    let stat_func =
      unsafe { rtti::ast_node_as::<AstStatFunction>(first_stat as *mut ast_node::AstNode) };
    assert!(!stat_func.is_null());

    let func = unsafe { (*stat_func).func };
    assert!(!func.is_null());

    let return_annotation = unsafe { (*func).return_annotation };
    assert!(!return_annotation.is_null());

    let type_pack = unsafe {
      rtti::ast_node_as::<AstTypePackExplicit>(return_annotation as *mut ast_node::AstNode)
    };
    assert!(!type_pack.is_null());

    let type_list = unsafe { &(*type_pack).type_list };
    assert!(type_list.tail_type.is_null());

    let ret_types = &type_list.types;
    assert_eq!(ret_types.size, 1);

    let fun_ty = unsafe {
      rtti::ast_node_as::<AstTypeFunction>(*ret_types.data.add(0) as *mut ast_node::AstNode)
    };
    assert!(!fun_ty.is_null());
  }
}

mod parser_functions_can_have_return_annotations {
  use super::*;
  #[cfg(test)]
  #[test]
  fn parser_functions_can_have_return_annotations() {
    use ulua_ast::records::{
      ast_stat_function::AstStatFunction, ast_type_pack_explicit::AstTypePackExplicit,
      parse_options::ParseOptions,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    let block = fixture.parse(
      "function foo(): number return 55 end",
      &ParseOptions::default(),
    );
    assert!(!block.is_null());

    let root = unsafe { &*block };
    assert!(root.body.size > 0);

    let stat_function =
      unsafe { rtti::ast_node_as::<AstStatFunction>((*root.body.data) as *mut ast_node::AstNode) };
    assert!(!stat_function.is_null());

    let func = unsafe { &*stat_function }.func;
    assert!(!func.is_null());

    let return_annotation = unsafe { &*func }.return_annotation;
    assert!(!return_annotation.is_null());

    let type_pack_explicit = unsafe {
      rtti::ast_node_as::<AstTypePackExplicit>(return_annotation as *mut ast_node::AstNode)
    };
    assert!(!type_pack_explicit.is_null());

    let type_list = unsafe { &*type_pack_explicit }.type_list;
    assert_eq!(type_list.types.size, 1);
    assert!(type_list.tail_type.is_null());
  }
}

mod parser_functions_can_return_0_values {

  #[cfg(test)]
  #[test]
  fn parser_functions_can_return_0_values() {
    use ulua_ast::records::parse_options::ParseOptions;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    let block = fixture.parse("local f: (number) -> ()", &ParseOptions::default());
    assert!(!block.is_null());
  }
}

mod parser_functions_can_return_multiple_values {

  #[cfg(test)]
  #[test]
  fn parser_functions_can_return_multiple_values() {
    use ulua_ast::records::parse_options::ParseOptions;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    let stat = fixture.parse(
      "local f: (number) -> (number, number)",
      &ParseOptions::default(),
    );
    assert!(!stat.is_null());
  }
}

mod parser_generic_function_declaration_parsing {
  use super::*;
  #[cfg(test)]
  #[test]
  fn parser_generic_function_declaration_parsing() {
    use ulua_ast::records::parse_options::ParseOptions;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    let result = fixture.parse_ex(
      &String::from("declare function f<a, b, c...>()"),
      &ParseOptions::default(),
    );

    let root = unsafe { &*result.root };
    assert!(!root.body.data.is_null());

    let decl = unsafe {
      rtti::ast_node_as::<ast_stat_declare_function::AstStatDeclareFunction>(
        (*root.body.data) as *mut ast_node::AstNode,
      )
    };
    assert!(!decl.is_null());

    let decl_ref = unsafe { &*decl };
    assert_eq!(2, decl_ref.generics.size);
    assert_eq!(1, decl_ref.generic_packs.size);
  }
}

mod parser_generic_pack_parsing {
  use super::*;
  #[cfg(test)]
  #[test]
  fn parser_generic_pack_parsing() {
    use ulua_ast::records::parse_options::ParseOptions;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    let result = fixture.parse_ex(
      &String::from(
        "function f<a...>(...: a...)\n\
               end\n\
               \n\
               type A = (a...) -> b...",
      ),
      &ParseOptions::default(),
    );
    let root = result.root;
    assert!(!root.is_null());

    let stat_block = unsafe { &*root };
    assert!(!stat_block.body.data.is_null());
    assert!(stat_block.body.size > 0);

    let first_stat = unsafe { *stat_block.body.data.add(0) };
    let fn_stat = unsafe {
      rtti::ast_node_as::<ast_stat_function::AstStatFunction>(first_stat as *mut ast_node::AstNode)
    };
    assert!(!fn_stat.is_null());
    let fn_stat = unsafe { &*fn_stat };
    assert!(!fn_stat.func.is_null());
    let func = unsafe { &*fn_stat.func };
    assert!(!func.vararg_annotation.is_null());

    let vararg_annot = unsafe {
      rtti::ast_node_as::<AstTypePackGeneric>(func.vararg_annotation as *mut ast_node::AstNode)
    };
    assert!(!vararg_annot.is_null());
    let vararg_annot = unsafe { &*vararg_annot };
    assert_eq!(
      unsafe { CStr::from_ptr(vararg_annot.generic_name.value).to_string_lossy() },
      "a"
    );

    let second_stat = unsafe { *stat_block.body.data.add(1) };
    let alias_stat = unsafe {
      rtti::ast_node_as::<ast_stat_type_alias::AstStatTypeAlias>(
        second_stat as *mut ast_node::AstNode,
      )
    };
    assert!(!alias_stat.is_null());
    let alias_stat = unsafe { &*alias_stat };
    assert!(!alias_stat.type_ptr.is_null());
    let type_ptr = unsafe { &*alias_stat.type_ptr };
    let fn_ty = unsafe {
      rtti::ast_node_as::<ast_type_function::AstTypeFunction>(
        type_ptr as *const _ as *mut ast_node::AstNode,
      )
    };
    assert!(!fn_ty.is_null());
    let fn_ty = unsafe { &*fn_ty };

    let arg_type_pack = fn_ty.arg_types.tail_type;
    assert!(!arg_type_pack.is_null());
    let arg_annot =
      unsafe { rtti::ast_node_as::<AstTypePackGeneric>(arg_type_pack as *mut ast_node::AstNode) };
    assert!(!arg_annot.is_null());
    let arg_annot = unsafe { &*arg_annot };
    assert_eq!(
      unsafe { CStr::from_ptr(arg_annot.generic_name.value).to_string_lossy() },
      "a"
    );

    let ret_type_pack = fn_ty.return_types;
    assert!(!ret_type_pack.is_null());
    let ret_annot =
      unsafe { rtti::ast_node_as::<AstTypePackGeneric>(ret_type_pack as *mut ast_node::AstNode) };
    assert!(!ret_annot.is_null());
    let ret_annot = unsafe { &*ret_annot };
    assert_eq!(
      unsafe { CStr::from_ptr(ret_annot.generic_name.value).to_string_lossy() },
      "b"
    );
  }
}

mod parser_generic_type_list_recovery {

  #[cfg(test)]
  #[test]
  fn parser_generic_type_list_recovery() {
    use ulua_ast::records::parse_options::ParseOptions;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fix = Fixture::default();
    let code = String::from(
      "local function foo<T..., U>(a: U, ...: T...): (U, ...T) return a, ... end\n\
           return foo(1, 2 -- to check for a second error after recovery",
    );

    let result = fix.try_parse(&code, &ParseOptions::new());

    assert_eq!(result.errors.len(), 2);
    assert_eq!(
      result.errors[0].get_message().as_str(),
      "Generic types come before generic type packs"
    );
  }
}

mod parser_get_a_nice_error_when_there_is_an_extra_comma_at_the_end_of_a_function_argument_list {

  #[cfg(test)]
  #[test]
  fn parser_get_a_nice_error_when_there_is_an_extra_comma_at_the_end_of_a_function_argument_list() {
    use ulua_ast::records::{location::Location, parse_options::ParseOptions, position::Position};
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let source = String::from("\n        foo(a, b, c,)\n    ");
    let options = ParseOptions::new();
    let result = fixture.try_parse(&source, &options);

    assert_eq!(result.errors.len(), 1);

    let expected_location = Location::new(Position::new(1, 20), Position::new(1, 21));
    assert_eq!(*result.errors[0].get_location(), expected_location);

    let expected_message = "Expected expression after ',' but got ')' instead";
    assert_eq!(result.errors[0].get_message(), expected_message);
  }
}

mod parser_get_a_nice_error_when_there_is_an_extra_comma_at_the_end_of_a_function_parameter_list {

  #[cfg(test)]
  #[test]
  fn parser_get_a_nice_error_when_there_is_an_extra_comma_at_the_end_of_a_function_parameter_list()
  {
    use ulua_ast::records::{
      location::Location, parse_options::ParseOptions, parse_result::ParseResult,
      position::Position,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let source = String::from(
      "\n        export type VisitFn = (\n            any,\n            Array<TAnyNode | Array<TAnyNode>>, -- extra comma here\n        ) -> any\n    ",
    );
    let options = ParseOptions::new();
    let result: ParseResult = fixture.try_parse(&source, &options);

    assert_eq!(result.errors.len(), 1);

    let error = &result.errors[0];
    let expected_location = Location::new(Position::new(4, 8), Position::new(4, 9));
    assert_eq!(*error.get_location(), expected_location);
    assert_eq!(
      error.get_message().as_str(),
      "Expected type after ',' but got ')' instead"
    );
  }
}

mod parser_get_a_nice_error_when_there_is_an_extra_comma_at_the_end_of_a_generic_parameter_list {
  use super::*;
  #[cfg(test)]
  #[test]
  fn parser_get_a_nice_error_when_there_is_an_extra_comma_at_the_end_of_a_generic_parameter_list() {
    use ulua_ast::records::{
      ast_stat_type_alias::AstStatTypeAlias, ast_type_function::AstTypeFunction,
      location::Location, parse_options::ParseOptions, parse_result::ParseResult,
      position::Position,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let source = String::from("\n        export type VisitFn = <A, B,>(a: A, b: B) -> ()\n    ");
    let options = ParseOptions::new();
    let result: ParseResult = fixture.try_parse(&source, &options);

    assert_eq!(result.errors.len(), 1);

    let error = &result.errors[0];
    let expected_location = Location::new(Position::new(1, 36), Position::new(1, 37));
    assert_eq!(*error.get_location(), expected_location);
    assert_eq!(
      error.get_message().as_str(),
      "Expected type after ',' but got '>' instead"
    );

    assert_eq!(unsafe { (*result.root).body.size }, 1);

    let stat = unsafe { *(*result.root).body.data.add(0) };
    let t: *mut AstStatTypeAlias =
      unsafe { rtti::ast_node_as::<AstStatTypeAlias>(stat as *mut ast_node::AstNode) };
    assert!(!t.is_null());

    // C++ navigates the alias's type (`->type`) — the function type — not the
    // alias node's own base.
    let f: *mut AstTypeFunction =
      unsafe { rtti::ast_node_as::<AstTypeFunction>((*t).type_ptr as *mut ast_node::AstNode) };
    assert!(!f.is_null());

    assert_eq!(unsafe { (*f).generics.size }, 2);
  }
}

mod parser_get_a_nice_error_when_there_is_no_comma_after_last_table_member {
  use super::*;
  #[cfg(test)]
  #[test]
  fn parser_get_a_nice_error_when_there_is_no_comma_after_last_table_member() {
    use ulua_ast::records::{
      ast_expr_table::AstExprTable, location::Location, parse_options::ParseOptions,
      parse_result::ParseResult, position::Position,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let source = String::from(
      "\n        local t = {\n            first = 1\n\n        local ok = true\n        local good = ok == true\n    ",
    );
    let options = ParseOptions::new();
    let result: ParseResult = fixture.try_parse(&source, &options);

    assert_eq!(result.errors.len(), 1);

    let error_location = result.errors[0].get_location();
    let expected_location = Location::new(Position::new(4, 8), Position::new(4, 13));
    assert_eq!(*error_location, expected_location);

    let error_message = result.errors[0].get_message();
    assert_eq!(
      *error_message,
      "Expected '}' (to close '{' at line 2), got 'local'"
    );

    assert_eq!(unsafe { (*result.root).body.size }, 3);

    // C++ `Luau::query<AstExprTable>(root)` recursively finds the first table; here
    // it's the value of `local t = { ... }` — body[0] (AstStatLocal).values[0].
    // (The AstQueryDsl `query`/findNthOccurenceOf visitor is still a todo! stub.)
    let table: *mut AstExprTable = unsafe {
      let local = rtti::ast_node_as::<ast_stat_local::AstStatLocal>(
        *(*result.root).body.data.add(0) as *mut ast_node::AstNode,
      );
      rtti::ast_node_as::<AstExprTable>(*(*local).values.data.add(0) as *mut ast_node::AstNode)
    };
    assert!(!table.is_null());

    assert_eq!(unsafe { (*table).items.size }, 1);
  }
}

mod parser_get_a_nice_error_when_there_is_no_comma_between_table_members {
  use super::*;
  #[cfg(test)]
  #[test]
  fn parser_get_a_nice_error_when_there_is_no_comma_between_table_members() {
    use ulua_ast::records::{
      ast_expr_table::AstExprTable, location::Location, parse_options::ParseOptions,
      parse_result::ParseResult, position::Position,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let source = String::from(
      "\n        local t = {\n            first = 1\n            second = 2,\n            third = 3,\n            fouth = 4,\n        }\n    ",
    );
    let options = ParseOptions::new();
    let result: ParseResult = fixture.try_parse(&source, &options);

    assert_eq!(result.errors.len(), 1);

    let error_location = result.errors[0].get_location();
    let expected_location = Location::new(Position::new(3, 12), Position::new(3, 18));
    assert_eq!(*error_location, expected_location);

    let error_message = result.errors[0].get_message();
    assert_eq!(
      *error_message,
      "Expected ',' after table constructor element"
    );

    assert_eq!(unsafe { (*result.root).body.size }, 1);

    // C++ `Luau::query<AstExprTable>(root)` recursively finds the first table; here
    // it's the value of `local t = { ... }`, i.e. body[0] (AstStatLocal).values[0].
    let table: *mut AstExprTable = unsafe {
      let local = rtti::ast_node_as::<ast_stat_local::AstStatLocal>(
        *(*result.root).body.data.add(0) as *mut ast_node::AstNode,
      );
      rtti::ast_node_as::<AstExprTable>(*(*local).values.data.add(0) as *mut ast_node::AstNode)
    };
    assert!(!table.is_null());

    assert_eq!(unsafe { (*table).items.size }, 4);
  }
}

mod parser_grouped_function_type {
  use ulua_ast::records::parse_options::ParseOptions;

  use super::*;
  #[cfg(test)]
  #[test]
  fn parser_grouped_function_type() {
    use ulua_ast::records::{
      ast_stat_local::AstStatLocal, ast_type_function::AstTypeFunction,
      ast_type_group::AstTypeGroup, ast_type_optional::AstTypeOptional,
      ast_type_reference::AstTypeReference, ast_type_union::AstTypeUnion,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let source = String::from("type X<T> = T\nlocal x: X<(() -> ())?>");
    let parse_options = ParseOptions::new();
    let root = fixture.parse(&source, &parse_options);

    assert!(!root.is_null());
    assert_eq!(2, unsafe { (*root).body.size });

    let assignment = unsafe {
      rtti::ast_node_as::<AstStatLocal>(*(*root).body.data.add(1) as *mut ast_node::AstNode)
    };
    assert!(!assignment.is_null());

    assert_eq!(1, unsafe { (*assignment).vars.size });
    assert_eq!(0, unsafe { (*assignment).values.size });

    let binding = unsafe { (*assignment).vars.data.add(0).read() };
    assert_eq!("x", unsafe {
      CStr::from_ptr((*binding).name.value).to_string_lossy()
    });

    let generic_ty = unsafe {
      rtti::ast_node_as::<AstTypeReference>((*binding).annotation as *mut ast_node::AstNode)
    };
    assert!(!generic_ty.is_null());

    assert_eq!(1, unsafe { (*generic_ty).parameters.size });

    let param_ty = unsafe { (*generic_ty).parameters.data.add(0).read() };
    assert!(!param_ty.r#type.is_null());

    let union_ty =
      unsafe { rtti::ast_node_as::<AstTypeUnion>(param_ty.r#type as *mut ast_node::AstNode) };
    assert!(!union_ty.is_null());

    assert_eq!(2, unsafe { (*union_ty).types.size });

    let group_ty = unsafe {
      rtti::ast_node_as::<AstTypeGroup>(*(*union_ty).types.data.add(0) as *mut ast_node::AstNode)
    };
    assert!(!group_ty.is_null());

    let function_ty =
      unsafe { rtti::ast_node_as::<AstTypeFunction>((*group_ty).type_ as *mut ast_node::AstNode) };
    assert!(!function_ty.is_null());

    let optional_ty = unsafe {
      rtti::ast_node_as::<AstTypeOptional>(*(*union_ty).types.data.add(1) as *mut ast_node::AstNode)
    };
    assert!(!optional_ty.is_null());
  }
}

mod parser_incomplete_method_call {
  use super::*;
  #[cfg(test)]
  #[test]
  fn parser_incomplete_method_call() {
    use ulua_analysis::records::source_module::SourceModule;
    use ulua_ast::records::{
      ast_stat_function::AstStatFunction, ast_stat_return::AstStatReturn,
      parse_options::ParseOptions, parse_result::ParseResult, parser::Parser,
    };

    let source = "        function howdy()
              return game:
          end
      ";
    let mut source_module = SourceModule::new();
    let names = Arc::get_mut(&mut source_module.names).unwrap();
    let allocator = Arc::get_mut(&mut source_module.allocator).unwrap();
    let options = ParseOptions::new();
    let result: ParseResult = Parser::parse(source, source.len(), names, allocator, options);

    assert_eq!(1, result.root.is_null() as i32 ^ 1);
    assert_eq!(1, unsafe { (*result.root).body.size });

    let howdy_function = unsafe { (*result.root).body.data.add(0).read() };
    let howdy_function =
      unsafe { rtti::ast_node_as::<AstStatFunction>(howdy_function as *mut ast_node::AstNode) };
    assert!(!howdy_function.is_null());

    let body = unsafe { (*howdy_function).func.read().body };
    assert_eq!(1, unsafe { (*body).body.size });

    let ret_stat = unsafe {
      rtti::ast_node_as::<AstStatReturn>((*body).body.data.add(0).read() as *mut ast_node::AstNode)
    };
    assert!(!ret_stat.is_null());

    let func_loc = unsafe { (*howdy_function).base.base.location };
    let body_loc = unsafe { (*body).base.base.location };
    assert!(func_loc.end > body_loc.end);
  }
}

mod parser_incomplete_method_call_2 {
  use super::*;
  #[cfg(test)]
  #[test]
  fn parser_incomplete_method_call_2() {
    use ulua_analysis::records::source_module::SourceModule;
    use ulua_ast::records::{
      ast_stat_block::AstStatBlock, ast_stat_error::AstStatError,
      ast_stat_function::AstStatFunction, parse_options::ParseOptions, parse_result::ParseResult,
      parser::Parser,
    };

    let source = String::from(
      "local game = { GetService=function(s) return 'hello' end }\n\n\
           function a()\n\
               game:a\n\
           end",
    );

    let mut source_module = SourceModule::new();
    let options = ParseOptions::new();
    let result: ParseResult = Parser::parse(
      source.as_str(),
      source.len(),
      Arc::get_mut(&mut source_module.names).unwrap(),
      Arc::get_mut(&mut source_module.allocator).unwrap(),
      options,
    );

    assert_eq!(2, unsafe { (*result.root).body.size });

    let howdy_function: *mut AstStatFunction = unsafe {
      rtti::ast_node_as::<AstStatFunction>(
        (*result.root).body.data.add(1).read() as *mut ast_node::AstNode
      )
    };
    assert!(!howdy_function.is_null());

    let body: *mut AstStatBlock = unsafe { (*howdy_function).func.read().body };
    assert_eq!(1, unsafe { (*body).body.size });

    let ret: *mut AstStatError = unsafe {
      rtti::ast_node_as::<AstStatError>((*body).body.data.add(0).read() as *mut ast_node::AstNode)
    };
    assert!(!ret.is_null());

    assert!(
      unsafe { (*howdy_function).base.base.location.end }
        > unsafe { (*body).base.base.location.end }
    );
  }
}

mod parser_incomplete_method_call_still_yields_an_ast_expr_index_name {
  use super::*;
  #[cfg(test)]
  #[test]
  fn parser_incomplete_method_call_still_yields_an_ast_expr_index_name() {
    use ulua_ast::records::{
      ast_expr_error::AstExprError, ast_expr_index_name::AstExprIndexName,
      ast_stat_error::AstStatError, parse_options::ParseOptions,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let source = String::from("game:\n");
    let options = ParseOptions::new();
    let result: parse_result::ParseResult = fixture.try_parse(&source, &options);

    assert_eq!(1, unsafe { (*result.root).body.size });

    let stat: *mut AstStatError = unsafe {
      rtti::ast_node_as::<AstStatError>(*(*result.root).body.data.add(0) as *mut ast_node::AstNode)
    };
    assert!(!stat.is_null());

    let expr: *mut AstExprError = unsafe {
      rtti::ast_node_as::<AstExprError>(*(*stat).expressions.data.add(0) as *mut ast_node::AstNode)
    };
    assert!(!expr.is_null());

    let index_name: *mut AstExprIndexName = unsafe {
      rtti::ast_node_as::<AstExprIndexName>(
        *(*expr).expressions.data.add(0) as *mut ast_node::AstNode
      )
    };
    assert!(!index_name.is_null());
  }
}

mod parser_incomplete_statement_error {

  #[cfg(test)]
  #[test]
  fn parser_incomplete_statement_error() {
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    fixture.match_parse_error(
      &String::from("fiddlesticks"),
      &String::from("Incomplete statement: expected assignment or a function call"),
      None,
    );
  }
}

mod parser_initial_double_is_aligned {
  use super::*;
  #[cfg(test)]
  #[test]
  fn parser_initial_double_is_aligned() {
    use ulua_ast::records::allocator::Allocator;

    let mut alloc = Allocator::new();
    let one = alloc.alloc::<f64>(0.0);
    let addr = one as usize;
    let align_mask = align_of::<f64>() - 1;
    assert_eq!(addr & align_mask, 0);
  }
}

mod parser_inner_and_outer_scope_of_functions_have_correct_end_position {
  use super::*;
  #[cfg(test)]
  #[test]
  fn parser_inner_and_outer_scope_of_functions_have_correct_end_position() {
    use ulua_ast::records::{
      ast_stat_block::AstStatBlock, ast_stat_local_function::AstStatLocalFunction,
      location::Location, parse_options::ParseOptions, parse_result::ParseResult,
      position::Position,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let source =
      String::from("\n        local function foo()\n            local x = 1\n        end\n    ");
    let options = ParseOptions::new();
    let result: ParseResult = fixture.try_parse(&source, &options);

    assert!(!result.root.is_null());
    let stat_block: *mut AstStatBlock = result.root;
    assert_eq!(1, unsafe { (*stat_block).body.size });

    let first_stat = unsafe { *(*stat_block).body.data.add(0) };
    let func_stat: *mut AstStatLocalFunction =
      unsafe { rtti::ast_node_as::<AstStatLocalFunction>(first_stat as *mut ast_node::AstNode) };
    assert!(!func_stat.is_null());

    let func_expr = unsafe { (*func_stat).func };
    let body_location = unsafe { (*(*func_expr).body).base.base.location };
    let expected_body_location = Location::new(Position::new(1, 28), Position::new(3, 8));
    assert_eq!(expected_body_location, body_location);

    let stat_location = unsafe { (*func_stat).base.base.location };
    let expected_stat_location = Location::new(Position::new(1, 8), Position::new(3, 11));
    assert_eq!(expected_stat_location, stat_location);
  }
}

mod parser_intersection_of_two_function_types_if_no_returns {
  use super::*;
  #[cfg(test)]
  #[test]
  fn parser_intersection_of_two_function_types_if_no_returns() {
    use ulua_ast::records::parse_options::ParseOptions;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    let block = fixture.parse(
      "local f: (string) -> () & (number) -> ()",
      &ParseOptions::default(),
    );
    assert!(!block.is_null());

    let local = unsafe { (*block).body.data.add(0) };
    let local = unsafe { &*local };
    let local = unsafe {
      rtti::ast_node_as::<ast_stat_local::AstStatLocal>(*local as *mut ast_node::AstNode)
    };
    assert!(!local.is_null());

    // C++ `local->vars.data[0]->annotation->as<AstTypeIntersection>()` — deref the
    // AstLocal slot, then take its `.annotation` type, then RTTI-cast.
    let var0 = unsafe { *(*local).vars.data.add(0) };
    let annotation = unsafe { (*var0).annotation };
    let annotation = unsafe {
      rtti::ast_node_as::<ast_type_intersection::AstTypeIntersection>(
        annotation as *mut ast_node::AstNode,
      )
    };
    assert!(!annotation.is_null());

    let annotation = unsafe {
      rtti::ast_node_as::<ast_type_intersection::AstTypeIntersection>(
        annotation as *const _ as *mut ast_node::AstNode,
      )
    };
    assert!(!annotation.is_null());

    // C++ `annotation->types.data[i]->as<AstTypeFunction>()` — deref the slot to the
    // AstType element, then RTTI-cast (was casting the slot address + double-casting).
    let ty0 = unsafe { *(*annotation).types.data.add(0) };
    let ty0 = unsafe {
      rtti::ast_node_as::<ast_type_function::AstTypeFunction>(ty0 as *mut ast_node::AstNode)
    };
    assert!(!ty0.is_null());

    let ty1 = unsafe { *(*annotation).types.data.add(1) };
    let ty1 = unsafe {
      rtti::ast_node_as::<ast_type_function::AstTypeFunction>(ty1 as *mut ast_node::AstNode)
    };
    assert!(!ty1.is_null());
    let ty1 = unsafe {
      rtti::ast_node_as::<ast_type_function::AstTypeFunction>(
        ty1 as *const _ as *mut ast_node::AstNode,
      )
    };
    assert!(!ty1.is_null());
  }
}

mod parser_intersection_of_two_function_types_if_two_or_more_returns {
  use super::*;
  #[cfg(test)]
  #[test]
  fn parser_intersection_of_two_function_types_if_two_or_more_returns() {
    use ulua_ast::records::{
      ast_stat_local::AstStatLocal, ast_type_function::AstTypeFunction,
      ast_type_intersection::AstTypeIntersection, parse_options::ParseOptions,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    let block = fixture.parse(
      "local f: (string) -> (string, number) & (number) -> (number, string)",
      &ParseOptions::default(),
    );
    assert!(!block.is_null());

    let stat_block = unsafe { &*block };
    assert!(!stat_block.body.data.is_null());
    let first_stat = unsafe { *stat_block.body.data.add(0) };
    assert!(!first_stat.is_null());

    let local = unsafe { rtti::ast_node_as::<AstStatLocal>(first_stat as *mut ast_node::AstNode) };
    assert!(!local.is_null());
    let local_ref = unsafe { &*local };

    assert!(!local_ref.vars.data.is_null());
    let first_var = unsafe { *local_ref.vars.data.add(0) };
    assert!(!first_var.is_null());
    let var_ref = unsafe { &*first_var };

    let annotation = var_ref.annotation;
    assert!(!annotation.is_null());

    let intersection =
      unsafe { rtti::ast_node_as::<AstTypeIntersection>(annotation as *mut ast_node::AstNode) };
    assert!(!intersection.is_null());
    let intersection_ref = unsafe { &*intersection };

    assert!(!intersection_ref.types.data.is_null());
    assert_eq!(intersection_ref.types.size, 2);

    let first_type = unsafe { *intersection_ref.types.data.add(0) };
    assert!(!first_type.is_null());
    let first_type_fn =
      unsafe { rtti::ast_node_as::<AstTypeFunction>(first_type as *mut ast_node::AstNode) };
    assert!(!first_type_fn.is_null());

    let second_type = unsafe { *intersection_ref.types.data.add(1) };
    assert!(!second_type.is_null());
    let second_type_fn =
      unsafe { rtti::ast_node_as::<AstTypeFunction>(second_type as *mut ast_node::AstNode) };
    assert!(!second_type_fn.is_null());
  }
}

mod parser_invalid_escape_literals_get_reported_but_parsing_continues {

  #[cfg(test)]
  #[test]
  fn parser_invalid_escape_literals_get_reported_but_parsing_continues() {
    use ulua_ast::records::{
      location::Location, parse_options::ParseOptions, parse_result::ParseResult,
      position::Position,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let source = String::from("\n        local foo = \"\\xQQ\"\n        print(foo)\n    ");
    let options = ParseOptions::new();
    let result: ParseResult = fixture.try_parse(&source, &options);

    assert_eq!(1, result.errors.len());

    let expected_location = Location::new(Position::new(1, 20), Position::new(1, 26));
    assert_eq!(&expected_location, result.errors[0].get_location());
    assert_eq!(
      "String literal contains malformed escape sequence",
      *result.errors[0].get_message()
    );

    assert!(!result.root.is_null());
    assert_eq!(2, unsafe { (*result.root).body.size });
  }
}

mod parser_invalid_type_forms {

  #[cfg(test)]
  #[test]
  fn parser_invalid_type_forms() {
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    fixture.match_parse_error(
      &String::from("type A = (b: number)"),
      &String::from("Expected '->' when parsing function type, got <eof>"),
      None,
    );
    fixture.match_parse_error(
      &String::from("type P<T...> = () -> T... type B = P<(x: number, y: string)>"),
      &String::from("Expected '->' when parsing function type, got '>'"),
      None,
    );
    fixture.match_parse_error(
      &String::from("type F<T... = (a: string)> = (T...) -> ()"),
      &String::from("Expected '->' when parsing function type, got '>'"),
      None,
    );
  }
}

mod parser_invalid_user_defined_type_functions {

  #[cfg(test)]
  #[test]
  fn parser_invalid_user_defined_type_functions() {
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    fixture.match_parse_error(
      &String::from("local foo = 1; type function bar() print(foo) end"),
      &String::from("Type function cannot reference outer local 'foo'"),
      None,
    );
    fixture.match_parse_error(
      &String::from("type function foo() local v1 = 1; type function bar() print(v1) end end"),
      &String::from("Type function cannot reference outer local 'v1'"),
      None,
    );
  }
}

mod parser_large_classes_example {

  use ulua_common::FFlag;
  #[cfg(test)]
  #[test]
  fn parser_large_classes_example() {
    use ulua_ast::records::parse_options::ParseOptions;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _g = ScopedFastFlag::new(&FFlag::DebugLuauUserDefinedClasses, true);

    let mut fix = Fixture::default();
    let src = String::from(
      "\n        class PlayerStats\n            public name: string\n            public health: number\n            public level: number\n\n            -- Static 'Constructor'\n            function new(name: string)\n                return PlayerStats {\n                    name = name,\n                    health = 100,\n                    level = 1\n                }\n            end\n\n            -- Method\n            function heal(self, amount: number)\n                self.health = math.min(100, self.health + amount)\n                print(self.name .. \" healed to \" .. self.health)\n            end\n\n            -- Metamethod for printing\n            function __tostring(self)\n                return self.name .. \" (Level \" .. self.level .. \") - Health: \" .. self.health\n            end\n        end\n\n        local player = PlayerStats.new(\"John Doe\")\n        print(player.name)\n        player:heal(20)\n        print(player.name)\n    ",
    );
    let result = fix.try_parse(&src, &ParseOptions::default());
    assert_eq!(result.errors.len(), 0);
  }
}

mod parser_last_line_does_not_have_to_be_blank {

  #[cfg(test)]
  #[test]
  fn parser_last_line_does_not_have_to_be_blank() {
    use ulua_ast::records::parse_options::ParseOptions;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let source = "-- print('hello')";
    let options = ParseOptions::new();
    let _result = fixture.parse(source, &options);
  }
}

mod parser_leading_union_intersection_with_single_type_preserves_the_union_intersection_ast_node {
  use super::*;
  #[cfg(test)]
  #[test]
  fn parser_leading_union_intersection_with_single_type_preserves_the_union_intersection_ast_node()
  {
    use ulua_ast::records::parse_options::ParseOptions;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    let source = String::from(
      "type Foo = | string\n\
           type Bar = & number",
    );
    let block = fixture.parse(&source, &ParseOptions::new());

    assert_eq!(2, unsafe { (*block).body.size });

    let alias1 = unsafe {
      rtti::ast_node_as::<ast_stat_type_alias::AstStatTypeAlias>(
        *(*block).body.data.add(0) as *mut ast_node::AstNode
      )
    };
    assert!(!alias1.is_null());

    let union_type = unsafe {
      rtti::ast_node_as::<ast_type_union::AstTypeUnion>(
        (*alias1).type_ptr as *mut ast_node::AstNode,
      )
    };
    assert!(!union_type.is_null());
    assert_eq!(1, unsafe { (*union_type).types.size });

    let alias2 = unsafe {
      rtti::ast_node_as::<ast_stat_type_alias::AstStatTypeAlias>(
        *(*block).body.data.add(1) as *mut ast_node::AstNode
      )
    };
    assert!(!alias2.is_null());

    let intersection_type = unsafe {
      rtti::ast_node_as::<ast_type_intersection::AstTypeIntersection>(
        (*alias2).type_ptr as *mut ast_node::AstNode,
      )
    };
    assert!(!intersection_type.is_null());
    assert_eq!(1, unsafe { (*intersection_type).types.size });
  }
}

mod parser_lex_broken_unicode {
  use super::*;
  #[cfg(test)]
  #[test]
  fn parser_lex_broken_unicode() {
    use ulua_ast::records::{
      allocator::Allocator, ast_name_table::AstNameTable, lexeme::Type, lexer::Lexer,
      location::Location, position::Position,
    };

    let test_input = [0xFFu8, 0xFE, 0xE2, 0x98, 0x83, 0xE2, 0x80, 0xA4];
    let mut alloc = Allocator::new();
    let mut names = AstNameTable::new(&mut alloc);
    let mut lexer = Lexer::new(
      test_input.as_ptr() as *const ffi::c_char,
      test_input.len(),
      &mut names,
      Position::default(),
    );

    let mut lexeme = *lexer.next_lexeme();
    assert_eq!(lexeme.r#type, Type::BROKEN_UNICODE);
    assert_eq!(unsafe { lexeme.data.codepoint }, 0);
    assert_eq!(
      lexeme.location,
      Location::new(Position::new(0, 0), Position::new(0, 1))
    );

    lexeme = *lexer.next_lexeme();
    assert_eq!(lexeme.r#type, Type::BROKEN_UNICODE);
    assert_eq!(unsafe { lexeme.data.codepoint }, 0);
    assert_eq!(
      lexeme.location,
      Location::new(Position::new(0, 1), Position::new(0, 2))
    );

    lexeme = *lexer.next_lexeme();
    assert_eq!(lexeme.r#type, Type::BROKEN_UNICODE);
    assert_eq!(unsafe { lexeme.data.codepoint }, 0x2603);
    assert_eq!(
      lexeme.location,
      Location::new(Position::new(0, 2), Position::new(0, 5))
    );

    lexeme = *lexer.next_lexeme();
    assert_eq!(lexeme.r#type, Type::BROKEN_UNICODE);
    assert_eq!(unsafe { lexeme.data.codepoint }, 0x2024);
    assert_eq!(
      lexeme.location,
      Location::new(Position::new(0, 5), Position::new(0, 8))
    );

    lexeme = *lexer.next_lexeme();
    assert_eq!(lexeme.r#type, Type::EOF);
  }
}

mod parser_local_with_annotation {
  use ulua_unit_test::functions::string_at_location::string_at_location;

  use super::*;
  #[cfg(test)]
  #[test]
  fn parser_local_with_annotation() {
    use ulua_ast::records::parse_options::ParseOptions;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    let block = fixture.parse(
      "local foo: string = \"Hello Types!\"",
      &ParseOptions::default(),
    );
    assert!(!block.is_null());

    let root = unsafe { &*block };
    assert!(root.body.size > 0);

    let first_stat = unsafe { *root.body.data.add(0) };
    let local = unsafe {
      rtti::ast_node_as::<ast_stat_local::AstStatLocal>(first_stat as *mut ast_node::AstNode)
    };
    assert!(!local.is_null());

    let local = unsafe { &*local };
    assert_eq!(local.vars.size, 1);

    let l = unsafe { *local.vars.data.add(0) };
    let l_ref = unsafe { &*l };
    assert!(!l_ref.annotation.is_null());

    assert_eq!(local.values.size, 1);

    let code = "local foo: string = \"Hello Types!\"";
    let loc = l_ref.location;
    let name = string_at_location(code, &loc);
    assert_eq!(name, "foo");
  }
}

mod parser_missing_declaration_prop {

  #[cfg(test)]
  #[test]
  fn parser_missing_declaration_prop() {
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    fixture.match_parse_error(
      &String::from(
        "\n        declare extern type Foo with\n            a: number,\n        end\n    ",
      ),
      &String::from("Expected identifier when parsing property name, got ','"),
      None,
    );
  }
}

mod parser_missing_default_type_pack_argument_after_variadic_type_parameter {
  use ulua_ast::records::location::Location;

  use super::*;
  #[cfg(test)]
  #[test]
  fn parser_missing_default_type_pack_argument_after_variadic_type_parameter() {
    use ulua_ast::records::parse_options::ParseOptions;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fix = Fixture::default();
    let code = String::from("\n        type Foo<T... = > = nil\n    ");

    let result = fix.try_parse(&code, &ParseOptions::new());

    assert_eq!(result.errors.len(), 2);

    let expected_location_0 =
      Location::new(UluaAstPosition::new(1, 23), UluaAstPosition::new(1, 25));
    assert_eq!(*result.errors[0].get_location(), expected_location_0);
    assert_eq!(*result.errors[0].get_message(), "Expected type, got '>'");

    let expected_location_1 =
      Location::new(UluaAstPosition::new(1, 23), UluaAstPosition::new(1, 24));
    assert_eq!(*result.errors[1].get_location(), expected_location_1);
    assert_eq!(
      *result.errors[1].get_message(),
      "Expected type pack after '=', got type"
    );
  }
}

mod parser_mixed_intersection_and_union_allowed_when_parenthesized {

  use ulua_ast::records::parse_options::ParseOptions;
  #[cfg(test)]
  #[test]
  fn parser_mixed_intersection_and_union_allowed_when_parenthesized() {
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    fixture.parse("type A = (number & string) | boolean", &ParseOptions::new());
  }
}

mod parser_mixed_intersection_and_union_not_allowed {

  #[cfg(test)]
  #[test]
  fn parser_mixed_intersection_and_union_not_allowed() {
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    fixture.match_parse_error(
      &String::from("type A = number & string | boolean"),
      &String::from(
        "Mixing union and intersection types is not allowed; consider wrapping in parentheses.",
      ),
      None,
    );
  }
}

mod parser_mixed_leading_intersection_and_union_not_allowed {

  #[cfg(test)]
  #[test]
  fn parser_mixed_leading_intersection_and_union_not_allowed() {
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    fixture.match_parse_error(
      &String::from("type A = & number | string | boolean"),
      &String::from(
        "Mixing union and intersection types is not allowed; consider wrapping in parentheses.",
      ),
      None,
    );
    fixture.match_parse_error(
      &String::from("type A = | number & string & boolean"),
      &String::from(
        "Mixing union and intersection types is not allowed; consider wrapping in parentheses.",
      ),
      None,
    );
  }
}

mod parser_mode_is_unset_if_no_hot_comment {

  #[cfg(test)]
  #[test]
  fn parser_mode_is_unset_if_no_hot_comment() {
    use ulua_ast::records::parse_options::ParseOptions;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let source = String::from("print('Hello World!')");
    let options = ParseOptions::new();
    let result = fixture.parse_ex(&source, &options);
    assert!(result.hotcomments.is_empty());
  }
}

mod parser_moved_out_allocator_can_still_be_used {

  #[cfg(test)]
  #[test]
  fn parser_moved_out_allocator_can_still_be_used() {
    use ulua_ast::records::allocator::Allocator;

    let mut outer = Allocator::new();
    let _inner = Allocator::allocator_allocator(&mut outer);

    // NOLINTNEXTLINE(bugprone-use-after-move) -- verifying moved-from state
    let i = outer.alloc::<i32>(55);
    assert!(!i.is_null());
    unsafe {
      assert_eq!(*i, 55);
    }
  }
}

mod parser_multiline_strings_newlines {
  use super::*;
  #[cfg(test)]
  #[test]
  fn parser_multiline_strings_newlines() {
    use ulua_ast::records::{
      ast_expr_constant_string::AstExprConstantString, ast_stat_return::AstStatReturn,
      parse_options::ParseOptions,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    let stat = fixture.parse(
      "return [=[\nfoo\r\nbar\n\nbaz\n]=]",
      &ParseOptions::default(),
    );
    assert!(!stat.is_null());

    let stat_block = unsafe { &*stat };
    assert!(!stat_block.body.data.is_null());

    let first_stat = unsafe { *stat_block.body.data };
    assert!(!first_stat.is_null());

    let ret = unsafe { (*first_stat).base.as_item::<AstStatReturn>() };
    assert!(!ret.is_null());

    let ret = unsafe { &*ret };
    assert!(!ret.list.data.is_null());

    let first_expr = unsafe { *ret.list.data };
    assert!(!first_expr.is_null());

    let str_expr = unsafe { (*first_expr).base.as_item::<AstExprConstantString>() };
    assert!(!str_expr.is_null());

    let str_expr = unsafe { &*str_expr };
    let s = unsafe { from_raw_parts(str_expr.value.data as *const u8, str_expr.value.size) };
    let actual = from_utf8(s).unwrap();
    assert_eq!(actual, "foo\nbar\n\nbaz\n");
  }
}

mod parser_multiple_parse_errors {

  use ulua_ast::records::parse_options::ParseOptions;
  #[cfg(test)]
  #[test]
  fn parser_multiple_parse_errors() {
    use ulua_unit_test::records::fixture::Fixture;

    let mut fix = Fixture::default();
    let source = String::from("local a = 3 * (\nreturn a +\n");

    let result = fix.try_parse(&source, &ParseOptions::default());

    assert_eq!(2, result.errors.len());
  }
}

mod parser_nil_can_not_be_a_field_name {

  #[cfg(test)]
  #[test]
  fn parser_nil_can_not_be_a_field_name() {
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    fixture.match_parse_error(
      &String::from("local n: {nil: number}"),
      &String::from("Expected '}' (to close '{' at column 10), got ':'"),
      None,
    );
  }
}

mod parser_nil_is_a_valid_type_name {

  #[cfg(test)]
  #[test]
  fn parser_nil_is_a_valid_type_name() {
    use ulua_ast::records::parse_options::ParseOptions;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    let stat = fixture.parse("local n: nil", &ParseOptions::default());
    assert!(!stat.is_null());
  }
}

mod parser_nocheck_mode {

  #[cfg(test)]
  #[test]
  fn parser_nocheck_mode() {
    use ulua_analysis::functions::parse_mode::parse_mode;
    use ulua_ast::{enums::mode::Mode, records::parse_options::ParseOptions};
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let source = String::from("--!nocheck");
    let mut options = ParseOptions::new();
    options.capture_comments = true;

    let result = fixture.parse_ex(&source, &options);

    assert!(result.errors.is_empty());

    let mode = parse_mode(&result.hotcomments);
    assert!(mode.is_some());

    let mode_val = mode.unwrap();
    assert_eq!(mode_val as i32, Mode::NoCheck as i32);
  }
}

mod parser_non_exported_class {
  use ulua_ast::records::parse_options::ParseOptions;

  use super::*;
  #[cfg(test)]
  #[test]
  fn parser_non_exported_class() {
    use ulua_common::FFlag::DebugLuauUserDefinedClasses;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _flag = ScopedFastFlag::new(&DebugLuauUserDefinedClasses, true);

    let mut fixture = Fixture::default();
    let source = r"class Foo
  end";

    let result = fixture.try_parse(&String::from(source), &ParseOptions::new());

    assert_eq!(result.errors.len(), 0);

    let block = unsafe { &*result.root };
    assert_eq!(block.body.size, 1);

    let class_decl = unsafe {
      rtti::ast_node_as::<ast_stat_class::AstStatClass>(
        *block.body.data.add(0) as *mut ast_node::AstNode
      )
    };
    assert!(!class_decl.is_null());
    let class_decl = unsafe { &*class_decl };
    assert!(!class_decl.exported);
  }
}

mod parser_non_header_hot_comments {

  #[cfg(test)]
  #[test]
  fn parser_non_header_hot_comments() {
    use ulua_analysis::functions::parse_mode::parse_mode;
    use ulua_ast::records::parse_options::ParseOptions;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let source = String::from("do end --!strict");
    let mut options = ParseOptions::new();
    options.capture_comments = true;
    let result = fixture.parse_ex(&source, &options);
    let mode = parse_mode(&result.hotcomments);
    assert!(mode.is_none());
  }
}

mod parser_non_literal_attribute_arguments_is_not_allowed {

  use ulua_ast::records::parse_options::ParseOptions;
  #[cfg(test)]
  #[test]
  fn parser_non_literal_attribute_arguments_is_not_allowed() {
    use ulua_ast::records::{location::Location, position::Position};
    use ulua_unit_test::{
      functions::check_first_error_for_attributes::check_first_error_for_attributes,
      records::fixture::Fixture,
    };

    let mut fix = Fixture::default();
    let code = String::from(
      "\n@[deprecated{ reason = reasonString }]\nfunction hello(x, y)\n    return x + y\nend",
    );

    let result = fix.try_parse(&code, &ParseOptions::default());

    let expected_location = Location::new(Position::new(1, 13), Position::new(1, 37));
    let expected_message = "Only literals can be passed as arguments for attributes";

    check_first_error_for_attributes(&result.errors, 1, expected_location, expected_message);
  }
}

mod parser_nonstrict_mode {

  #[cfg(test)]
  #[test]
  fn parser_nonstrict_mode() {
    use ulua_analysis::functions::parse_mode::parse_mode;
    use ulua_ast::{enums::mode::Mode, records::parse_options::ParseOptions};
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let source = String::from("--!nonstrict");
    let mut options = ParseOptions::new();
    options.capture_comments = true;

    let result = fixture.parse_ex(&source, &options);

    assert!(result.errors.is_empty());

    let mode = parse_mode(&result.hotcomments);
    assert!(mode.is_some());

    let mode_val = mode.unwrap();
    assert_eq!(mode_val as i32, Mode::Nonstrict as i32);
  }
}

mod parser_number_literals {
  use ulua_ast::records::parse_options::ParseOptions;

  use super::*;
  #[cfg(test)]
  #[test]
  fn parser_number_literals() {
    use ulua_ast::records::{
      ast_expr_constant_number::AstExprConstantNumber, ast_node::AstNode,
      ast_stat_return::AstStatReturn,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    let stat = fixture.parse(
      "return\n\
           1,\n\
           1.5,\n\
           .5,\n\
           12_34_56,\n\
           0x1234,\n\
           0b010101",
      &ParseOptions::default(),
    );
    assert!(!stat.is_null());

    // C++ `stat->body.data[0]->as<AstStatReturn>()` — the return is the first body
    // element, not the block's own base node.
    let ret = unsafe { rtti::ast_node_as::<AstStatReturn>(*(&*stat).body.data as *mut AstNode) };
    assert!(!ret.is_null());

    let ret_ref = unsafe { &*ret };
    assert_eq!(ret_ref.list.size, 6);

    let num0 = unsafe {
      rtti::ast_node_as::<AstExprConstantNumber>(*ret_ref.list.data.add(0) as *mut AstNode)
    };
    assert!(!num0.is_null());
    assert_eq!(unsafe { &*num0 }.value, 1.0);

    let num1 = unsafe {
      rtti::ast_node_as::<AstExprConstantNumber>(*ret_ref.list.data.add(1) as *mut AstNode)
    };
    assert!(!num1.is_null());
    assert_eq!(unsafe { &*num1 }.value, 1.5);

    let num2 = unsafe {
      rtti::ast_node_as::<AstExprConstantNumber>(*ret_ref.list.data.add(2) as *mut AstNode)
    };
    assert!(!num2.is_null());
    assert_eq!(unsafe { &*num2 }.value, 0.5);

    let num3 = unsafe {
      rtti::ast_node_as::<AstExprConstantNumber>(*ret_ref.list.data.add(3) as *mut AstNode)
    };
    assert!(!num3.is_null());
    assert_eq!(unsafe { &*num3 }.value, 123456.0);

    let num4 = unsafe {
      rtti::ast_node_as::<AstExprConstantNumber>(*ret_ref.list.data.add(4) as *mut AstNode)
    };
    assert!(!num4.is_null());
    assert_eq!(unsafe { &*num4 }.value, 0x1234 as f64);

    let num5 = unsafe {
      rtti::ast_node_as::<AstExprConstantNumber>(*ret_ref.list.data.add(5) as *mut AstNode)
    };
    assert!(!num5.is_null());
    assert_eq!(unsafe { &*num5 }.value, 0x15 as f64);
  }
}

mod parser_other_places_where_type_annotations_are_allowed {

  #[cfg(test)]
  #[test]
  fn parser_other_places_where_type_annotations_are_allowed() {
    use ulua_ast::records::parse_options::ParseOptions;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    let stat = fixture.parse(
      "for i: number = 0, 50 do end\n\
           for i: number, s: string in expr() do end",
      &ParseOptions::default(),
    );
    assert!(!stat.is_null());
  }
}

mod parser_overlapping_property_and_method_names {

  use ulua_common::FFlag;
  #[cfg(test)]
  #[test]
  fn parser_overlapping_property_and_method_names() {
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _g = ScopedFastFlag::new(&FFlag::DebugLuauUserDefinedClasses, true);

    let mut fix = Fixture::default();
    fix.match_parse_error(
      &String::from(
        "\nclass Hello\n    public helloagain\n    function helloagain() end\nend\n        ",
      ),
      &String::from("Duplicate class member 'helloagain'"),
      None,
    );
  }
}

mod parser_parse_attribute_for_function_expression {
  use ulua_unit_test::functions::check_attribute::check_attribute;

  use super::*;
  #[cfg(test)]
  #[test]
  fn parser_parse_attribute_for_function_expression() {
    use ulua_ast::records::{
      ast_attr::AstAttrType, ast_expr_call::AstExprCall, ast_expr_function::AstExprFunction,
      ast_stat_expr::AstStatExpr, ast_stat_local::AstStatLocal, location::Location,
      parse_options::ParseOptions, position::Position,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let source1 = String::from(
      "\nlocal function invoker(f)\n    return f(1)\nend\n\ninvoker(@checked function(x) return (x + 2) end)\n",
    );
    let options = ParseOptions::new();
    let stat1 = fixture.parse(&source1, &options);

    ulua_common::LUAU_ASSERT!(!stat1.is_null());

    let stat1_block = unsafe { &*stat1 };
    let body1 = stat1_block.body;
    let stmt2_ptr = unsafe { *body1.data.add(1) };
    let expr_stmt =
      unsafe { rtti::ast_node_as::<AstStatExpr>(stmt2_ptr as *mut ast_node::AstNode) };
    ulua_common::LUAU_ASSERT!(!expr_stmt.is_null());
    let expr_stmt = unsafe { &*expr_stmt };
    let call_ptr = expr_stmt.expr;
    let call = unsafe { rtti::ast_node_as::<AstExprCall>(call_ptr as *mut ast_node::AstNode) };
    ulua_common::LUAU_ASSERT!(!call.is_null());
    let call = unsafe { &*call };
    let args = call.args;
    let arg0_ptr = unsafe { *args.data.add(0) };
    let func_expr =
      unsafe { rtti::ast_node_as::<AstExprFunction>(arg0_ptr as *mut ast_node::AstNode) };
    ulua_common::LUAU_ASSERT!(!func_expr.is_null());
    let func_expr = unsafe { &*func_expr };
    let attributes1 = func_expr.attributes;

    ulua_common::LUAU_ASSERT!(attributes1.size == 1);

    let attr0 = unsafe { &**attributes1.data.add(0) };
    let expected_location1 = Location::new(Position::new(5, 8), Position::new(5, 16));
    check_attribute(attr0, AstAttrType::Checked, expected_location1);

    let source2 = String::from("\nlocal f = @checked function(x) return (x + 2) end\n");
    let stat2 = fixture.parse(&source2, &options);

    ulua_common::LUAU_ASSERT!(!stat2.is_null());

    let stat2_block = unsafe { &*stat2 };
    let body2 = stat2_block.body;
    let stmt0_ptr = unsafe { *body2.data.add(0) };
    let local_stmt =
      unsafe { rtti::ast_node_as::<AstStatLocal>(stmt0_ptr as *mut ast_node::AstNode) };
    ulua_common::LUAU_ASSERT!(!local_stmt.is_null());
    let local_stmt = unsafe { &*local_stmt };
    let values = local_stmt.values;
    let val0_ptr = unsafe { *values.data.add(0) };
    let func_expr2 =
      unsafe { rtti::ast_node_as::<AstExprFunction>(val0_ptr as *mut ast_node::AstNode) };
    ulua_common::LUAU_ASSERT!(!func_expr2.is_null());
    let func_expr2 = unsafe { &*func_expr2 };
    let attributes2 = func_expr2.attributes;

    ulua_common::LUAU_ASSERT!(attributes2.size == 1);

    let attr0_2 = unsafe { &**attributes2.data.add(0) };
    let expected_location2 = Location::new(Position::new(1, 10), Position::new(1, 18));
    check_attribute(attr0_2, AstAttrType::Checked, expected_location2);
  }
}

mod parser_parse_attribute_on_export_function_stat {
  use ulua_ast::records::parse_options::ParseOptions;

  use super::*;
  #[cfg(test)]
  #[test]
  fn parser_parse_attribute_on_export_function_stat() {
    use ulua_ast::records::{
      ast_stat_local_function::AstStatLocalFunction, location::Location, position::Position,
    };
    use ulua_common::FFlag::LuauExportValueSyntax;
    use ulua_unit_test::{
      functions::check_attribute::check_attribute, records::fixture::Fixture,
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let mut fixture = Fixture::fixture_bool(false);
    let _sff_luau_export_value_syntax = ScopedFastFlag::new(&LuauExportValueSyntax, true);

    let source = String::from("\n@checked\nexport function hello(x, y)\n    return x + y\nend");
    let parse_options = ParseOptions::default();
    let stat = fixture.parse(&source, &parse_options);

    assert!(!stat.is_null(), "parse should succeed");

    let first_stat = unsafe { (*stat).body.data.add(0) };
    let stat_fun =
      unsafe { rtti::ast_node_as::<AstStatLocalFunction>(*first_stat as *mut ast_node::AstNode) };
    assert!(
      !stat_fun.is_null(),
      "first stat should be AstStatLocalFunction"
    );

    assert_eq!(
      unsafe { (*stat_fun).base.base.location.begin },
      Position::new(1, 0)
    );
    assert!(unsafe { (*stat_fun).name.as_ref().unwrap().is_exported });
    assert!(unsafe { (*stat_fun).name.as_ref().unwrap().is_const });

    let func = unsafe { (*stat_fun).func };
    let attributes = unsafe { (*func).attributes };

    assert_eq!(attributes.size, 1);

    let attr = unsafe { *attributes.data.add(0) };
    check_attribute(
      unsafe { &*attr },
      UluaAstAstAttrType::Checked,
      Location::new(Position::new(1, 0), Position::new(1, 8)),
    );
  }
}

mod parser_parse_attribute_on_function_stat {
  use ulua_ast::records::parse_options::ParseOptions;

  use super::*;
  #[cfg(test)]
  #[test]
  fn parser_parse_attribute_on_function_stat() {
    use ulua_ast::records::{
      ast_stat_function::AstStatFunction, location::Location, position::Position,
    };
    use ulua_unit_test::{functions::check_attribute::check_attribute, records::fixture::Fixture};

    let mut fix = Fixture::default();
    // C++ R"(\n@checked\nfunction hello(x, y)\n    return x + y\nend)" — `@checked`
    // is ONE token; the port split `@` and `checked` onto separate lines, so the
    // lexer saw `@` with no following name ("Attribute name is missing").
    let code = "\n@checked\nfunction hello(x, y)\n    return x + y\nend";

    let stat = fix.parse(code, &ParseOptions::default());

    assert!(!stat.is_null());

    let stat_fun = unsafe {
      rtti::ast_node_as::<AstStatFunction>(*(&*stat).body.data as *mut ast_node::AstNode)
    };
    assert!(!stat_fun.is_null());

    let attributes = unsafe { (*stat_fun).func.as_ref().unwrap().attributes };

    assert_eq!(attributes.size, 1);

    let attr = unsafe { *attributes.data.add(0) };
    let expected_location = Location::new(Position::new(1, 0), Position::new(1, 8));
    check_attribute(
      unsafe { &*attr },
      UluaAstAstAttrType::Checked,
      expected_location,
    );
  }
}

mod parser_parse_attribute_on_function_type_declaration {
  use ulua_unit_test::functions::check_attribute::check_attribute;

  use super::*;
  #[cfg(test)]
  #[test]
  fn parser_parse_attribute_on_function_type_declaration() {
    use ulua_ast::records::{
      ast_attr::AstAttrType, ast_stat::AstStat, ast_stat_declare_function::AstStatDeclareFunction,
      location::Location, parse_options::ParseOptions, position::Position,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fix = Fixture::default();
    let code = String::from("\n@checked declare function abs(n: number): number\n");
    let mut opts = ParseOptions::new();
    opts.allow_declaration_syntax = true;
    let result = fix.try_parse(&code, &opts);
    assert_eq!(result.errors.len(), 0);

    let root_block = unsafe { &*result.root };
    assert_eq!(root_block.body.size, 1);

    let root = unsafe { &**root_block.body.data };
    let func = unsafe {
      rtti::ast_node_as::<AstStatDeclareFunction>(root as *const AstStat as *mut ast_node::AstNode)
    };
    assert!(!func.is_null());

    assert!(unsafe { (*func).is_checked_function() });

    let attributes = unsafe { (*func).attributes };
    assert_eq!(attributes.size, 1);

    let attr = unsafe { &**attributes.data };
    let expected_location = Location::new(Position::new(1, 0), Position::new(1, 8));
    check_attribute(attr, AstAttrType::Checked, expected_location);
  }
}

mod parser_parse_attribute_on_local_function_stat {
  use ulua_ast::records::parse_options::ParseOptions;

  use super::*;
  #[cfg(test)]
  #[test]
  fn parser_parse_attribute_on_local_function_stat() {
    use ulua_ast::records::{
      ast_array::AstArray,
      ast_attr::{AstAttr, AstAttrType},
      ast_stat_block::AstStatBlock,
      ast_stat_local_function::AstStatLocalFunction,
      location::Location,
      position::Position,
    };
    use ulua_unit_test::{functions::check_attribute::check_attribute, records::fixture::Fixture};

    let mut fix = Fixture::default();
    // C++ R"(\n    @checked\nlocal function hello(x, y)\n    return x + y\nend)" —
    // `@checked` is one token at column 4 (4-space indent). The port split it across
    // lines and dropped the indent.
    let code = "\n    @checked\nlocal function hello(x, y)\n    return x + y\nend";

    let stat: *mut AstStatBlock = fix.parse(code, &ParseOptions::default());

    assert!(!stat.is_null());

    let stat_fun: *mut AstStatLocalFunction = unsafe {
      rtti::ast_node_as::<AstStatLocalFunction>(*(*stat).body.data.add(0) as *mut ast_node::AstNode)
    };
    assert!(!stat_fun.is_null());

    let attributes: AstArray<*mut AstAttr> = unsafe { (*(*stat_fun).func).attributes };

    assert_eq!(attributes.size, 1);

    check_attribute(
      unsafe { &**attributes.data.add(0) },
      AstAttrType::Checked,
      Location::new(Position::new(1, 4), Position::new(1, 12)),
    );
  }
}

mod parser_parse_attributes_on_function_type_declaration_in_table {
  use super::*;
  #[cfg(test)]
  #[test]
  fn parser_parse_attributes_on_function_type_declaration_in_table() {
    use ulua_ast::records::{
      ast_stat::AstStat, ast_stat_declare_global::AstStatDeclareGlobal,
      ast_type_function::AstTypeFunction, ast_type_table::AstTypeTable, location::Location,
      parse_options::ParseOptions, position::Position,
    };
    use ulua_unit_test::{functions::check_attribute::check_attribute, records::fixture::Fixture};

    let mut fixture = Fixture::fixture_bool(false);
    let source = String::from("\ndeclare bit32: {\n    band: @checked (...number) -> number\n}");
    let mut opts = ParseOptions::new();
    opts.allow_declaration_syntax = true;

    let result = fixture.try_parse(&source, &opts);

    assert_eq!(result.errors.len(), 0);

    let root_block = unsafe { &*result.root };
    assert_eq!(root_block.body.size, 1);

    let root_stat = unsafe { &*(*root_block.body.data.add(0)) as *const AstStat };
    let glob =
      unsafe { rtti::ast_node_as::<AstStatDeclareGlobal>(root_stat as *mut ast_node::AstNode) };
    assert!(!glob.is_null());

    let tbl = unsafe { rtti::ast_node_as::<AstTypeTable>((*glob).type_ as *mut ast_node::AstNode) };
    assert!(!tbl.is_null());

    assert_eq!(unsafe { (*tbl).props.size }, 1);
    let prop = unsafe { &*(*tbl).props.data.add(0) };

    let func =
      unsafe { rtti::ast_node_as::<AstTypeFunction>(prop.r#type as *mut ast_node::AstNode) };
    assert!(!func.is_null());

    let attributes = unsafe { &(*func).attributes };

    assert_eq!(attributes.size, 1);
    let attr = unsafe { &**attributes.data.add(0) };

    check_attribute(
      attr,
      UluaAstAstAttrType::Checked,
      Location::new(Position::new(2, 10), Position::new(2, 18)),
    );
  }
}

mod parser_parse_checked_as_function_name_fails {

  #[cfg(test)]
  #[test]
  fn parser_parse_checked_as_function_name_fails() {
    use ulua_ast::records::parse_options::ParseOptions;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fix = Fixture::default();
    let code = r"(
      @checked function(x: number) : number
      end
  )";
    let mut opts = ParseOptions::new();
    opts.allow_declaration_syntax = true;

    let result = fix.try_parse(&String::from(code), &opts);
    assert!(!result.errors.is_empty());
  }
}

mod parser_parse_checked_in_and_out_of_decl_fails {

  #[cfg(test)]
  #[test]
  fn parser_parse_checked_in_and_out_of_decl_fails() {
    use ulua_ast::records::parse_options::ParseOptions;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fix = Fixture::default();
    let code = String::from(
      "\n    local @checked = 3\n    @checked declare function abs(n: number): number\n",
    );
    let mut opts = ParseOptions::new();
    opts.allow_declaration_syntax = true;
    let result = fix.try_parse(&code, &opts);
    assert_eq!(result.errors.len(), 2);
    assert_eq!(result.errors[0].get_location().begin.line, 1);
    assert_eq!(result.errors[1].get_location().begin.line, 1);
  }
}

mod parser_parse_checked_outside_decl_fails {

  #[cfg(test)]
  #[test]
  fn parser_parse_checked_outside_decl_fails() {
    use ulua_ast::records::parse_options::ParseOptions;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let source = String::from(
      "local @checked = 3
  ",
    );
    let mut opts = ParseOptions::new();
    opts.allow_declaration_syntax = true;

    let result = fixture.try_parse(&source, &opts);

    assert!(!result.errors.is_empty());
    let _ts = result.errors[1].get_message();
  }
}

mod parser_parse_class_declarations {
  use super::*;
  #[cfg(test)]
  #[test]
  fn parser_parse_class_declarations() {
    use ulua_ast::{
      records::{
        ast_stat_block::AstStatBlock, ast_stat_declare_extern_type::AstStatDeclareExternType,
        ast_type_function::AstTypeFunction, ast_type_reference::AstTypeReference,
        location::Location, parse_options::ParseOptions,
      },
      rtti::{ast_node_as, ast_node_is},
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    let result = fixture.parse_ex(
          // Faithful to the C++ R-string: leading newline + 8-space (declare/end) and
          // 12-space (prop/function) indentation; the port used tabs + line-continuations.
          &String::from(
              "\n        declare extern type Foo with\n            prop: number\n            function method(self, foo: number): string\n        end\n\n        declare extern type Bar extends Foo with\n            prop2: string\n        end\n    ",
          ),
          &ParseOptions::default(),
      );
    let root = result.root;
    assert!(!root.is_null());

    let root_block: *mut AstStatBlock =
      unsafe { ast_node_as::<AstStatBlock>(root as *mut ast_node::AstNode) };
    assert!(!root_block.is_null());

    let body = unsafe { &(*root_block).body };
    assert_eq!(2, body.size);

    let stat0 = unsafe { *body.data.add(0) };
    assert!(!stat0.is_null());
    assert!(ast_node_is::<AstStatDeclareExternType>(
      stat0 as *mut ast_node::AstNode
    ));

    let extern_type_foo: *mut AstStatDeclareExternType =
      unsafe { ast_node_as::<AstStatDeclareExternType>(stat0 as *mut ast_node::AstNode) };
    assert!(!extern_type_foo.is_null());

    let foo_name = unsafe { (*extern_type_foo).name };
    assert_eq!("Foo", unsafe {
      CStr::from_ptr(foo_name.value).to_string_lossy()
    });

    let foo_super_name = unsafe { (*extern_type_foo).super_name };
    assert!(foo_super_name.is_none());

    let foo_props = unsafe { &(*extern_type_foo).props };
    assert_eq!(2, foo_props.size);

    let prop0 = unsafe { *foo_props.data.add(0) };
    let prop0_name = prop0.name;
    assert_eq!("prop", unsafe {
      CStr::from_ptr(prop0_name.value).to_string_lossy()
    });

    let expected_prop_loc = Location::new(UluaAstPosition::new(2, 12), UluaAstPosition::new(2, 16));
    assert_eq!(expected_prop_loc, prop0.name_location);

    let expected_prop_full_loc =
      Location::new(UluaAstPosition::new(2, 12), UluaAstPosition::new(2, 24));
    assert_eq!(expected_prop_full_loc, prop0.location);

    assert!(ast_node_is::<AstTypeReference>(
      prop0.ty as *mut ast_node::AstNode
    ));

    let method_prop = unsafe { *foo_props.data.add(1) };
    let method_name = method_prop.name;
    assert_eq!("method", unsafe {
      CStr::from_ptr(method_name.value).to_string_lossy()
    });

    let expected_method_name_loc =
      Location::new(UluaAstPosition::new(3, 21), UluaAstPosition::new(3, 27));
    assert_eq!(expected_method_name_loc, method_prop.name_location);

    let expected_method_full_loc =
      Location::new(UluaAstPosition::new(3, 12), UluaAstPosition::new(3, 54));
    assert_eq!(expected_method_full_loc, method_prop.location);

    assert!(method_prop.is_method);
    assert!(ast_node_is::<AstTypeFunction>(
      method_prop.ty as *mut ast_node::AstNode
    ));

    let stat1 = unsafe { *body.data.add(1) };
    assert!(!stat1.is_null());
    assert!(ast_node_is::<AstStatDeclareExternType>(
      stat1 as *mut ast_node::AstNode
    ));

    let extern_type_bar: *mut AstStatDeclareExternType =
      unsafe { ast_node_as::<AstStatDeclareExternType>(stat1 as *mut ast_node::AstNode) };
    assert!(!extern_type_bar.is_null());

    let bar_name = unsafe { (*extern_type_bar).name };
    assert_eq!("Bar", unsafe {
      CStr::from_ptr(bar_name.value).to_string_lossy()
    });

    let bar_super_name = unsafe { (*extern_type_bar).super_name };
    assert!(bar_super_name.is_some());
    let super_name_ptr = bar_super_name.unwrap().value;
    assert_eq!("Foo", unsafe {
      CStr::from_ptr(super_name_ptr).to_string_lossy()
    });

    let bar_props = unsafe { &(*extern_type_bar).props };
    assert_eq!(1, bar_props.size);

    let prop1 = unsafe { *bar_props.data.add(0) };
    let prop1_name = prop1.name;
    assert_eq!("prop2", unsafe {
      CStr::from_ptr(prop1_name.value).to_string_lossy()
    });

    let expected_prop2_loc =
      Location::new(UluaAstPosition::new(7, 12), UluaAstPosition::new(7, 17));
    assert_eq!(expected_prop2_loc, prop1.name_location);

    let expected_prop2_full_loc =
      Location::new(UluaAstPosition::new(7, 12), UluaAstPosition::new(7, 25));
    assert_eq!(expected_prop2_full_loc, prop1.location);

    assert!(ast_node_is::<AstTypeReference>(
      prop1.ty as *mut ast_node::AstNode
    ));
  }
}

mod parser_parse_class_declarations_unaffected_by_global_flag {
  use super::*;
  #[cfg(test)]
  #[test]
  fn parser_parse_class_declarations_unaffected_by_global_flag() {
    use ulua_ast::records::parse_options::ParseOptions;
    use ulua_common::FFlag::LuauAllowGlobalDeclarationToBeCalledClass;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _flag = ScopedFastFlag::new(&LuauAllowGlobalDeclarationToBeCalledClass, true);
    let mut fixture = Fixture::default();
    let result = fixture.parse_ex(
      &String::from(
        "declare extern type Foo with\n\
               \tprop: number\n\
               end",
      ),
      &ParseOptions::default(),
    );

    let root = unsafe { &*result.root };
    assert!(!root.body.data.is_null());
    assert_eq!(root.body.size, 1);

    let declared = unsafe {
      rtti::ast_node_as::<ast_stat_declare_extern_type::AstStatDeclareExternType>(
        *root.body.data as *mut ast_node::AstNode,
      )
    };
    assert!(!declared.is_null());
    assert_eq!(unsafe { CStr::from_ptr((*declared).name.value) }, c"Foo");
  }
}

mod parser_parse_compound_assignment {
  use super::*;
  #[cfg(test)]
  #[test]
  fn parser_parse_compound_assignment() {
    use ulua_ast::records::{
      ast_stat_compound_assign::AstStatCompoundAssign, parse_options::ParseOptions,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    let block = fixture.parse("a += 5", &ParseOptions::default());

    assert!(!block.is_null());

    let block_ref = unsafe { &*block };
    assert_eq!(block_ref.body.size, 1);

    let first_stat = unsafe { *block_ref.body.data.add(0) };
    assert!(unsafe { &*first_stat }.base.is::<AstStatCompoundAssign>());

    let compound_assign = unsafe { &*first_stat }
      .base
      .as_item::<AstStatCompoundAssign>();
    assert!(!compound_assign.is_null());
    assert_eq!(unsafe { &*compound_assign }.op, AstExprBinaryOp::Add);
  }
}

mod parser_parse_compound_assignment_error_call {

  #[cfg(test)]
  #[test]
  fn parser_parse_compound_assignment_error_call() {
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    fixture.match_parse_error(
      &String::from("a() += 5"),
      &String::from("Expected identifier when parsing expression, got '+='"),
      None,
    );
  }
}

mod parser_parse_compound_assignment_error_multiple {

  use ulua_ast::records::parse_options::ParseOptions;
  #[cfg(test)]
  #[test]
  fn parser_parse_compound_assignment_error_multiple() {
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    let source = String::from("a, b += 5");
    // C++ catches the thrown ParseErrors. `parse_ex` PANICS on errors, so use
    // try_parse (returns the errors) and inspect them.
    let result = fixture.try_parse(&source, &ParseOptions::default());

    // Check that parsing failed with ParseErrors
    if result.errors.is_empty() {
      panic!("Expected ParseErrors to be thrown");
    }

    let errors = &result.errors;
    let error_vec = errors;

    // Check that we have at least one error
    if error_vec.is_empty() {
      panic!("Expected at least one parse error");
    }

    // Get the first error's message
    let first_error = &error_vec[0];
    let actual_message = first_error.get_message();
    let expected_message = String::from("Expected '=' when parsing assignment, got '+='");

    if actual_message != &expected_message {
      panic!(
        "Expected error message '{}' but got '{}'",
        expected_message, actual_message
      );
    }
  }
}

mod parser_parse_compound_assignment_error_not_lvalue {

  #[cfg(test)]
  #[test]
  fn parser_parse_compound_assignment_error_not_lvalue() {
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    fixture.match_parse_error(
      &String::from("(a) += 5"),
      &String::from("Assigned expression must be a variable or a field"),
      None,
    );
  }
}

mod parser_parse_const {
  use super::*;
  #[cfg(test)]
  #[test]
  fn parser_parse_const() {
    use ulua_ast::records::{
      ast_expr_constant_number::AstExprConstantNumber, ast_stat_local::AstStatLocal,
      parse_options::ParseOptions,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    let stat = fixture.parse("const f = 42", &ParseOptions::default());
    let root = unsafe { &*stat };
    assert!(!root.body.data.is_null());
    assert_eq!(root.body.size, 1);
    let first_stat = unsafe { *root.body.data.add(0) };
    assert!(unsafe { (*first_stat).base.is::<AstStatLocal>() });
    let stat_local = unsafe { (*first_stat).base.as_item::<AstStatLocal>() };
    assert!(!stat_local.is_null());
    let stat_local = unsafe { &*stat_local };
    assert_eq!(stat_local.vars.size, 1);
    assert_eq!(stat_local.values.size, 1);
    let local = unsafe { *stat_local.vars.data.add(0) };
    assert!(!local.is_null());
    let local = unsafe { &*local };
    let name_str = unsafe { CStr::from_ptr(local.name.value).to_string_lossy() };
    assert_eq!(name_str, "f");
    assert!(local.is_const);
    let value = unsafe { *stat_local.values.data.add(0) };
    assert!(!value.is_null());
    assert!(unsafe { (*value).base.is::<AstExprConstantNumber>() });
    let expr_const = unsafe { (*value).base.as_item::<AstExprConstantNumber>() };
    assert!(!expr_const.is_null());
    let expr_const = unsafe { &*expr_const };
    assert_eq!(expr_const.value, 42.0);
  }
}

mod parser_parse_const_call {

  use ulua_ast::records::parse_options::ParseOptions;
  #[cfg(test)]
  #[test]
  fn parser_parse_const_call() {
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);

    let source = String::from("local const = function(t) return t end\nconst { a = \"a\" }");
    let _stat = fixture.parse(&source, &ParseOptions::default());
  }
}

mod parser_parse_const_function {

  use ulua_ast::records::parse_options::ParseOptions;
  #[cfg(test)]
  #[test]
  fn parser_parse_const_function() {
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::default();

    let source = String::from("const function f() return 42 end");
    let result = fixture.parse(&source, &ParseOptions::default());

    assert!(!result.is_null());
  }
}

mod parser_parse_const_function_with_attr {

  use ulua_ast::records::parse_options::ParseOptions;
  #[cfg(test)]
  #[test]
  fn parser_parse_const_function_with_attr() {
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::default();

    let source = String::from("@deprecated\nconst function f() return 42 end\n");
    let result = fixture.parse(&source, &ParseOptions::default());

    assert!(!result.is_null());
  }
}

mod parser_parse_const_multi_initialize {

  use ulua_ast::records::parse_options::ParseOptions;
  #[cfg(test)]
  #[test]
  fn parser_parse_const_multi_initialize() {
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    let _stat = fixture.parse(
      r#"const a, b = 42, 32
  
  const a, b, c = 42, f()
  
  const a, b, c = 42, ...
  "#,
      &ParseOptions::default(),
    );
    assert!(!_stat.is_null());
  }
}

mod parser_parse_continue {
  use super::*;
  #[cfg(test)]
  #[test]
  fn parser_parse_continue() {
    use ulua_ast::records::parse_options::ParseOptions;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    let stat = fixture.parse(
      "while true do\n\
           continue()\n\
           continue = 5\n\
           continue, continue = continue\n\
           continue\n\
           end",
      &ParseOptions::default(),
    );
    assert!(!stat.is_null());

    let block =
      unsafe { rtti::ast_node_as::<ast_stat_block::AstStatBlock>(stat as *mut ast_node::AstNode) };
    assert!(!block.is_null());
    assert_eq!(1, unsafe { (*block).body.size });

    let wb = unsafe {
      rtti::ast_node_as::<AstStatWhile>(*(*block).body.data.add(0) as *mut ast_node::AstNode)
    };
    assert!(!wb.is_null());

    let wblock = unsafe {
      rtti::ast_node_as::<ast_stat_block::AstStatBlock>((*wb).body as *mut ast_node::AstNode)
    };
    assert!(!wblock.is_null());
    assert_eq!(4, unsafe { (*wblock).body.size });

    assert!(
      !unsafe {
        rtti::ast_node_as::<ast_stat_expr::AstStatExpr>(
          *(*wblock).body.data.add(0) as *mut ast_node::AstNode
        )
      }
      .is_null()
    );
    assert!(
      !unsafe {
        rtti::ast_node_as::<ast_stat_assign::AstStatAssign>(
          *(*wblock).body.data.add(1) as *mut ast_node::AstNode
        )
      }
      .is_null()
    );
    assert!(
      !unsafe {
        rtti::ast_node_as::<ast_stat_assign::AstStatAssign>(
          *(*wblock).body.data.add(2) as *mut ast_node::AstNode
        )
      }
      .is_null()
    );
    assert!(
      !unsafe {
        rtti::ast_node_as::<AstStatContinue>(*(*wblock).body.data.add(3) as *mut ast_node::AstNode)
      }
      .is_null()
    );
  }
}

mod parser_parse_debugnoinline_on_local_function {
  use ulua_ast::records::parse_options::ParseOptions;

  use super::*;
  #[cfg(test)]
  #[test]
  fn parser_parse_debugnoinline_on_local_function() {
    use ulua_ast::records::{
      ast_stat_local_function::AstStatLocalFunction, location::Location, position::Position,
    };
    use ulua_common::FFlag::DebugLuauNoInline;
    use ulua_unit_test::{
      functions::check_attribute::check_attribute, records::fixture::Fixture,
      type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _no_inline = ScopedFastFlag::new(&DebugLuauNoInline, true);

    let mut fix = Fixture::default();
    let source = "\n    @debugnoinline\nlocal function hello(x, y)\n    return x + y\nend";

    let stat = fix.parse(source, &ParseOptions::default());

    assert!(!stat.is_null());

    let stat_local_function = unsafe {
      rtti::ast_node_as::<AstStatLocalFunction>(*(*stat).body.data.add(0) as *mut ast_node::AstNode)
    };
    assert!(!stat_local_function.is_null());

    let attributes = unsafe { (*(*stat_local_function).func).attributes };

    assert_eq!(attributes.size, 1);

    check_attribute(
      unsafe { &**attributes.data },
      UluaAstAstAttrType::DebugNoinline,
      Location::new(Position::new(1, 4), Position::new(1, 18)),
    );
  }
}

mod parser_parse_declarations {
  use ulua_ast::records::location::Location;

  use super::*;
  #[cfg(test)]
  #[test]
  fn parser_parse_declarations() {
    use ulua_ast::records::parse_options::ParseOptions;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    let result = fixture.parse_ex(
          &String::from(
              "\n        declare foo: number\n        declare function bar(x: number): string\n        declare function var(...: any)\n    ",
          ),
          &ParseOptions::default(),
      );
    let root = unsafe { &*result.root };
    assert_eq!(root.body.size, 3);

    let global = unsafe {
      rtti::ast_node_as::<ast_stat_declare_global::AstStatDeclareGlobal>(
        *root.body.data.add(0) as *mut ast_node::AstNode
      )
      .as_ref()
    };
    assert!(global.is_some());
    let global = global.unwrap();
    assert_eq!(
      unsafe { CStr::from_ptr(global.name.value).to_string_lossy() },
      "foo"
    );
    assert_eq!(
      global.name_location,
      Location::new(UluaAstPosition::new(1, 16), UluaAstPosition::new(1, 19),)
    );
    assert!(!global.type_.is_null());

    let func = unsafe {
      rtti::ast_node_as::<ast_stat_declare_function::AstStatDeclareFunction>(
        *root.body.data.add(1) as *mut ast_node::AstNode
      )
      .as_ref()
    };
    assert!(func.is_some());
    let func = func.unwrap();
    assert_eq!(
      unsafe { CStr::from_ptr(func.name.value).to_string_lossy() },
      "bar"
    );
    assert_eq!(
      func.name_location,
      Location::new(UluaAstPosition::new(2, 25), UluaAstPosition::new(2, 28),)
    );
    assert_eq!(func.params.types.size, 1);

    let ret_type_pack = unsafe {
      rtti::ast_node_as::<ast_type_pack_explicit::AstTypePackExplicit>(
        func.ret_types as *mut ast_node::AstNode,
      )
      .as_ref()
    };
    assert!(ret_type_pack.is_some());
    let ret_type_pack = ret_type_pack.unwrap();
    assert_eq!(ret_type_pack.type_list.types.size, 1);

    let var_func = unsafe {
      rtti::ast_node_as::<ast_stat_declare_function::AstStatDeclareFunction>(
        *root.body.data.add(2) as *mut ast_node::AstNode
      )
      .as_ref()
    };
    assert!(var_func.is_some());
    let var_func = var_func.unwrap();
    assert_eq!(
      unsafe { CStr::from_ptr(var_func.name.value).to_string_lossy() },
      "var"
    );
    assert_eq!(
      var_func.name_location,
      Location::new(UluaAstPosition::new(3, 25), UluaAstPosition::new(3, 28),)
    );
    assert!(var_func.vararg);
    assert_eq!(
      var_func.vararg_location,
      Location::new(UluaAstPosition::new(3, 29), UluaAstPosition::new(3, 32),)
    );

    fixture.match_parse_error(
      &String::from("declare function foo(x)"),
      &String::from("All declaration parameters must be annotated"),
      None,
    );
    fixture.match_parse_error(
      &String::from("declare foo"),
      &String::from("Expected ':' when parsing global variable declaration, got <eof>"),
      None,
    );
  }
}

mod parser_parse_declared_table_checked_member {
  use super::*;
  #[cfg(test)]
  #[test]
  fn parser_parse_declared_table_checked_member() {
    use ulua_ast::records::{
      ast_node::AstNode, ast_stat_declare_global::AstStatDeclareGlobal,
      ast_type_function::AstTypeFunction, ast_type_table::AstTypeTable,
      parse_options::ParseOptions,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fix = Fixture::default();
    // Faithful to the C++ R"BUILTIN_SRC(...)" raw string: leading newline, then
    // `    declare math : {` (4 spaces), `        abs : ...` (8 spaces), then `}`.
    let code =
      String::from("\n    declare math : {\n        abs : @checked (number) -> number\n}\n");
    let mut opts = ParseOptions::new();
    opts.allow_declaration_syntax = true;

    let pr = fix.try_parse(&code, &opts);
    ulua_common::LUAU_ASSERT!(pr.errors.is_empty());

    ulua_common::LUAU_ASSERT!(unsafe { (*pr.root).body.size } == 1);
    // C++: `AstStat* root = *(pr.root->body.data);` — the FIRST statement, not the block.
    let root = unsafe { *(*pr.root).body.data.add(0) };
    let glob = unsafe { rtti::ast_node_as::<AstStatDeclareGlobal>(root as *mut AstNode) };
    ulua_common::LUAU_ASSERT!(!glob.is_null());
    let glob = unsafe { &*glob };
    let tbl = unsafe { rtti::ast_node_as::<AstTypeTable>(glob.type_ as *mut AstNode) };
    ulua_common::LUAU_ASSERT!(!tbl.is_null());
    let tbl = unsafe { &*tbl };
    ulua_common::LUAU_ASSERT!(tbl.props.size == 1);
    let prop = unsafe { &*tbl.props.data };
    let func = unsafe { rtti::ast_node_as::<AstTypeFunction>(prop.r#type as *mut AstNode) };
    ulua_common::LUAU_ASSERT!(!func.is_null());
    let func = unsafe { &*func };
    ulua_common::LUAU_ASSERT!(func.is_checked_function());
  }
}

mod parser_parse_error_assignment_lvalue {

  #[cfg(test)]
  #[test]
  fn parser_parse_error_assignment_lvalue() {
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    fixture.match_parse_error(
      &String::from("local a, b\n(2), b = b, a\n"),
      &String::from("Assigned expression must be a variable or a field"),
      None,
    );
    fixture.match_parse_error(
      &String::from("local a, b\na, (3) = b, a\n"),
      &String::from("Assigned expression must be a variable or a field"),
      None,
    );
  }
}

mod parser_parse_error_broken_comment {

  #[cfg(test)]
  #[test]
  fn parser_parse_error_broken_comment() {
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    let expected =
      String::from("Expected identifier when parsing expression, got unfinished comment");
    fixture.match_parse_error(&String::from("--[[unfinished work"), &expected, None);
    fixture.match_parse_error(
      &String::from("--!strict\n--[[unfinished work"),
      &expected,
      None,
    );
    fixture.match_parse_error(
      &String::from("local x = 1 --[[unfinished work"),
      &expected,
      None,
    );
  }
}

mod parser_parse_error_confusing_function_call {
  use super::*;
  #[cfg(test)]
  #[test]
  fn parser_parse_error_confusing_function_call() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fix = Fixture::default();

    let code1 = String::from(
      "function add(x, y) return x + y end\n\
           add\n\
           (4, 7)",
    );
    let message1 = String::from(
      "Ambiguous syntax: this looks like an argument list for a function call, but could also be a start of new statement; use ';' to separate statements",
    );
    let result1 = fix.match_parse_error(&code1, &message1, None);
    assert_eq!(result1.errors.len(), 1);

    let code2 = String::from(
      "function add(x, y) return x + y end\n\
           local f = add\n\
           (f :: any)['x'] = 2",
    );
    let message2 = String::from(
      "Ambiguous syntax: this looks like an argument list for a function call, but could also be a start of new statement; use ';' to separate statements",
    );
    let result2 = fix.match_parse_error(&code2, &message2, None);
    assert_eq!(result2.errors.len(), 1);

    let code3 = String::from(
      "local x = {}\n\
           function x:add(a, b) return a + b end\n\
           x:add\n\
           (1, 2)",
    );
    let message3 = String::from(
      "Ambiguous syntax: this looks like an argument list for a function call, but could also be a start of new statement; use ';' to separate statements",
    );
    let result3 = fix.match_parse_error(&code3, &message3, None);
    assert_eq!(result3.errors.len(), 1);

    let code4 = String::from(
      "local t = {}\n\
           function f() return t end\n\
           t.x, (f)\n\
           ().y = 5, 6",
    );
    let message4 = String::from(
      "Ambiguous syntax: this looks like an argument list for a function call, but could also be a start of new statement; use ';' to separate statements",
    );
    let result4 = fix.match_parse_error(&code4, &message4, None);
    assert_eq!(result4.errors.len(), 1);
  }
}

mod parser_parse_error_function_call {

  #[cfg(test)]
  #[test]
  fn parser_parse_error_function_call() {
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    fixture.match_parse_error(
      &String::from("function stringifyTable(t)\n    local foo = t:Parse 2\n    return foo\nend"),
      &String::from("Expected '(', '{' or <string> when parsing function call, got '2'"),
      None,
    );
  }
}

mod parser_parse_error_function_call_newline {

  #[cfg(test)]
  #[test]
  fn parser_parse_error_function_call_newline() {
    use ulua_ast::records::parse_options::ParseOptions;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fix = Fixture::default();
    let code = String::from(
      "\nfunction stringifyTable(t)\n    local foo = t:Parse\n    return foo\nend\n        ",
    );
    let result = fix.try_parse(&code, &ParseOptions::new());

    assert_eq!(result.errors.len(), 1);
    let first_error = &result.errors[0];
    assert_eq!(first_error.get_location().begin.line, 2);
    assert_eq!(
      first_error.get_message(),
      "Expected function call arguments after '('"
    );
  }
}

mod parser_parse_error_loop_control {

  #[cfg(test)]
  #[test]
  fn parser_parse_error_loop_control() {
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    fixture.match_parse_error(
      &String::from("break"),
      &String::from("break statement must be inside a loop"),
      None,
    );
    fixture.match_parse_error(
      &String::from("repeat local function a() break end until false"),
      &String::from("break statement must be inside a loop"),
      None,
    );
    fixture.match_parse_error(
      &String::from("continue"),
      &String::from("continue statement must be inside a loop"),
      None,
    );
    fixture.match_parse_error(
      &String::from("repeat local function a() continue end until false"),
      &String::from("continue statement must be inside a loop"),
      None,
    );
  }
}

mod parser_parse_error_messages {

  #[cfg(test)]
  #[test]
  fn parser_parse_error_messages() {
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    fixture.match_parse_error(
      &String::from("\n        local a: (number, number) -> (string\n    "),
      &String::from("Expected ')' (to close '(' at line 2), got <eof>"),
      None,
    );
    fixture.match_parse_error(
      &String::from("\n        local a: (number, number) -> (\n            string\n    "),
      &String::from("Expected ')' (to close '(' at line 2), got <eof>"),
      None,
    );
    fixture.match_parse_error(
      &String::from("\n        local a: (number, number)\n    "),
      &String::from("Expected '->' when parsing function type, got <eof>"),
      None,
    );
    fixture.match_parse_error(
      &String::from("\n        local a: (number, number\n    "),
      &String::from("Expected ')' (to close '(' at line 2), got <eof>"),
      None,
    );
    fixture.match_parse_error(
      &String::from("\n        local a: {foo: string,\n    "),
      &String::from("Expected identifier when parsing table field, got <eof>"),
      None,
    );
    fixture.match_parse_error(
      &String::from("\n        local a: {foo: string\n    "),
      &String::from("Expected '}' (to close '{' at line 2), got <eof>"),
      None,
    );
    fixture.match_parse_error(
      &String::from("\n        local a: { [string]: number, [number]: string }\n    "),
      &String::from("Cannot have more than one table indexer"),
      None,
    );
    fixture.match_parse_error(
      &String::from("\n        type T = <a>foo\n    "),
      &String::from("Expected '(' when parsing function parameters, got 'foo'"),
      None,
    );
  }
}

mod parser_parse_error_missing_type_annotation {

  #[cfg(test)]
  #[test]
  fn parser_parse_error_missing_type_annotation() {
    use ulua_ast::records::parse_options::ParseOptions;
    use ulua_unit_test::records::fixture::Fixture;

    {
      let mut fix = Fixture::default();
      let code = String::from("local x:");
      let result = fix.try_parse(&code, &ParseOptions::new());

      assert_eq!(result.errors.len(), 1);
      let begin = result.errors[0].get_location().begin;
      let end = result.errors[0].get_location().end;
      assert_eq!(begin.line, end.line);
      let width = end.column - begin.column;
      assert_eq!(width, 0);
      assert_eq!(result.errors[0].get_message(), "Expected type, got <eof>");
    }

    {
      let mut fix = Fixture::default();
      let code = String::from("local x:=42");
      let result = fix.try_parse(&code, &ParseOptions::new());

      assert_eq!(result.errors.len(), 1);
      let begin = result.errors[0].get_location().begin;
      let end = result.errors[0].get_location().end;
      assert_eq!(begin.line, end.line);
      let width = end.column - begin.column;
      assert_eq!(width, 1);
      assert_eq!(result.errors[0].get_message(), "Expected type, got '='");
    }

    {
      let mut fix = Fixture::default();
      let code = String::from("function func():end");
      let result = fix.try_parse(&code, &ParseOptions::new());

      assert_eq!(result.errors.len(), 1);
      let begin = result.errors[0].get_location().begin;
      let end = result.errors[0].get_location().end;
      assert_eq!(begin.line, end.line);
      let width = end.column - begin.column;
      assert_eq!(width, 3);
      assert_eq!(result.errors[0].get_message(), "Expected type, got 'end'");
    }
  }
}

mod parser_parse_error_table_literal {

  #[cfg(test)]
  #[test]
  fn parser_parse_error_table_literal() {
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    fixture.match_parse_error(
          &String::from(
              "function stringifyTable(t)\n    local foo = (name = t)\n    return foo\nend\n",
          ),
          &String::from(
              "Expected ')' (to close '(' at column 17), got '='; did you mean to use '{' when defining a table?",
          ),
          None,
      );
  }
}

mod parser_parse_error_type_annotation {

  #[cfg(test)]
  #[test]
  fn parser_parse_error_type_annotation() {
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    fixture.match_parse_error(
      &String::from("local a : 2 = 2"),
      &String::from("Expected type, got '2'"),
      None,
    );
  }
}

mod parser_parse_error_type_name {

  #[cfg(test)]
  #[test]
  fn parser_parse_error_type_name() {
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    fixture.match_parse_error(
      &String::from("local a: Foo.=\n"),
      &String::from("Expected identifier when parsing field name, got '='"),
      None,
    );
  }
}

mod parser_parse_error_varargs {

  #[cfg(test)]
  #[test]
  fn parser_parse_error_varargs() {
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    fixture.match_parse_error(
      &String::from("function add(x, y) return ... end"),
      &String::from("Cannot use '...' outside of a vararg function"),
      None,
    );
  }
}

mod parser_parse_error_with_too_many_changed_elseif_statements {

  #[cfg(test)]
  #[test]
  fn parser_parse_error_with_too_many_changed_elseif_statements() {
    use ulua_common::FInt;
    use ulua_unit_test::{records::fixture::Fixture, type_aliases::scoped_fast_int::ScopedFastInt};

    let _sfi = ScopedFastInt::new(&FInt::LuauRecursionLimit, 10);

    let mut fix = Fixture::default();
    fix.match_parse_error_prefix(
      &String::from(
        "function f() if false then elseif false then elseif false then elseif false then \
               elseif false then elseif false then elseif false then elseif false then \
               elseif false then elseif false then elseif false then end end",
      ),
      &String::from("Exceeded allowed recursion depth;"),
    );
  }
}

mod parser_parse_error_with_too_many_nested_if_statements {

  #[cfg(test)]
  #[test]
  fn parser_parse_error_with_too_many_nested_if_statements() {
    use ulua_common::FInt;
    use ulua_unit_test::{records::fixture::Fixture, type_aliases::scoped_fast_int::ScopedFastInt};

    let _sfi = ScopedFastInt::new(&FInt::LuauRecursionLimit, 10);

    let mut fix = Fixture::default();
    fix.match_parse_error_prefix(
      &String::from(
        "function f() if true then if true then if true then if true then if true then \
               if true then if true then if true then if true then if true then if true then \
               end end end end end end end end end end end end",
      ),
      &String::from("Exceeded allowed recursion depth;"),
    );
  }
}

mod parser_parse_error_with_too_many_nested_ifelse_expressions1 {

  #[cfg(test)]
  #[test]
  fn parser_parse_error_with_too_many_nested_ifelse_expressions1() {
    use ulua_common::FInt;
    use ulua_unit_test::{records::fixture::Fixture, type_aliases::scoped_fast_int::ScopedFastInt};

    let mut fix = Fixture::fixture_bool(false);
    let _sfis = ScopedFastInt::new(&FInt::LuauRecursionLimit, 10);

    let source = "function f() return if true then 1 elseif true then 2 elseif true then 3 elseif true then 4 elseif true then 5 elseif true then 6 elseif true then 7 elseif true then 8 elseif true then 9 elseif true then 10 else 11 end";
    let message =
      "Exceeded allowed recursion depth; simplify your expression to make the code compile";
    fix.match_parse_error(source, message, None);
  }
}

mod parser_parse_error_with_too_many_nested_ifelse_expressions2 {

  #[cfg(test)]
  #[test]
  fn parser_parse_error_with_too_many_nested_ifelse_expressions2() {
    use ulua_common::FInt;
    use ulua_unit_test::{records::fixture::Fixture, type_aliases::scoped_fast_int::ScopedFastInt};

    let mut fix = Fixture::fixture_bool(false);
    let _scoped = ScopedFastInt::new(&FInt::LuauRecursionLimit, 10);

    let source = "function f() return if if if if if if if if if if true then false else true then false else true then false else true then false else true then false else true then false else true then false else true then false else true then false else true then 1 else 2 end";
    let message =
      "Exceeded allowed recursion depth; simplify your expression to make the code compile";

    fix.match_parse_error(source, message, None);
  }
}

mod parser_parse_error_with_too_many_nested_type_group {

  #[cfg(test)]
  #[test]
  fn parser_parse_error_with_too_many_nested_type_group() {
    use ulua_common::FInt;
    use ulua_unit_test::{records::fixture::Fixture, type_aliases::scoped_fast_int::ScopedFastInt};

    let mut fixture = Fixture::fixture_bool(false);

    // Set recursion limit to 10 for all tests in this fixture
    let _sfis = ScopedFastInt::new(&FInt::LuauRecursionLimit, 10);

    // Test 1: Too many nested parentheses in function return type
    fixture.match_parse_error(
      "function f(): ((((((((((Fail)))))))))) end",
      "Exceeded allowed recursion depth; simplify your type annotation to make the code compile",
      None,
    );

    // Test 2: Too many nested function arrow types
    fixture.match_parse_error(
      "function f(): () -> () -> () -> () -> () -> () -> () -> () -> () -> () -> () end",
      "Exceeded allowed recursion depth; simplify your type annotation to make the code compile",
      None,
    );

    // Test 3: Too many nested table types
    fixture.match_parse_error(
      "local t: {a: {b: {c: {d: {e: {f: {g: {h: {i: {j: {}}}}}}}}}}}",
      "Exceeded allowed recursion depth; simplify your type annotation to make the code compile",
      None,
    );

    // Test 4: Too many nested parentheses in type annotation
    fixture.match_parse_error(
      "local f: ((((((((((Fail))))))))))",
      "Exceeded allowed recursion depth; simplify your type annotation to make the code compile",
      None,
    );

    // Test 5: Too many nested intersection types
    fixture.match_parse_error(
      "local t: a & (b & (c & (d & (e & (f & (g & (h & (i & (j & nil)))))))))",
      "Exceeded allowed recursion depth; simplify your type annotation to make the code compile",
      None,
    );
  }
}

mod parser_parse_export_type {
  use super::*;
  #[cfg(test)]
  #[test]
  fn parser_parse_export_type() {
    use ulua_ast::records::{
      ast_stat_assign::AstStatAssign, ast_stat_block::AstStatBlock, ast_stat_expr::AstStatExpr,
      ast_stat_type_alias::AstStatTypeAlias, parse_options::ParseOptions,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    let stat = fixture.parse(
      "export()\n\
           export = 5\n\
           export, export = export\n\
           export type A = number\n\
           type A = number",
      &ParseOptions::default(),
    );
    assert!(!stat.is_null());

    let block_ptr =
      unsafe { rtti::ast_node_as::<AstStatBlock>(stat as *mut _ as *mut ast_node::AstNode) };
    assert!(!block_ptr.is_null());
    let block = unsafe { &*block_ptr };
    assert_eq!(5, block.body.size);

    assert!(
      !unsafe {
        rtti::ast_node_as::<AstStatExpr>(*block.body.data.add(0) as *mut ast_node::AstNode)
      }
      .is_null()
    );
    assert!(
      !unsafe {
        rtti::ast_node_as::<AstStatAssign>(*block.body.data.add(1) as *mut ast_node::AstNode)
      }
      .is_null()
    );
    assert!(
      !unsafe {
        rtti::ast_node_as::<AstStatAssign>(*block.body.data.add(2) as *mut ast_node::AstNode)
      }
      .is_null()
    );
    assert!(
      !unsafe {
        rtti::ast_node_as::<AstStatTypeAlias>(*block.body.data.add(3) as *mut ast_node::AstNode)
      }
      .is_null()
    );
    assert!(
      !unsafe {
        rtti::ast_node_as::<AstStatTypeAlias>(*block.body.data.add(4) as *mut ast_node::AstNode)
      }
      .is_null()
    );
  }
}

mod parser_parse_extern_type_declarations {
  use ulua_ast::records::ast_stat::AstStat;

  use super::*;
  #[cfg(test)]
  #[test]
  fn parser_parse_extern_type_declarations() {
    use ulua_ast::records::{
      ast_node::AstNode, ast_stat_block::AstStatBlock,
      ast_stat_declare_extern_type::AstStatDeclareExternType, ast_type_function::AstTypeFunction,
      ast_type_reference::AstTypeReference, location::Location, parse_options::ParseOptions,
      parse_result::ParseResult,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    let source = String::from(
      r#"
        declare extern type Foo with
            prop: number
            function method(self, foo: number): string
        end

        declare extern type Bar extends Foo with
            prop2: string
        end
    "#,
    );
    let result: ParseResult = fixture.parse_ex(&source, &ParseOptions::default());
    let root: *mut AstStatBlock = result.root;

    assert!(!root.is_null());
    let root_ref = unsafe { &*root };
    assert_eq!(2, root_ref.body.size);

    let first_stat: *mut AstStat = unsafe { *root_ref.body.data.add(0) };
    let first_stat_node: *mut AstNode = first_stat as *mut AstNode;
    let declared_extern_type: *mut AstStatDeclareExternType =
      unsafe { AstNode::as_item_mut::<AstStatDeclareExternType>(&mut *first_stat_node) };
    assert!(!declared_extern_type.is_null());

    let det = unsafe { &*declared_extern_type };
    assert_eq!("Foo", unsafe {
      CStr::from_ptr(det.name.value).to_string_lossy()
    });
    assert!(det.super_name.is_none());

    assert_eq!(2, det.props.size);

    let prop = unsafe { *det.props.data.add(0) };
    assert_eq!("prop", unsafe {
      CStr::from_ptr(prop.name.value).to_string_lossy()
    });
    assert_eq!(
      Location::new(UluaAstPosition::new(2, 12), UluaAstPosition::new(2, 16)),
      prop.name_location
    );
    assert!(unsafe {
      !rtti::ast_node_as::<AstTypeReference>(prop.ty as *mut ast_node::AstNode).is_null()
    });
    assert_eq!(
      Location::new(UluaAstPosition::new(2, 12), UluaAstPosition::new(2, 24)),
      prop.location
    );

    let method = unsafe { *det.props.data.add(1) };
    assert_eq!("method", unsafe {
      CStr::from_ptr(method.name.value).to_string_lossy()
    });
    assert_eq!(
      Location::new(UluaAstPosition::new(3, 21), UluaAstPosition::new(3, 27)),
      method.name_location
    );
    assert!(unsafe {
      !rtti::ast_node_as::<AstTypeFunction>(method.ty as *mut ast_node::AstNode).is_null()
    });
    assert_eq!(
      Location::new(UluaAstPosition::new(3, 12), UluaAstPosition::new(3, 54)),
      method.location
    );
    assert!(method.is_method);

    let second_stat: *mut AstStat = unsafe { *root_ref.body.data.add(1) };
    let second_stat_node: *mut AstNode = second_stat as *mut AstNode;
    let subclass: *mut AstStatDeclareExternType =
      unsafe { AstNode::as_item_mut::<AstStatDeclareExternType>(&mut *second_stat_node) };
    assert!(!subclass.is_null());

    let sub = unsafe { &*subclass };
    assert_eq!("Bar", unsafe {
      CStr::from_ptr(sub.name.value).to_string_lossy()
    });
    assert!(sub.super_name.is_some());
    assert_eq!("Foo", unsafe {
      CStr::from_ptr(sub.super_name.unwrap().value).to_string_lossy()
    });

    assert_eq!(1, sub.props.size);

    let prop2 = unsafe { *sub.props.data.add(0) };
    assert_eq!("prop2", unsafe {
      CStr::from_ptr(prop2.name.value).to_string_lossy()
    });
    assert_eq!(
      Location::new(UluaAstPosition::new(7, 12), UluaAstPosition::new(7, 17)),
      prop2.name_location
    );
    assert!(unsafe {
      !rtti::ast_node_as::<AstTypeReference>(prop2.ty as *mut ast_node::AstNode).is_null()
    });
    assert_eq!(
      Location::new(UluaAstPosition::new(7, 12), UluaAstPosition::new(7, 25)),
      prop2.location
    );
  }
}

mod parser_parse_extern_type_declarations_missing_with {
  use super::*;
  #[cfg(test)]
  #[test]
  fn parser_parse_extern_type_declarations_missing_with() {
    use ulua_ast::records::{location::Location, parse_options::ParseOptions, position::Position};
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let source = String::from(
      r#"
        declare extern type Foo
            prop: number
            function method(self, foo: number): string
        end

        declare extern type Bar extends Foo
            prop2: string
        end
    "#,
    );
    let options = ParseOptions::new();
    let result = fixture.try_parse(&source, &options);

    assert_eq!(result.errors.len(), 2);

    assert_eq!(
      "Expected `with` keyword before listing properties of the external type, but got prop instead",
      result.errors[0].get_message()
    );
    assert_eq!(
      "Expected `with` keyword before listing properties of the external type, but got prop2 instead",
      result.errors[1].get_message()
    );

    let stat = unsafe { &*result.root };
    assert_eq!(stat.body.size, 2);

    let declared_extern_type = unsafe {
      rtti::ast_node_as::<ast_stat_declare_extern_type::AstStatDeclareExternType>(
        *stat.body.data.add(0) as *mut ast_node::AstNode,
      )
    };
    assert!(!declared_extern_type.is_null());
    assert_eq!(
      unsafe { CStr::from_ptr((*declared_extern_type).name.value) },
      c"Foo"
    );
    assert!(unsafe { (*declared_extern_type).super_name }.is_none());

    assert_eq!(unsafe { (*declared_extern_type).props.size }, 2);

    let prop = unsafe { *(*declared_extern_type).props.data.add(0) };
    assert_eq!(unsafe { CStr::from_ptr(prop.name.value) }, c"prop");
    assert_eq!(
      prop.name_location,
      Location::new(Position::new(2, 12), Position::new(2, 16))
    );
    assert!(
      !unsafe {
        rtti::ast_node_as::<ast_type_reference::AstTypeReference>(prop.ty as *mut ast_node::AstNode)
      }
      .is_null()
    );
    assert_eq!(
      prop.location,
      Location::new(Position::new(2, 12), Position::new(2, 24))
    );

    let method = unsafe { *(*declared_extern_type).props.data.add(1) };
    assert_eq!(unsafe { CStr::from_ptr(method.name.value) }, c"method");
    assert_eq!(
      method.name_location,
      Location::new(Position::new(3, 21), Position::new(3, 27))
    );
    assert!(
      !unsafe {
        rtti::ast_node_as::<ast_type_function::AstTypeFunction>(method.ty as *mut ast_node::AstNode)
      }
      .is_null()
    );
    assert_eq!(
      method.location,
      Location::new(Position::new(3, 12), Position::new(3, 54))
    );
    assert!(method.is_method);

    let subclass = unsafe {
      rtti::ast_node_as::<ast_stat_declare_extern_type::AstStatDeclareExternType>(
        *stat.body.data.add(1) as *mut ast_node::AstNode,
      )
    };
    assert!(!subclass.is_null());
    assert!(unsafe { (*subclass).super_name }.is_some());
    assert_eq!(
      unsafe { CStr::from_ptr((*subclass).super_name.unwrap().value) },
      c"Foo"
    );
    assert_eq!(unsafe { CStr::from_ptr((*subclass).name.value) }, c"Bar");

    assert_eq!(unsafe { (*subclass).props.size }, 1);
    let prop2 = unsafe { *(*subclass).props.data.add(0) };
    assert_eq!(unsafe { CStr::from_ptr(prop2.name.value) }, c"prop2");
    assert_eq!(
      prop2.name_location,
      Location::new(Position::new(7, 12), Position::new(7, 17))
    );
    assert!(
      !unsafe {
        rtti::ast_node_as::<ast_type_reference::AstTypeReference>(
          prop2.ty as *mut ast_node::AstNode,
        )
      }
      .is_null()
    );
    assert_eq!(
      prop2.location,
      Location::new(Position::new(7, 12), Position::new(7, 25))
    );
  }
}

mod parser_parse_global_declaration_called_class {
  use super::*;
  #[cfg(test)]
  #[test]
  fn parser_parse_global_declaration_called_class() {
    use ulua_ast::records::{
      ast_stat_declare_global::AstStatDeclareGlobal, ast_type_table::AstTypeTable,
      parse_options::ParseOptions,
    };
    use ulua_common::FFlag::LuauAllowGlobalDeclarationToBeCalledClass;
    use ulua_unit_test::records::fixture::Fixture;

    LuauAllowGlobalDeclarationToBeCalledClass.set(true);

    let mut fixture = Fixture::default();
    let result = fixture.parse_ex(
      &String::from("declare class: { x: number }"),
      &ParseOptions::default(),
    );
    let root = unsafe { &*result.root };

    assert!(root.body.size > 0);
    assert_eq!(root.body.size, 1);

    let global = unsafe { &*root.body.data.add(0) };
    assert!(rtti::ast_node_is::<AstStatDeclareGlobal>(
      *global as *mut ast_node::AstNode
    ));

    // `global` is `&(*mut AstStat)` (a ref to the slot); RTTI-cast `*global`, not the
    // ref's own address (which made ast_node_as return null -> null deref on `&*`).
    let global =
      unsafe { &*rtti::ast_node_as::<AstStatDeclareGlobal>(*global as *mut ast_node::AstNode) };
    // C++ `CHECK(global->name == "class")` — the global IS named "class"; the port
    // wrongly asserted a null/default name.
    assert_eq!(unsafe { CStr::from_ptr(global.name.value) }, c"class");

    assert!(!global.type_.is_null());
    assert!(rtti::ast_node_is::<AstTypeTable>(unsafe { &*global.type_ }));
  }
}

mod parser_parse_if_else_expression {
  use super::*;
  #[cfg(test)]
  #[test]
  fn parser_parse_if_else_expression() {
    use ulua_ast::records::{
      ast_expr_if_else::AstExprIfElse, ast_stat_block::AstStatBlock,
      ast_stat_return::AstStatReturn, parse_options::ParseOptions,
    };
    use ulua_unit_test::records::fixture::Fixture;

    {
      let mut fixture = Fixture::default();
      let stat = fixture.parse("return if true then 1 else 2", &ParseOptions::default());
      assert!(!stat.is_null());
      let block = unsafe {
        rtti::ast_node_as::<AstStatBlock>(
          core::ptr::addr_of_mut!((*stat).base) as *mut ast_node::AstNode
        )
      };
      assert!(!block.is_null());
      assert!((unsafe { (*block).body.size }) > 0);
      let return_stat = unsafe { (*block).body.data.add(0).read() };
      let return_stat =
        unsafe { rtti::ast_node_as::<AstStatReturn>(core::ptr::addr_of_mut!((*return_stat).base)) };
      assert!(!return_stat.is_null());
      assert!((unsafe { (*return_stat).list.size }) == 1);
      let expr = unsafe { (*return_stat).list.data.add(0).read() };
      let if_else_expr =
        unsafe { rtti::ast_node_as::<AstExprIfElse>(core::ptr::addr_of_mut!((*expr).base)) };
      assert!(!if_else_expr.is_null());
    }

    {
      let mut fixture = Fixture::default();
      let stat = fixture.parse(
        "return if true then 1 elseif true then 2 else 3",
        &ParseOptions::default(),
      );
      assert!(!stat.is_null());
      let block = unsafe {
        rtti::ast_node_as::<AstStatBlock>(
          core::ptr::addr_of_mut!((*stat).base) as *mut ast_node::AstNode
        )
      };
      assert!(!block.is_null());
      assert!((unsafe { (*block).body.size }) > 0);
      let return_stat = unsafe { (*block).body.data.add(0).read() };
      let return_stat =
        unsafe { rtti::ast_node_as::<AstStatReturn>(core::ptr::addr_of_mut!((*return_stat).base)) };
      assert!(!return_stat.is_null());
      assert!((unsafe { (*return_stat).list.size }) == 1);
      let expr = unsafe { (*return_stat).list.data.add(0).read() };
      let if_else_expr1 =
        unsafe { rtti::ast_node_as::<AstExprIfElse>(core::ptr::addr_of_mut!((*expr).base)) };
      assert!(!if_else_expr1.is_null());
      let false_expr = unsafe { (*if_else_expr1).false_expr };
      let if_else_expr2 =
        unsafe { rtti::ast_node_as::<AstExprIfElse>(core::ptr::addr_of_mut!((*false_expr).base)) };
      assert!(!if_else_expr2.is_null());
    }

    {
      let mut fixture = Fixture::default();
      let stat = fixture.parse(
        "return if true then 1 else if true then 2 else 3",
        &ParseOptions::default(),
      );
      assert!(!stat.is_null());
      let block = unsafe {
        rtti::ast_node_as::<AstStatBlock>(
          core::ptr::addr_of_mut!((*stat).base) as *mut ast_node::AstNode
        )
      };
      assert!(!block.is_null());
      assert!((unsafe { (*block).body.size }) > 0);
      let return_stat = unsafe { (*block).body.data.add(0).read() };
      let return_stat =
        unsafe { rtti::ast_node_as::<AstStatReturn>(core::ptr::addr_of_mut!((*return_stat).base)) };
      assert!(!return_stat.is_null());
      assert!((unsafe { (*return_stat).list.size }) == 1);
      let expr = unsafe { (*return_stat).list.data.add(0).read() };
      let if_else_expr1 =
        unsafe { rtti::ast_node_as::<AstExprIfElse>(core::ptr::addr_of_mut!((*expr).base)) };
      assert!(!if_else_expr1.is_null());
      let false_expr = unsafe { (*if_else_expr1).false_expr };
      let if_else_expr2 =
        unsafe { rtti::ast_node_as::<AstExprIfElse>(core::ptr::addr_of_mut!((*false_expr).base)) };
      assert!(!if_else_expr2.is_null());
    }

    {
      let mut fixture = Fixture::default();
      let stat = fixture.parse(
        "return if if true then false else true then 1 else 2",
        &ParseOptions::default(),
      );
      assert!(!stat.is_null());
      let block = unsafe {
        rtti::ast_node_as::<AstStatBlock>(
          core::ptr::addr_of_mut!((*stat).base) as *mut ast_node::AstNode
        )
      };
      assert!(!block.is_null());
      assert!((unsafe { (*block).body.size }) > 0);
      let return_stat = unsafe { (*block).body.data.add(0).read() };
      let return_stat =
        unsafe { rtti::ast_node_as::<AstStatReturn>(core::ptr::addr_of_mut!((*return_stat).base)) };
      assert!(!return_stat.is_null());
      assert!((unsafe { (*return_stat).list.size }) == 1);
      let expr = unsafe { (*return_stat).list.data.add(0).read() };
      let if_else_expr =
        unsafe { rtti::ast_node_as::<AstExprIfElse>(core::ptr::addr_of_mut!((*expr).base)) };
      assert!(!if_else_expr.is_null());
      let condition = unsafe { (*if_else_expr).condition };
      let nested_if_else_expr =
        unsafe { rtti::ast_node_as::<AstExprIfElse>(core::ptr::addr_of_mut!((*condition).base)) };
      assert!(!nested_if_else_expr.is_null());
    }
  }
}

mod parser_parse_interpolated_string_as_type_fail {

  #[cfg(test)]
  #[test]
  fn parser_parse_interpolated_string_as_type_fail() {
    use ulua_ast::records::parse_options::ParseOptions;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    let parse_options = ParseOptions::default();

    let source = String::from(
      "\n            local a: `what` = `???`\n            local b: `what {\"the\"}` = `???`\n            local c: `what {\"the\"} heck` = `???`\n        ",
    );

    // C++ catches ParseErrors and checks getErrors().size() == 3, each with the same
    // message. The port wrapped fixture.parse (which panics with a "ParseErrors: ..."
    // prefix) and only checked one message. Use try_parse and inspect all 3 errors.
    let result = fixture.try_parse(&source, &parse_options);

    assert_eq!(3, result.errors.len());
    for error in result.errors.iter() {
      assert_eq!(
        "Interpolated string literals cannot be used as types",
        error.get_message().as_str()
      );
    }
  }
}

mod parser_parse_interpolated_string_call_without_parens {

  #[cfg(test)]
  #[test]
  fn parser_parse_interpolated_string_call_without_parens() {
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    let source = String::from("_ = print `{42}`");
    let expected = String::from("Expected identifier when parsing expression, got `{");

    fixture.match_parse_error(&source, &expected, None);
  }
}

mod parser_parse_interpolated_string_double_brace_begin {

  use ulua_ast::records::parse_options::ParseOptions;
  #[cfg(test)]
  #[test]
  fn parser_parse_interpolated_string_double_brace_begin() {
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::default();

    let source = String::from("\n            _ = `{{oops}}`\n        ");
    let parse_options = ParseOptions::default();
    // C++ catches the thrown ParseErrors. `fixture.parse` PANICS on errors (it
    // emulates the throw), so don't call it — use try_parse and assert the error,
    // which `errors.first().unwrap()` below already enforces.
    let parse_result = fixture.try_parse(&source, &parse_options);
    let errors = parse_result.errors;
    let first_error = errors.first().unwrap();
    let message = first_error.get_message();

    let expected_message =
      "Double braces are not permitted within interpolated strings; did you mean '\\{'?";
    assert_eq!(message.as_str(), expected_message);
  }
}

mod parser_parse_interpolated_string_double_brace_mid {

  use ulua_ast::records::parse_options::ParseOptions;
  #[cfg(test)]
  #[test]
  fn parser_parse_interpolated_string_double_brace_mid() {
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::default();

    let source = String::from("\n            _ = `{nice} {{oops}}`\n        ");

    let result = fixture.try_parse(&source, &ParseOptions::default());

    let errors = result.errors;

    if errors.is_empty() {
      panic!("Expected ParseErrors to be thrown");
    }

    let first_error = errors.first().unwrap();
    let expected_msg =
      "Double braces are not permitted within interpolated strings; did you mean '\\{'?";
    let actual_msg = first_error.get_message();

    if actual_msg.as_str() != expected_msg {
      panic!(
        "Expected error message '{}' but got '{}'",
        expected_msg, actual_msg
      );
    }
  }
}

mod parser_parse_interpolated_string_malformed_escape {

  #[cfg(test)]
  #[test]
  fn parser_parse_interpolated_string_malformed_escape() {
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    let expected = "Interpolated string literal contains malformed escape sequence";

    fixture.match_parse_error(
      &String::from("local a = `???\\xQQ {1}`"),
      &String::from(expected),
      None,
    );
  }
}

mod parser_parse_interpolated_string_mid_without_end_brace_in_table {

  use ulua_ast::records::parse_options::ParseOptions;
  #[cfg(test)]
  #[test]
  fn parser_parse_interpolated_string_mid_without_end_brace_in_table() {
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let source = String::from("\n            _ = { `x {\"y\"} {z` }\n        ");

    let result = fixture.try_parse(&source, &ParseOptions::new());

    let errors = result.errors;
    assert_eq!(2, errors.len());

    let first_error = &errors[0];
    assert_eq!(
      "Malformed interpolated string; did you forget to add a '}'?",
      first_error.get_message().as_str()
    );

    let last_error = &errors[errors.len() - 1];
    assert_eq!(
      "Expected '}' (to close '{' at line 2), got <eof>",
      last_error.get_message().as_str()
    );
  }
}

mod parser_parse_interpolated_string_weird_token {

  #[cfg(test)]
  #[test]
  fn parser_parse_interpolated_string_weird_token() {
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    fixture.match_parse_error(
      &String::from("\n            local a = `??? {42 !!}`\n        "),
      // C++ expects no trailing period; the port added one.
      &String::from("Malformed interpolated string, got '!'"),
      None,
    );
  }
}

mod parser_parse_interpolated_string_with_lookahead_involved {

  #[cfg(test)]
  #[test]
  fn parser_parse_interpolated_string_with_lookahead_involved() {
    use ulua_ast::records::parse_options::ParseOptions;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let source = String::from("local x = `{ {y} }`");
    let options = ParseOptions::new();
    let result = fixture.try_parse(&source, &options);
    assert!(result.errors.is_empty());
  }
}

mod parser_parse_interpolated_string_with_lookahead_involved2 {

  #[cfg(test)]
  #[test]
  fn parser_parse_interpolated_string_with_lookahead_involved2() {
    use ulua_ast::records::parse_options::ParseOptions;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let source = String::from("local x = `{ { y{} } }`");
    let options = ParseOptions::new();
    let result = fixture.try_parse(&source, &options);
    assert!(result.errors.is_empty());
  }
}

mod parser_parse_interpolated_string_without_end_brace {

  #[cfg(test)]
  #[test]
  fn parser_parse_interpolated_string_without_end_brace() {
    use ulua_ast::records::{parse_options::ParseOptions, parse_result::ParseResult};
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let source = String::from("\n_ = `{a`");
    let options = ParseOptions::new();
    let result: ParseResult = fixture.try_parse(&source, &options);

    assert_eq!(1, result.errors.len());

    let error = &result.errors[0];
    assert_eq!(
      "Malformed interpolated string; did you forget to add a '}'?",
      error.get_message().as_str()
    );

    // C++ only checks begin.column (the column of the closing brace), not the full
    // span — the parser's end column is one past, which is correct.
    assert_eq!(7, error.get_location().begin.column);

    let source2 = String::from("\n_ = `{abcdefg`");
    let result2: ParseResult = fixture.try_parse(&source2, &options);
    assert_eq!(1, result2.errors.len());
    let error2 = &result2.errors[0];
    assert_eq!(
      "Malformed interpolated string; did you forget to add a '}'?",
      error2.get_message().as_str()
    );

    assert_eq!(13, error2.get_location().begin.column);

    let source3 = String::from("\n_ =       `{a`");
    let result3: ParseResult = fixture.try_parse(&source3, &options);
    assert_eq!(1, result3.errors.len());
    let error3 = &result3.errors[0];
    assert_eq!(
      "Malformed interpolated string; did you forget to add a '}'?",
      error3.get_message().as_str()
    );

    // C++: columnOfEndBraceError("_ =       `{a`") == columnOfEndBraceError("_ = `{abcdefg`")
    assert_eq!(13, error3.get_location().begin.column);
  }
}

mod parser_parse_interpolated_string_without_end_brace_in_table {

  use ulua_ast::records::parse_options::ParseOptions;
  #[cfg(test)]
  #[test]
  fn parser_parse_interpolated_string_without_end_brace_in_table() {
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let source = String::from("\n            _ = { `{a` }\n        ");

    let result = fixture.try_parse(&source, &ParseOptions::new());

    let errors = result.errors;
    assert_eq!(2, errors.len());

    let first_error = &errors[0];
    assert_eq!(
      "Malformed interpolated string; did you forget to add a '}'?",
      first_error.get_message().as_str()
    );

    let last_error = &errors[1];
    assert_eq!(
      "Expected '}' (to close '{' at line 2), got <eof>",
      last_error.get_message().as_str()
    );
  }
}

mod parser_parse_interpolated_string_without_expression {
  use ulua_ast::records::parse_options::ParseOptions;

  use super::*;
  #[cfg(test)]
  #[test]
  fn parser_parse_interpolated_string_without_expression() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);

    // C++ catches a thrown ParseErrors and checks the first message. try_parse does
    // error recovery and returns a (partial) non-null root, so gating on
    // root.is_null() is wrong — just assert the error directly.
    let source1 = String::from("print(`{}`)");
    let parse_result1 = fixture.try_parse(&source1, &ParseOptions::new());
    assert!(
      !parse_result1.errors.is_empty(),
      "Expected ParseErrors to be thrown"
    );
    assert_eq!(
      "Malformed interpolated string, expected expression inside '{}'",
      parse_result1.errors[0].get_message().as_str()
    );

    let source2 = String::from("print(`{}{1}`)");
    let parse_result2 = fixture.try_parse(&source2, &ParseOptions::new());
    assert!(
      !parse_result2.errors.is_empty(),
      "Expected ParseErrors to be thrown"
    );
    assert_eq!(
      "Malformed interpolated string, expected expression inside '{}'",
      parse_result2.errors[0].get_message().as_str()
    );
  }
}

mod parser_parse_local_const {

  #[cfg(test)]
  #[test]
  fn parser_parse_local_const() {
    use ulua_ast::records::parse_options::ParseOptions;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    let _stat = fixture.parse("local const", &ParseOptions::default());
    assert!(!_stat.is_null());
  }
}

mod parser_parse_nested_ast_type_group {
  use super::*;
  #[cfg(test)]
  #[test]
  fn parser_parse_nested_ast_type_group() {
    use ulua_ast::records::parse_options::ParseOptions;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    let stat = fixture.parse("type Foo = ((string))", &ParseOptions::default());
    let root = unsafe { &*stat };
    assert_eq!(1, root.body.size);

    let alias1 = unsafe { (*stat).body.data.add(0) };
    let alias1 = unsafe {
      rtti::ast_node_as::<ast_stat_type_alias::AstStatTypeAlias>(*alias1 as *mut ast_node::AstNode)
    };
    assert!(!alias1.is_null());

    let group1 = unsafe { (*alias1).type_ptr };
    let group1 = unsafe {
      rtti::ast_node_as::<ast_type_group::AstTypeGroup>(group1 as *mut ast_node::AstNode)
    };
    assert!(!group1.is_null());

    let group2 = unsafe { (*group1).type_ };
    let group2 = unsafe {
      rtti::ast_node_as::<ast_type_group::AstTypeGroup>(group2 as *mut ast_node::AstNode)
    };
    assert!(!group2.is_null());

    let ref_node = unsafe { (*group2).type_ };
    assert!(rtti::ast_node_is::<ast_type_reference::AstTypeReference>(
      ref_node as *mut ast_node::AstNode,
    ));
  }
}

mod parser_parse_nested_type_function {

  #[cfg(test)]
  #[test]
  fn parser_parse_nested_type_function() {
    use ulua_ast::records::parse_options::ParseOptions;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    let _stat = fixture.parse(
      r#"local v1 = 1
  type function foo()
      local v2 = 2
      local function bar()
          v2 += 1
          type function inner() end
          v2 += 2
      end
      local function bar2()
          v2 += 3
      end
  end
  local function bar() v1 += 1 end"#,
      &ParseOptions::default(),
    );
    assert!(!_stat.is_null());
  }
}

mod parser_parse_nesting_based_end_detection {

  use ulua_ast::records::parse_options::ParseOptions;
  #[cfg(test)]
  #[test]
  fn parser_parse_nesting_based_end_detection() {
    use ulua_unit_test::records::fixture::Fixture;

    let mut fix = Fixture::default();
    let code = String::from(
      "-- i am line 1
  function BottomUpTree(item, depth)
    if depth > 0 then
      local i = item + item
      depth = depth - 1
      local left, right = BottomUpTree(i-1, depth), BottomUpTree(i, depth)
      return { item, left, right }
    else
      return { item }
  end
  
  function ItemCheck(tree)
    if tree[2] then
      return tree[1] + ItemCheck(tree[2]) - ItemCheck(tree[3])
    else
      return tree[1]
    end
  end
  ",
    );
    let result = fix.try_parse(&code, &ParseOptions::new());

    ulua_common::LUAU_ASSERT!(result.errors.len() == 1);
    let expected_msg = "Expected 'end' (to close 'function' at line 2), got <eof>; did you forget to close 'else' at line 8?";
    assert_eq!(result.errors[0].get_message(), expected_msg);
  }
}

mod parser_parse_nesting_based_end_detection_failsafe_earlier {

  use ulua_ast::records::parse_options::ParseOptions;
  #[cfg(test)]
  #[test]
  fn parser_parse_nesting_based_end_detection_failsafe_earlier() {
    use ulua_unit_test::records::fixture::Fixture;

    let mut fix = Fixture::default();
    let code = String::from(
      r#"-- i am line 1
  local function ItemCheck(tree)
    if tree[2] then
      return tree[1] + ItemCheck(tree[2]) - ItemCheck(tree[3])
    else
      return tree[1]
        end
  end
  
  local function BottomUpTree(item, depth)
    if depth > 0 then
      local i = item + item
      depth = depth - 1
      local left, right = BottomUpTree(i-1, depth), BottomUpTree(i, depth)
      return { item, left, right }
    else
      return { item }
    end
  "#,
    );
    let result = fix.try_parse(&code, &ParseOptions::new());
    if result.errors.is_empty() {
      panic!("Expected ParseErrors to be thrown");
    }
    let first_error = &result.errors[0];
    assert_eq!(
      first_error.get_message().as_str(),
      "Expected 'end' (to close 'function' at line 10), got <eof>"
    );
  }
}

mod parser_parse_nesting_based_end_detection_local_function {

  #[cfg(test)]
  #[test]
  fn parser_parse_nesting_based_end_detection_local_function() {
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    fixture.match_parse_error(
          &String::from("-- i am line 1\nlocal function BottomUpTree(item, depth)\n  if depth > 0 then\n    local i = item + item\n    depth = depth - 1\n    local left, right = BottomUpTree(i-1, depth), BottomUpTree(i, depth)\n    return { item, left, right }\n  else\n    return { item }\nend\n\nlocal function ItemCheck(tree)\n  if tree[2] then\n    return tree[1] + ItemCheck(tree[2]) - ItemCheck(tree[3])\n  else\n    return tree[1]\n  end\nend\n        "),
          &String::from("Expected 'end' (to close 'function' at line 2), got <eof>; did you forget to close 'else' at line 8?"),
          None,
      );
  }
}

mod parser_parse_nesting_based_end_detection_local_repeat {

  use ulua_ast::records::parse_options::ParseOptions;
  #[cfg(test)]
  #[test]
  fn parser_parse_nesting_based_end_detection_local_repeat() {
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let source = String::from(
      "-- i am line 1
  repeat
    print(1)
    repeat
      print(2)
    print(3)
  until false
          ",
    );

    let result = fixture.try_parse(&source, &ParseOptions::new());

    assert_eq!(1, result.errors.len());

    let expected_message = "Expected 'until' (to close 'repeat' at line 2), got <eof>; did you forget to close 'repeat' at line 4?";
    assert_eq!(expected_message, result.errors[0].get_message().as_str());
  }
}

mod parser_parse_nesting_based_end_detection_nested {

  #[cfg(test)]
  #[test]
  fn parser_parse_nesting_based_end_detection_nested() {
    use ulua_ast::records::parse_options::ParseOptions;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fix = Fixture::default();
    let code = String::from(
      "-- i am line 1
  function stringifyTable(t)
      local entries = {}
      for k, v in pairs(t) do
          -- if we find a nested table, convert that recursively
          if type(v) == \"table\" then
              v = stringifyTable(v)
          else
              v = tostring(v)
          k = tostring(k)
  
          -- add another entry to our stringified table
          entries[#entries + 1] = (\"s = s\"):format(k, v)
      end
  
      -- the memory location of the table
      local id = tostring(t):sub(8)
  
      return (\"{s}@s\"):format(table.concat(entries, \", \"), id)
  end
  ",
    );

    let result = fix.try_parse(&code, &ParseOptions::new());

    assert_eq!(result.errors.len(), 1);
    let msg = result.errors[0].get_message();
    assert_eq!(
      msg,
      "Expected 'end' (to close 'function' at line 2), got <eof>; did you forget to close 'else' at line 8?"
    );
  }
}

mod parser_parse_nesting_based_end_detection_single_line {

  use ulua_ast::records::parse_options::ParseOptions;
  #[cfg(test)]
  #[test]
  fn parser_parse_nesting_based_end_detection_single_line() {
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    let source = String::from(
      "-- i am line 1\n\
           function ItemCheck(tree)\n\
             if tree[2] then return tree[1] + ItemCheck(tree[2]) - ItemCheck(tree[3]) else return tree[1]\n\
           end\n\
           \n\
           function BottomUpTree(item, depth)\n\
             if depth > 0 then\n\
               local i = item + item\n\
               depth = depth - 1\n\
               local left, right = BottomUpTree(i-1, depth), BottomUpTree(i, depth)\n\
               return { item, left, right }\n\
             else\n\
               return { item }\n\
             end\n\
           end\n\
           ",
    );
    let result = fixture.try_parse(&source, &ParseOptions::new());

    assert_eq!(result.errors.len(), 1);
    let error_message = result.errors[0].get_message();
    assert_eq!(
      error_message,
      "Expected 'end' (to close 'function' at line 2), got <eof>; did you forget to close 'else' at line 3?"
    );
  }
}

mod parser_parse_numbers_binary {
  use super::*;
  #[cfg(test)]
  #[test]
  fn parser_parse_numbers_binary() {
    use ulua_ast::records::{
      ast_expr_constant_integer::AstExprConstantInteger,
      ast_expr_constant_number::AstExprConstantNumber, ast_stat_return::AstStatReturn,
      parse_options::ParseOptions,
    };
    use ulua_common::FFlag;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    let stat = fixture.parse(
      "return 0b1, 0b0, 0b101010, 0b1111111111111111111111111111111111111111111111111111111111111111",
      &ParseOptions::default(),
    );
    assert!(!stat.is_null());

    let block = unsafe { &*stat };
    assert!(!block.body.data.is_null());
    assert!(block.body.size > 0);

    let first_stat = unsafe { &*block.body.data };
    let return_stat =
      unsafe { rtti::ast_node_as::<AstStatReturn>(*first_stat as *mut ast_node::AstNode) };
    assert!(!return_stat.is_null());
    let return_stat = unsafe { &*return_stat };

    assert_eq!(return_stat.list.size, 4);

    let expr0 = unsafe { &*return_stat.list.data.add(0) };
    let const_num0 =
      unsafe { rtti::ast_node_as::<AstExprConstantNumber>(*expr0 as *mut ast_node::AstNode) };
    assert!(!const_num0.is_null());
    assert_eq!(unsafe { &*const_num0 }.value, 1.0);

    let expr1 = unsafe { &*return_stat.list.data.add(1) };
    let const_num1 =
      unsafe { rtti::ast_node_as::<AstExprConstantNumber>(*expr1 as *mut ast_node::AstNode) };
    assert!(!const_num1.is_null());
    assert_eq!(unsafe { &*const_num1 }.value, 0.0);

    let expr2 = unsafe { &*return_stat.list.data.add(2) };
    let const_num2 =
      unsafe { rtti::ast_node_as::<AstExprConstantNumber>(*expr2 as *mut ast_node::AstNode) };
    assert!(!const_num2.is_null());
    assert_eq!(unsafe { &*const_num2 }.value, 42.0);

    let expr3 = unsafe { &*return_stat.list.data.add(3) };
    let const_num3 =
      unsafe { rtti::ast_node_as::<AstExprConstantNumber>(*expr3 as *mut ast_node::AstNode) };
    assert!(!const_num3.is_null());
    assert_eq!(unsafe { &*const_num3 }.value, u64::MAX as f64);

    if FFlag::LuauIntegerType2.get() {
      let mut fixture = Fixture::default();
      let stat = fixture.parse(
              "return 0b1i, 0b0i, 0b101010i, 0b111111111111111111111111111111111111111111111111111111111111111i, 0b1000000000000000000000000000000000000000000000000000000000000000i, 0b1111111111111111111111111111111111111111111111111111111111111111i",
              &ParseOptions::default(),
          );
      assert!(!stat.is_null());

      let block = unsafe { &*stat };
      assert!(!block.body.data.is_null());
      assert!(block.body.size > 0);

      let first_stat = unsafe { &*block.body.data };
      let return_stat =
        unsafe { rtti::ast_node_as::<AstStatReturn>(*first_stat as *mut ast_node::AstNode) };
      assert!(!return_stat.is_null());
      let return_stat = unsafe { &*return_stat };

      assert_eq!(return_stat.list.size, 6);

      let expr0 = unsafe { &*return_stat.list.data.add(0) };
      let const_int0 =
        unsafe { rtti::ast_node_as::<AstExprConstantInteger>(*expr0 as *mut ast_node::AstNode) };
      assert!(!const_int0.is_null());
      assert_eq!(unsafe { &*const_int0 }.value, 1);

      let expr1 = unsafe { &*return_stat.list.data.add(1) };
      let const_int1 =
        unsafe { rtti::ast_node_as::<AstExprConstantInteger>(*expr1 as *mut ast_node::AstNode) };
      assert!(!const_int1.is_null());
      assert_eq!(unsafe { &*const_int1 }.value, 0);

      let expr2 = unsafe { &*return_stat.list.data.add(2) };
      let const_int2 =
        unsafe { rtti::ast_node_as::<AstExprConstantInteger>(*expr2 as *mut ast_node::AstNode) };
      assert!(!const_int2.is_null());
      assert_eq!(unsafe { &*const_int2 }.value, 42);

      let expr3 = unsafe { &*return_stat.list.data.add(3) };
      let const_int3 =
        unsafe { rtti::ast_node_as::<AstExprConstantInteger>(*expr3 as *mut ast_node::AstNode) };
      assert!(!const_int3.is_null());
      assert_eq!(unsafe { &*const_int3 }.value, i64::MAX);

      let expr4 = unsafe { &*return_stat.list.data.add(4) };
      let const_int4 =
        unsafe { rtti::ast_node_as::<AstExprConstantInteger>(*expr4 as *mut ast_node::AstNode) };
      assert!(!const_int4.is_null());
      assert_eq!(unsafe { &*const_int4 }.value, i64::MIN);

      let expr5 = unsafe { &*return_stat.list.data.add(5) };
      let const_int5 =
        unsafe { rtti::ast_node_as::<AstExprConstantInteger>(*expr5 as *mut ast_node::AstNode) };
      assert!(!const_int5.is_null());
      assert_eq!(unsafe { &*const_int5 }.value, -1);
    }
  }
}

mod parser_parse_numbers_decimal {
  use super::*;
  #[cfg(test)]
  #[test]
  fn parser_parse_numbers_decimal() {
    use ulua_ast::records::{
      ast_expr::AstExpr, ast_expr_constant_integer::AstExprConstantInteger,
      ast_expr_constant_number::AstExprConstantNumber, ast_node::AstNode,
      ast_stat_return::AstStatReturn, parse_options::ParseOptions,
    };
    use ulua_common::FFlag;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    let stat = fixture.parse(
      "return 1, .5, 1.5, 1e-5, 1.5e-5, 12_345.1_25",
      &ParseOptions::default(),
    );
    assert!(!stat.is_null());

    let block = unsafe { &*stat };
    assert!(!block.body.data.is_null());

    let first_stat = unsafe { *block.body.data.add(0) };
    let return_stat = unsafe { rtti::ast_node_as::<AstStatReturn>(first_stat as *mut AstNode) };
    assert!(!return_stat.is_null());

    let return_stat = unsafe { &*return_stat };
    assert_eq!(return_stat.list.size, 6);

    let expr0 = unsafe { &*(*return_stat.list.data.add(0)) };
    let const_num0 = unsafe {
      rtti::ast_node_as::<AstExprConstantNumber>(expr0 as *const AstExpr as *mut AstNode)
    };
    assert!(!const_num0.is_null());
    assert_eq!(unsafe { &*const_num0 }.value, 1.0);

    let expr1 = unsafe { &*(*return_stat.list.data.add(1)) };
    let const_num1 = unsafe {
      rtti::ast_node_as::<AstExprConstantNumber>(expr1 as *const AstExpr as *mut AstNode)
    };
    assert!(!const_num1.is_null());
    assert_eq!(unsafe { &*const_num1 }.value, 0.5);

    let expr2 = unsafe { &*(*return_stat.list.data.add(2)) };
    let const_num2 = unsafe {
      rtti::ast_node_as::<AstExprConstantNumber>(expr2 as *const AstExpr as *mut AstNode)
    };
    assert!(!const_num2.is_null());
    assert_eq!(unsafe { &*const_num2 }.value, 1.5);

    let expr3 = unsafe { &*(*return_stat.list.data.add(3)) };
    let const_num3 = unsafe {
      rtti::ast_node_as::<AstExprConstantNumber>(expr3 as *const AstExpr as *mut AstNode)
    };
    assert!(!const_num3.is_null());
    assert_eq!(unsafe { &*const_num3 }.value, 1.0e-5);

    let expr4 = unsafe { &*(*return_stat.list.data.add(4)) };
    let const_num4 = unsafe {
      rtti::ast_node_as::<AstExprConstantNumber>(expr4 as *const AstExpr as *mut AstNode)
    };
    assert!(!const_num4.is_null());
    assert_eq!(unsafe { &*const_num4 }.value, 1.5e-5);

    let expr5 = unsafe { &*(*return_stat.list.data.add(5)) };
    let const_num5 = unsafe {
      rtti::ast_node_as::<AstExprConstantNumber>(expr5 as *const AstExpr as *mut AstNode)
    };
    assert!(!const_num5.is_null());
    assert_eq!(unsafe { &*const_num5 }.value, 12345.125);

    if FFlag::LuauIntegerType2.get() {
      let stat2 = fixture.parse("return 1i, 1_000_000i", &ParseOptions::default());
      assert!(!stat2.is_null());

      let block2 = unsafe { &*stat2 };
      assert!(!block2.body.data.is_null());

      let first_stat2 = unsafe { *block2.body.data.add(0) };
      let return_stat2 = unsafe { rtti::ast_node_as::<AstStatReturn>(first_stat2 as *mut AstNode) };
      assert!(!return_stat2.is_null());

      let return_stat2 = unsafe { &*return_stat2 };
      assert_eq!(return_stat2.list.size, 2);

      let expr_int0 = unsafe { &*(*return_stat2.list.data.add(0)) };
      assert!(rtti::ast_node_is::<AstExprConstantInteger>(expr_int0));

      let const_int0 = unsafe {
        rtti::ast_node_as::<AstExprConstantInteger>(expr_int0 as *const AstExpr as *mut AstNode)
      };
      assert!(!const_int0.is_null());
      assert_eq!(unsafe { &*const_int0 }.value, 1);

      let expr_int1 = unsafe { &*(*return_stat2.list.data.add(1)) };
      assert!(rtti::ast_node_is::<AstExprConstantInteger>(expr_int1));

      let const_int1 = unsafe {
        rtti::ast_node_as::<AstExprConstantInteger>(expr_int1 as *const AstExpr as *mut AstNode)
      };
      assert!(!const_int1.is_null());
      assert_eq!(unsafe { &*const_int1 }.value, 1000000);
    }
  }
}

mod parser_parse_numbers_error {

  #[cfg(test)]
  #[test]
  fn parser_parse_numbers_error() {
    use ulua_common::FFlag;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    fixture.match_parse_error(
      &String::from("return 0b123"),
      &String::from("Malformed number"),
      None,
    );
    fixture.match_parse_error(
      &String::from("return 123x"),
      &String::from("Malformed number"),
      None,
    );
    fixture.match_parse_error(
      &String::from("return 0xg"),
      &String::from("Malformed number"),
      None,
    );
    fixture.match_parse_error(
      &String::from("return 0x0x123"),
      &String::from("Malformed number"),
      None,
    );
    fixture.match_parse_error(
      &String::from("return 0xffffffffffffffffffffllllllg"),
      &String::from("Malformed number"),
      None,
    );
    fixture.match_parse_error(
      &String::from("return 0x0xffffffffffffffffffffffffffff"),
      &String::from("Malformed number"),
      None,
    );
    if FFlag::LuauIntegerType2.get() {
      fixture.match_parse_error(
        &String::from("return 0x0xABCi"),
        &String::from("Malformed integer"),
        None,
      );
      fixture.match_parse_error(
        &String::from("return 0xABCMi"),
        &String::from("Malformed integer"),
        None,
      );
      fixture.match_parse_error(
        &String::from("return 0b250i"),
        &String::from("Malformed integer"),
        None,
      );
      fixture.match_parse_error(
        &String::from("return 0bbbbi"),
        &String::from("Malformed integer"),
        None,
      );
      fixture.match_parse_error(
        &String::from("return 123ii"),
        &String::from("Malformed integer"),
        None,
      );
      fixture.match_parse_error(
        &String::from("return 0xABii"),
        &String::from("Malformed integer"),
        None,
      );
      fixture.match_parse_error(
        &String::from("return 99999999999999999999i"),
        &String::from("Integer overflow"),
        None,
      );
      fixture.match_parse_error(
        &String::from("return 0xFFFFFFFFFFFFFFFFFFi"),
        &String::from("Integer overflow"),
        None,
      );
      fixture.match_parse_error(
        &String::from(
          "return 0b10000000000000000000000000000000000000000000000000000000000000000i",
        ),
        &String::from("Integer overflow"),
        None,
      );
      fixture.match_parse_error(
        &String::from("return 123ii"),
        &String::from("Malformed integer"),
        None,
      );
      fixture.match_parse_error(
        &String::from("return 0xABii"),
        &String::from("Malformed integer"),
        None,
      );
    }
  }
}

mod parser_parse_numbers_hexadecimal {
  use ulua_common::FFlag;

  use super::*;
  #[cfg(test)]
  #[test]
  fn parser_parse_numbers_hexadecimal() {
    use ulua_ast::records::parse_options::ParseOptions;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    let stat = fixture.parse(
      "return 0xab, 0xAB05, 0xff_ff, 0xffffffffffffffff",
      &ParseOptions::default(),
    );
    assert!(!stat.is_null());

    let block = unsafe { &*stat };
    assert!(!block.body.data.is_null());
    assert!(block.body.size > 0);

    let first_stat = unsafe { &*block.body.data };
    let return_stat = unsafe {
      &*rtti::ast_node_as::<ast_stat_return::AstStatReturn>(*first_stat as *mut ast_node::AstNode)
    };
    assert_eq!(return_stat.list.size, 4);

    let expr0 = unsafe { &*return_stat.list.data.add(0) };
    let const_num0 = unsafe {
      &*rtti::ast_node_as::<ast_expr_constant_number::AstExprConstantNumber>(
        *expr0 as *mut ast_node::AstNode,
      )
    };
    assert_eq!(const_num0.value, 0xab as f64);

    let expr1 = unsafe { &*return_stat.list.data.add(1) };
    let const_num1 = unsafe {
      &*rtti::ast_node_as::<ast_expr_constant_number::AstExprConstantNumber>(
        *expr1 as *mut ast_node::AstNode,
      )
    };
    assert_eq!(const_num1.value, 0xAB05 as f64);

    let expr2 = unsafe { &*return_stat.list.data.add(2) };
    let const_num2 = unsafe {
      &*rtti::ast_node_as::<ast_expr_constant_number::AstExprConstantNumber>(
        *expr2 as *mut ast_node::AstNode,
      )
    };
    assert_eq!(const_num2.value, 0xFFFF as f64);

    let expr3 = unsafe { &*return_stat.list.data.add(3) };
    let const_num3 = unsafe {
      &*rtti::ast_node_as::<ast_expr_constant_number::AstExprConstantNumber>(
        *expr3 as *mut ast_node::AstNode,
      )
    };
    // C++ `double(ULLONG_MAX)` — u64::MAX as f64 (1.844e19), NOT f64::MAX (1.797e308).
    assert_eq!(const_num3.value, u64::MAX as f64);

    if FFlag::LuauIntegerType2.get() {
      let mut fixture = Fixture::default();
      let stat = fixture.parse(
              "return 0xabi, 0XAB05i, 0xff_ffi, 0x7fffffffffffffffi, 0x8000000000000000i, 0xffffffffffffffffi",
              &ParseOptions::default(),
          );
      assert!(!stat.is_null());

      let block = unsafe { &*stat };
      assert!(!block.body.data.is_null());
      assert!(block.body.size > 0);

      let first_stat = unsafe { &*block.body.data };
      let return_stat = unsafe {
        &*rtti::ast_node_as::<ast_stat_return::AstStatReturn>(*first_stat as *mut ast_node::AstNode)
      };
      assert_eq!(return_stat.list.size, 6);

      let expr0 = unsafe { &*return_stat.list.data.add(0) };
      let const_int0 = unsafe {
        &*rtti::ast_node_as::<ast_expr_constant_integer::AstExprConstantInteger>(
          *expr0 as *mut ast_node::AstNode,
        )
      };
      assert_eq!(const_int0.value, 0xab);

      let expr1 = unsafe { &*return_stat.list.data.add(1) };
      let const_int1 = unsafe {
        &*rtti::ast_node_as::<ast_expr_constant_integer::AstExprConstantInteger>(
          *expr1 as *mut ast_node::AstNode,
        )
      };
      assert_eq!(const_int1.value, 0xAB05);

      let expr2 = unsafe { &*return_stat.list.data.add(2) };
      let const_int2 = unsafe {
        &*rtti::ast_node_as::<ast_expr_constant_integer::AstExprConstantInteger>(
          *expr2 as *mut ast_node::AstNode,
        )
      };
      assert_eq!(const_int2.value, 0xFFFF);

      let expr3 = unsafe { &*return_stat.list.data.add(3) };
      let const_int3 = unsafe {
        &*rtti::ast_node_as::<ast_expr_constant_integer::AstExprConstantInteger>(
          *expr3 as *mut ast_node::AstNode,
        )
      };
      assert_eq!(const_int3.value, i64::MAX);

      let expr4 = unsafe { &*return_stat.list.data.add(4) };
      let const_int4 = unsafe {
        &*rtti::ast_node_as::<ast_expr_constant_integer::AstExprConstantInteger>(
          *expr4 as *mut ast_node::AstNode,
        )
      };
      assert_eq!(const_int4.value, i64::MIN);

      let expr5 = unsafe { &*return_stat.list.data.add(5) };
      let const_int5 = unsafe {
        &*rtti::ast_node_as::<ast_expr_constant_integer::AstExprConstantInteger>(
          *expr5 as *mut ast_node::AstNode,
        )
      };
      assert_eq!(const_int5.value, -1);
    }
  }
}

mod parser_parse_parametrized_attribute_on_function_stat {
  use ulua_ast::records::parse_options::ParseOptions;

  use super::*;
  #[cfg(test)]
  #[test]
  fn parser_parse_parametrized_attribute_on_function_stat() {
    use ulua_ast::records::{
      ast_array::AstArray,
      ast_attr::{AstAttr, AstAttrType},
      ast_stat::AstStat,
      ast_stat_block::AstStatBlock,
      ast_stat_function::AstStatFunction,
      location::Location,
      position::Position,
    };
    use ulua_unit_test::{functions::check_attribute::check_attribute, records::fixture::Fixture};

    let mut fix = Fixture::default();
    let code = String::from(
      r#"
@[deprecated{ use = "greetng", reason = "Using <hello> is too causal"}]
function hello(x, y)
    return x + y
end"#,
    );

    let result = fix.parse(&code, &ParseOptions::default());

    let stat_block: *mut AstStatBlock = result;
    assert!(!stat_block.is_null());

    let first_stat: *mut AstStat = unsafe { *(*stat_block).body.data.add(0) };
    let stat_fun: *mut AstStatFunction =
      unsafe { rtti::ast_node_as::<AstStatFunction>(first_stat as *mut ast_node::AstNode) };
    assert!(!stat_fun.is_null());

    let attributes: AstArray<*mut AstAttr> = unsafe {
      (*rtti::ast_node_as::<ast_expr_function::AstExprFunction>(
        (*stat_fun).func as *mut ast_node::AstNode,
      ))
      .attributes
    };

    assert_eq!(attributes.size, 1);

    let attr: *mut AstAttr = unsafe { *attributes.data.add(0) };
    let expected_location = Location::new(Position::new(1, 2), Position::new(1, 70));
    check_attribute(
      unsafe { &*attr },
      AstAttrType::Deprecated,
      expected_location,
    );
  }
}

mod parser_parse_return_type_ast_type_pack_explicit {
  use ulua_common::FFlag;

  use super::*;
  #[cfg(test)]
  #[test]
  fn parser_parse_return_type_ast_type_pack_explicit() {
    use ulua_ast::records::parse_options::ParseOptions;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _sff = ScopedFastFlag::new(
      &FFlag::LuauSingleTypeOptionalPackReturnsAttributeParens,
      true,
    );

    let mut fixture = Fixture::default();
    let stat = fixture.parse(
      r#"
          type Foo = () -> (string)
      "#,
      &ParseOptions::default(),
    );
    let root = unsafe { &*stat };
    assert_eq!(1, root.body.size);

    let alias1 = unsafe {
      rtti::ast_node_as::<ast_stat_type_alias::AstStatTypeAlias>(
        *root.body.data.add(0) as *mut ast_node::AstNode
      )
    };
    assert!(!alias1.is_null());

    let func_type = unsafe {
      rtti::ast_node_as::<ast_type_function::AstTypeFunction>(
        (*alias1).type_ptr as *mut ast_node::AstNode,
      )
    };
    assert!(!func_type.is_null());

    let return_type_pack = unsafe {
      rtti::ast_node_as::<ast_type_pack_explicit::AstTypePackExplicit>(
        (*func_type).return_types as *mut ast_node::AstNode,
      )
    };
    assert!(!return_type_pack.is_null());
    assert_eq!(1, unsafe { (*return_type_pack).type_list.types.size });
    assert!(unsafe { (*return_type_pack).type_list.tail_type }.is_null());
    assert!(unsafe {
      rtti::ast_node_is::<ast_type_reference::AstTypeReference>(
        *(*return_type_pack).type_list.types.data.add(0) as *mut ast_node::AstNode,
      )
    });
  }
}

mod parser_parse_simple_ast_type_group {
  use super::*;
  #[cfg(test)]
  #[test]
  fn parser_parse_simple_ast_type_group() {
    use ulua_ast::records::parse_options::ParseOptions;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    let stat = fixture.parse("type Foo = (string)\n", &ParseOptions::default());
    let root = unsafe { &*stat };
    assert_eq!(1, root.body.size);

    let alias1 = unsafe {
      rtti::ast_node_as::<ast_stat_type_alias::AstStatTypeAlias>(
        *root.body.data.add(0) as *mut ast_node::AstNode
      )
    };
    assert!(!alias1.is_null());

    let group1 = unsafe {
      rtti::ast_node_as::<ast_type_group::AstTypeGroup>(
        (*alias1).type_ptr as *mut ast_node::AstNode,
      )
    };
    assert!(!group1.is_null());

    let _ = unsafe {
      !rtti::ast_node_as::<ast_type_reference::AstTypeReference>(
        (*group1).type_ as *mut ast_node::AstNode,
      )
      .is_null()
    };
  }
}

mod parser_parse_top_level_checked_fn {
  use super::*;
  #[cfg(test)]
  #[test]
  fn parser_parse_top_level_checked_fn() {
    use ulua_ast::records::parse_options::ParseOptions;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fix = Fixture::default();
    let code = String::from("@checked declare function abs(n: number): number\n");
    let mut opts = ParseOptions::new();
    opts.allow_declaration_syntax = true;

    let result = fix.try_parse(&code, &opts);
    assert_eq!(result.errors.len(), 0);

    assert_eq!(unsafe { (*result.root).body.size }, 1);
    let root = unsafe { *(*result.root).body.data };
    let func = unsafe {
      rtti::ast_node_as::<ast_stat_declare_function::AstStatDeclareFunction>(
        root as *mut ast_node::AstNode,
      )
    };
    assert!(!func.is_null());
    assert!(unsafe { (*func).is_checked_function() });
  }
}

mod parser_parse_type_alias_default_type {

  #[cfg(test)]
  #[test]
  fn parser_parse_type_alias_default_type() {
    use ulua_ast::records::parse_options::ParseOptions;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    let stat = fixture.parse(
      r#"type A<T = string> = {}
  type B<T... = ...number> = {}
  type C<T..., U... = T...> = {}
  type D<T..., U... = ()> = {}
  type E<T... = (), U... = ()> = {}
  type F<T... = (string), U... = ()> = (T...) -> U...
  type G<T... = ...number, U... = (string, number, boolean)> = (U...) -> T...
      "#,
      &ParseOptions::default(),
    );
    assert!(!stat.is_null());
  }
}

mod parser_parse_type_alias_default_type_errors {

  #[cfg(test)]
  #[test]
  fn parser_parse_type_alias_default_type_errors() {
    use ulua_ast::records::{location::Location, position::Position};
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    fixture.match_parse_error(
      &String::from("type Y<T = number, U> = {}"),
      &String::from("Expected default type after type name"),
      Some(Location::new(Position::new(0, 20), Position::new(0, 21))),
    );
    fixture.match_parse_error(
      &String::from("type Y<T... = ...number, U...> = {}"),
      &String::from("Expected default type pack after type pack name"),
      Some(Location::new(Position::new(0, 29), Position::new(0, 30))),
    );
    fixture.match_parse_error(
      &String::from("type Y<T... = (string) -> number> = {}"),
      &String::from("Expected type pack after '=', got type"),
      Some(Location::new(Position::new(0, 14), Position::new(0, 32))),
    );
  }
}

mod parser_parse_type_name {
  use super::*;
  #[cfg(test)]
  #[test]
  fn parser_parse_type_name() {
    use alloc::string::String;

    use ulua_ast::records::{
      allocator::Allocator, ast_name_table::AstNameTable, ast_type_function::AstTypeFunction,
      ast_type_pack_explicit::AstTypePackExplicit, parse_node_result::ParseNodeResult,
      parse_options::ParseOptions, parser::Parser,
    };

    let code = String::from("<A>(A, string, boolean?) -> number");
    let mut allocator = Allocator::new();
    let mut names = AstNameTable::new(&mut allocator);

    let result: ParseNodeResult<AstType> =
      Parser::parse_type_c_char_usize_ast_name_table_allocator_parse_options(
        &code,
        &mut names,
        &mut allocator,
        ParseOptions::new(),
      );

    assert!(result.errors.is_empty());
    assert!(!result.root.is_null());

    let fun = unsafe { (*result.root).base.as_item::<AstTypeFunction>() };
    assert!(!fun.is_null());

    let generics = unsafe { (*fun).generics };
    assert_eq!(1, generics.size);

    let arg_types = unsafe { (*fun).arg_types };
    assert_eq!(3, arg_types.types.size);

    let return_types = unsafe { (*fun).return_types };
    let return_pack = unsafe { (*return_types).base.as_item::<AstTypePackExplicit>() };
    assert!(!return_pack.is_null());

    let type_list = unsafe { (*return_pack).type_list };
    assert_eq!(1, type_list.types.size);
  }
}

mod parser_parse_type_pack_errors {
  use super::*;
  #[cfg(test)]
  #[test]
  fn parser_parse_type_pack_errors() {
    use ulua_ast::records::location::Location;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    fixture.match_parse_error(
      &String::from("type Y<T...> = {a: T..., b: number}"),
      &String::from("Unexpected '...' after type name; type pack is not allowed in this context"),
      Some(Location::new(
        UluaAstPosition::new(0, 20),
        UluaAstPosition::new(0, 23),
      )),
    );
    fixture.match_parse_error(
      &String::from("type Y<T...> = {a: (number | string)..."),
      &String::from("Unexpected '...' after type annotation"),
      Some(Location::new(
        UluaAstPosition::new(0, 36),
        UluaAstPosition::new(0, 39),
      )),
    );
  }
}

mod parser_parse_type_pack_type_parameters {

  #[cfg(test)]
  #[test]
  fn parser_parse_type_pack_type_parameters() {
    use ulua_ast::records::parse_options::ParseOptions;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    let _stat = fixture.parse(
      r#"type Packed<T...> = () -> T...
  
  type A<X...> = Packed<X...>
  type B<X...> = Packed<...number>
  type C<X...> = Packed<(number, X...)>
      "#,
      &ParseOptions::default(),
    );
    assert!(!_stat.is_null());
  }
}

mod parser_parse_user_defined_type_functions {
  use super::*;
  #[cfg(test)]
  #[test]
  fn parser_parse_user_defined_type_functions() {
    use ulua_ast::records::{
      ast_stat_type_function::AstStatTypeFunction, parse_options::ParseOptions,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    let block = fixture.parse(
      "type function foo()\n\
           return types.number\n\
           end\n\
           \n\
           export type function bar()\n\
           return types.string\n\
           end",
      &ParseOptions::default(),
    );
    let root = unsafe { &*block };
    assert!(!root.body.data.is_null());

    let first_stat = unsafe { *root.body.data.add(0) };
    let type_function =
      unsafe { rtti::ast_node_as::<AstStatTypeFunction>(first_stat as *mut ast_node::AstNode) };
    assert!(!type_function.is_null());

    let f = unsafe { &*type_function };
    // C++ `CHECK(f->name == "foo")` — AstName == "literal" is strcmp on content.
    assert_eq!(unsafe { CStr::from_ptr(f.name.value) }, c"foo");
  }
}

mod parser_parse_variadics {
  use super::*;
  #[cfg(test)]
  #[test]
  fn parser_parse_variadics() {
    use ulua_ast::records::parse_options::ParseOptions;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    let stat = fixture.parse_ex(
      &String::from(
        "function foo(bar, ...: number): ...string\n\
               end\n\
               \n\
               type Foo = (string, number, ...number) -> ...boolean\n\
               type Bar = () -> (number, ...boolean)",
      ),
      &ParseOptions::default(),
    );
    let root = unsafe { &*stat.root };
    assert_eq!(3, root.body.size);

    let fn_stat = unsafe {
      rtti::ast_node_as::<ast_stat_function::AstStatFunction>(
        (*root.body.data.add(0)) as *mut ast_node::AstNode,
      )
    };
    assert!(!fn_stat.is_null());
    let func = unsafe { &*(*fn_stat).func };
    assert!(func.vararg);
    assert!(!func.vararg_annotation.is_null());

    let foo_stat = unsafe {
      rtti::ast_node_as::<ast_stat_type_alias::AstStatTypeAlias>(
        (*root.body.data.add(1)) as *mut ast_node::AstNode,
      )
    };
    assert!(!foo_stat.is_null());
    let foo_type = unsafe { &*(*foo_stat).type_ptr };
    let foo_fn = unsafe {
      rtti::ast_node_as::<ast_type_function::AstTypeFunction>(
        foo_type as *const _ as *mut ast_node::AstNode,
      )
    };
    assert!(!foo_fn.is_null());
    let foo_fn = unsafe { &*foo_fn };
    assert_eq!(2, foo_fn.arg_types.types.size);
    assert!(!foo_fn.arg_types.tail_type.is_null());
    let return_tp = unsafe { &*foo_fn.return_types };
    assert!(rtti::ast_node_is::<AstTypePackVariadic>(return_tp));

    let bar_stat = unsafe {
      rtti::ast_node_as::<ast_stat_type_alias::AstStatTypeAlias>(
        (*root.body.data.add(2)) as *mut ast_node::AstNode,
      )
    };
    assert!(!bar_stat.is_null());
    let bar_type = unsafe { &*(*bar_stat).type_ptr };
    let bar_fn = unsafe {
      rtti::ast_node_as::<ast_type_function::AstTypeFunction>(
        bar_type as *const _ as *mut ast_node::AstNode,
      )
    };
    assert!(!bar_fn.is_null());
    let bar_fn = unsafe { &*bar_fn };
    assert_eq!(0, bar_fn.arg_types.types.size);
    assert!(bar_fn.arg_types.tail_type.is_null());
    let return_tp = unsafe { &*bar_fn.return_types };
    let explicit_pack = unsafe {
      rtti::ast_node_as::<ast_type_pack_explicit::AstTypePackExplicit>(
        return_tp as *const _ as *mut ast_node::AstNode,
      )
    };
    assert!(!explicit_pack.is_null());
    let explicit_pack = unsafe { &*explicit_pack };
    assert_eq!(1, explicit_pack.type_list.types.size);
    assert!(!explicit_pack.type_list.tail_type.is_null());
  }
}

mod parser_parsing_incomplete_string_interpolation_missing_backtick_at_eof {
  use super::*;
  #[cfg(test)]
  #[test]
  fn parser_parsing_incomplete_string_interpolation_missing_backtick_at_eof() {
    use ulua_ast::records::{
      location::Location, parse_options::ParseOptions, parse_result::ParseResult,
      position::Position,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let source = String::from("print(`{e.x} {e.a}");
    let options = ParseOptions::new();
    let result: ParseResult = fixture.try_parse(&source, &options);

    assert_eq!(2, result.errors.len());

    let first_error = &result.errors[0];
    assert_eq!(
      "Malformed interpolated string; did you forget to add a '`'?",
      first_error.get_message()
    );
    let expected_location = Location::new(Position::new(0, 17), Position::new(0, 18));
    assert_eq!(expected_location, *first_error.get_location());

    assert!(!result.root.is_null());
    let body = unsafe { (*result.root).body };
    assert_eq!(1, body.size);

    let first_stmt = unsafe { *body.data.add(0) };
    let stat_expr = unsafe {
      rtti::ast_node_as::<ast_stat_expr::AstStatExpr>(first_stmt as *mut ast_node::AstNode)
    };
    assert!(!stat_expr.is_null());

    let call = unsafe {
      rtti::ast_node_as::<ast_expr_call::AstExprCall>((*stat_expr).expr as *mut ast_node::AstNode)
    };
    assert!(!call.is_null());

    let args = unsafe { (*call).args };
    assert_eq!(1, args.size);

    let interp_string = unsafe {
      rtti::ast_node_as::<ast_expr_interp_string::AstExprInterpString>(
        *args.data.add(0) as *mut ast_node::AstNode
      )
    };
    assert!(!interp_string.is_null());

    let expressions = unsafe { (*interp_string).expressions };
    assert_eq!(2, expressions.size);

    let interp_string_location = unsafe { (*interp_string).base.base.location };
    let expected_begin = Position::new(0, 6);
    let expected_end = Position::new(0, 18);
    assert_eq!(expected_begin, interp_string_location.begin);
    assert_eq!(expected_end, interp_string_location.end);
  }
}

mod parser_parsing_incomplete_string_interpolation_missing_backtick_broken_string {
  use super::*;
  #[cfg(test)]
  #[test]
  fn parser_parsing_incomplete_string_interpolation_missing_backtick_broken_string() {
    use ulua_ast::records::{
      ast_expr_call::AstExprCall, ast_expr_interp_string::AstExprInterpString,
      ast_stat_block::AstStatBlock, ast_stat_expr::AstStatExpr, location::Location,
      parse_options::ParseOptions, parse_result::ParseResult, position::Position,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let source = String::from("print(`{e.x} {e.a}\n");
    let options = ParseOptions::new();
    let result: ParseResult = fixture.try_parse(&source, &options);

    assert_eq!(2, result.errors.len());

    let first_error = &result.errors[0];
    assert_eq!(
      "Malformed interpolated string; did you forget to add a '`'?",
      first_error.get_message()
    );
    assert_eq!(
      Location::new(Position::new(0, 17), Position::new(0, 18)),
      *first_error.get_location()
    );

    assert!(!result.root.is_null());
    let root_block: *mut AstStatBlock = result.root;
    assert_eq!(1, unsafe { (*root_block).body.size });

    let first_stat_ptr = unsafe { (*root_block).body.data.add(0) };
    let first_stat = unsafe { *first_stat_ptr };
    let stat_expr = unsafe {
      rtti::ast_node_as::<AstStatExpr>(
        first_stat as *const ast_node::AstNode as *mut ast_node::AstNode,
      )
    };
    assert!(!stat_expr.is_null());

    let expr_call = unsafe {
      rtti::ast_node_as::<AstExprCall>(
        (*stat_expr).expr as *const ast_node::AstNode as *mut ast_node::AstNode,
      )
    };
    assert!(!expr_call.is_null());

    let interp_string = unsafe {
      rtti::ast_node_as::<AstExprInterpString>(
        *(*expr_call).args.data.add(0) as *mut ast_node::AstNode
      )
    };
    assert!(!interp_string.is_null());

    assert_eq!(2, unsafe { (*interp_string).expressions.size });
    assert_eq!(
      Location::new(Position::new(0, 6), Position::new(0, 18)),
      unsafe { (*interp_string).base.base.location }
    );
  }
}

mod parser_parsing_incomplete_string_interpolation_missing_curly_at_eof {
  use super::*;
  #[cfg(test)]
  #[test]
  fn parser_parsing_incomplete_string_interpolation_missing_curly_at_eof() {
    use ulua_ast::records::{
      location::Location, parse_options::ParseOptions, parse_result::ParseResult,
      position::Position,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let source = String::from("print(`{e.x} {e.a");
    let options = ParseOptions::new();
    let result: ParseResult = fixture.try_parse(&source, &options);

    assert_eq!(2, result.errors.len());

    let first = unsafe { *result.root.as_ref().unwrap().body.data.add(0) };
    let expr =
      unsafe { rtti::ast_node_as::<ast_stat_expr::AstStatExpr>(first as *mut ast_node::AstNode) };
    assert!(!expr.is_null());

    let call = unsafe {
      rtti::ast_node_as::<ast_expr_call::AstExprCall>((*expr).expr as *mut ast_node::AstNode)
    };
    assert!(!call.is_null());

    let interp_string = unsafe {
      rtti::ast_node_as::<ast_expr_interp_string::AstExprInterpString>(
        *(*call).args.data.add(0) as *mut ast_node::AstNode
      )
    };
    assert!(!interp_string.is_null());

    assert_eq!(2, unsafe { (*interp_string).expressions.size });

    let expected_location = Location::new(Position::new(0, 6), Position::new(0, 17));
    assert_eq!(expected_location, unsafe {
      (*interp_string).base.base.location
    });

    let err = &result.errors[0];
    assert_eq!(
      "Malformed interpolated string; did you forget to add a '}'?",
      err.get_message()
    );
    let expected_err_location = Location::new(Position::new(0, 16), Position::new(0, 17));
    assert_eq!(expected_err_location, *err.get_location());
  }
}

mod parser_parsing_incomplete_string_interpolation_missing_curly_broken_string {
  use ulua_ast::records::parse_options::ParseOptions;

  use super::*;
  #[cfg(test)]
  #[test]
  fn parser_parsing_incomplete_string_interpolation_missing_curly_broken_string() {
    use ulua_ast::records::{location::Location, parse_result::ParseResult, position::Position};
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let source = String::from("print(`{e.x} {e.a\n");
    let result: ParseResult = fixture.try_parse(&source, &ParseOptions::new());

    assert_eq!(2, result.errors.len());

    let first_error = &result.errors[0];
    assert_eq!(
      "Malformed interpolated string; did you forget to add a '}'?",
      first_error.get_message()
    );
    assert_eq!(
      Location::new(Position::new(0, 16), Position::new(0, 17)),
      *first_error.get_location()
    );

    let root = result.root;
    assert!(!root.is_null());

    let body = unsafe { &(*root).body };
    assert_eq!(1, body.size);

    let first_stmt = unsafe { *body.data.add(0) };
    let expr_stat = unsafe {
      rtti::ast_node_as::<ast_stat_expr::AstStatExpr>(first_stmt as *mut ast_node::AstNode)
    };
    assert!(!expr_stat.is_null());

    let call_expr = unsafe {
      rtti::ast_node_as::<ast_expr_call::AstExprCall>((*expr_stat).expr as *mut ast_node::AstNode)
    };
    assert!(!call_expr.is_null());

    let interp_string = unsafe {
      rtti::ast_node_as::<ast_expr_interp_string::AstExprInterpString>(
        *(*call_expr).args.data.add(0) as *mut ast_node::AstNode,
      )
    };
    assert!(!interp_string.is_null());

    assert_eq!(2, unsafe { (*interp_string).expressions.size });
    assert_eq!(Position::new(0, 6), unsafe {
      (*interp_string).base.base.location.begin
    });
    assert_eq!(Position::new(0, 17), unsafe {
      (*interp_string).base.base.location.end
    });
  }
}

mod parser_parsing_incomplete_string_interpolation_missing_curly_with_backtick_at_eof {
  use super::*;
  #[cfg(test)]
  #[test]
  fn parser_parsing_incomplete_string_interpolation_missing_curly_with_backtick_at_eof() {
    use ulua_ast::records::{
      location::Location, parse_options::ParseOptions, parse_result::ParseResult,
      position::Position,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let source = String::from("print(`{e.x} {e.a`");
    let options = ParseOptions::new();
    let result: ParseResult = fixture.try_parse(&source, &options);

    assert_eq!(2, result.errors.len());

    let first_error = &result.errors[0];
    assert_eq!(
      "Malformed interpolated string; did you forget to add a '}'?",
      first_error.get_message()
    );
    let expected_location = Location::new(Position::new(0, 17), Position::new(0, 18));
    assert_eq!(expected_location, *first_error.get_location());

    assert!(!result.root.is_null());
    let body = unsafe { &(*result.root).body };
    assert_eq!(1, body.size);

    let first_stmt = unsafe { *body.data.add(0) };
    let stat_expr = unsafe {
      rtti::ast_node_as::<ast_stat_expr::AstStatExpr>(first_stmt as *mut ast_node::AstNode)
    };
    assert!(!stat_expr.is_null());

    let call = unsafe {
      rtti::ast_node_as::<ast_expr_call::AstExprCall>((*stat_expr).expr as *mut ast_node::AstNode)
    };
    assert!(!call.is_null());

    let interp_string = unsafe {
      rtti::ast_node_as::<ast_expr_interp_string::AstExprInterpString>(
        *(*call).args.data.add(0) as *mut ast_node::AstNode
      )
    };
    assert!(!interp_string.is_null());

    let interp_string_ref = unsafe { &*interp_string };
    assert_eq!(2, interp_string_ref.expressions.size);

    let interp_string_location = unsafe { (*interp_string).base.base.location };
    assert_eq!(Position::new(0, 6), interp_string_location.begin);
    assert_eq!(Position::new(0, 18), interp_string_location.end);
  }
}

mod parser_parsing_incomplete_string_interpolation_missing_curly_with_backtick_broken_string {
  use super::*;
  #[cfg(test)]
  #[test]
  fn parser_parsing_incomplete_string_interpolation_missing_curly_with_backtick_broken_string() {
    use ulua_ast::records::{
      location::Location, parse_options::ParseOptions, parse_result::ParseResult,
      position::Position,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let source = String::from("print(`{e.x} {e.a`\n");
    let options = ParseOptions::new();
    let result: ParseResult = fixture.try_parse(&source, &options);

    assert_eq!(2, result.errors.len());

    let expected_location_0 = Location::new(Position::new(0, 17), Position::new(0, 18));
    assert_eq!(expected_location_0, *result.errors[0].get_location());
    assert_eq!(
      "Malformed interpolated string; did you forget to add a '}'?",
      result.errors[0].get_message()
    );

    let first = unsafe { *result.root.as_ref().unwrap().body.data.add(0) };
    let expr =
      unsafe { rtti::ast_node_as::<ast_stat_expr::AstStatExpr>(first as *mut ast_node::AstNode) };
    assert!(!expr.is_null());

    let call = unsafe {
      rtti::ast_node_as::<ast_expr_call::AstExprCall>((*expr).expr as *mut ast_node::AstNode)
    };
    assert!(!call.is_null());

    let interp_string = unsafe {
      rtti::ast_node_as::<ast_expr_interp_string::AstExprInterpString>(
        *(*call).args.data.add(0) as *mut ast_node::AstNode
      )
    };
    assert!(!interp_string.is_null());

    assert_eq!(2, unsafe { (*interp_string).expressions.size });

    let expected_location = Location::new(Position::new(0, 6), Position::new(0, 18));
    assert_eq!(expected_location, unsafe {
      (*interp_string).base.base.location
    });
  }
}

mod parser_parsing_string_union_indexers {

  #[cfg(test)]
  #[test]
  fn parser_parsing_string_union_indexers() {
    use ulua_ast::records::parse_options::ParseOptions;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let source = String::from(r#"type foo = { ["bar" | "baz"]: number }"#);
    let options = ParseOptions::new();
    let _result = fixture.parse_ex(&source, &options);
  }
}

mod parser_parsing_type_suffix_for_return_type_with_variadic {

  #[cfg(test)]
  #[test]
  fn parser_parsing_type_suffix_for_return_type_with_variadic() {
    use core::sync::atomic::Ordering;

    use ulua_ast::records::parse_options::ParseOptions;
    use ulua_common::DFFlag;
    use ulua_unit_test::records::fixture::Fixture;

    // C++ sets the DFFlag (which gates the parser's telemetry side-effect) and then
    // checks the telemetry GLOBAL, not the flag itself.
    DFFlag::DebugLuauReportReturnTypeVariadicWithTypeSuffix.set(true);
    ulua_ast::LUAU_TELEMETRY_PARSED_RETURN_TYPE_VARIADIC_WITH_TYPE_SUFFIX
      .store(false, Ordering::Relaxed);

    let mut fix = Fixture::default();
    let code = r"function foo(): (string, ...number) | boolean
  end";

    let result = fix.try_parse(code, &ParseOptions::new());

    // TODO(CLI-140667): this should produce a ParseError in future when we fix the invalid syntax
    assert_eq!(result.errors.len(), 0);
    assert!(
      ulua_ast::LUAU_TELEMETRY_PARSED_RETURN_TYPE_VARIADIC_WITH_TYPE_SUFFIX.load(Ordering::Relaxed)
    );
  }
}

mod parser_read_write_table_properties {

  use ulua_ast::records::parse_options::ParseOptions;
  #[cfg(test)]
  #[test]
  fn parser_read_write_table_properties() {
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    let source = String::from(
      "type A = {read x: number}\n\
           type B = {write x: number}\n\
           type C = {read x: number, write x: number}\n\
           type D = {read: () -> string}\n\
           type E = {write: (string) -> ()}\n\
           type F = {read read: () -> string}\n\
           type G = {read write: (string) -> ()}\n\
           type H = {read [\"A\"]: number}\n\
           type I = {write [\"A\"]: string}\n\
           type J = {read [number]: number}\n\
           type K = {write [number]: string}",
    );
    let result = fixture.try_parse(&source, &ParseOptions::default());
    assert_eq!(result.errors.len(), 0);
  }
}

mod parser_reassigned_class {

  use ulua_common::FFlag;
  #[cfg(test)]
  #[test]
  fn parser_reassigned_class() {
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _g = ScopedFastFlag::new(&FFlag::DebugLuauUserDefinedClasses, true);
    let _g_export = ScopedFastFlag::new(&FFlag::LuauExportValueSyntax, true);

    let mut fix = Fixture::default();
    fix.match_parse_error(
      &String::from("\nclass Animal end\nAnimal = nil\n        "),
      &String::from("Variable 'Animal' is constant and may not be reassigned"),
      None,
    );
  }
}

mod parser_recover_confusables {

  #[cfg(test)]
  #[test]
  fn parser_recover_confusables() {
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::default();

    // Binary
    fixture.match_parse_error(
      &String::from("local a = 4 != 10"),
      &String::from("Unexpected '!='; did you mean '~='?"),
      None,
    );
    fixture.match_parse_error(
      &String::from("local a = true && false"),
      &String::from("Unexpected '&&'; did you mean 'and'?"),
      None,
    );
    fixture.match_parse_error(
      &String::from("local a = false || true"),
      &String::from("Unexpected '||'; did you mean 'or'?"),
      None,
    );

    // Unary
    fixture.match_parse_error(
      &String::from("local a = !false"),
      &String::from("Unexpected '!'; did you mean 'not'?"),
      None,
    );

    // Check that separate tokens are not considered as a single one
    fixture.match_parse_error(
      &String::from("local a = 4 ! = 10"),
      &String::from("Expected identifier when parsing expression, got '!'"),
      None,
    );
    fixture.match_parse_error(
      &String::from("local a = true & & false"),
      &String::from("Expected identifier when parsing expression, got '&'"),
      None,
    );
    fixture.match_parse_error(
      &String::from("local a = false | | true"),
      &String::from("Expected identifier when parsing expression, got '|'"),
      None,
    );
  }
}

mod parser_recover_expected_type_pack {

  #[cfg(test)]
  #[test]
  fn parser_recover_expected_type_pack() {
    use ulua_ast::records::parse_options::ParseOptions;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let source = String::from("type Y<T..., U = T...> = (T...) -> U...\n");
    let options = ParseOptions::new();
    let result = fixture.try_parse(&source, &options);
    assert_eq!(1, result.errors.len());
  }
}

mod parser_recover_from_bad_table_type {

  #[cfg(test)]
  #[test]
  fn parser_recover_from_bad_table_type() {
    use ulua_ast::records::parse_options::ParseOptions;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fix = Fixture::default();
    let code = String::from(
      "\n        declare extern type Widget with\n            state: {string: function(string, Widget)}\n        end\n    ",
    );
    let mut opts = ParseOptions::new();
    opts.allow_declaration_syntax = true;
    let result = fix.try_parse(&code, &opts);
    assert_eq!(result.errors.len(), 2);
  }
}

mod parser_recover_function_return_type_annotations {

  #[cfg(test)]
  #[test]
  fn parser_recover_function_return_type_annotations() {
    use ulua_ast::records::parse_options::ParseOptions;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fix = Fixture::default();
    let code = String::from(
      "type Custom<A, B, C> = { x: A, y: B, z: C }\n\
           type Packed<A...> = { x: (A...) -> () }\n\
           type F = (number): Custom<boolean, number, string>\n\
           type G = Packed<(number): (string, number, boolean)>\n\
           local function f(x: number) -> Custom<string, boolean, number>\n\
           end",
    );

    let result = fix.try_parse(&code, &ParseOptions::new());

    assert_eq!(result.errors.len(), 3);
    assert_eq!(
      result.errors[0].get_message().as_str(),
      "Return types in function type annotations are written after '->' instead of ':'"
    );
    assert_eq!(
      result.errors[1].get_message().as_str(),
      "Return types in function type annotations are written after '->' instead of ':'"
    );
    assert_eq!(
      result.errors[2].get_message().as_str(),
      "Function return type annotations are written after ':' instead of '->'"
    );
  }
}

mod parser_recover_index_name_keyword {

  #[cfg(test)]
  #[test]
  fn parser_recover_index_name_keyword() {
    use ulua_ast::records::parse_options::ParseOptions;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let source1 = String::from("local b\nlocal a = b.do\n");
    let options = ParseOptions::new();
    let result1 = fixture.try_parse(&source1, &options);
    assert_eq!(1, result1.errors.len());

    let source2 = String::from("local b\nlocal a = b.\ndo end\n");
    let result2 = fixture.try_parse(&source2, &options);
    assert_eq!(1, result2.errors.len());
  }
}

mod parser_recover_self_call_keyword {

  use ulua_ast::records::parse_options::ParseOptions;
  #[cfg(test)]
  #[test]
  fn parser_recover_self_call_keyword() {
    use ulua_unit_test::records::fixture::Fixture;

    let mut fix = Fixture::default();
    let source1 = String::from("local b\nlocal a = b:do\n    ");
    let result1 = fix.try_parse(&source1, &ParseOptions::default());
    assert_eq!(2, result1.errors.len());

    let mut fix = Fixture::default();
    let source2 = String::from("local b\nlocal a = b:\ndo end\n    ");
    let result2 = fix.try_parse(&source2, &ParseOptions::default());
    assert_eq!(2, result2.errors.len());
  }
}

mod parser_recover_type_index_name_keyword {

  #[cfg(test)]
  #[test]
  fn parser_recover_type_index_name_keyword() {
    use ulua_ast::records::parse_options::ParseOptions;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    let source1 = String::from("local A\nlocal b : A.do\n");
    let options = ParseOptions::new();
    let result1 = fixture.try_parse(&source1, &options);
    assert_eq!(1, result1.errors.len());

    let source2 = String::from("local A\nlocal b : A.do\ndo end\n");
    let result2 = fixture.try_parse(&source2, &options);
    assert_eq!(1, result2.errors.len());
  }
}

mod parser_recover_unexpected_type_pack {

  use ulua_ast::records::parse_options::ParseOptions;
  #[cfg(test)]
  #[test]
  fn parser_recover_unexpected_type_pack() {
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    let source = String::from(
      "type X<T...> = { a: T..., b: number }\n\
           type Y<T> = { a: T..., b: number }\n\
           type Z<T> = { a: string | T..., b: number }",
    );
    let result = fixture.try_parse(&source, &ParseOptions::new());
    assert_eq!(3, result.errors.len());
  }
}

mod parser_recovery_error_limit_1 {

  use ulua_ast::records::parse_options::ParseOptions;
  use ulua_common::FInt;
  #[cfg(test)]
  #[test]
  fn parser_recovery_error_limit_1() {
    use ulua_unit_test::{records::fixture::Fixture, type_aliases::scoped_fast_int::ScopedFastInt};

    let mut fixture = Fixture::fixture_bool(false);
    let _guard = ScopedFastInt::new(&FInt::LuauParseErrorLimit, 1);

    let source = String::from("local a = ");
    let result = fixture.try_parse(&source, &ParseOptions::default());

    assert_eq!(1, result.errors.len());
    assert_eq!(
      result.errors.first().unwrap().get_message(),
      &result.errors.first().unwrap().what().to_string()
    );
  }
}

mod parser_recovery_error_limit_2 {
  use super::*;
  #[cfg(test)]
  #[test]
  fn parser_recovery_error_limit_2() {
    use ulua_ast::records::{parse_errors::ParseErrors, parse_options::ParseOptions};
    use ulua_common::FInt;
    use ulua_unit_test::{records::fixture::Fixture, type_aliases::scoped_fast_int::ScopedFastInt};

    let _sfi = ScopedFastInt::new(&FInt::LuauParseErrorLimit, 2);

    let mut fix = Fixture::default();

    // C++: parse(...) throws ParseErrors; FAIL if it does not. Fixture::parse
    // panics with the ParseErrors payload (see fixture_parse.rs).
    let result = catch_unwind(AssertUnwindSafe(|| {
      fix.parse("escape escape escape", &ParseOptions::new());
    }));
    assert!(result.is_err(), "Expected ParseErrors to be thrown");

    let err = result.unwrap_err();
    let errors = err
      .downcast_ref::<ParseErrors>()
      .expect("Expected ParseErrors");
    assert_eq!(errors.get_errors().len(), 3);
    assert_eq!(errors.what(), "3 parse errors");
    assert_eq!(
      errors.get_errors().last().unwrap().get_message().as_str(),
      "Reached error limit (2)"
    );
  }
}

mod parser_recovery_of_parenthesized_expressions {
  use ulua_ast::records::parse_errors::ParseErrors;
  use ulua_unit_test::records::count_ast_nodes::CountAstNodes;

  use super::*;
  #[cfg(test)]
  #[test]
  fn parser_recovery_of_parenthesized_expressions() {
    use ulua_ast::{records::parse_options::ParseOptions, visit::AstVisitable};
    use ulua_unit_test::records::fixture::Fixture;

    let mut fix = Fixture::default();
    let parse_options = ParseOptions::new();

    let check_ast_equivalence = |fix: &mut Fixture, code_with_errors: &str, code: &str| {
      // Parse with errors, count AST nodes
      let _ = catch_unwind(AssertUnwindSafe(|| {
        fix.parse(code_with_errors, &parse_options);
      }));

      let mut counter_with_errors = CountAstNodes::default();
      unsafe {
        let root = fix.source_module.as_ref().unwrap().as_ref().root;
        (*root).visit(&mut counter_with_errors);
      }

      // Parse correct code, count AST nodes
      fix.parse(code, &parse_options);

      let mut counter = CountAstNodes::default();
      unsafe {
        let root = fix.source_module.as_ref().unwrap().as_ref().root;
        (*root).visit(&mut counter);
      }

      assert_eq!(counter_with_errors.count, counter.count);
    };

    let mut check_recovery = |code_with_errors: &str, code: &str, expected_error_count: u32| {
      let result = catch_unwind(AssertUnwindSafe(|| {
        fix.parse(code_with_errors, &parse_options);
      }));

      if result.is_ok() {
        panic!("Expected ParseErrors to be thrown");
      }

      let err = result.unwrap_err();
      let parse_errors = err.downcast_ref::<ParseErrors>();
      if let Some(errors) = parse_errors {
        assert_eq!(errors.get_errors().len() as u32, expected_error_count);
      } else {
        panic!("Expected ParseErrors");
      }

      check_ast_equivalence(&mut fix, code_with_errors, code);
    };

    check_recovery(
      "function foo(a, b. c) return a + b end",
      "function foo(a, b) return a + b end",
      1,
    );

    check_recovery(
      "function foo(a, b: { a: number, b: number. c:number }) return a + b end",
      "function foo(a, b: { a: number, b: number }) return a + b end",
      1,
    );

    check_recovery(
      "function foo(a, b): (number -> number return a + b end",
      "function foo(a, b): (number) -> number return a + b end",
      1,
    );

    check_recovery(
      "function foo(a, b): (number, number -> number return a + b end",
      "function foo(a, b): (number) -> number return a + b end",
      1,
    );

    check_recovery(
      "function foo(a, b): (number; number) -> number return a + b end",
      "function foo(a, b): (number) -> number return a + b end",
      1,
    );

    check_recovery(
      "function foo(a, b): (number, number return a + b end",
      "function foo(a, b): (number, number) end",
      1,
    );

    check_recovery(
      "local function foo(a, b): (number, number return a + b end",
      "local function foo(a, b): (number, number) end",
      1,
    );

    check_recovery(
      "type F = (number, number -> number",
      "type F = (number, number) -> number",
      1,
    );

    check_recovery(
      "function foo(a, b: { a: number, b: number) return a + b end",
      "function foo(a, b: { a: number, b: number }) return a + b end",
      1,
    );

    check_recovery(
      "function foo(a, b: { [number: number}) return a + b end",
      "function foo(a, b: { [number]: number}) return a + b end",
      1,
    );

    check_recovery(
      "local n: (string | number = 2",
      "local n: (string | number) = 2",
      1,
    );

    check_recovery(
      "\nfunction foo(a, b\n    return a + b\nend\n",
      "function foo(a, b) return a + b end",
      1,
    );
  }
}

mod parser_return_type_is_an_intersection_type_if_led_with_one_parenthesized_type {
  use super::*;
  #[cfg(test)]
  #[test]
  fn parser_return_type_is_an_intersection_type_if_led_with_one_parenthesized_type() {
    use ulua_ast::records::{
      ast_stat_local::AstStatLocal, ast_type_function::AstTypeFunction,
      ast_type_group::AstTypeGroup, ast_type_intersection::AstTypeIntersection,
      ast_type_pack_explicit::AstTypePackExplicit, parse_options::ParseOptions,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    let block = fixture.parse(
      "local f: (string) -> (string) & (number) -> (number)",
      &ParseOptions::default(),
    );
    assert!(!block.is_null());

    let local = unsafe {
      rtti::ast_node_as::<AstStatLocal>((*block).body.data.add(0).read() as *mut ast_node::AstNode)
    };
    assert!(!local.is_null());

    let annotation = unsafe { (*(*local).vars.data.add(0).read()).annotation };
    let annotation =
      unsafe { rtti::ast_node_as::<AstTypeFunction>(annotation as *mut ast_node::AstNode) };
    assert!(!annotation.is_null());

    let return_types = unsafe { (*annotation).return_types };
    let return_type_pack =
      unsafe { rtti::ast_node_as::<AstTypePackExplicit>(return_types as *mut ast_node::AstNode) };
    assert!(!return_type_pack.is_null());

    let types = unsafe { (*return_type_pack).type_list.types };
    let first_type =
      unsafe { rtti::ast_node_as::<AstTypeIntersection>(*types.data as *mut ast_node::AstNode) };
    assert!(!first_type.is_null());

    let inner_types = unsafe { (*first_type).types };
    let first_inner =
      unsafe { rtti::ast_node_as::<AstTypeGroup>(*inner_types.data as *mut ast_node::AstNode) };
    assert!(!first_inner.is_null());

    let second_inner = unsafe {
      rtti::ast_node_as::<AstTypeFunction>(inner_types.data.add(1).read() as *mut ast_node::AstNode)
    };
    assert!(!second_inner.is_null());
  }
}

mod parser_sense_hot_comment_on_first_line {

  #[cfg(test)]
  #[test]
  fn parser_sense_hot_comment_on_first_line() {
    use ulua_analysis::functions::parse_mode::parse_mode;
    use ulua_ast::{enums::mode::Mode, records::parse_options::ParseOptions};
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let source = String::from("   --!strict ");
    let mut options = ParseOptions::new();
    options.capture_comments = true;

    let result = fixture.parse_ex(&source, &options);

    let mode = parse_mode(&result.hotcomments);
    assert!(
      mode.is_some(),
      "Expected a mode to be parsed from hotcomments"
    );
    assert_eq!(
      mode.unwrap() as i32,
      Mode::Strict as i32,
      "Mode should be Strict"
    );
  }
}

mod parser_short_array_types {
  use super::*;
  #[cfg(test)]
  #[test]
  fn parser_short_array_types() {
    use ulua_ast::records::parse_options::ParseOptions;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    let stat = fixture.parse("local n: {string}", &ParseOptions::default());
    let root = unsafe { &*stat };
    assert!(!root.body.data.is_null());

    let local_stmt = unsafe {
      rtti::ast_node_as::<ast_stat_local::AstStatLocal>(
        *root.body.data.add(0) as *mut ast_node::AstNode
      )
    };
    assert!(!local_stmt.is_null());

    let annotation = unsafe {
      rtti::ast_node_as::<ast_type_table::AstTypeTable>(
        (**(*local_stmt).vars.data.add(0)).annotation as *mut ast_node::AstNode,
      )
    };
    assert!(!annotation.is_null());

    assert_eq!(unsafe { (*annotation).props.size }, 0);
    assert!(!unsafe { (*annotation).indexer }.is_null());

    let indexer = unsafe { &*(*annotation).indexer };
    let index_type = unsafe {
      rtti::ast_node_as::<ast_type_reference::AstTypeReference>(
        indexer.index_type as *mut ast_node::AstNode,
      )
    };
    assert!(!index_type.is_null());
    assert_eq!(
      unsafe { CStr::from_ptr((*index_type).name.value).to_string_lossy() },
      "number"
    );

    let result_type = unsafe {
      rtti::ast_node_as::<ast_type_reference::AstTypeReference>(
        indexer.result_type as *mut ast_node::AstNode,
      )
    };
    assert!(!result_type.is_null());
    assert_eq!(
      unsafe { CStr::from_ptr((*result_type).name.value).to_string_lossy() },
      "string"
    );
  }
}

mod parser_short_array_types_are_not_field_names_when_complex {

  #[cfg(test)]
  #[test]
  fn parser_short_array_types_are_not_field_names_when_complex() {
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    fixture.match_parse_error(
      &String::from("local n: {string | number: number}"),
      &String::from("Expected '}' (to close '{' at column 10), got ':'"),
      None,
    );
  }
}

mod parser_short_array_types_do_not_break_field_names {
  use super::*;
  #[cfg(test)]
  #[test]
  fn parser_short_array_types_do_not_break_field_names() {
    use ulua_ast::records::parse_options::ParseOptions;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    let stat = fixture.parse("local n: {string: number}", &ParseOptions::default());
    assert!(!stat.is_null());
    let local = unsafe {
      rtti::ast_node_as::<ast_stat_local::AstStatLocal>(
        (*stat).body.data.add(0).read() as *mut ast_node::AstNode
      )
    };
    assert!(!local.is_null());
    let annotation = unsafe {
      rtti::ast_node_as::<ast_type_table::AstTypeTable>(
        (*(*local).vars.data.add(0).read()).annotation as *mut ast_node::AstNode,
      )
    };
    assert!(!annotation.is_null());
    assert_eq!(unsafe { (*annotation).props.size }, 1);
    assert!(unsafe { (*annotation).indexer }.is_null());
    let prop = unsafe { (*annotation).props.data.add(0).read() };
    let prop_name = unsafe { CStr::from_ptr(prop.name.value).to_string_lossy() };
    assert_eq!(prop_name, "string");
    let prop_type = unsafe {
      rtti::ast_node_as::<ast_type_reference::AstTypeReference>(
        prop.r#type as *mut ast_node::AstNode,
      )
    };
    assert!(!prop_type.is_null());
    let type_name = unsafe { CStr::from_ptr((*prop_type).name.value).to_string_lossy() };
    assert_eq!(type_name, "number");
  }
}

mod parser_short_array_types_must_be_alone {

  #[cfg(test)]
  #[test]
  fn parser_short_array_types_must_be_alone() {
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    fixture.match_parse_error(
      &String::from("local n: {string, number}"),
      &String::from("Expected '}' (to close '{' at column 10), got ','"),
      None,
    );
    fixture.match_parse_error(
      &String::from("local n: {[number]: string, number}"),
      &String::from("Expected ':' when parsing table field, got '}'"),
      None,
    );
    fixture.match_parse_error(
      &String::from("local n: {x: string, number}"),
      &String::from("Expected ':' when parsing table field, got '}'"),
      None,
    );
    fixture.match_parse_error(
      &String::from("local n: {x: string, nil}"),
      &String::from("Expected identifier when parsing table field, got 'nil'"),
      None,
    );
  }
}

mod parser_stat_end_includes_semicolon_position {

  #[cfg(test)]
  #[test]
  fn parser_stat_end_includes_semicolon_position() {
    use ulua_ast::records::{parse_options::ParseOptions, position::Position};
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let source =
      String::from("\n        local x = 1\n        local y = 2;\n        local z = 3  ;\n    ");
    let options = ParseOptions::new();
    let block = fixture.parse(&source, &options);

    unsafe {
      assert_eq!(3, (*block).body.size);

      let stat1 = (*block).body.data.add(0);
      assert!(!(*stat1).is_null());
      assert!(!(*stat1).as_ref().unwrap().has_semicolon);
      assert_eq!(
        Position::new(1, 19),
        (*stat1).as_ref().unwrap().base.location.end
      );

      let stat2 = (*block).body.data.add(1);
      assert!(!(*stat2).is_null());
      assert!((*stat2).as_ref().unwrap().has_semicolon);
      assert_eq!(
        Position::new(2, 20),
        (*stat2).as_ref().unwrap().base.location.end
      );

      let stat3 = (*block).body.data.add(2);
      assert!(!(*stat3).is_null());
      assert!((*stat3).as_ref().unwrap().has_semicolon);
      assert_eq!(
        Position::new(3, 22),
        (*stat3).as_ref().unwrap().base.location.end
      );
    }
  }
}

mod parser_statement_error_recovery_expected {

  #[cfg(test)]
  #[test]
  fn parser_statement_error_recovery_expected() {
    use ulua_ast::records::parse_options::ParseOptions;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fix = Fixture::default();
    let code = String::from("function a(a, b) return a + b end\nsome\na(2, 5)");
    let opts = ParseOptions::new();

    let result = fix.try_parse(&code, &opts);

    assert_eq!(result.errors.len(), 1);
  }
}

mod parser_statement_error_recovery_unexpected {

  use ulua_ast::records::parse_options::ParseOptions;
  #[cfg(test)]
  #[test]
  fn parser_statement_error_recovery_unexpected() {
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let source = String::from("+");

    let result = fixture.try_parse(&source, &ParseOptions::new());

    assert_eq!(1, result.errors.len());
  }
}

mod parser_stop_if_line_ends_with_hyphen {

  use ulua_ast::records::parse_options::ParseOptions;
  #[cfg(test)]
  #[test]
  fn parser_stop_if_line_ends_with_hyphen() {
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    // C++ CHECK_THROWS_AS(parse("   -"), std::exception) — parsing must fail.
    // `fixture.parse` PANICS on errors (emulating the throw), so use try_parse and
    // assert the error instead of letting the panic escape the test.
    let result = fixture.try_parse(&String::from("   -"), &ParseOptions::new());
    assert!(
      !result.errors.is_empty(),
      "Expected a parse error for a trailing hyphen"
    );
  }
}

mod parser_string_literal_call {
  use super::*;
  #[cfg(test)]
  #[test]
  fn parser_string_literal_call() {
    use ulua_ast::records::{
      ast_expr_call::AstExprCall, ast_expr_constant_string::AstExprConstantString,
      ast_stat_block::AstStatBlock, ast_stat_expr::AstStatExpr, parse_options::ParseOptions,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    let stat = fixture.parse("do foo 'bar' end", &ParseOptions::default());
    let root = unsafe { &*stat };
    assert!(!root.body.data.is_null());

    let dob =
      unsafe { rtti::ast_node_as::<AstStatBlock>(*root.body.data as *mut ast_node::AstNode) };
    assert!(!dob.is_null());

    let stc =
      unsafe { rtti::ast_node_as::<AstStatExpr>(*(*dob).body.data as *mut ast_node::AstNode) };
    assert!(!stc.is_null());

    let ec = unsafe { rtti::ast_node_as::<AstExprCall>((*stc).expr as *mut ast_node::AstNode) };
    assert!(!ec.is_null());

    assert_eq!(unsafe { (*ec).args.size }, 1);

    let arg = unsafe {
      rtti::ast_node_as::<AstExprConstantString>(*(*ec).args.data as *mut ast_node::AstNode)
    };
    assert!(!arg.is_null());

    let s = unsafe {
      let p = (*arg).value.data as *const u8;
      from_raw_parts(p, (*arg).value.size as usize)
    };
    assert_eq!(from_utf8(s).unwrap(), "bar");
  }
}

mod parser_string_literals_broken {

  #[cfg(test)]
  #[test]
  fn parser_string_literals_broken() {
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    fixture.match_parse_error(
      &String::from("return \""),
      &String::from("Malformed string; did you forget to finish it?"),
      None,
    );
    fixture.match_parse_error(
      &String::from("return \"\\"),
      &String::from("Malformed string; did you forget to finish it?"),
      None,
    );
    fixture.match_parse_error(
      &String::from("return \"\r\r"),
      &String::from("Malformed string; did you forget to finish it?"),
      None,
    );
  }
}

mod parser_string_literals_escape {
  use super::*;
  #[cfg(test)]
  #[test]
  fn parser_string_literals_escape() {
    use ulua_ast::records::{
      ast_expr_constant_string::AstExprConstantString, ast_stat_return::AstStatReturn,
      parse_options::ParseOptions,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    let stat = fixture.parse(
      "return\n\
           \"foo\\n\\r\",\n\
           \"foo\\0324\",\n\
           \"foo\\x204\",\n\
           \"foo\\u{20}\",\n\
           \"foo\\u{0451}\"",
      &ParseOptions::default(),
    );
    assert!(!stat.is_null());

    let ret = unsafe {
      rtti::ast_node_as::<AstStatReturn>(*(*stat).body.data.add(0) as *mut ast_node::AstNode)
    };
    assert!(!ret.is_null());
    let ret = unsafe { &*ret };
    assert_eq!(ret.list.size, 5);

    let str0 = unsafe {
      &*rtti::ast_node_as::<AstExprConstantString>(*ret.list.data.add(0) as *mut ast_node::AstNode)
    };
    assert_eq!(
      unsafe { from_raw_parts(str0.value.data as *const u8, str0.value.size as usize) },
      b"foo\n\r"
    );

    let str1 = unsafe {
      &*rtti::ast_node_as::<AstExprConstantString>(*ret.list.data.add(1) as *mut ast_node::AstNode)
    };
    assert_eq!(
      unsafe { from_raw_parts(str1.value.data as *const u8, str1.value.size as usize) },
      b"foo 4"
    );

    let str2 = unsafe {
      &*rtti::ast_node_as::<AstExprConstantString>(*ret.list.data.add(2) as *mut ast_node::AstNode)
    };
    assert_eq!(
      unsafe { from_raw_parts(str2.value.data as *const u8, str2.value.size as usize) },
      b"foo 4"
    );

    let str3 = unsafe {
      &*rtti::ast_node_as::<AstExprConstantString>(*ret.list.data.add(3) as *mut ast_node::AstNode)
    };
    assert_eq!(
      unsafe { from_raw_parts(str3.value.data as *const u8, str3.value.size as usize) },
      b"foo "
    );

    let str4 = unsafe {
      &*rtti::ast_node_as::<AstExprConstantString>(*ret.list.data.add(4) as *mut ast_node::AstNode)
    };
    assert_eq!(
      unsafe { from_raw_parts(str4.value.data as *const u8, str4.value.size as usize) },
      b"foo\xd1\x91"
    );
  }
}

mod parser_string_literals_escape_newline {
  use super::*;
  #[cfg(test)]
  #[test]
  fn parser_string_literals_escape_newline() {
    use ulua_ast::records::{
      ast_expr_constant_string::AstExprConstantString, ast_stat_return::AstStatReturn,
      parse_options::ParseOptions,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    let stat = fixture.parse(
      "return \"foo\\z\n   bar\", \"foo\\\n    bar\", \"foo\\\r\nbar\"",
      &ParseOptions::default(),
    );
    assert!(!stat.is_null());

    // C++ `stat->body.data[0]->as<AstStatReturn>()`: deref the first body slot
    // and RTTI-cast it. The port cast the array pointer itself to *AstStatReturn.
    let ret =
      unsafe { rtti::ast_node_as::<AstStatReturn>(*(&*stat).body.data as *mut ast_node::AstNode) };
    assert!(!ret.is_null());

    let ret = unsafe { &*ret };
    assert_eq!(ret.list.size, 3);

    let str0 = unsafe {
      rtti::ast_node_as::<AstExprConstantString>(*ret.list.data.add(0) as *mut ast_node::AstNode)
    };
    assert!(!str0.is_null());
    let str0 = unsafe { &*str0 };
    let s0 = unsafe { from_raw_parts(str0.value.data as *const u8, str0.value.size as usize) };
    assert_eq!(from_utf8(s0).unwrap(), "foobar");

    let str1 = unsafe {
      rtti::ast_node_as::<AstExprConstantString>(*ret.list.data.add(1) as *mut ast_node::AstNode)
    };
    assert!(!str1.is_null());
    let str1 = unsafe { &*str1 };
    let s1 = unsafe { from_raw_parts(str1.value.data as *const u8, str1.value.size as usize) };
    assert_eq!(from_utf8(s1).unwrap(), "foo\n    bar");

    let str2 = unsafe {
      rtti::ast_node_as::<AstExprConstantString>(*ret.list.data.add(2) as *mut ast_node::AstNode)
    };
    assert!(!str2.is_null());
    let str2 = unsafe { &*str2 };
    let s2 = unsafe { from_raw_parts(str2.value.data as *const u8, str2.value.size as usize) };
    assert_eq!(from_utf8(s2).unwrap(), "foo\nbar");
  }
}

mod parser_string_literals_escapes {
  use super::*;
  #[cfg(test)]
  #[test]
  fn parser_string_literals_escapes() {
    use ulua_ast::records::{
      ast_expr_constant_string::AstExprConstantString, ast_stat_return::AstStatReturn,
      parse_options::ParseOptions,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    let stat = fixture.parse(
      "return\n\
           \"\\xAB\",\n\
           \"\\u{2024}\",\n\
           \"\\121\",\n\
           \"\\1x\",\n\
           \"\\t\",\n\
           \"\\n\"",
      &ParseOptions::default(),
    );
    assert!(!stat.is_null());

    // C++ `stat->body.data[0]->as<AstStatReturn>()`: the return is the first body
    // element, not the block's own base node.
    let ret =
      unsafe { rtti::ast_node_as::<AstStatReturn>(*(&*stat).body.data as *mut ast_node::AstNode) };
    assert!(!ret.is_null());
    let ret = unsafe { &*ret };
    assert_eq!(ret.list.size, 6);

    let check_str = |idx: usize, expected: &[u8]| {
      let expr = unsafe { *ret.list.data.add(idx) };
      let str_node = unsafe { (*expr).base.as_item::<AstExprConstantString>() };
      assert!(!str_node.is_null());
      let str_node = unsafe { &*str_node };
      let bytes = unsafe {
        from_raw_parts(
          str_node.value.data as *const u8,
          str_node.value.size as usize,
        )
      };
      assert_eq!(bytes, expected);
    };

    check_str(0, &[0xAB]);
    check_str(1, &[0xE2, 0x80, 0xA4]);
    check_str(2, &[0x79]);
    check_str(3, &[0x01, b'x']);
    check_str(4, b"\t");
    check_str(5, b"\n");
  }
}

mod parser_string_literals_escapes_broken {

  #[cfg(test)]
  #[test]
  fn parser_string_literals_escapes_broken() {
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    let expected = "String literal contains malformed escape sequence";

    fixture.match_parse_error(
      &String::from("return \"\\u{\""),
      &String::from(expected),
      None,
    );
    fixture.match_parse_error(
      &String::from("return \"\\u{FO}\""),
      &String::from(expected),
      None,
    );
    fixture.match_parse_error(
      &String::from("return \"\\u{123456789}\""),
      &String::from(expected),
      None,
    );
    fixture.match_parse_error(
      &String::from("return \"\\359\""),
      &String::from(expected),
      None,
    );
    fixture.match_parse_error(
      &String::from("return \"\\xFO\""),
      &String::from(expected),
      None,
    );
    fixture.match_parse_error(
      &String::from("return \"\\xF\""),
      &String::from(expected),
      None,
    );
    fixture.match_parse_error(
      &String::from("return \"\\x\""),
      &String::from(expected),
      None,
    );
  }
}

mod parser_table_type_keys_cant_contain_nul {

  use ulua_ast::records::parse_options::ParseOptions;
  #[cfg(test)]
  #[test]
  fn parser_table_type_keys_cant_contain_nul() {
    use ulua_ast::records::{location::Location, position::Position};
    use ulua_unit_test::records::fixture::Fixture;

    let mut fix = Fixture::default();
    let code = String::from("\n        type Foo = { [\"\\0\"]: number }\n    ");
    let result = fix.try_parse(&code, &ParseOptions::default());

    assert_eq!(1, result.errors.len());

    let error = &result.errors[0];
    let expected_location = Location::new(Position::new(1, 21), Position::new(1, 22));
    assert_eq!(expected_location, *error.get_location());
    assert_eq!(
      "String literal contains malformed escape sequence or \\0",
      *error.get_message()
    );
  }
}

mod parser_tables_can_have_trailing_separator {

  #[cfg(test)]
  #[test]
  fn parser_tables_can_have_trailing_separator() {
    use ulua_ast::records::parse_options::ParseOptions;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    let stat = fixture.parse(
      "local zero: number\n\
           local one: {x: number, y: string, }",
      &ParseOptions::default(),
    );
    assert!(!stat.is_null());
  }
}

mod parser_tables_can_use_semicolons {

  #[cfg(test)]
  #[test]
  fn parser_tables_can_use_semicolons() {
    use ulua_ast::records::parse_options::ParseOptions;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    let stat = fixture.parse(
      "local zero: number\n\
           local one: {x: number; y: string; }",
      &ParseOptions::default(),
    );
    assert!(!stat.is_null());
  }
}

mod parser_tables_should_have_an_indexer_and_keys {

  #[cfg(test)]
  #[test]
  fn parser_tables_should_have_an_indexer_and_keys() {
    use ulua_ast::records::parse_options::ParseOptions;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    // Faithful to the C++ R-string; the port injected stray `"` at each line start
    // (turning it into a malformed string literal).
    let stat = fixture.parse(
          "\n        local t: {\n            [string]: number,\n            f: () -> nil\n        }\n    ",
          &ParseOptions::default(),
      );
    assert!(!stat.is_null());
  }
}

mod parser_two_left_and_right_arrows_but_no_explicit_type_instantiation {

  #[cfg(test)]
  #[test]
  fn parser_two_left_and_right_arrows_but_no_explicit_type_instantiation() {
    use ulua_ast::records::parse_options::ParseOptions;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    let _stat = fixture.parse(r#"type A = C<B<<T>() -> T>>"#, &ParseOptions::default());
    assert!(!_stat.is_null());
  }
}

mod parser_type_alias_error_messages {

  #[cfg(test)]
  #[test]
  fn parser_type_alias_error_messages() {
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    fixture.match_parse_error(
      &String::from("type 5 = number"),
      &String::from("Expected identifier when parsing type name, got '5'"),
      None,
    );
    fixture.match_parse_error(
      &String::from("type A"),
      &String::from("Expected '=' when parsing type alias, got <eof>"),
      None,
    );
    fixture.match_parse_error(
      &String::from("type A<"),
      &String::from("Expected identifier, got <eof>"),
      None,
    );
    fixture.match_parse_error(
      &String::from("type A<B"),
      &String::from("Expected '>' (to close '<' at column 7), got <eof>"),
      None,
    );
  }
}

mod parser_type_alias_should_not_interfere_with_type_function_call_or_assignment {
  use super::*;
  #[cfg(test)]
  #[test]
  fn parser_type_alias_should_not_interfere_with_type_function_call_or_assignment() {
    use ulua_ast::{
      records::{
        ast_expr_call::AstExprCall, ast_stat_assign::AstStatAssign, ast_stat_expr::AstStatExpr,
        parse_options::ParseOptions,
      },
      rtti::{ast_node_as, ast_node_is},
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    let block = fixture.parse(
      "type(\"a\")\n\
           type = nil",
      &ParseOptions::default(),
    );
    assert!(!block.is_null());

    let block_ref = unsafe { &*block };
    assert!(block_ref.body.size > 0);

    let first_stat = unsafe { *block_ref.body.data.add(0) };
    assert!(!first_stat.is_null());

    let stat_expr = unsafe { ast_node_as::<AstStatExpr>(first_stat as *mut ast_node::AstNode) };
    assert!(!stat_expr.is_null());

    let expr_call =
      unsafe { ast_node_as::<AstExprCall>((*stat_expr).expr as *mut ast_node::AstNode) };
    assert!(!expr_call.is_null());

    let second_stat = unsafe { *block_ref.body.data.add(1) };
    assert!(!second_stat.is_null());

    let is_assign = ast_node_is::<AstStatAssign>(second_stat as *const ast_node::AstNode);
    assert!(is_assign);
  }
}

mod parser_type_alias_should_point_to_string {
  use super::*;
  #[cfg(test)]
  #[test]
  fn parser_type_alias_should_point_to_string() {
    use ulua_ast::records::parse_options::ParseOptions;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    let block = fixture.parse("type A = string", &ParseOptions::default());
    assert!(!block.is_null());
    let root = unsafe { &*block };
    assert!(root.body.size > 0);
    let first_stat = unsafe { &*root.body.data.add(0) };
    assert!(rtti::ast_node_is::<ast_stat_type_alias::AstStatTypeAlias>(
      *first_stat as *mut ast_node::AstNode
    ));
  }
}

mod parser_type_alias_should_work_when_name_is_also_local {
  use super::*;
  #[cfg(test)]
  #[test]
  fn parser_type_alias_should_work_when_name_is_also_local() {
    use ulua_ast::records::parse_options::ParseOptions;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    let block = fixture.parse(
      "local A = nil\n        type A = string",
      &ParseOptions::default(),
    );
    assert!(!block.is_null());
    let root = unsafe { &*block };
    assert_eq!(root.body.size, 2);
    let first_stat = unsafe { *root.body.data.add(0) };
    assert!(
      unsafe { &*first_stat }
        .base
        .is::<ast_stat_local::AstStatLocal>()
    );
    let second_stat = unsafe { *root.body.data.add(1) };
    assert!(
      unsafe { &*second_stat }
        .base
        .is::<ast_stat_type_alias::AstStatTypeAlias>()
    );
  }
}

mod parser_type_alias_span_is_correct {
  use super::*;
  #[cfg(test)]
  #[test]
  fn parser_type_alias_span_is_correct() {
    use ulua_ast::records::{
      ast_stat_type_alias::AstStatTypeAlias, location::Location, parse_options::ParseOptions,
      position::Position,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    let block = fixture.parse(
          "\n        type Packed1<T...> = (T...) -> (T...)\n        type Packed2<T...> = (Packed1<T...>, T...) -> (Packed1<T...>, T...)\n    ",
          &ParseOptions::default(),
      );
    assert!(!block.is_null());

    let root = unsafe { &*block };
    assert_eq!(root.body.size, 2);

    let t1 = unsafe {
      rtti::ast_node_as::<AstStatTypeAlias>(*root.body.data.add(0) as *mut ast_node::AstNode)
    };
    assert!(!t1.is_null());
    let t1 = unsafe { &*t1 };
    assert_eq!(
      t1.base.base.location,
      Location::new(Position::new(1, 8), Position::new(1, 45))
    );

    let t2 = unsafe {
      rtti::ast_node_as::<AstStatTypeAlias>(*root.body.data.add(1) as *mut ast_node::AstNode)
    };
    assert!(!t2.is_null());
    let t2 = unsafe { &*t2 };
    assert_eq!(
      t2.base.base.location,
      Location::new(Position::new(2, 8), Position::new(2, 75))
    );
  }
}

mod parser_type_alias_to_a_typeof {
  use super::*;
  #[cfg(test)]
  #[test]
  fn parser_type_alias_to_a_typeof() {
    use ulua_ast::records::{
      ast_stat_type_alias::AstStatTypeAlias, location::Location, parse_options::ParseOptions,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    let block = fixture.parse(
      // C++ R"(\n        type A = typeof(1)\n    )" — leading newline + 8-space
      // indent put the alias at {1,8}; the port dropped both.
      "\n        type A = typeof(1)\n    ",
      &ParseOptions::default(),
    );
    assert!(!block.is_null());

    let root = unsafe { &*block };
    assert!(root.body.size > 0);

    let type_alias_stat =
      unsafe { rtti::ast_node_as::<AstStatTypeAlias>(*root.body.data as *mut ast_node::AstNode) };
    assert!(!type_alias_stat.is_null());

    let expected_location = Location::new(UluaAstPosition::new(1, 8), UluaAstPosition::new(1, 26));
    assert_eq!(
      unsafe { (*type_alias_stat).base.base.location },
      expected_location
    );
  }
}

mod parser_type_assertion_expression {

  #[cfg(test)]
  #[test]
  fn parser_type_assertion_expression() {
    use ulua_ast::records::parse_options::ParseOptions;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    let _stat = fixture.parse(r#"local a = something() :: any"#, &ParseOptions::new());
    assert!(!_stat.is_null());
  }
}

mod parser_type_assertion_expression_binds_tightly {

  #[cfg(test)]
  #[test]
  fn parser_type_assertion_expression_binds_tightly() {
    use ulua_ast::{
      records::{
        ast_expr_binary::AstExprBinary, ast_expr_type_assertion::AstExprTypeAssertion,
        ast_node::AstNode, ast_stat_block::AstStatBlock, ast_stat_local::AstStatLocal,
        parse_options::ParseOptions,
      },
      rtti::ast_node_as,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    let stat = fixture.parse(
      "local a = one :: any + two :: any",
      &ParseOptions::default(),
    );
    let root = unsafe { &*stat };
    assert!(!root.body.data.is_null());

    let block = unsafe { ast_node_as::<AstStatBlock>(root as *const _ as *mut AstNode) };
    assert!(!block.is_null());
    assert_eq!(1, unsafe { (*block).body.size });

    let local = unsafe { ast_node_as::<AstStatLocal>(*(*block).body.data.add(0) as *mut AstNode) };
    assert!(!local.is_null());
    assert_eq!(1, unsafe { (*local).values.size });

    let bin = unsafe { ast_node_as::<AstExprBinary>(*(*local).values.data.add(0) as *mut AstNode) };
    assert!(!bin.is_null());

    assert!(!unsafe { ast_node_as::<AstExprTypeAssertion>((*bin).left as *mut AstNode).is_null() });
    assert!(!unsafe {
      ast_node_as::<AstExprTypeAssertion>((*bin).right as *mut AstNode).is_null()
    });
  }
}

mod parser_type_group_with_cst {
  use super::*;
  #[cfg(test)]
  #[test]
  fn parser_type_group_with_cst() {
    use alloc::string::String;

    use ulua_ast::{
      records::{
        ast_node::AstNode, ast_stat_type_alias::AstStatTypeAlias, ast_type_group::AstTypeGroup,
        cst_type_group::CstTypeGroup, parse_options::ParseOptions, position::Position,
      },
      rtti::{ast_node_as, cst_node_as},
    };
    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _ff = ScopedFastFlag::new(&FFlag::LuauCstTypeGroup, true);

    let mut fix = Fixture::default();
    let code = String::from("\n        type t = (number)\n    ");
    let mut parse_options = ParseOptions::new();
    parse_options.store_cst_data = true;
    let result = fix.parse_ex(&code, &parse_options);

    assert!(!result.root.is_null());
    assert_eq!(1, unsafe { (*result.root).body.size });

    let type_alias =
      unsafe { ast_node_as::<AstStatTypeAlias>(*(*result.root).body.data.add(0) as *mut AstNode) };
    assert!(!type_alias.is_null());

    let group = unsafe { ast_node_as::<AstTypeGroup>((*type_alias).type_ptr as *mut AstNode) };
    assert!(!group.is_null());

    let base_cst_node = result.cst_node_map.find(&(group as *mut AstNode));
    assert!(base_cst_node.is_some());

    let cst_node = unsafe { cst_node_as::<CstTypeGroup>(*base_cst_node.unwrap()) };
    assert!(!cst_node.is_null());

    assert_eq!(Position::new(1, 24), unsafe { (*cst_node).close_position });
  }
}

mod parser_type_names_can_contain_dots {

  #[cfg(test)]
  #[test]
  fn parser_type_names_can_contain_dots() {
    use ulua_ast::records::parse_options::ParseOptions;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    let block = fixture.parse("local foo: SomeModule.CoolType", &ParseOptions::default());
    assert!(!block.is_null());
  }
}

mod parser_unfinished_string_literal_types_get_reported_but_parsing_continues {
  use super::*;
  #[cfg(test)]
  #[test]
  fn parser_unfinished_string_literal_types_get_reported_but_parsing_continues() {
    use alloc::string::String;

    use ulua_ast::records::{location::Location, parse_options::ParseOptions, position::Position};
    use ulua_unit_test::records::fixture::Fixture;

    let mut fix = Fixture::default();
    let code = String::from("\n        type Foo = \"hi\n        print(foo)\n    ");
    let result = fix.try_parse(&code, &ParseOptions::new());

    assert_eq!(1, result.errors.len());

    let expected_location = Location::new(Position::new(1, 19), Position::new(1, 22));
    assert_eq!(expected_location, *result.errors[0].get_location());
    assert_eq!(
      "Malformed string; did you forget to finish it?",
      result.errors[0].get_message()
    );

    assert!(!result.root.is_null());
    assert_eq!(2, unsafe { (*result.root).body.size });
  }
}

mod parser_unfinished_string_literals_get_reported_but_parsing_continues {
  use super::*;
  #[cfg(test)]
  #[test]
  fn parser_unfinished_string_literals_get_reported_but_parsing_continues() {
    use alloc::string::String;

    use ulua_ast::records::{
      location::Location, parse_options::ParseOptions, parse_result::ParseResult,
      position::Position,
    };
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let source = String::from("\n        local foo = \"hi\n        print(foo)\n    ");
    let options = ParseOptions::new();
    let result: ParseResult = fixture.try_parse(&source, &options);

    assert_eq!(1, result.errors.len());

    let expected_location = Location::new(Position::new(1, 20), Position::new(1, 23));
    assert_eq!(expected_location, *result.errors[0].get_location());
    assert_eq!(
      "Malformed string; did you forget to finish it?",
      result.errors[0].get_message()
    );

    assert!(!result.root.is_null());
    assert_eq!(2, unsafe { (*result.root).body.size });
  }
}

mod parser_unknown_arguments_for_depricated_is_not_allowed {
  use super::*;
  #[cfg(test)]
  #[test]
  fn parser_unknown_arguments_for_depricated_is_not_allowed() {
    use alloc::string::String;

    use ulua_ast::records::{location::Location, parse_options::ParseOptions, position::Position};
    use ulua_unit_test::{
      functions::check_first_error_for_attributes::check_first_error_for_attributes,
      records::fixture::Fixture,
    };

    let mut fix = Fixture::default();

    let result = fix.try_parse(
      &String::from(
        "\n@[deprecated({}, \"Very deprecated\")]\nfunction hello(x, y)\n    return x + y\nend",
      ),
      &ParseOptions::default(),
    );
    check_first_error_for_attributes(
      &result.errors,
      1,
      Location::new(Position::new(1, 2), Position::new(1, 12)),
      "@deprecated can be parametrized only by 1 argument",
    );

    let result = fix.try_parse(
      &String::from(
        "\n@[deprecated \"Very deprecated\"]\nfunction hello(x, y)\n    return x + y\nend",
      ),
      &ParseOptions::default(),
    );
    check_first_error_for_attributes(
      &result.errors,
      1,
      Location::new(Position::new(1, 13), Position::new(1, 30)),
      "Unknown argument type for @deprecated",
    );

    let result = fix.try_parse(
      &String::from(
        "\n@[deprecated{ foo = \"bar\" }]\nfunction hello(x, y)\n    return x + y\nend",
      ),
      &ParseOptions::default(),
    );
    check_first_error_for_attributes(
      &result.errors,
      1,
      Location::new(Position::new(1, 14), Position::new(1, 17)),
      "Unknown argument 'foo' for @deprecated. Only string constants for 'use' and 'reason' are allowed",
    );

    let result = fix.try_parse(
      &String::from("\n@[deprecated{ use = 5 }]\nfunction hello(x, y)\n    return x + y\nend"),
      &ParseOptions::default(),
    );
    check_first_error_for_attributes(
      &result.errors,
      1,
      Location::new(Position::new(1, 20), Position::new(1, 21)),
      "Only constant string allowed as value for 'use'",
    );
  }
}

mod parser_unparenthesized_function_return_type_list {
  use super::*;
  #[cfg(test)]
  #[test]
  fn parser_unparenthesized_function_return_type_list() {
    use alloc::string::String;

    use ulua_ast::records::parse_options::ParseOptions;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    fixture.match_parse_error(
          &String::from("function foo(): string, number end"),
          &String::from("Expected a statement, got ','; did you forget to wrap the list of return types in parentheses?"),
          None,
      );
    fixture.match_parse_error(
          &String::from("function foo(): (number) -> string, string"),
          &String::from("Expected a statement, got ','; did you forget to wrap the list of return types in parentheses?"),
          None,
      );

    // Will throw if the parse fails
    fixture.parse(
      r#"
          type Vector3MT = {
              __add: (Vector3MT, Vector3MT) -> Vector3MT,
              __mul: (Vector3MT, Vector3MT|number) -> Vector3MT
          }
      "#,
      &ParseOptions::default(),
    );
  }
}

mod parser_unsupported_attributes_are_not_allowed {

  #[cfg(test)]
  #[test]
  fn parser_unsupported_attributes_are_not_allowed() {
    use ulua_ast::records::{location::Location, parse_options::ParseOptions, position::Position};
    use ulua_unit_test::{
      functions::check_first_error_for_attributes::check_first_error_for_attributes,
      records::fixture::Fixture,
    };

    let mut fix = Fixture::default();
    let code = "\n@checked\n    @cool_attribute\nfunction hello(x, y)\n    return x + y\nend";

    let result = fix.try_parse(code, &ParseOptions::default());

    let expected_location = Location::new(Position::new(2, 4), Position::new(2, 19));
    let expected_message = "Invalid attribute '@cool_attribute'";

    check_first_error_for_attributes(&result.errors, 1, expected_location, expected_message);
  }
}

mod parser_variadic_definition_parsing {
  use super::*;
  #[cfg(test)]
  #[test]
  fn parser_variadic_definition_parsing() {
    use alloc::string::String;

    use ulua_ast::records::parse_options::ParseOptions;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    let stat = fixture.parse_ex(
      &String::from(
        "declare function foo(...: string): ...string\n\
               declare extern type Foo with\n\
                   function a(self, ...: string): ...string\n\
               end",
      ),
      &ParseOptions::default(),
    );
    let root = unsafe { &*stat.root };
    assert!(!root.body.data.is_null());

    fixture.match_parse_error(
      &String::from("declare function foo(...)"),
      &String::from("All declaration parameters must be annotated"),
      None,
    );
    fixture.match_parse_error(
      &String::from("declare extern type Foo with function a(self, ...) end"),
      &String::from("All declaration parameters aside from 'self' must be annotated"),
      None,
    );
  }
}

mod parser_variadics_must_be_last {
  use super::*;
  #[cfg(test)]
  #[test]
  fn parser_variadics_must_be_last() {
    use alloc::string::String;

    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    fixture.match_parse_error(
      &String::from("function foo(): (...number, string) end"),
      &String::from("Expected ')' (to close '(' at column 17), got ','"),
      None,
    );
    fixture.match_parse_error(
      &String::from("type Foo = (...number, string) -> (...string, number)"),
      &String::from("Expected ')' (to close '(' at column 12), got ','"),
      None,
    );
  }
}

mod parser_vertical_space {
  use super::*;
  #[cfg(test)]
  #[test]
  fn parser_vertical_space() {
    use alloc::string::String;

    use ulua_ast::records::parse_options::ParseOptions;
    use ulua_unit_test::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    // C++ "a()\vb()" — \v is a vertical tab (U+000B). Rust has no \v escape and a
    // raw string would embed a literal backslash-v, so use an explicit \u escape.
    let source = String::from("a()\u{0B}b()");
    let options = ParseOptions::new();
    let result = fixture.parse_ex(&source, &options);
    assert!(result.errors.is_empty());
  }
}
