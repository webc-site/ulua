use alloc::vec::Vec;

use ulua_analysis::type_aliases::module_name_type::ModuleName;
extern crate alloc;

// 源 cpp 侧为整文件共享的 using 声明；fn 体内逐例重复的 use 统一上提至此（借 tst-r16/tst-r17 上提先例）。
use alloc::string::String;

use ulua_analysis::enums::{
  autocomplete_context::AutocompleteContext, autocomplete_entry_kind::AutocompleteEntryKind,
  parentheses_recommendation::ParenthesesRecommendation, type_correct_kind::TypeCorrectKind,
};
use ulua_ast::records::position::Position;
use ulua_common::fflag;
use ulua_unit_test::{
  functions::null_callback_autocomplete_test::null_callback,
  records::{ac_builtins_fixture::ACBuiltinsFixture, ac_fixture::AcFixture},
  type_aliases::scoped_fast_flag::ScopedFastFlag,
};

// 样板收口助手：原逐例重复的 AcFixture/ACBuiltinsFixture 构造 + check(源文本) +
// autocomplete_marker 恒等三件套收口为宏，行为与原语句逐字一致（借 tst-r16/tst-r17 助手宏先例）。
macro_rules! ac_check {
  ($src:expr) => {
    ac_check!($src, '1')
  };
  ($src:expr, $marker:expr) => {{
    let mut fixture = AcFixture::default();
    fixture.base.check($src);
    let ac = fixture.base.autocomplete_marker($marker);
    (fixture, ac)
  }};
}

macro_rules! acb_check {
  ($src:expr) => {
    acb_check!($src, '1')
  };
  ($src:expr, $marker:expr) => {{
    let mut fixture = ACBuiltinsFixture::default();
    fixture.base.check($src);
    let ac = fixture.base.autocomplete_marker($marker);
    (fixture, ac)
  }};
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_ac_dont_overflow_on_recursive_union() {
  use ulua_unit_test::{
    functions::register_ac_extern_type_fixture_types::register_ac_extern_type_fixture_types,
    records::ac_extern_type_fixture::AcExternTypeFixture,
  };

  let mut fixture = AcExternTypeFixture::default();
  register_ac_extern_type_fixture_types(&mut fixture.base);

  fixture.base.check(
    r#"
        local table1: {ChildClass} = {}
        local table2 = {}

        for index, value in table2[1] do
            table.insert(table1, value)
            value.@1
        end
    "#,
  );

  let ac = fixture.base.autocomplete_marker('1');

  if !fflag::DebugLuauForceOldSolver.get() {
    assert!(
      ac.entry_map.contains_key("BaseMethod"),
      "entries: {:?}",
      ac.entry_map.keys().collect::<Vec<_>>()
    );
    assert!(
      ac.entry_map.contains_key("Method"),
      "entries: {:?}",
      ac.entry_map.keys().collect::<Vec<_>>()
    );
  } else {
    assert!(ac.entry_map.is_empty());
  }
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_ac_static_method_autocomplete() {
  let _new_solver = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);
  let _classes = ScopedFastFlag::new(&fflag::DebugLuauUserDefinedClasses, true);

  let (_fixture, ac) = ac_check!(
    r#"
        class Bar
            public value: number
            function new()
                return Bar { value = 0 }
            end
        end

        Bar.@1
    "#,
    '1'
  );
  assert!(ac.entry_map.contains_key("new"));
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_anonymous_autofilled_args() {
  let (_fixture, ac) = ac_check!(
    r#"
local function foo(a: (number, string) -> ())
    a()
end

foo(@1)
    "#,
    '1'
  );
  let entry = ac
    .entry_map
    .get("function (anonymous autofilled)")
    .expect("generated anonymous function completion");

  assert_eq!(entry.kind, AutocompleteEntryKind::GeneratedFunction);
  assert_eq!(entry.type_correct, TypeCorrectKind::Correct);
  assert_eq!(
    entry.insert_text.as_deref(),
    Some("function(a0: number, a1: string)  end")
  );
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_anonymous_autofilled_args_multi_return() {
  let (_fixture, ac) = ac_check!(
    r#"
local function foo(a: (number, string) -> (string, number))
    a()
end

foo(@1)
    "#,
    '1'
  );
  let entry = ac
    .entry_map
    .get("function (anonymous autofilled)")
    .expect("generated anonymous function completion");

  assert_eq!(entry.kind, AutocompleteEntryKind::GeneratedFunction);
  assert_eq!(entry.type_correct, TypeCorrectKind::Correct);
  assert_eq!(
    entry.insert_text.as_deref(),
    Some("function(a0: number, a1: string): (string, number)  end")
  );
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_anonymous_autofilled_args_single_return() {
  let (_fixture, ac) = ac_check!(
    r#"
local function foo(a: (number, string) -> (string))
    a()
end

foo(@1)
    "#,
    '1'
  );
  let entry = ac
    .entry_map
    .get("function (anonymous autofilled)")
    .expect("generated anonymous function completion");

  assert_eq!(entry.kind, AutocompleteEntryKind::GeneratedFunction);
  assert_eq!(entry.type_correct, TypeCorrectKind::Correct);
  assert_eq!(
    entry.insert_text.as_deref(),
    Some("function(a0: number, a1: string): string  end")
  );
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_anonymous_autofilled_empty() {
  let (_fixture, ac) = ac_check!(
    r#"
local function foo(a: () -> ())
    a()
end

foo(@1)
    "#,
    '1'
  );
  let entry = ac
    .entry_map
    .get("function (anonymous autofilled)")
    .expect("generated anonymous function completion");

  assert_eq!(entry.kind, AutocompleteEntryKind::GeneratedFunction);
  assert_eq!(entry.type_correct, TypeCorrectKind::Correct);
  assert_eq!(entry.insert_text.as_deref(), Some("function()  end"));
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_anonymous_autofilled_generic_named_arg() {
  let (_fixture, ac) = ac_check!(
    r#"
local function foo<A>(f: (a: A) -> number, a: A)
	return f(a)
end

foo(@1)
    "#,
    '1'
  );
  let entry = ac
    .entry_map
    .get("function (anonymous autofilled)")
    .expect("generated anonymous function completion");

  assert_eq!(entry.kind, AutocompleteEntryKind::GeneratedFunction);
  assert_eq!(entry.type_correct, TypeCorrectKind::Correct);
  assert_eq!(
    entry.insert_text.as_deref(),
    Some("function(a): number  end")
  );
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_anonymous_autofilled_generic_on_argument_type_pack_vararg() {
  let (_fixture, ac) = ac_check!(
    r#"
        local function foo(a: <T...>(...: T...) -> number)
            return a(4, 5, 6)
        end

        foo(@1)
    "#,
    '1'
  );
  let entry = ac
    .entry_map
    .get("function (anonymous autofilled)")
    .expect("generated anonymous function completion");

  assert_eq!(entry.kind, AutocompleteEntryKind::GeneratedFunction);
  assert_eq!(entry.type_correct, TypeCorrectKind::Correct);
  let expected_insert = if !fflag::DebugLuauForceOldSolver.get() {
    "function(...: number): number  end"
  } else {
    "function(...): number  end"
  };
  assert_eq!(entry.insert_text.as_deref(), Some(expected_insert));
}

// Source: `tests/Autocomplete.test.cpp:4641`
//
// 光标紧跟 `function` 关键字之后、`(` 尚未键入（AstExprFunction 无
// argLocation）——此时必须补全整个 `function(...) end` 表达式，而非裸参数表。
#[test]
fn autocomplete_anonymous_autofilled_cursor_after_function_keyword() {
  let (_fixture, ac) = ac_check!(
    r#"
local function foo(a: (number, string) -> ())
    a()
end

foo(function@1)
    "#,
    '1'
  );
  let entry = ac
    .entry_map
    .get("function (anonymous autofilled)")
    .expect("generated anonymous function completion");

  assert_eq!(entry.kind, AutocompleteEntryKind::GeneratedFunction);
  assert_eq!(entry.type_correct, TypeCorrectKind::Correct);
  // cpp CHECK_EQ("function(a0: number, a1: string)  end", *insertText)
  assert_eq!(
    entry.insert_text.as_deref(),
    Some("function(a0: number, a1: string)  end")
  );
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_anonymous_autofilled_generic_return_type() {
  let (_fixture, ac) = ac_check!(
    r#"
local function foo<A>(f: () -> A)
	return f()
end

foo(@1)
    "#,
    '1'
  );
  let entry = ac
    .entry_map
    .get("function (anonymous autofilled)")
    .expect("generated anonymous function completion");

  assert_eq!(entry.kind, AutocompleteEntryKind::GeneratedFunction);
  assert_eq!(entry.type_correct, TypeCorrectKind::Correct);
  assert_eq!(entry.insert_text.as_deref(), Some("function()  end"));
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_anonymous_autofilled_generic_type_pack_vararg() {
  let (_fixture, ac) = ac_check!(
    r#"
local function foo<A>(a: (...A) -> number, ...: A)
	return a(...)
end

foo(@1)
    "#,
    '1'
  );
  let entry = ac
    .entry_map
    .get("function (anonymous autofilled)")
    .expect("generated anonymous function completion");

  assert_eq!(entry.kind, AutocompleteEntryKind::GeneratedFunction);
  assert_eq!(entry.type_correct, TypeCorrectKind::Correct);
  assert_eq!(
    entry.insert_text.as_deref(),
    Some("function(...): number  end")
  );
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_anonymous_autofilled_multi_varargs_multi_return() {
  let (_fixture, ac) = ac_check!(
    r#"
local function foo(a: (string, ...number) -> (string, number))
    a()
end

foo(@1)
    "#,
    '1'
  );
  let entry = ac
    .entry_map
    .get("function (anonymous autofilled)")
    .expect("generated anonymous function completion");

  assert_eq!(entry.kind, AutocompleteEntryKind::GeneratedFunction);
  assert_eq!(entry.type_correct, TypeCorrectKind::Correct);
  assert_eq!(
    entry.insert_text.as_deref(),
    Some("function(a0: string, ...: number): (string, number)  end")
  );
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_anonymous_autofilled_multi_varargs_multi_varargs_return() {
  let (_fixture, ac) = ac_check!(
    r#"
local function foo(a: (string, ...number) -> (boolean, ...number))
    a()
end

foo(@1)
    "#,
    '1'
  );
  let entry = ac
    .entry_map
    .get("function (anonymous autofilled)")
    .expect("generated anonymous function completion");

  assert_eq!(entry.kind, AutocompleteEntryKind::GeneratedFunction);
  assert_eq!(entry.type_correct, TypeCorrectKind::Correct);
  assert_eq!(
    entry.insert_text.as_deref(),
    Some("function(a0: string, ...: number): (boolean, ...number)  end")
  );
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_anonymous_autofilled_multi_varargs_varargs_return() {
  let (_fixture, ac) = ac_check!(
    r#"
local function foo(a: (string, ...number) -> ...number)
    a()
end

foo(@1)
    "#,
    '1'
  );
  let entry = ac
    .entry_map
    .get("function (anonymous autofilled)")
    .expect("generated anonymous function completion");

  assert_eq!(entry.kind, AutocompleteEntryKind::GeneratedFunction);
  assert_eq!(entry.type_correct, TypeCorrectKind::Correct);
  assert_eq!(
    entry.insert_text.as_deref(),
    Some("function(a0: string, ...: number): ...number  end")
  );
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_anonymous_autofilled_named_args() {
  let (_fixture, ac) = ac_check!(
    r#"
local function foo(a: (foo: number, bar: string) -> (string, number))
    a()
end

foo(@1)
    "#,
    '1'
  );
  let entry = ac
    .entry_map
    .get("function (anonymous autofilled)")
    .expect("generated anonymous function completion");

  assert_eq!(entry.kind, AutocompleteEntryKind::GeneratedFunction);
  assert_eq!(entry.type_correct, TypeCorrectKind::Correct);
  assert_eq!(
    entry.insert_text.as_deref(),
    Some("function(foo: number, bar: string): (string, number)  end")
  );
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_anonymous_autofilled_noargs_multi_return() {
  let (_fixture, ac) = ac_check!(
    r#"
local function foo(a: () -> (string, number))
    a()
end

foo(@1)
    "#,
    '1'
  );
  let entry = ac
    .entry_map
    .get("function (anonymous autofilled)")
    .expect("generated anonymous function completion");

  assert_eq!(entry.kind, AutocompleteEntryKind::GeneratedFunction);
  assert_eq!(entry.type_correct, TypeCorrectKind::Correct);
  assert_eq!(
    entry.insert_text.as_deref(),
    Some("function(): (string, number)  end")
  );
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_anonymous_autofilled_partially_args() {
  let (_fixture, ac) = ac_check!(
    r#"
local function foo(a: (number, bar: string) -> (string, number))
    a()
end

foo(@1)
    "#,
    '1'
  );
  let entry = ac
    .entry_map
    .get("function (anonymous autofilled)")
    .expect("generated anonymous function completion");

  assert_eq!(entry.kind, AutocompleteEntryKind::GeneratedFunction);
  assert_eq!(entry.type_correct, TypeCorrectKind::Correct);
  assert_eq!(
    entry.insert_text.as_deref(),
    Some("function(a0: number, bar: string): (string, number)  end")
  );
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_anonymous_autofilled_partially_args_last() {
  let (_fixture, ac) = ac_check!(
    r#"
local function foo(a: (foo: number, string) -> (string, number))
    a()
end

foo(@1)
    "#,
    '1'
  );
  let entry = ac
    .entry_map
    .get("function (anonymous autofilled)")
    .expect("generated anonymous function completion");

  assert_eq!(entry.kind, AutocompleteEntryKind::GeneratedFunction);
  assert_eq!(entry.type_correct, TypeCorrectKind::Correct);
  assert_eq!(
    entry.insert_text.as_deref(),
    Some("function(foo: number, a1: string): (string, number)  end")
  );
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_anonymous_autofilled_table_literal_args() {
  let (_fixture, ac) = ac_check!(
    r#"
local function foo(a: (tbl: { x: number, y: number }) -> number) return a({x=2, y = 3}) end
foo(@1)
    "#,
    '1'
  );
  let entry = ac
    .entry_map
    .get("function (anonymous autofilled)")
    .expect("generated anonymous function completion");

  assert_eq!(entry.kind, AutocompleteEntryKind::GeneratedFunction);
  assert_eq!(entry.type_correct, TypeCorrectKind::Correct);
  assert_eq!(
    entry.insert_text.as_deref(),
    Some("function(tbl: { x: number, y: number }): number  end")
  );
}

mod autocomplete_anonymous_autofilled_table_literal_args_autocomplete_test_case_2 {
  //! Source: `tests/Autocomplete.test.cpp`
  use super::{AcFixture, AutocompleteEntryKind, TypeCorrectKind};

  #[test]
  fn autocomplete_anonymous_autofilled_table_literal_args() {
    let mut fixture = AcFixture::default();
    fixture.base.check(
      r#"
local function foo(a: () -> { x: number, y: number }) return {x=2, y = 3} end
foo(@1)
    "#,
    );

    let ac = fixture.base.autocomplete_marker('1');
    let entry = ac
      .entry_map
      .get("function (anonymous autofilled)")
      .expect("generated anonymous function completion");

    assert_eq!(entry.kind, AutocompleteEntryKind::GeneratedFunction);
    assert_eq!(entry.type_correct, TypeCorrectKind::Correct);
    assert_eq!(
      entry.insert_text.as_deref(),
      Some("function(): { x: number, y: number }  end")
    );
  }
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_anonymous_autofilled_typeof_args() {
  let (_fixture, ac) = ac_check!(
    r#"
local t = { a = 1, b = 2 }

local function foo(a: (foo: typeof(t)) -> ())
    a()
end

foo(@1)
    "#,
    '1'
  );
  let entry = ac
    .entry_map
    .get("function (anonymous autofilled)")
    .expect("generated anonymous function completion");

  assert_eq!(entry.kind, AutocompleteEntryKind::GeneratedFunction);
  assert_eq!(entry.type_correct, TypeCorrectKind::Correct);
  assert_eq!(entry.insert_text.as_deref(), Some("function(foo)  end"));
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_anonymous_autofilled_typeof_returns() {
  let (_fixture, ac) = ac_check!(
    r#"
local t = { a = 1, b = 2 }

local function foo(a: () -> typeof(t))
    a()
end

foo(@1)
    "#,
    '1'
  );
  let entry = ac
    .entry_map
    .get("function (anonymous autofilled)")
    .expect("generated anonymous function completion");

  assert_eq!(entry.kind, AutocompleteEntryKind::GeneratedFunction);
  assert_eq!(entry.type_correct, TypeCorrectKind::Correct);
  assert_eq!(entry.insert_text.as_deref(), Some("function()  end"));
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_anonymous_autofilled_typeof_vararg() {
  let (_fixture, ac) = ac_check!(
    r#"
local t = { a = 1, b = 2 }

local function foo(a: (...typeof(t)) -> ())
    a()
end

foo(@1)
    "#,
    '1'
  );
  let entry = ac
    .entry_map
    .get("function (anonymous autofilled)")
    .expect("generated anonymous function completion");

  assert_eq!(entry.kind, AutocompleteEntryKind::GeneratedFunction);
  assert_eq!(entry.type_correct, TypeCorrectKind::Correct);
  assert_eq!(entry.insert_text.as_deref(), Some("function(...)  end"));
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_anonymous_autofilled_varargs_multi_return() {
  let (_fixture, ac) = ac_check!(
    r#"
local function foo(a: (...number) -> (string, number))
    a()
end

foo(@1)
    "#,
    '1'
  );
  let entry = ac
    .entry_map
    .get("function (anonymous autofilled)")
    .expect("generated anonymous function completion");

  assert_eq!(entry.kind, AutocompleteEntryKind::GeneratedFunction);
  assert_eq!(entry.type_correct, TypeCorrectKind::Correct);
  assert_eq!(
    entry.insert_text.as_deref(),
    Some("function(...: number): (string, number)  end")
  );
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_argument_types() {
  let (_fixture, ac) = ac_check!(
    r#"
local function f(a: n@1
local b: string = "don't trip"
    "#,
    '1'
  );

  assert!(ac.entry_map.contains_key("nil"));
  assert!(ac.entry_map.contains_key("number"));
  assert_eq!(ac.context, AutocompleteContext::Type);
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_arguments_to_global_lambda() {
  let (_fixture, ac) = ac_check!(
    r#"
        abc = function(def, ghi@1)
        end
    "#,
    '1'
  );
  assert!(ac.entry_map.is_empty());
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_as_types() {
  let (_fixture, ac) = ac_check!(
    r#"
local a: any = 5
local b: number = (a :: n@1
    "#,
    '1'
  );

  assert!(ac.entry_map.contains_key("nil"));
  assert!(ac.entry_map.contains_key("number"));
  assert_eq!(ac.context, AutocompleteContext::Type);
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_autocomplete_after_semicolon_should_complete_a_new_statement() {
  let (_fixture, ac) = ac_check!(
    r#"
local data = { x = 1 }
local var = data;@1
    "#,
    '1'
  );
  assert!(!ac.entry_map.is_empty());
  assert!(ac.entry_map.contains_key("table"));
  assert!(ac.entry_map.contains_key("math"));
  assert_eq!(ac.context, AutocompleteContext::Statement);
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_autocomplete_at_end_of_stmt_should_continue_as_part_of_stmt() {
  let (_fixture, ac) = ac_check!(
    r#"
local data = { x = 1 }
local var = data.@1
    "#,
    '1'
  );
  assert!(!ac.entry_map.is_empty());
  assert!(ac.entry_map.contains_key("x"));
  assert_eq!(ac.context, AutocompleteContext::Property);
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_autocomplete_boolean_singleton() {
  let (_fixture, ac) = ac_check!(
    r#"
local function f(x: true) end
f(@1)
    "#,
    '1'
  );

  assert!(ac.entry_map.contains_key("true"));
  assert_eq!(ac.entry_map["true"].type_correct, TypeCorrectKind::Correct);
  assert!(ac.entry_map.contains_key("false"));
  assert_eq!(ac.entry_map["false"].type_correct, TypeCorrectKind::None);
  assert_eq!(ac.context, AutocompleteContext::Expression);
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_autocomplete_default_type_pack_parameters() {
  let (_fixture, ac) = ac_check!(
    r#"
type A<T... = ...@1> = () -> T
    "#,
    '1'
  );

  assert!(ac.entry_map.contains_key("number"));
  assert!(ac.entry_map.contains_key("string"));
  assert_eq!(ac.context, AutocompleteContext::Type);
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_autocomplete_default_type_parameters() {
  let (_fixture, ac) = ac_check!(
    r#"
type A<T = @1> = () -> T
    "#,
    '1'
  );

  assert!(ac.entry_map.contains_key("number"));
  assert!(ac.entry_map.contains_key("string"));
  assert_eq!(ac.context, AutocompleteContext::Type);
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_autocomplete_deprecated_attribute() {
  let (_fixture, ac) = acb_check!(
    r#"
        \@dep@1
        function foo() return 42 end
    "#,
    '1'
  );
  assert!(ac.entry_map.contains_key("deprecated"));
  assert!(ac.entry_map.contains_key("checked"));
  assert!(ac.entry_map.contains_key("native"));
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_autocomplete_deprecated_braced_attribute() {
  let (_fixture, ac) = acb_check!(
    r#"
        \@[dep@1]
        function foo() return 42 end
    "#,
    '1'
  );
  assert!(ac.entry_map.contains_key("deprecated"));
  assert!(ac.entry_map.contains_key("checked"));
  assert!(ac.entry_map.contains_key("native"));
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_autocomplete_documentation_symbols() {
  let mut fixture = AcFixture::default();
  fixture.base.load_definition(
    r#"
        declare y: {
            x: number,
        }
    "#,
  );

  fixture.base.check(
    r#"
        local a = y.@1
    "#,
  );

  let ac = fixture.base.autocomplete_marker('1');

  assert!(ac.entry_map.contains_key("x"));
  assert_eq!(
    ac.entry_map["x"].documentation_symbol,
    Some(String::from("@test/global/y.x"))
  );
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_autocomplete_empty_attribute() {
  let (_fixture, ac) = acb_check!(
    r#"
        \@@1
        function foo() return 42 end
    "#,
    '1'
  );
  assert!(ac.entry_map.contains_key("deprecated"));
  assert!(ac.entry_map.contains_key("checked"));
  assert!(ac.entry_map.contains_key("native"));
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_autocomplete_empty_braced_attribute() {
  let (_fixture, ac) = acb_check!(
    r#"
        \@[@1]
        function foo() return 42 end
    "#,
    '1'
  );
  assert!(ac.entry_map.contains_key("deprecated"));
  assert!(ac.entry_map.contains_key("checked"));
  assert!(ac.entry_map.contains_key("native"));
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_autocomplete_end_of_do_block() {
  let mut fixture = AcFixture::default();
  fixture.base.check("do @1");

  let mut ac = fixture.base.autocomplete_marker('1');
  assert!(ac.entry_map.contains_key("end"));

  fixture.base.check(
    r#"
        function f()
            do
                @1
        end
        @2
    "#,
  );

  ac = fixture.base.autocomplete_marker('1');
  assert!(ac.entry_map.contains_key("end"));

  ac = fixture.base.autocomplete_marker('2');
  assert!(ac.entry_map.contains_key("end"));
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_autocomplete_end_with_fn_exprs() {
  let (_fixture, ac) = ac_check!(
    r#"
        local function f()  @1
    "#,
    '1'
  );
  assert_eq!(1, ac.entry_map.get("end").map_or(0, |_| 1));
  assert_eq!(ac.context, AutocompleteContext::Statement);
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_autocomplete_end_with_lambda() {
  let (_fixture, ac) = ac_check!(
    r#"
        local a = function() local bar = foo en@1
    "#,
    '1'
  );
  assert_eq!(1, ac.entry_map.get("end").map_or(0, |_| 1));
  assert_eq!(ac.context, AutocompleteContext::Statement);
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_autocomplete_exclude_break_continue_expr_func() {
  let (_fixture, ac) = ac_check!(
    r#"while true do
        local _ = function ()
        @1
        end
    end"#,
    '1'
  );
  assert!(!ac.entry_map.contains_key("break"));
  assert!(!ac.entry_map.contains_key("continue"));
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_autocomplete_exclude_break_continue_function_boundary() {
  let (_fixture, ac) = ac_check!(
    r#"for i = 1, 10 do
    local function helper()
        @1
    end
    end"#,
    '1'
  );
  assert!(!ac.entry_map.contains_key("break"));
  assert!(!ac.entry_map.contains_key("continue"));
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_autocomplete_exclude_break_continue_in_incomplete_loop() {
  let (_fixture, ac) = ac_check!(
    r#"while foo() do
        @1"#,
    '1'
  );
  assert!(!ac.entry_map.contains_key("break"));
  assert!(!ac.entry_map.contains_key("continue"));
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_autocomplete_exclude_break_continue_in_param() {
  let (_fixture, ac) = ac_check!(
    r#"while @1 do
        end"#,
    '1'
  );
  assert!(!ac.entry_map.contains_key("break"));
  assert!(!ac.entry_map.contains_key("continue"));
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_autocomplete_exclude_break_continue_incomplete_for() {
  let mut fixture = AcFixture::default();
  fixture.base.check("for @1 in @2 do");

  let ac1 = fixture.base.autocomplete_marker('1');
  assert!(!ac1.entry_map.contains_key("break"));
  assert!(!ac1.entry_map.contains_key("continue"));

  let ac2 = fixture.base.autocomplete_marker('2');
  assert!(!ac2.entry_map.contains_key("break"));
  assert!(!ac2.entry_map.contains_key("continue"));
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_autocomplete_exclude_break_continue_incomplete_while() {
  let mut fixture = AcFixture::default();
  fixture.base.check("while @1");

  let ac = fixture.base.autocomplete_marker('1');
  assert!(!ac.entry_map.contains_key("break"));
  assert!(!ac.entry_map.contains_key("continue"));
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_autocomplete_exclude_break_continue_outside_loop() {
  let (mut fixture, ac1) = ac_check!(
    r#"@1if true then
        @2
    end"#,
    '1'
  );
  assert!(!ac1.entry_map.contains_key("break"));
  assert!(!ac1.entry_map.contains_key("continue"));

  let ac2 = fixture.base.autocomplete_marker('2');
  assert!(!ac2.entry_map.contains_key("break"));
  assert!(!ac2.entry_map.contains_key("continue"));
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_autocomplete_explicit_type_pack() {
  let (_fixture, ac) = ac_check!(
    r#"
type A<T...> = () -> T...
local a: A<(number, s@1>
    "#,
    '1'
  );

  assert!(ac.entry_map.contains_key("number"));
  assert!(ac.entry_map.contains_key("string"));
  assert_eq!(ac.context, AutocompleteContext::Type);
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_autocomplete_first_function_arg_expected_type() {
  let (mut fixture, ac1) = ac_check!(
    r#"
local function foo1() return 1 end
local function foo2() return "1" end

local function bar0() return "got" .. a end
local function bar1(a: number) return "got " .. a end
local function bar2(a: number, b: string) return "got " .. a .. b end

local t = {}
function t:bar1(a: number) return "got " .. a end

local r1 = bar0(@1)
local r2 = bar1(@2)
local r3 = bar2(@3)
local r4 = t:bar1(@4)
    "#,
    '1'
  );

  assert!(ac1.entry_map.contains_key("foo1"));
  assert_eq!(ac1.entry_map["foo1"].type_correct, TypeCorrectKind::None);
  assert!(ac1.entry_map.contains_key("foo2"));
  assert_eq!(ac1.entry_map["foo2"].type_correct, TypeCorrectKind::None);

  let ac2 = fixture.base.autocomplete_marker('2');

  assert!(ac2.entry_map.contains_key("foo1"));
  assert_eq!(
    ac2.entry_map["foo1"].type_correct,
    TypeCorrectKind::CorrectFunctionResult
  );
  assert!(ac2.entry_map.contains_key("foo2"));
  assert_eq!(ac2.entry_map["foo2"].type_correct, TypeCorrectKind::None);

  let ac3 = fixture.base.autocomplete_marker('3');

  assert!(ac3.entry_map.contains_key("foo1"));
  assert_eq!(
    ac3.entry_map["foo1"].type_correct,
    TypeCorrectKind::CorrectFunctionResult
  );
  assert!(ac3.entry_map.contains_key("foo2"));
  assert_eq!(ac3.entry_map["foo2"].type_correct, TypeCorrectKind::None);

  let ac4 = fixture.base.autocomplete_marker('4');

  assert!(ac4.entry_map.contains_key("foo1"));
  assert_eq!(
    ac4.entry_map["foo1"].type_correct,
    TypeCorrectKind::CorrectFunctionResult
  );
  assert!(ac4.entry_map.contains_key("foo2"));
  assert_eq!(ac4.entry_map["foo2"].type_correct, TypeCorrectKind::None);
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_autocomplete_for_assignment() {
  let (_fixture, ac) = ac_check!(
    r#"
        local function foobar(tbl: { tag: "left" | "right" })
            tbl.tag = "@1"
        end
    "#,
    '1'
  );
  assert!(ac.entry_map.contains_key("left"));
  assert!(ac.entry_map.contains_key("right"));
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_autocomplete_for_in_middle_keywords() {
  let (mut fixture, ac1) = ac_check!(
    r#"
        for @1
    "#,
    '1'
  );
  assert_eq!(0, ac1.entry_map.len());
  assert_eq!(ac1.context, AutocompleteContext::Unknown);

  fixture.base.check(
    r#"
        for x@1 @2
    "#,
  );

  let ac2 = fixture.base.autocomplete_marker('1');
  assert_eq!(0, ac2.entry_map.len());
  assert_eq!(ac2.context, AutocompleteContext::Unknown);

  let ac2a = fixture.base.autocomplete_marker('2');
  assert_eq!(1, ac2a.entry_map.len());
  assert_eq!(1, ac2a.entry_map.get("in").map_or(0, |_| 1));
  assert_eq!(ac2a.context, AutocompleteContext::Keyword);

  fixture.base.check(
    r#"
        for x in y@1
    "#,
  );

  let ac3 = fixture.base.autocomplete_marker('1');
  assert_eq!(1, ac3.entry_map.get("table").map_or(0, |_| 1));
  assert_eq!(0, ac3.entry_map.get("do").map_or(0, |_| 1));
  assert_eq!(ac3.context, AutocompleteContext::Expression);

  fixture.base.check(
    r#"
        for x in y @1
    "#,
  );

  let ac4 = fixture.base.autocomplete_marker('1');
  assert_eq!(1, ac4.entry_map.len());
  assert_eq!(1, ac4.entry_map.get("do").map_or(0, |_| 1));
  assert_eq!(ac4.context, AutocompleteContext::Keyword);

  fixture.base.check(
    r#"
        for x in f f@1
    "#,
  );

  let ac5 = fixture.base.autocomplete_marker('1');
  assert_eq!(1, ac5.entry_map.len());
  assert_eq!(1, ac5.entry_map.get("do").map_or(0, |_| 1));
  assert_eq!(ac5.context, AutocompleteContext::Keyword);

  fixture.base.check(
    r#"
        for x in y do  @1
    "#,
  );

  let ac6 = fixture.base.autocomplete_marker('1');
  assert_eq!(0, ac6.entry_map.get("in").map_or(0, |_| 1));
  assert_eq!(1, ac6.entry_map.get("table").map_or(0, |_| 1));
  assert_eq!(1, ac6.entry_map.get("end").map_or(0, |_| 1));
  assert_eq!(1, ac6.entry_map.get("function").map_or(0, |_| 1));
  assert_eq!(ac6.context, AutocompleteContext::Statement);

  fixture.base.check(
    r#"
        for x in y do e@1
    "#,
  );

  let ac7 = fixture.base.autocomplete_marker('1');
  assert_eq!(0, ac7.entry_map.get("in").map_or(0, |_| 1));
  assert_eq!(1, ac7.entry_map.get("table").map_or(0, |_| 1));
  assert_eq!(1, ac7.entry_map.get("end").map_or(0, |_| 1));
  assert_eq!(1, ac7.entry_map.get("function").map_or(0, |_| 1));
  assert_eq!(ac7.context, AutocompleteContext::Statement);
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_autocomplete_for_middle_keywords() {
  let (mut fixture, ac1) = ac_check!(
    r#"
        for x @1=
    "#,
    '1'
  );
  assert_eq!(0, ac1.entry_map.get("do").map_or(0, |_| 1));
  assert_eq!(0, ac1.entry_map.get("end").map_or(0, |_| 1));
  assert_eq!(ac1.context, AutocompleteContext::Unknown);

  fixture.base.check(
    r#"
        for x =@1 1
    "#,
  );

  let ac2 = fixture.base.autocomplete_marker('1');
  assert_eq!(0, ac2.entry_map.get("do").map_or(0, |_| 1));
  assert_eq!(0, ac2.entry_map.get("end").map_or(0, |_| 1));
  assert_eq!(ac2.context, AutocompleteContext::Unknown);

  fixture.base.check(
    r#"
        for x = 1,@1 2
    "#,
  );

  let ac3 = fixture.base.autocomplete_marker('1');
  assert_eq!(1, ac3.entry_map.len());
  assert_eq!(1, ac3.entry_map.get("do").map_or(0, |_| 1));
  assert_eq!(ac3.context, AutocompleteContext::Keyword);

  fixture.base.check(
    r#"
        for x = 1, @12,
    "#,
  );

  let ac4 = fixture.base.autocomplete_marker('1');
  assert_eq!(0, ac4.entry_map.get("do").map_or(0, |_| 1));
  assert_eq!(0, ac4.entry_map.get("end").map_or(0, |_| 1));
  assert_eq!(ac4.context, AutocompleteContext::Expression);

  fixture.base.check(
    r#"
        for x = 1, 2, @15
    "#,
  );

  let ac5 = fixture.base.autocomplete_marker('1');
  assert_eq!(1, ac5.entry_map.get("math").map_or(0, |_| 1));
  assert_eq!(0, ac5.entry_map.get("do").map_or(0, |_| 1));
  assert_eq!(0, ac5.entry_map.get("end").map_or(0, |_| 1));
  assert_eq!(ac5.context, AutocompleteContext::Expression);

  fixture.base.check(
    r#"
        for x = 1, 2, 5 f@1
    "#,
  );

  let ac6 = fixture.base.autocomplete_marker('1');
  assert_eq!(1, ac6.entry_map.len());
  assert_eq!(1, ac6.entry_map.get("do").map_or(0, |_| 1));
  assert_eq!(ac6.context, AutocompleteContext::Keyword);

  fixture.base.check(
    r#"
        for x = 1, 2, 5 do      @1
    "#,
  );

  let ac7 = fixture.base.autocomplete_marker('1');
  assert_eq!(1, ac7.entry_map.get("end").map_or(0, |_| 1));
  assert_eq!(ac7.context, AutocompleteContext::Statement);

  fixture.base.check(
    r#"local Foo = 1
        for x = @11, @22, @35
    "#,
  );

  for i in 0..3 {
    let marker = b'1' + i;
    let ac8 = fixture.base.autocomplete_marker(marker);
    assert_eq!(1, ac8.entry_map.get("Foo").map_or(0, |_| 1));
    assert_eq!(0, ac8.entry_map.get("do").map_or(0, |_| 1));
  }

  fixture.base.check(
    r#"local Foo = 1
        for x = @11, @22
    "#,
  );

  for i in 0..2 {
    let marker = b'1' + i;
    let ac9 = fixture.base.autocomplete_marker(marker);
    assert_eq!(1, ac9.entry_map.get("Foo").map_or(0, |_| 1));
    assert_eq!(0, ac9.entry_map.get("do").map_or(0, |_| 1));
  }
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_autocomplete_if_else_regression() {
  let (mut fixture, ac1) = ac_check!(
    r#"
local abcdef = 0;
local temp = false
local even = true;
local a
a = if temp then even else@1
a = if temp then even else @2
a = if temp then even else abc@3
        "#,
    '1'
  );
  assert!(!ac1.entry_map.contains_key("else"));

  let ac2 = fixture.base.autocomplete_marker('2');
  assert!(!ac2.entry_map.contains_key("else"));

  let ac3 = fixture.base.autocomplete_marker('3');
  assert!(ac3.entry_map.contains_key("abcdef"));
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_autocomplete_if_middle_keywords() {
  let (mut fixture, ac1) = ac_check!(
    r#"
        if   @1
    "#,
    '1'
  );
  assert_eq!(0, ac1.entry_map.get("then").map_or(0, |_| 1));
  assert_eq!(1, ac1.entry_map.get("function").map_or(0, |_| 1));
  assert_eq!(1, ac1.entry_map.get("table").map_or(0, |_| 1));
  assert_eq!(0, ac1.entry_map.get("else").map_or(0, |_| 1));
  assert_eq!(0, ac1.entry_map.get("elseif").map_or(0, |_| 1));
  assert_eq!(0, ac1.entry_map.get("end").map_or(0, |_| 1));
  assert_eq!(ac1.context, AutocompleteContext::Expression);

  fixture.base.check(
    r#"
        if x  @1
    "#,
  );

  let ac2 = fixture.base.autocomplete_marker('1');
  assert_eq!(1, ac2.entry_map.get("then").map_or(0, |_| 1));
  assert_eq!(0, ac2.entry_map.get("function").map_or(0, |_| 1));
  assert_eq!(0, ac2.entry_map.get("else").map_or(0, |_| 1));
  assert_eq!(0, ac2.entry_map.get("elseif").map_or(0, |_| 1));
  assert_eq!(0, ac2.entry_map.get("end").map_or(0, |_| 1));
  assert_eq!(ac2.context, AutocompleteContext::Keyword);

  fixture.base.check(
    r#"
        if x t@1
    "#,
  );

  let ac3 = fixture.base.autocomplete_marker('1');
  assert_eq!(3, ac3.entry_map.len());
  assert_eq!(1, ac3.entry_map.get("then").map_or(0, |_| 1));
  assert_eq!(1, ac3.entry_map.get("and").map_or(0, |_| 1));
  assert_eq!(1, ac3.entry_map.get("or").map_or(0, |_| 1));
  assert_eq!(ac3.context, AutocompleteContext::Keyword);

  fixture.base.check(
    r#"
        if x then
@1
        end
    "#,
  );

  let ac4 = fixture.base.autocomplete_marker('1');
  assert_eq!(0, ac4.entry_map.get("then").map_or(0, |_| 1));
  assert_eq!(1, ac4.entry_map.get("else").map_or(0, |_| 1));
  assert_eq!(1, ac4.entry_map.get("function").map_or(0, |_| 1));
  assert_eq!(1, ac4.entry_map.get("elseif").map_or(0, |_| 1));
  assert_eq!(0, ac4.entry_map.get("end").map_or(0, |_| 1));
  assert_eq!(ac4.context, AutocompleteContext::Statement);

  fixture.base.check(
    r#"
        if x then
            t@1
        end
    "#,
  );

  let ac4a = fixture.base.autocomplete_marker('1');
  assert_eq!(0, ac4a.entry_map.get("then").map_or(0, |_| 1));
  assert_eq!(1, ac4a.entry_map.get("table").map_or(0, |_| 1));
  assert_eq!(1, ac4a.entry_map.get("else").map_or(0, |_| 1));
  assert_eq!(1, ac4a.entry_map.get("elseif").map_or(0, |_| 1));
  assert_eq!(ac4a.context, AutocompleteContext::Statement);

  fixture.base.check(
    r#"
        if x then
@1
        elseif x then
        end
    "#,
  );

  let ac5 = fixture.base.autocomplete_marker('1');
  assert_eq!(0, ac5.entry_map.get("then").map_or(0, |_| 1));
  assert_eq!(1, ac5.entry_map.get("function").map_or(0, |_| 1));
  assert_eq!(0, ac5.entry_map.get("else").map_or(0, |_| 1));
  assert_eq!(0, ac5.entry_map.get("elseif").map_or(0, |_| 1));
  assert_eq!(0, ac5.entry_map.get("end").map_or(0, |_| 1));
  assert_eq!(ac5.context, AutocompleteContext::Statement);

  fixture.base.check(
    r#"
        if t@1
    "#,
  );

  let ac6 = fixture.base.autocomplete_marker('1');
  assert_eq!(1, ac6.entry_map.get("true").map_or(0, |_| 1));
  assert_eq!(1, ac6.entry_map.get("false").map_or(0, |_| 1));
  assert_eq!(0, ac6.entry_map.get("then").map_or(0, |_| 1));
  assert_eq!(1, ac6.entry_map.get("function").map_or(0, |_| 1));
  assert_eq!(0, ac6.entry_map.get("else").map_or(0, |_| 1));
  assert_eq!(0, ac6.entry_map.get("elseif").map_or(0, |_| 1));
  assert_eq!(0, ac6.entry_map.get("end").map_or(0, |_| 1));
  assert_eq!(ac6.context, AutocompleteContext::Expression);
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_autocomplete_ifelse_expressions() {
  let (mut fixture, ac1) = ac_check!(
    r#"
local temp = false
local even = true;
local a = true
a = if t@1emp then t
a = if temp t@2
a = if temp then e@3
a = if temp then even e@4
a = if temp then even elseif t@5
a = if temp then even elseif true t@6
a = if temp then even elseif true then t@7
a = if temp then even elseif true then temp e@8
a = if temp then even elseif true then temp else e@9
        "#,
    '1'
  );
  assert!(ac1.entry_map.contains_key("temp"));
  assert!(ac1.entry_map.contains_key("true"));
  assert!(!ac1.entry_map.contains_key("then"));
  assert!(!ac1.entry_map.contains_key("else"));
  assert!(!ac1.entry_map.contains_key("elseif"));
  assert_eq!(ac1.context, AutocompleteContext::Expression);

  let ac2 = fixture.base.autocomplete_marker('2');
  assert!(!ac2.entry_map.contains_key("temp"));
  assert!(!ac2.entry_map.contains_key("true"));
  assert!(ac2.entry_map.contains_key("then"));
  assert!(!ac2.entry_map.contains_key("else"));
  assert!(!ac2.entry_map.contains_key("elseif"));
  assert_eq!(ac2.context, AutocompleteContext::Keyword);

  let ac3 = fixture.base.autocomplete_marker('3');
  assert!(ac3.entry_map.contains_key("even"));
  assert!(!ac3.entry_map.contains_key("then"));
  assert!(!ac3.entry_map.contains_key("else"));
  assert!(!ac3.entry_map.contains_key("elseif"));
  assert_eq!(ac3.context, AutocompleteContext::Expression);

  let ac4 = fixture.base.autocomplete_marker('4');
  assert!(!ac4.entry_map.contains_key("even"));
  assert!(!ac4.entry_map.contains_key("then"));
  assert!(ac4.entry_map.contains_key("else"));
  assert!(ac4.entry_map.contains_key("elseif"));
  assert_eq!(ac4.context, AutocompleteContext::Keyword);

  let ac5 = fixture.base.autocomplete_marker('5');
  assert!(ac5.entry_map.contains_key("temp"));
  assert!(ac5.entry_map.contains_key("true"));
  assert!(!ac5.entry_map.contains_key("then"));
  assert!(!ac5.entry_map.contains_key("else"));
  assert!(!ac5.entry_map.contains_key("elseif"));
  assert_eq!(ac5.context, AutocompleteContext::Expression);

  let ac6 = fixture.base.autocomplete_marker('6');
  assert!(!ac6.entry_map.contains_key("temp"));
  assert!(!ac6.entry_map.contains_key("true"));
  assert!(ac6.entry_map.contains_key("then"));
  assert!(!ac6.entry_map.contains_key("else"));
  assert!(!ac6.entry_map.contains_key("elseif"));
  assert_eq!(ac6.context, AutocompleteContext::Keyword);

  let ac7 = fixture.base.autocomplete_marker('7');
  assert!(ac7.entry_map.contains_key("temp"));
  assert!(ac7.entry_map.contains_key("true"));
  assert!(!ac7.entry_map.contains_key("then"));
  assert!(!ac7.entry_map.contains_key("else"));
  assert!(!ac7.entry_map.contains_key("elseif"));
  assert_eq!(ac7.context, AutocompleteContext::Expression);

  let ac8 = fixture.base.autocomplete_marker('8');
  assert!(!ac8.entry_map.contains_key("even"));
  assert!(!ac8.entry_map.contains_key("then"));
  assert!(ac8.entry_map.contains_key("else"));
  assert!(ac8.entry_map.contains_key("elseif"));
  assert_eq!(ac8.context, AutocompleteContext::Keyword);

  let ac9 = fixture.base.autocomplete_marker('9');
  assert!(!ac9.entry_map.contains_key("then"));
  assert!(!ac9.entry_map.contains_key("else"));
  assert!(!ac9.entry_map.contains_key("elseif"));
  assert_eq!(ac9.context, AutocompleteContext::Expression);
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_autocomplete_implicit_named_index_index_expr() {
  let _new_solver = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);

  let (_fixture, ac) = ac_check!(
    r#"
        type Constraint = "A" | "B" | "C"
        local foo : { [Constraint]: string } = {
            A = "Value for A",
            B = "Value for B",
            C = "Value for C",
        }
        foo["@1"]
    "#,
    '1'
  );
  assert!(ac.entry_map.contains_key("A"));
  assert_eq!(ac.entry_map["A"].kind, AutocompleteEntryKind::String);
  assert!(ac.entry_map.contains_key("B"));
  assert_eq!(ac.entry_map["B"].kind, AutocompleteEntryKind::String);
  assert!(ac.entry_map.contains_key("C"));
  assert_eq!(ac.entry_map["C"].kind, AutocompleteEntryKind::String);
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_autocomplete_implicit_named_index_index_expr_without_annotation() {
  use ulua_analysis::functions::to_string_to_string::to_string_type_id;
  let _new_solver = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);

  let (_fixture, ac) = ac_check!(
    r#"
        local foo = {
            ["Item/Foo"] = 42,
            ["Item/Bar"] = "it's true",
            ["Item/Baz"] = true,
        }
        foo["@1"]
    "#,
    '1'
  );
  for (key, expected_type) in [
    ("Item/Foo", "number"),
    ("Item/Bar", "string"),
    ("Item/Baz", "boolean"),
  ] {
    assert!(ac.entry_map.contains_key(key));
    let entry = &ac.entry_map[key];
    assert_eq!(entry.kind, AutocompleteEntryKind::Property);
    let ty = entry.r#type.expect("autocomplete entry should have a type");
    assert_eq!(expected_type, to_string_type_id(ty));
  }
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_autocomplete_in_local_table() {
  let (mut fixture, ac1) = ac_check!(
    r#"
        type Entry = { field: number, prop: string }
        local x : {Entry} = {}
        x[1] = {
           f@1,
           p@2,
        }

        local t : { key1: boolean, thing2: CFrame, aaa3: vector } = {
            k@3,
            th@4,
        }
    "#,
    '1'
  );
  assert!(ac1.entry_map.contains_key("field"));
  let ac2 = fixture.base.autocomplete_marker('2');
  assert!(ac2.entry_map.contains_key("prop"));
  let ac3 = fixture.base.autocomplete_marker('3');
  assert!(ac3.entry_map.contains_key("key1"));
  let ac4 = fixture.base.autocomplete_marker('4');
  assert!(ac4.entry_map.contains_key("thing2"));
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_autocomplete_in_type_assertion() {
  let (mut fixture, ac1) = ac_check!(
    r#"
        type Entry = { field: number, prop: string }
        return ( { f@1, p@2 } :: Entry )
    "#,
    '1'
  );
  assert!(ac1.entry_map.contains_key("field"));
  let ac2 = fixture.base.autocomplete_marker('2');
  assert!(ac2.entry_map.contains_key("prop"));
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_autocomplete_include_break_continue_in_loop() {
  let (mut fixture, ac1) = ac_check!(
    r#"for x in y do
        @1
        if true then
            @2
        end
    end"#,
    '1'
  );
  assert!(ac1.entry_map.contains_key("break"));
  assert!(ac1.entry_map.contains_key("continue"));

  let ac2 = fixture.base.autocomplete_marker('2');
  assert!(ac2.entry_map.contains_key("break"));
  assert!(ac2.entry_map.contains_key("continue"));
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_autocomplete_include_break_continue_in_nests() {
  let (_fixture, ac) = ac_check!(
    r#"while ((function ()
        while true do
            @1
        end
        end)()) do
    end"#,
    '1'
  );
  assert!(ac.entry_map.contains_key("break"));
  assert!(ac.entry_map.contains_key("continue"));
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_autocomplete_include_break_continue_in_repeat() {
  let (_fixture, ac) = ac_check!(
    r#"repeat
        @1
    until foo()"#,
    '1'
  );
  assert!(ac.entry_map.contains_key("break"));
  assert!(ac.entry_map.contains_key("continue"));
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_autocomplete_interpolated_string_as_singleton() {
  let (mut fixture, ac1) = ac_check!(
    r#"
        --!strict
        local function f(a: "cat" | "dog") end

        f(`@1`)
        f(`uhhh{'try'}@2`)
    "#,
    '1'
  );
  assert!(ac1.entry_map.contains_key("cat"));
  assert_eq!(ac1.context, AutocompleteContext::String);

  let ac2 = fixture.base.autocomplete_marker('2');
  assert!(ac2.entry_map.is_empty());
  assert_eq!(ac2.context, AutocompleteContext::String);
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_autocomplete_interpolated_string_constant() {
  let mut fixture = AcFixture::default();

  fixture.base.check(r#"f(`@1`)"#);
  let ac1 = fixture.base.autocomplete_marker('1');
  assert!(ac1.entry_map.is_empty());
  assert_eq!(ac1.context, AutocompleteContext::String);

  fixture.base.check(String::from(r#"f(`@1 {"a"}`)"#));
  let ac2 = fixture.base.autocomplete_marker('1');
  assert!(ac2.entry_map.is_empty());
  assert_eq!(ac2.context, AutocompleteContext::String);

  fixture.base.check(String::from(r#"f(`{"a"} @1`)"#));
  let ac3 = fixture.base.autocomplete_marker('1');
  assert!(ac3.entry_map.is_empty());
  assert_eq!(ac3.context, AutocompleteContext::String);

  fixture.base.check(String::from(r#"f(`{"a"} @1 {"b"}`)"#));
  let ac4 = fixture.base.autocomplete_marker('1');
  assert!(ac4.entry_map.is_empty());
  assert_eq!(ac4.context, AutocompleteContext::String);
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_autocomplete_interpolated_string_expression() {
  let mut fixture = AcFixture::default();
  fixture.base.check(r#"f(`expression = {@1}`)"#);

  let ac = fixture.base.autocomplete_marker('1');
  assert!(ac.entry_map.contains_key("table"));
  assert_eq!(ac.context, AutocompleteContext::Expression);
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_autocomplete_interpolated_string_expression_with_comments() {
  let mut fixture = AcFixture::default();

  fixture
    .base
    .check(r#"f(`expression = {--[[ bla bla bla ]]@1`)"#);

  let ac1 = fixture.base.autocomplete_marker('1');
  assert!(ac1.entry_map.contains_key("table"));
  assert_eq!(ac1.context, AutocompleteContext::Expression);

  fixture
    .base
    .check(r#"f(`expression = {@1 --[[ bla bla bla ]]`)"#);

  let ac2 = fixture.base.autocomplete_marker('1');
  assert!(!ac2.entry_map.is_empty());
  assert!(ac2.entry_map.contains_key("table"));
  assert_eq!(ac2.context, AutocompleteContext::Expression);
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_autocomplete_metatable_fill_writeonly_prop_no_crash() {
  let _new_solver = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);

  let (_fixture, ac) = acb_check!(
    r#"

local t0 = { thing = 5 }

type function evil(x)
    local tbl = types.newtable(nil, nil, nil)
    tbl:setwriteproperty(types.singleton("__index"), types.any)
    return tbl
end

type BadMTType = evil<{ thing : number}>
local function foo(t : BadMTType)
        local t2 = setmetatable({}, t)
        return t2
end

local x = foo(nil :: any)
x.@1
    "#,
    '1'
  );
  assert!(ac.entry_map.is_empty());
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_autocomplete_method_in_unfinished_repeat_body_eof() {
  let (_fixture, ac) = ac_check!(
    r#"local t = {}
        function t:Foo() end
        repeat
        t:@1"#,
    '1'
  );
  assert!(!ac.entry_map.is_empty());
  assert!(ac.entry_map.contains_key("Foo"));
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_autocomplete_method_in_unfinished_repeat_body_not_eof() {
  let (_fixture, ac) = ac_check!(
    r#"local t = {}
        function t:Foo() end
        repeat
        t:@1
        "#,
    '1'
  );
  assert!(!ac.entry_map.is_empty());
  assert!(ac.entry_map.contains_key("Foo"));
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_autocomplete_method_in_unfinished_while_body() {
  let (_fixture, ac) = ac_check!(
    r#"local t = {}
        function t:Foo() end
        while true do
        t:@1"#,
    '1'
  );
  assert!(!ac.entry_map.is_empty());
  assert!(ac.entry_map.contains_key("Foo"));
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_autocomplete_on_string_singletons() {
  let (_fixture, ac) = acb_check!(
    r#"
        --!strict
        local foo: "hello" | "bye" = "hello"
        foo:@1
    "#,
    '1'
  );

  assert!(ac.entry_map.contains_key("format"));
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_autocomplete_oop_implicit_self() {
  let (_fixture, ac) = acb_check!(
    r#"
--!strict
local Class = {}
Class.__index = Class
type Class = typeof(setmetatable({} :: { x: number }, Class))
function Class.new(x: number): Class
    return setmetatable({x = x}, Class)
end
function Class.getx(self: Class)
    return self.x
end
function test()
    local c = Class.new(42)
    local n = c:@1
    print(n)
end
    "#,
    '1'
  );

  assert!(ac.entry_map.contains_key("getx"));
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_autocomplete_prop_index_function_metamethod_is_variadic() {
  let mut fixture = ACBuiltinsFixture::default();
  fixture.base.base.file_resolver.source.insert(
    String::from("Module/A"),
    r#"
        type Foo = {x: number}
        local t = {}
        setmetatable(t, {
            __index = function(index: string): ...Foo
                return {x = 1}, {x = 2}
            end
        })

        local a = t. -- Line 9
        --          | Column 20
    "#,
  );

  let module = ModuleName::from("Module/A");
  let ac = fixture
    .base
    .autocomplete_module_name_position_string_completion_callback(
      &module,
      Position {
        line: 9,
        column: 20,
      },
      Box::new(null_callback),
    );

  assert_eq!(1, ac.entry_map.len());
  assert!(ac.entry_map.contains_key("x"));
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_autocomplete_react() {
  let _new_solver = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);

  let (mut fixture, ac) = ac_check!(
    r#"
        type React_Node = any
        type ReactElement<P, T> = any

        type React_StatelessFunctionalComponent<Props> = (props: Props, context: any) -> React_Node
        type React_Component<Props, State = nil> = {}
        type createElementFn = <P, T>(
            type_:
              | React_StatelessFunctionalComponent<P>
              | React_Component<P>
              | string,
            props: P?,
            ...(React_Node | (...any) -> React_Node)
        ) -> ReactElement<P, T>

        local createElement: createElementFn = nil :: any

        local function MyComponent(props: { foobar: string, barbaz: { bazquxx: string } })
        	return nil
        end

        createElement(MyComponent, { f@1 })
        createElement(MyComponent, { barbaz = { b@2 } })
        createElement(MyComponent, { foobar = {}, b@3 })
    "#,
    '1'
  );
  assert!(ac.entry_map.contains_key("foobar"));

  let ac = fixture.base.autocomplete_marker('2');
  assert!(ac.entry_map.contains_key("bazquxx"));

  let ac = fixture.base.autocomplete_marker('3');
  assert!(ac.entry_map.contains_key("barbaz"));
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_autocomplete_repeat_middle_keyword() {
  let (mut fixture, ac1) = ac_check!(
    r#"
        repeat @1
    "#,
    '1'
  );
  assert_eq!(1, ac1.entry_map.get("do").map_or(0, |_| 1));
  assert_eq!(1, ac1.entry_map.get("function").map_or(0, |_| 1));
  assert_eq!(1, ac1.entry_map.get("until").map_or(0, |_| 1));

  fixture.base.check(
    r#"
        repeat f f@1
    "#,
  );

  let ac2 = fixture.base.autocomplete_marker('1');
  assert_eq!(1, ac2.entry_map.get("function").map_or(0, |_| 1));
  assert_eq!(1, ac2.entry_map.get("until").map_or(0, |_| 1));

  fixture.base.check(
    r#"
        repeat
            u@1
        until
    "#,
  );

  let ac3 = fixture.base.autocomplete_marker('1');
  assert_eq!(0, ac3.entry_map.get("until").map_or(0, |_| 1));
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_autocomplete_response_perf_1() {
  use alloc::format;

  if !fflag::DebugLuauForceOldSolver.get() {
    return;
  }

  let parts = 100;
  let mut source = String::new();

  for i in 0..parts {
    source.push_str(&format!("type T{} = {{ f{}: number }}\n", i, i));
  }

  source.push_str("type Instance = { new: (('s0', extra: Instance?) -> T0)");

  for i in 1..parts {
    source.push_str(&format!(" & (('s{}', extra: Instance?) -> T{})", i, i));
  }

  source.push_str(" }\n");
  source.push_str("local Instance: Instance = {} :: any\n");
  source.push_str("local function c(): boolean return t@1 end\n");

  let mut fixture = AcFixture::default();
  fixture.base.check(&source);

  let ac = fixture.base.autocomplete_marker('1');
  assert!(ac.entry_map.contains_key("true"));
  assert!(ac.entry_map.contains_key("Instance"));
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_autocomplete_string_singleton_disjoint_intersection_arg() {
  let _intersection =
    ScopedFastFlag::new(&fflag::LuauAutocompleteStringSingletonIntersection, true);
  let _new_solver = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);

  let (mut fixture, ac) = ac_check!(
    r#"
        local function f(_: "foo"&"baz") end
        f("@1")
        f(@2)
    "#,
    '1'
  );
  assert!(ac.entry_map.contains_key("foo"));
  assert!(ac.entry_map.contains_key("baz"));
  assert_eq!(ac.context, AutocompleteContext::String);

  let ac = fixture.base.autocomplete_marker('2');
  assert!(ac.entry_map.contains_key("\"foo\""));
  assert!(ac.entry_map.contains_key("\"baz\""));
  assert_eq!(ac.context, AutocompleteContext::Expression);
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_autocomplete_string_singleton_equality() {
  let (mut fixture, ac) = ac_check!(
    r#"
        type tagged = {tag:"cat", fieldx:number} | {tag:"dog", fieldy:number}
        local x: tagged = {tag="cat", fieldx=2}
        if x.tag == "@1" or "@2" ~= x.tag then end
    "#,
    '1'
  );

  assert!(ac.entry_map.contains_key("cat"));
  assert!(ac.entry_map.contains_key("dog"));

  let ac = fixture.base.autocomplete_marker('2');

  assert!(ac.entry_map.contains_key("cat"));
  assert!(ac.entry_map.contains_key("dog"));
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_autocomplete_string_singleton_escape() {
  let (mut fixture, ac) = ac_check!(
    r#"
        type tag = "strange\t\"cat\"" | 'nice\t"dog"'
        local function f(x: tag) end
        f(@1)
        f("@2")
    "#,
    '1'
  );

  assert!(ac.entry_map.contains_key("\"strange\\t\\\"cat\\\"\""));
  assert!(ac.entry_map.contains_key("\"nice\\t\\\"dog\\\"\""));

  let ac = fixture.base.autocomplete_marker('2');

  assert!(ac.entry_map.contains_key("strange\\t\\\"cat\\\""));
  assert!(ac.entry_map.contains_key("nice\\t\\\"dog\\\""));
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_autocomplete_string_singleton_intersection_multiple() {
  let _sff = ScopedFastFlag::new(&fflag::LuauAutocompleteStringSingletonIntersection, true);

  let (mut fixture, ac) = ac_check!(
    r#"
        local function C(_: "Example"&"Example") end
        C("@1")
        C(@2)
        local x: "Example"&"Example" = "@3"
    "#,
    '1'
  );
  assert!(ac.entry_map.contains_key("Example"));
  assert_eq!(ac.context, AutocompleteContext::String);

  let ac = fixture.base.autocomplete_marker('2');
  assert!(ac.entry_map.contains_key("\"Example\""));
  assert_eq!(ac.context, AutocompleteContext::Expression);

  let ac = fixture.base.autocomplete_marker('3');
  assert!(ac.entry_map.contains_key("Example"));
  assert_eq!(ac.context, AutocompleteContext::String);
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_autocomplete_string_singleton_intersection_variable() {
  let _sff = ScopedFastFlag::new(&fflag::LuauAutocompleteStringSingletonIntersection, true);

  let (_fixture, ac) = ac_check!(
    r#"
        local _: "cat"&"cat" = "@1"
    "#,
    '1'
  );
  assert!(ac.entry_map.contains_key("cat"));
  assert_eq!(ac.context, AutocompleteContext::String);
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_autocomplete_string_singleton_keyof_intersection() {
  let _intersection =
    ScopedFastFlag::new(&fflag::LuauAutocompleteStringSingletonIntersection, true);
  let _new_solver = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);

  let (_fixture, ac) = acb_check!(
    r#"
        local foo = {
            Element1 = "Value1",
            Element2 = "Value2",
        }
        local function bar<T>(key: keyof<typeof(foo)>&T) end
        bar("@1")
    "#,
    '1'
  );
  assert!(ac.entry_map.contains_key("Element1"));
  assert!(ac.entry_map.contains_key("Element2"));
  assert_eq!(ac.context, AutocompleteContext::String);
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_autocomplete_string_singletons() {
  let (mut fixture, ac) = ac_check!(
    r#"
        type tag = "cat" | "dog"
        local function f(a: tag) end
        f("@1")
        f(@2)
        local x: tag = "@3"
    "#,
    '1'
  );

  assert!(ac.entry_map.contains_key("cat"));
  assert!(ac.entry_map.contains_key("dog"));
  assert_eq!(ac.context, AutocompleteContext::String);

  let ac = fixture.base.autocomplete_marker('2');

  assert!(ac.entry_map.contains_key("\"cat\""));
  assert!(ac.entry_map.contains_key("\"dog\""));
  assert_eq!(ac.context, AutocompleteContext::Expression);

  let ac = fixture.base.autocomplete_marker('3');

  assert!(ac.entry_map.contains_key("cat"));
  assert!(ac.entry_map.contains_key("dog"));
  assert_eq!(ac.context, AutocompleteContext::String);
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_autocomplete_string_singletons_in_intersection() {
  let _intersection =
    ScopedFastFlag::new(&fflag::LuauAutocompleteStringSingletonIntersection, true);
  let _new_solver = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);

  let (_fixture, ac) = ac_check!(
    r#"
        local _: "foo"&"baz" = "@1"
    "#,
    '1'
  );
  assert!(ac.entry_map.contains_key("foo"));
  assert!(ac.entry_map.contains_key("baz"));
  assert_eq!(ac.context, AutocompleteContext::String);
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_autocomplete_string_singletons_in_literal() {
  if !fflag::DebugLuauForceOldSolver.get() {
    return;
  }

  let (_fixture, ac) = ac_check!(
    r#"
        type tagged = {tag:"cat", fieldx:number} | {tag:"dog", fieldy:number}
        local x: tagged = {tag="@1"}
    "#,
    '1'
  );

  assert!(ac.entry_map.contains_key("cat"));
  assert!(ac.entry_map.contains_key("dog"));
  assert_eq!(ac.context, AutocompleteContext::String);
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_autocomplete_subtyping_recursion_limit() {
  use alloc::format;

  use ulua_common::{dfint, fint};
  use ulua_unit_test::type_aliases::scoped_fast_int::ScopedFastInt;

  if fflag::DebugLuauForceOldSolver.get() {
    return;
  }

  let _type_infer_recursion_limit = ScopedFastInt::new(&fint::LuauTypeInferRecursionLimit, 10);
  let _subtyping_recursion_limit = ScopedFastInt::new(&dfint::LuauSubtypingRecursionLimit, 10);

  let parts = 100;
  let mut source = String::new();

  source.push_str("function f()\n");

  let mut prefix = String::new();
  for i in 0..parts {
    prefix.push_str(&format!("(nil|({{a{}:number}}&", i));
  }
  prefix.push_str(&format!("(nil|{{a{}:number}})", parts));
  for _ in 0..parts {
    prefix.push_str("))");
  }

  source.push_str("local x1 : ");
  source.push_str(&prefix);
  source.push('\n');
  source.push_str("local y : {a1:number} = x@1\n");
  source.push_str("end\n");

  let mut fixture = AcFixture::default();
  fixture.base.check(&source);

  let ac = fixture.base.autocomplete_marker('1');
  assert!(ac.entry_map.contains_key("true"));
  assert!(ac.entry_map.contains_key("x1"));
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_autocomplete_suggest_hot_comments() {
  let mut fixture = AcFixture::default();
  fixture.base.check("--!@1");

  let ac = fixture.base.autocomplete_marker('1');
  assert!(!ac.entry_map.is_empty());
  assert!(ac.entry_map.contains_key("strict"));
  assert!(ac.entry_map.contains_key("nonstrict"));
  assert!(ac.entry_map.contains_key("nocheck"));
  assert!(ac.entry_map.contains_key("native"));
  assert!(ac.entry_map.contains_key("nolint"));
  assert!(ac.entry_map.contains_key("optimize"));
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_autocomplete_table_insert() {
  let _new_solver = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);

  let (_fixture, ac) = acb_check!(
    r#"
        local function addToTable(t: {{ foobar: number }})
            table.insert(t, { f@1 })
        end
    "#,
    '1'
  );
  assert!(ac.entry_map.contains_key("foobar"));
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_autocomplete_until_expression() {
  let (_fixture, ac) = ac_check!(
    r#"
        repeat
        until   @1
    "#,
    '1'
  );
  assert_eq!(1, ac.entry_map.get("table").map_or(0, |_| 1));
  assert_eq!(ac.context, AutocompleteContext::Expression);
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_autocomplete_until_in_repeat() {
  let (_fixture, ac) = ac_check!(
    r#"
        repeat  @1
    "#,
    '1'
  );
  assert_eq!(1, ac.entry_map.get("table").map_or(0, |_| 1));
  assert_eq!(1, ac.entry_map.get("until").map_or(0, |_| 1));
  assert_eq!(ac.context, AutocompleteContext::Statement);
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_autocomplete_using_function_with_singleton_arg() {
  let (_fixture, ac) = ac_check!(
    r#"
        local function foo(...: "Val1") end
        foo(@1)
    "#,
    '1'
  );
  assert!(ac.entry_map.contains_key("\"Val1\""));
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_autocomplete_using_function_with_singleton_intersection_arg() {
  let _sff = ScopedFastFlag::new(&fflag::LuauAutocompleteStringSingletonIntersection, true);

  let (_fixture, ac) = ac_check!(
    r#"
        local function foo(_: "Val1"&"Val1") end
        foo(@1)
    "#,
    '1'
  );
  assert!(ac.entry_map.contains_key("\"Val1\""));
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_autocomplete_using_function_with_singleton_union_arg() {
  let (_fixture, ac) = ac_check!(
    r#"
        local function foo(...: "Val1" | "Val2") end
        foo(@1)
    "#,
    '1'
  );
  assert!(ac.entry_map.contains_key("\"Val1\""));
  assert!(ac.entry_map.contains_key("\"Val2\""));
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_autocomplete_using_indexer_with_singleton_keys() {
  let (_fixture, ac) = ac_check!(
    r#"
        type List = "Val1" | "Val2" | "Val3"
        local Table: { [List]: boolean }
        local _ = Table.@1
    "#,
    '1'
  );
  assert!(ac.entry_map.contains_key("Val1"));
  assert!(ac.entry_map.contains_key("Val2"));
  assert!(ac.entry_map.contains_key("Val3"));
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_autocomplete_via_bidirectional_self() {
  let _new_solver = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);

  let (_fixture, ac) = acb_check!(
    r#"
        type IAccount = {
            __index: IAccount,
            new : (string, number) -> Account,
            report: (self: Account) -> (),
        }

        export type Account = setmetatable<{
            name: string,
            balance: number
        }, IAccount>;

        local Account = {} :: IAccount
        Account.__index = Account

        function Account.new(name, balance): Account
            local self = {}
            self.name = name
            self.balance = balance
            return setmetatable(self, Account)
        end

        function Account:report()
            print("My balance is: " .. self.@1)
        end
    "#,
    '1'
  );
  assert!(ac.entry_map.contains_key("name"));
  assert!(ac.entry_map.contains_key("balance"));
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_autocomplete_while_middle_keywords() {
  let (mut fixture, ac1) = ac_check!(
    r#"
        while@1
    "#,
    '1'
  );
  assert_eq!(0, ac1.entry_map.get("do").map_or(0, |_| 1));
  assert_eq!(0, ac1.entry_map.get("end").map_or(0, |_| 1));
  assert_eq!(ac1.context, AutocompleteContext::Expression);

  fixture.base.check(
    r#"
        while true @1
    "#,
  );

  let ac2 = fixture.base.autocomplete_marker('1');
  assert_eq!(3, ac2.entry_map.len());
  assert_eq!(1, ac2.entry_map.get("do").map_or(0, |_| 1));
  assert_eq!(1, ac2.entry_map.get("and").map_or(0, |_| 1));
  assert_eq!(1, ac2.entry_map.get("or").map_or(0, |_| 1));
  assert_eq!(ac2.context, AutocompleteContext::Keyword);

  fixture.base.check(
    r#"
        while true do  @1
    "#,
  );

  let ac3 = fixture.base.autocomplete_marker('1');
  assert_eq!(1, ac3.entry_map.get("end").map_or(0, |_| 1));
  assert_eq!(ac3.context, AutocompleteContext::Statement);

  fixture.base.check(
    r#"
        while true d@1
    "#,
  );

  let ac4 = fixture.base.autocomplete_marker('1');
  assert_eq!(3, ac4.entry_map.len());
  assert_eq!(1, ac4.entry_map.get("do").map_or(0, |_| 1));
  assert_eq!(1, ac4.entry_map.get("and").map_or(0, |_| 1));
  assert_eq!(1, ac4.entry_map.get("or").map_or(0, |_| 1));
  assert_eq!(ac4.context, AutocompleteContext::Keyword);

  fixture.base.check(
    r#"
        while t@1
    "#,
  );

  let ac5 = fixture.base.autocomplete_marker('1');
  assert_eq!(0, ac5.entry_map.get("do").map_or(0, |_| 1));
  assert_eq!(1, ac5.entry_map.get("true").map_or(0, |_| 1));
  assert_eq!(1, ac5.entry_map.get("false").map_or(0, |_| 1));
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_bias_toward_inner_scope() {
  use ulua_analysis::{
    enums::autocomplete_context::AutocompleteContext,
    functions::{follow_type, get_type},
    records::table_type::TableType,
  };
  let (_fixture, ac) = ac_check!(
    r#"
        local A = {one=1}

        function B()
            local A = {two=2}

            A  @1
        end
    "#
  );

  assert!(ac.entry_map.contains_key("A"));
  assert_eq!(ac.context, AutocompleteContext::Statement);

  let ty = ac.entry_map["A"]
    .r#type
    .expect("A entry should have a type");
  let ty = follow_type::follow(ty);
  let table = get_type::get::<TableType>(ty).expect("A should be a table");
  assert!(table.props.contains_key("two"));
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_bidirectional_autocomplete_in_function_call() {
  let _new_solver = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);

  let (_fixture, ac) = ac_check!(
    r#"
        local function take(_: { choice: "left" | "right" }) end

        take({ choice = "@1" })
    "#,
    '1'
  );
  assert!(ac.entry_map.contains_key("left"));
  assert!(ac.entry_map.contains_key("right"));
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_class_autocomplete_classname_inside_method() {
  let _new_solver = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);
  let _classes = ScopedFastFlag::new(&fflag::DebugLuauUserDefinedClasses, true);

  let (mut fixture, ac) = ac_check!(
    r#"
        class Bar
            function new()
                return Bar {}
            end
            function hmm(self)
                self:h@2
            end
        end

        class Bar
            function make()
                return Bar {}
            end
            function huh(self)
                self:h@3
            end
        end

        Bar.@1
    "#,
    '1'
  );
  assert!(ac.entry_map.contains_key("new"));
  assert!(!ac.entry_map.contains_key("make"));

  let ac = fixture.base.autocomplete_marker('2');
  assert!(ac.entry_map.contains_key("hmm"));
  assert!(!ac.entry_map.contains_key("huh"));

  let ac = fixture.base.autocomplete_marker('3');
  assert!(!ac.entry_map.contains_key("huh"));
  assert!(!ac.entry_map.contains_key("hmm"));
}

mod autocomplete_class_autocomplete_classname_inside_method_autocomplete_test_case_2 {
  //! Source: `tests/Autocomplete.test.cpp`
  use super::{AcFixture, ScopedFastFlag, fflag};

  #[test]
  fn autocomplete_class_autocomplete_classname_inside_method() {
    let _new_solver = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);
    let _classes = ScopedFastFlag::new(&fflag::DebugLuauUserDefinedClasses, true);

    let mut fixture = AcFixture::default();
    fixture.base.check(
      r#"
        class Bar
            public value: number
            function new()
                return B@1
            end
        end
    "#,
    );

    let ac = fixture.base.autocomplete_marker('1');
    assert!(ac.entry_map.contains_key("Bar"));
  }
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_cli_197197_autocomplete_generic_keyof() {
  let _new_solver = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);

  let (_fixture, ac) = acb_check!(
    r#"
        local function ToggleButton<T>(Table: T, Key: keyof<T>)
            -- don't need to do anything here.
        end

        local tbl: { Changed: bool, RemoveTag: bool } = nil :: any

        ToggleButton(tbl, "@1")
    "#,
    '1'
  );
  assert!(ac.entry_map.contains_key("Changed"));
  assert!(ac.entry_map.contains_key("RemoveTag"));
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_comments() {
  let mut fixture = AcFixture::default();
  fixture
    .base
    .base
    .file_resolver
    .source
    .insert(String::from("Comments"), String::from("--foo"));

  let module = ModuleName::from("Comments");
  let ac = fixture
    .base
    .autocomplete_module_name_position_string_completion_callback(
      &module,
      Position { line: 0, column: 5 },
      Box::new(null_callback),
    );

  assert_eq!(0, ac.entry_map.len());
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_cyclic_table() {
  let (_fixture, ac) = ac_check!(
    r#"
        local abc = {}
        local def = { abc = abc }
        abc.def = def
        abc.def. @1
    "#,
    '1'
  );

  assert!(ac.entry_map.contains_key("abc"));
  assert_eq!(ac.context, AutocompleteContext::Property);
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_do_compatible_self_calls() {
  let (_fixture, ac) = ac_check!(
    r#"
local t = {}
function t:m() end
t:@1
    "#,
    '1'
  );

  assert!(ac.entry_map.contains_key("m"));
  assert!(!ac.entry_map["m"].wrong_index_type);
  assert!(ac.entry_map["m"].indexed_with_self);
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_do_not_overwrite_context_sensitive_kws() {
  let (_fixture, ac) = ac_check!(
    r#"
        local function continue()
        end


@1    "#,
    '1'
  );
  let entry = &ac.entry_map["continue"];

  assert_eq!(entry.kind, AutocompleteEntryKind::Binding);
  assert_eq!(ac.context, AutocompleteContext::Statement);
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_do_not_suggest_internal_module_type() {
  let mut fixture = AcFixture::default();
  fixture.base.base.file_resolver.source.insert(
    String::from("Module/A"),
    r#"
type done = { x: number, y: number }
local function a(a: (done) -> number) return a({x=1, y=2}) end
local function b(a: ((done) -> number) -> number) return a(function(done) return 1 end) end
return {a = a, b = b}
    "#,
  );

  let module_a = ModuleName::from("Module/A");
  let result = fixture
    .base
    .get_frontend()
    .check_module_name_optional_frontend_options(&module_a, None);
  assert!(result.errors.is_empty());

  fixture.base.base.file_resolver.source.insert(
    String::from("Module/B"),
    r#"
local ex = require(script.Parent.A)
ex.a(function(x:
    "#,
  );

  let module_b = ModuleName::from("Module/B");
  fixture
    .base
    .get_frontend()
    .check_module_name_optional_frontend_options(&module_b, None);

  let ac1 = fixture
    .base
    .autocomplete_module_name_position_string_completion_callback(
      &module_b,
      Position {
        line: 2,
        column: 16,
      },
      Box::new(null_callback),
    );

  assert!(!ac1.entry_map.contains_key("done"));

  fixture.base.base.file_resolver.source.insert(
    String::from("Module/C"),
    r#"
local ex = require(script.Parent.A)
ex.b(function(x:
    "#,
  );

  let module_c = ModuleName::from("Module/C");
  fixture
    .base
    .get_frontend()
    .check_module_name_optional_frontend_options(&module_c, None);

  let ac2 = fixture
    .base
    .autocomplete_module_name_position_string_completion_callback(
      &module_c,
      Position {
        line: 2,
        column: 16,
      },
      Box::new(null_callback),
    );

  assert!(!ac2.entry_map.contains_key("(done) -> number"));
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_do_not_suggest_synthetic_table_name() {
  let (_fixture, ac) = ac_check!(
    r#"
local foo = { a = 1, b = 2 }
local bar: @1= foo
    "#,
    '1'
  );

  assert!(!ac.entry_map.contains_key("foo"));
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_do_wrong_compatible_nonself_calls() {
  let (_fixture, ac) = ac_check!(
    r#"
local t = {}
function t:m(x: string) end
t.@1
    "#,
    '1'
  );

  assert!(ac.entry_map.contains_key("m"));
  if !fflag::DebugLuauForceOldSolver.get() {
    assert!(ac.entry_map["m"].wrong_index_type);
  } else {
    assert!(!ac.entry_map["m"].wrong_index_type);
  }
  assert!(!ac.entry_map["m"].indexed_with_self);
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_do_wrong_compatible_self_calls() {
  let (_fixture, ac) = ac_check!(
    r#"
local t = {}
function t.m(x: typeof(t)) end
t:@1
    "#,
    '1'
  );

  assert!(ac.entry_map.contains_key("m"));
  assert!(!ac.entry_map["m"].wrong_index_type);
  assert!(ac.entry_map["m"].indexed_with_self);
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_dont_offer_any_suggestions_from_within_a_broken_comment() {
  let (_fixture, ac) = ac_check!(
    r#"
        --[[ @1
    "#,
    '1'
  );

  assert_eq!(0, ac.entry_map.len());
  assert_eq!(ac.context, AutocompleteContext::Unknown);
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_dont_offer_any_suggestions_from_within_a_broken_comment_at_the_very_end_of_the_file()
 {
  let mut fixture = AcFixture::default();
  fixture.base.check("--[[@1");

  let ac = fixture.base.autocomplete_marker('1');

  assert_eq!(0, ac.entry_map.len());
  assert_eq!(ac.context, AutocompleteContext::Unknown);
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_dont_offer_any_suggestions_from_within_a_comment() {
  let (_fixture, ac) = ac_check!(
    r#"
        --!strict
        local foo = {}
        function foo:bar() end

        --[[
            foo:@1
        ]]
    "#,
    '1'
  );

  assert_eq!(0, ac.entry_map.len());
  assert_eq!(ac.context, AutocompleteContext::Unknown);
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_dont_suggest_local_before_its_definition() {
  let (mut fixture, ac) = ac_check!(
    r#"
        local myLocal = 4
        function abc()
@1            local myInnerLocal = 1
@2
        end
@3    "#,
    '1'
  );
  assert!(ac.entry_map.contains_key("myLocal"));
  assert!(!ac.entry_map.contains_key("myInnerLocal"));

  let ac = fixture.base.autocomplete_marker('2');
  assert!(ac.entry_map.contains_key("myLocal"));
  assert!(ac.entry_map.contains_key("myInnerLocal"));

  let ac = fixture.base.autocomplete_marker('3');
  assert!(ac.entry_map.contains_key("myLocal"));
  assert!(!ac.entry_map.contains_key("myInnerLocal"));
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_empty_program() {
  let mut fixture = AcFixture::default();
  fixture.base.check(" @1");

  let ac = fixture.base.autocomplete_marker('1');

  assert!(!ac.entry_map.is_empty());
  assert!(ac.entry_map.contains_key("table"));
  assert!(ac.entry_map.contains_key("math"));
  assert_eq!(ac.context, AutocompleteContext::Statement);
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_function_expr_params() {
  let mut fixture = AcFixture::default();
  fixture.base.check(
    r#"
        abc = function(def) @1
    "#,
  );

  for i in 20..27 {
    assert!(
      fixture
        .base
        .autocomplete_position(1, i)
        .entry_map
        .is_empty()
    );
  }
  assert!(!fixture.base.autocomplete_marker('1').entry_map.is_empty());

  fixture.base.check(
    r#"
        abc = function(def) @1
        end
    "#,
  );

  for i in 20..27 {
    assert!(
      fixture
        .base
        .autocomplete_position(1, i)
        .entry_map
        .is_empty()
    );
  }
  assert!(!fixture.base.autocomplete_marker('1').entry_map.is_empty());

  fixture.base.check(
    r#"
        abc = function(def)
@1
        end
    "#,
  );

  let ac2 = fixture.base.autocomplete_marker('1');
  assert_eq!(1, ac2.entry_map.get("def").map_or(0, |_| 1));
  assert_eq!(ac2.context, AutocompleteContext::Statement);
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_function_in_assignment_has_parentheses() {
  let (_fixture, ac) = ac_check!(
    r#"
local function bar(a: number) return -a end
local abc = b@1
    "#,
    '1'
  );

  assert!(ac.entry_map.contains_key("bar"));
  assert_eq!(
    ac.entry_map["bar"].parens,
    ParenthesesRecommendation::CursorInside
  );
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_function_in_assignment_has_parentheses_2() {
  let (_fixture, ac) = ac_check!(
    r#"
local bar: ((number) -> number) & (number, number) -> number)
local abc = b@1
    "#,
    '1'
  );

  assert!(ac.entry_map.contains_key("bar"));
  assert_eq!(
    ac.entry_map["bar"].parens,
    ParenthesesRecommendation::CursorInside
  );
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_function_parameters() {
  let (_fixture, ac) = ac_check!(
    r#"
        function abc(test)

@1        end
    "#,
    '1'
  );

  assert!(ac.entry_map.contains_key("test"));
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_function_result_passed_to_function_has_parentheses() {
  let (_fixture, ac) = ac_check!(
    r#"
local function foo() return 1 end
local function bar(a: number) return -a end
local abc = bar(@1)
    "#,
    '1'
  );

  assert!(ac.entry_map.contains_key("foo"));
  assert_eq!(
    ac.entry_map["foo"].parens,
    ParenthesesRecommendation::CursorAfter
  );
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_function_type_types() {
  let mut fixture = AcFixture::default();
  fixture.base.check(
    r#"
local a: (n@1
local b: (number, (n@2
local c: (number, (number) -> n@3
local d: (number, (number) -> (number, n@4
local e: (n: n@5
    "#,
  );

  for marker in b'1'..=b'5' {
    let ac = fixture.base.autocomplete_marker(marker);

    assert!(ac.entry_map.contains_key("nil"));
    assert!(ac.entry_map.contains_key("number"));
  }
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_generic_types() {
  let (_fixture, ac) = ac_check!(
    r#"
function f<Tee, Use>(a: T@1
local b: string = "don't trip"
    "#,
    '1'
  );

  assert!(ac.entry_map.contains_key("Tee"));
  assert_eq!(ac.context, AutocompleteContext::Type);
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_get_frontend_use_correct_global_scope() {
  let mut fixture = AcFixture::default();
  fixture.base.load_definition(
    r#"
        declare extern type Instance with
            Name: string
        end
    "#,
  );

  fixture.base.check(
    r#"
        local a: unknown = nil
        if typeof(a) == "Instance" then
            local b = a.@1
        end
    "#,
  );

  let ac = fixture.base.autocomplete_marker('1');
  assert_eq!(ac.entry_map.len(), 1);
  assert!(ac.entry_map.contains_key("Name"));
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_get_member_completions() {
  let (_fixture, ac) = acb_check!(
    r#"
        local a = table.@1
    "#,
    '1'
  );

  assert_eq!(17, ac.entry_map.len());
  assert!(ac.entry_map.contains_key("find"));
  assert!(ac.entry_map.contains_key("pack"));
  assert!(!ac.entry_map.contains_key("math"));
  assert_eq!(ac.context, AutocompleteContext::Property);
}

mod autocomplete_get_member_completions_autocomplete_test_case_2 {
  //! Source: `tests/Autocomplete.test.cpp`
  use super::AcFixture;

  #[test]
  fn autocomplete_get_member_completions() {
    let mut fixture = AcFixture::default();
    fixture.base.check(
      r#"
        local a = 12.@13
    "#,
    );

    let ac = fixture.base.autocomplete_marker('1');
    assert!(ac.entry_map.is_empty());
  }
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_get_string_completions() {
  let (_fixture, ac) = acb_check!(
    r#"
        local a = ("foo"):@1
    "#,
    '1'
  );

  assert_eq!(17, ac.entry_map.len());
  assert_eq!(ac.context, AutocompleteContext::Property);
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_get_suggestions_for_new_statement() {
  let mut fixture = AcFixture::default();
  fixture.base.check("@1");

  let ac = fixture.base.autocomplete_marker('1');

  assert_ne!(0, ac.entry_map.len());
  assert!(ac.entry_map.contains_key("table"));
  assert_eq!(ac.context, AutocompleteContext::Statement);
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_get_suggestions_for_the_very_start_of_the_script() {
  let (_fixture, ac) = ac_check!(
    r#"@1

        function aaa() end
    "#,
    '1'
  );

  assert!(ac.entry_map.contains_key("table"));
  assert_eq!(ac.context, AutocompleteContext::Statement);
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_global_function_params() {
  let mut fixture = AcFixture::default();
  fixture.base.check(
    r#"
        function abc(def)
    "#,
  );

  for i in 17..25 {
    assert!(
      fixture
        .base
        .autocomplete_position(1, i)
        .entry_map
        .is_empty()
    );
  }
  assert!(
    !fixture
      .base
      .autocomplete_position(1, 26)
      .entry_map
      .is_empty()
  );

  fixture.base.check(
    r#"
        function abc(def)
        end
    "#,
  );

  for i in 17..25 {
    assert!(
      fixture
        .base
        .autocomplete_position(1, i)
        .entry_map
        .is_empty()
    );
  }
  assert!(
    !fixture
      .base
      .autocomplete_position(1, 26)
      .entry_map
      .is_empty()
  );

  fixture.base.check(
    r#"
        function abc(def)
@1
        end
    "#,
  );

  let ac2 = fixture.base.autocomplete_marker('1');
  assert_eq!(1, ac2.entry_map.get("abc").map_or(0, |_| 1));
  assert_eq!(1, ac2.entry_map.get("def").map_or(0, |_| 1));
  assert_eq!(ac2.context, AutocompleteContext::Statement);

  fixture.base.check(
    r#"
        function abc(def, ghi@1)
        end
    "#,
  );

  let ac3 = fixture.base.autocomplete_marker('1');
  assert!(ac3.entry_map.is_empty());
  assert_eq!(ac3.context, AutocompleteContext::Unknown);
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_global_functions_are_not_scoped_lexically() {
  let (_fixture, ac) = ac_check!(
    r#"
        if true then
            function abc()

            end
        end
@1    "#,
    '1'
  );

  assert!(!ac.entry_map.is_empty());
  assert!(ac.entry_map.contains_key("abc"));
  assert!(ac.entry_map.contains_key("table"));
  assert!(ac.entry_map.contains_key("math"));
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_globals_are_order_independent() {
  let (_fixture, ac) = ac_check!(
    r#"
        local myLocal = 4
        function abc0()
            local myInnerLocal = 1
@1
        end

        function abc1()
            local myInnerLocal = 1
        end
    "#,
    '1'
  );

  assert!(ac.entry_map.contains_key("myLocal"));
  assert!(ac.entry_map.contains_key("myInnerLocal"));
  assert!(ac.entry_map.contains_key("abc0"));
  assert!(ac.entry_map.contains_key("abc1"));
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_if_then_else_elseif_completions() {
  let (mut fixture, ac1) = ac_check!(
    r#"
local elsewhere = false

if true then
    return 1
el@1
end
    "#,
    '1'
  );
  assert!(ac1.entry_map.contains_key("else"));
  assert!(ac1.entry_map.contains_key("elseif"));
  assert!(!ac1.entry_map.contains_key("elsewhere"));

  fixture.base.check(
    r#"
local elsewhere = false

if true then
    return 1
else
    return 2
el@1
end
    "#,
  );

  let ac2 = fixture.base.autocomplete_marker('1');
  assert!(!ac2.entry_map.contains_key("else"));
  assert!(!ac2.entry_map.contains_key("elseif"));
  assert!(ac2.entry_map.contains_key("elsewhere"));

  fixture.base.check(
    r#"
local elsewhere = false

if true then
    print("1")
elif true then
    print("2")
el@1
end
    "#,
  );

  let ac3 = fixture.base.autocomplete_marker('1');
  assert!(ac3.entry_map.contains_key("else"));
  assert!(ac3.entry_map.contains_key("elseif"));
  assert!(ac3.entry_map.contains_key("elsewhere"));
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_if_then_else_full_keywords() {
  let (mut fixture, ac1) = ac_check!(
    r#"
local thenceforth = false
local elsewhere = false
local doover = false
local endurance = true

if 1 then@1
else@2
end

while false do@3
end

repeat@4
until
    "#,
    '1'
  );
  assert_eq!(1, ac1.entry_map.len());
  assert!(ac1.entry_map.contains_key("then"));

  let ac2 = fixture.base.autocomplete_marker('2');
  assert!(ac2.entry_map.contains_key("else"));
  assert!(ac2.entry_map.contains_key("elseif"));

  let ac3 = fixture.base.autocomplete_marker('3');
  assert!(ac3.entry_map.contains_key("do"));

  let ac4 = fixture.base.autocomplete_marker('4');
  assert!(ac4.entry_map.contains_key("do"));
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_keyword_members() {
  let (mut fixture, ac1) = ac_check!(
    r#"
local a = { done = 1, forever = 2 }
local b = a.do@1
local c = a.for@2
local d = a.@3
do
end
    "#,
    '1'
  );

  assert_eq!(2, ac1.entry_map.len());
  assert!(ac1.entry_map.contains_key("done"));
  assert!(ac1.entry_map.contains_key("forever"));

  let ac2 = fixture.base.autocomplete_marker('2');

  assert_eq!(2, ac2.entry_map.len());
  assert!(ac2.entry_map.contains_key("done"));
  assert!(ac2.entry_map.contains_key("forever"));

  let ac3 = fixture.base.autocomplete_marker('3');

  assert_eq!(2, ac3.entry_map.len());
  assert!(ac3.entry_map.contains_key("done"));
  assert!(ac3.entry_map.contains_key("forever"));
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_keyword_methods() {
  let (_fixture, ac) = ac_check!(
    r#"
local a = {}
function a:done() end
local b = a:do@1
    "#,
    '1'
  );

  assert_eq!(1, ac.entry_map.len());
  assert!(ac.entry_map.contains_key("done"));
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_keyword_types() {
  let mut fixture = AcFixture::default();
  fixture.base.base.file_resolver.source.insert(
    String::from("Module/A"),
    r#"
export type done = { x: number, y: number }
export type other = { z: number, w: number }
return {}
    "#,
  );

  let module_a = ModuleName::from("Module/A");
  let result = fixture
    .base
    .get_frontend()
    .check_module_name_optional_frontend_options(&module_a, None);
  assert!(result.errors.is_empty());

  fixture.base.base.file_resolver.source.insert(
    String::from("Module/B"),
    r#"
local aaa = require(script.Parent.A)
local a: aaa.do
    "#,
  );

  let module_b = ModuleName::from("Module/B");
  fixture
    .base
    .get_frontend()
    .check_module_name_optional_frontend_options(&module_b, None);

  let ac = fixture
    .base
    .autocomplete_module_name_position_string_completion_callback(
      &module_b,
      Position {
        line: 2,
        column: 15,
      },
      Box::new(null_callback),
    );

  assert_eq!(2, ac.entry_map.len());
  assert!(ac.entry_map.contains_key("done"));
  assert!(ac.entry_map.contains_key("other"));
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_leave_numbers_alone() {
  let mut fixture = AcFixture::default();
  fixture.base.check("local a = 3.@11");

  let ac = fixture.base.autocomplete_marker('1');

  assert!(ac.entry_map.is_empty());
  assert_eq!(ac.context, AutocompleteContext::Unknown);
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_library_non_self_calls_are_fine() {
  let (mut fixture, ac) = acb_check!(
    r#"
string.@1
    "#,
    '1'
  );

  assert!(ac.entry_map.contains_key("byte"));
  assert!(!ac.entry_map["byte"].wrong_index_type);
  assert!(!ac.entry_map["byte"].indexed_with_self);
  assert!(ac.entry_map.contains_key("char"));
  assert!(!ac.entry_map["char"].wrong_index_type);
  assert!(!ac.entry_map["char"].indexed_with_self);
  assert!(ac.entry_map.contains_key("sub"));
  assert!(!ac.entry_map["sub"].wrong_index_type);
  assert!(!ac.entry_map["sub"].indexed_with_self);

  fixture.base.check(
    r#"
table.@1
    "#,
  );

  let ac = fixture.base.autocomplete_marker('1');

  assert!(ac.entry_map.contains_key("remove"));
  assert!(!ac.entry_map["remove"].wrong_index_type);
  assert!(!ac.entry_map["remove"].indexed_with_self);
  assert!(ac.entry_map.contains_key("getn"));
  assert!(!ac.entry_map["getn"].wrong_index_type);
  assert!(!ac.entry_map["getn"].indexed_with_self);
  assert!(ac.entry_map.contains_key("insert"));
  assert!(!ac.entry_map["insert"].wrong_index_type);
  assert!(!ac.entry_map["insert"].indexed_with_self);
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_library_self_calls_are_invalid() {
  let (_fixture, ac) = acb_check!(
    r#"
string:@1
    "#,
    '1'
  );

  assert!(ac.entry_map.contains_key("byte"));
  assert!(ac.entry_map["byte"].wrong_index_type);
  assert!(ac.entry_map["byte"].indexed_with_self);
  assert!(ac.entry_map.contains_key("char"));
  assert!(ac.entry_map["char"].wrong_index_type);
  assert!(ac.entry_map["char"].indexed_with_self);
  assert!(ac.entry_map.contains_key("sub"));
  assert!(!ac.entry_map["sub"].wrong_index_type);
  assert!(ac.entry_map["sub"].indexed_with_self);
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_local_function() {
  let (mut fixture, ac1) = ac_check!(
    r#"
        local f@1
    "#,
    '1'
  );
  assert_eq!(1, ac1.entry_map.len());
  assert_eq!(1, ac1.entry_map.get("function").map_or(0, |_| 1));

  fixture.base.check(
    r#"
        local f@1, cd
    "#,
  );

  let ac2 = fixture.base.autocomplete_marker('1');
  assert!(ac2.entry_map.is_empty());
}

mod autocomplete_local_function_autocomplete_test_case_2 {
  //! Source: `tests/Autocomplete.test.cpp`
  use super::AcFixture;

  #[test]
  fn autocomplete_local_function() {
    let mut fixture = AcFixture::default();
    fixture.base.check(
      r#"
        local function @1
    "#,
    );

    let mut ac = fixture.base.autocomplete_marker('1');
    assert!(ac.entry_map.is_empty());

    fixture.base.check(
      r#"
        local function @1s@2
    "#,
    );

    ac = fixture.base.autocomplete_marker('1');
    assert!(ac.entry_map.is_empty());

    ac = fixture.base.autocomplete_marker('2');
    assert!(ac.entry_map.is_empty());

    fixture.base.check(
      r#"
        local function @1()@2
    "#,
    );

    ac = fixture.base.autocomplete_marker('1');
    assert!(ac.entry_map.is_empty());

    ac = fixture.base.autocomplete_marker('2');
    assert!(ac.entry_map.contains_key("end"));

    fixture.base.check(
      r#"
        local function something@1
    "#,
    );

    ac = fixture.base.autocomplete_marker('1');
    assert!(ac.entry_map.is_empty());

    fixture.base.check(
      r#"
        local tbl = {}
        function tbl.something@1() end
    "#,
    );

    ac = fixture.base.autocomplete_marker('1');
    assert!(ac.entry_map.is_empty());
  }
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_local_function_params() {
  let mut fixture = AcFixture::default();
  fixture.base.check(
    r#"
        local function @1a@2bc(@3d@4ef)@5 @6
    "#,
  );

  for marker in *b"1234" {
    assert!(
      fixture
        .base
        .autocomplete_marker(marker)
        .entry_map
        .is_empty()
    );
  }

  assert!(!fixture.base.autocomplete_marker('5').entry_map.is_empty());
  assert!(!fixture.base.autocomplete_marker('6').entry_map.is_empty());

  fixture.base.check(
    r#"
        local function abc(def)
@1        end
    "#,
  );

  for i in 23..31 {
    assert!(
      fixture
        .base
        .autocomplete_position(1, i)
        .entry_map
        .is_empty()
    );
  }
  assert!(
    !fixture
      .base
      .autocomplete_position(1, 32)
      .entry_map
      .is_empty()
  );

  let ac2 = fixture.base.autocomplete_marker('1');
  assert_eq!(1, ac2.entry_map.get("abc").map_or(0, |_| 1));
  assert_eq!(1, ac2.entry_map.get("def").map_or(0, |_| 1));
  assert_eq!(ac2.context, AutocompleteContext::Statement);

  fixture.base.check(
    r#"
        local function abc(def, ghi@1)
        end
    "#,
  );

  let ac3 = fixture.base.autocomplete_marker('1');
  assert!(ac3.entry_map.is_empty());
  assert_eq!(ac3.context, AutocompleteContext::Unknown);
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_local_functions_fall_out_of_scope() {
  let (_fixture, ac) = ac_check!(
    r#"
        if true then
            local function abc()

            end
        end
@1    "#,
    '1'
  );

  assert_ne!(0, ac.entry_map.len());
  assert!(!ac.entry_map.contains_key("abc"));
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_local_initializer_2() {
  let (_fixture, ac) = ac_check!(
    r#"
        local a=@1
    "#,
    '1'
  );
  assert!(ac.entry_map.contains_key("table"));
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_local_initializer() {
  let mut fixture = AcFixture::default();
  fixture.base.check("local a = @1");

  let ac = fixture.base.autocomplete_marker('1');

  assert!(ac.entry_map.contains_key("table"));
  assert!(ac.entry_map.contains_key("math"));
  assert_eq!(ac.context, AutocompleteContext::Expression);
}

mod autocomplete_local_initializer_autocomplete_test_case_2 {
  //! Ported from `tests/Autocomplete.test.cpp`.
  //! Source: `tests/Autocomplete.test.cpp`
  use super::AcFixture;

  #[test]
  fn autocomplete_local_initializer() {
    let mut fixture = AcFixture::default();
    fixture.base.check(
      r#"
        local a = t@1
    "#,
    );

    let ac = fixture.base.autocomplete_marker('1');
    assert!(ac.entry_map.contains_key("table"));
    assert!(ac.entry_map.contains_key("true"));
  }
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_local_names() {
  let (mut fixture, ac1) = ac_check!(
    r#"
        local ab@1
    "#,
    '1'
  );
  assert_eq!(1, ac1.entry_map.len());
  assert_eq!(1, ac1.entry_map.get("function").map_or(0, |_| 1));
  assert_eq!(ac1.context, AutocompleteContext::Unknown);

  fixture.base.check(
    r#"
        local ab, cd@1
    "#,
  );

  let ac2 = fixture.base.autocomplete_marker('1');
  assert!(ac2.entry_map.is_empty());
  assert_eq!(ac2.context, AutocompleteContext::Unknown);
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_local_types_builtin() {
  let (_fixture, ac) = ac_check!(
    r#"
local a: n@1
local b: string = "don't trip"
    "#,
    '1'
  );

  assert!(ac.entry_map.contains_key("nil"));
  assert!(ac.entry_map.contains_key("number"));
  assert_eq!(ac.context, AutocompleteContext::Type);
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_method_call_inside_function_body() {
  let (_fixture, ac) = ac_check!(
    r#"
        local game = { GetService=function(s) return 'hello' end }

        function a()
            game:  @1
        end
    "#,
    '1'
  );

  assert_ne!(0, ac.entry_map.len());
  assert!(!ac.entry_map.contains_key("math"));
  assert_eq!(ac.context, AutocompleteContext::Property);
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_method_call_inside_if_conditional() {
  let (_fixture, ac) = acb_check!(
    r#"
        if table:  @1
    "#,
    '1'
  );

  assert_ne!(0, ac.entry_map.len());
  assert!(ac.entry_map.contains_key("concat"));
  assert!(!ac.entry_map.contains_key("math"));
  assert_eq!(ac.context, AutocompleteContext::Property);
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_module_type_members() {
  let mut fixture = AcFixture::default();
  fixture.base.base.file_resolver.source.insert(
    String::from("Module/A"),
    r#"
export type A = { x: number, y: number }
export type B = { z: number, w: number }
return {}
    "#,
  );

  let module_a = ModuleName::from("Module/A");
  let result = fixture
    .base
    .get_frontend()
    .check_module_name_optional_frontend_options(&module_a, None);
  assert!(result.errors.is_empty());

  fixture.base.base.file_resolver.source.insert(
    String::from("Module/B"),
    r#"
local aaa = require(script.Parent.A)
local a: aaa.
    "#,
  );

  let module_b = ModuleName::from("Module/B");
  fixture
    .base
    .get_frontend()
    .check_module_name_optional_frontend_options(&module_b, None);

  let ac = fixture
    .base
    .autocomplete_module_name_position_string_completion_callback(
      &module_b,
      Position {
        line: 2,
        column: 13,
      },
      Box::new(null_callback),
    );

  assert_eq!(2, ac.entry_map.len());
  assert!(ac.entry_map.contains_key("A"));
  assert!(ac.entry_map.contains_key("B"));
  assert_eq!(ac.context, AutocompleteContext::Type);
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_modules_with_types() {
  let mut fixture = AcFixture::default();
  fixture.base.base.file_resolver.source.insert(
    String::from("Module/A"),
    r#"
export type A = { x: number, y: number }
export type B = { z: number, w: number }
return {}
    "#,
  );

  let module_a = ModuleName::from("Module/A");
  let result = fixture
    .base
    .get_frontend()
    .check_module_name_optional_frontend_options(&module_a, None);
  assert!(result.errors.is_empty());

  fixture.base.base.file_resolver.source.insert(
    String::from("Module/B"),
    r#"
local aaa = require(script.Parent.A)
local a: aa
    "#,
  );

  let module_b = ModuleName::from("Module/B");
  fixture
    .base
    .get_frontend()
    .check_module_name_optional_frontend_options(&module_b, None);

  let ac = fixture
    .base
    .autocomplete_module_name_position_string_completion_callback(
      &module_b,
      Position {
        line: 2,
        column: 11,
      },
      Box::new(null_callback),
    );

  assert!(ac.entry_map.contains_key("aaa"));
  assert_eq!(ac.context, AutocompleteContext::Type);
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_nested_member_completions() {
  let (_fixture, ac) = ac_check!(
    r#"
        local tbl = { abc = { def = 1234, egh = false } }
        tbl.abc. @1
    "#,
    '1'
  );

  assert_eq!(2, ac.entry_map.len());
  assert!(ac.entry_map.contains_key("def"));
  assert!(ac.entry_map.contains_key("egh"));
  assert_eq!(ac.context, AutocompleteContext::Property);
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_nested_recursive_function() {
  let (_fixture, ac) = ac_check!(
    r#"
        local function outer()
            local function inner()
@1            end
        end
    "#,
    '1'
  );

  assert!(ac.entry_map.contains_key("inner"));
  assert!(ac.entry_map.contains_key("outer"));
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_no_function_name_suggestions() {
  let (mut fixture, ac1) = ac_check!(
    r#"
function na@1
    "#,
    '1'
  );

  assert!(ac1.entry_map.is_empty());

  fixture.base.check(
    r#"
local function @1
    "#,
  );

  let ac2 = fixture.base.autocomplete_marker('1');

  assert!(ac2.entry_map.is_empty());

  fixture.base.check(
    r#"
local function na@1
    "#,
  );

  let ac3 = fixture.base.autocomplete_marker('1');

  assert!(ac3.entry_map.is_empty());
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_no_incompatible_self_calls() {
  let (_fixture, ac) = ac_check!(
    r#"
local t = {}
function t.m() end
t:@1
    "#,
    '1'
  );

  assert!(ac.entry_map.contains_key("m"));
  assert!(ac.entry_map["m"].wrong_index_type);
  assert!(ac.entry_map["m"].indexed_with_self);
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_no_incompatible_self_calls_2() {
  let (_fixture, ac) = ac_check!(
    r#"
local f: (() -> number) & ((number) -> number) = function(x: number?) return 2 end
local t = {}
t.f = f
t:@1
    "#,
    '1'
  );

  assert!(ac.entry_map.contains_key("f"));
  assert!(ac.entry_map["f"].wrong_index_type);
  assert!(ac.entry_map["f"].indexed_with_self);
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_no_incompatible_self_calls_on_class() {
  let mut fixture = AcFixture::default();
  fixture.base.load_definition(
    r#"
declare extern type Foo with
    function one(self): number
    two: () -> number
end
    "#,
  );

  fixture.base.check(
    r#"
local function f(t: Foo)
    t:@1
end
    "#,
  );

  let ac = fixture.base.autocomplete_marker('1');

  assert!(ac.entry_map.contains_key("one"));
  assert!(ac.entry_map.contains_key("two"));
  assert!(!ac.entry_map["one"].wrong_index_type);
  assert!(ac.entry_map["two"].wrong_index_type);
  assert!(ac.entry_map["one"].indexed_with_self);
  assert!(ac.entry_map["two"].indexed_with_self);

  fixture.base.check(
    r#"
local function f(t: Foo)
    t.@1
end
    "#,
  );

  let ac = fixture.base.autocomplete_marker('1');

  assert!(ac.entry_map.contains_key("one"));
  assert!(ac.entry_map.contains_key("two"));
  assert!(ac.entry_map["one"].wrong_index_type);
  assert!(!ac.entry_map["two"].wrong_index_type);
  assert!(!ac.entry_map["one"].indexed_with_self);
  assert!(!ac.entry_map["two"].indexed_with_self);
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_no_wrong_compatible_self_calls_with_generics() {
  let (_fixture, ac) = ac_check!(
    r#"
local t = {}
function t.m<T>(a: T) end
t:@1
    "#,
    '1'
  );

  assert!(ac.entry_map.contains_key("m"));
  assert!(ac.entry_map["m"].wrong_index_type);
  assert!(ac.entry_map["m"].indexed_with_self);
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_not_the_var_we_are_defining() {
  let mut fixture = AcFixture::default();
  fixture
    .base
    .base
    .file_resolver
    .source
    .insert(String::from("Module/A"), String::from("abc,de"));

  let module = ModuleName::from("Module/A");
  let ac = fixture
    .base
    .autocomplete_module_name_position_string_completion_callback(
      &module,
      Position { line: 0, column: 6 },
      Box::new(null_callback),
    );

  assert!(!ac.entry_map.contains_key("de"));
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_optional_members() {
  let (mut fixture, ac1) = ac_check!(
    r#"
local a = { x = 2, y = 3 }
type A = typeof(a)
local b: A? = a
return b.@1
    "#,
    '1'
  );

  assert_eq!(2, ac1.entry_map.len());
  assert!(ac1.entry_map.contains_key("x"));
  assert!(ac1.entry_map.contains_key("y"));

  fixture.base.check(
    r#"
local a = { x = 2, y = 3 }
type A = typeof(a)
local b: nil | A = a
return b.@1
    "#,
  );

  let ac2 = fixture.base.autocomplete_marker('1');

  assert_eq!(2, ac2.entry_map.len());
  assert!(ac2.entry_map.contains_key("x"));
  assert!(ac2.entry_map.contains_key("y"));

  fixture.base.check(
    r#"
local b: nil | nil
return b.@1
    "#,
  );

  let ac3 = fixture.base.autocomplete_marker('1');

  assert_eq!(0, ac3.entry_map.len());
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_private_types() {
  let (mut fixture, ac1) = ac_check!(
    r#"
do
    type num = number
    local a: n@1u
    local b: nu@2m
end
local a: nu@3
    "#,
    '1'
  );

  assert!(ac1.entry_map.contains_key("num"));
  assert!(ac1.entry_map.contains_key("number"));

  let ac2 = fixture.base.autocomplete_marker('2');

  assert!(ac2.entry_map.contains_key("num"));
  assert!(ac2.entry_map.contains_key("number"));

  let ac3 = fixture.base.autocomplete_marker('3');

  assert!(!ac3.entry_map.contains_key("num"));
  assert!(ac3.entry_map.contains_key("number"));
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_recommend_statement_starting_keywords() {
  let mut fixture = AcFixture::default();
  fixture.base.check("@1");
  let ac = fixture.base.autocomplete_marker('1');
  assert!(ac.entry_map.contains_key("local"));
  assert_eq!(ac.context, AutocompleteContext::Statement);

  fixture.base.check("local i = @1");
  let ac2 = fixture.base.autocomplete_marker('1');
  assert!(!ac2.entry_map.contains_key("local"));
  assert_eq!(ac2.context, AutocompleteContext::Expression);
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_recursive_function() {
  let (_fixture, ac) = ac_check!(
    r#"
        function foo()
@1        end
    "#,
    '1'
  );

  assert!(ac.entry_map.contains_key("foo"));
  assert_eq!(ac.context, AutocompleteContext::Statement);
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_recursive_function_global() {
  let mut fixture = AcFixture::default();
  fixture.base.base.file_resolver.source.insert(
    String::from("global"),
    r#"function abc()

end
"#,
  );

  let module = ModuleName::from("global");
  let ac = fixture
    .base
    .autocomplete_module_name_position_string_completion_callback(
      &module,
      Position { line: 1, column: 0 },
      Box::new(null_callback),
    );

  assert!(ac.entry_map.contains_key("abc"));
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_recursive_function_local() {
  let mut fixture = AcFixture::default();
  fixture.base.base.file_resolver.source.insert(
    String::from("local"),
    r#"local function abc()

end
"#,
  );

  let module = ModuleName::from("local");
  let ac = fixture
    .base
    .autocomplete_module_name_position_string_completion_callback(
      &module,
      Position { line: 1, column: 0 },
      Box::new(null_callback),
    );

  assert!(ac.entry_map.contains_key("abc"));
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_require_by_string() {
  use ulua_analysis::type_aliases::autocomplete_entry_map::AutocompleteEntryMap;
  fn check_entries(entry_map: &AutocompleteEntryMap, completions: &[(&str, &str)]) {
    assert_eq!(completions.len(), entry_map.len());

    for (label, insert_text) in completions {
      let entry = entry_map
        .get(*label)
        .unwrap_or_else(|| panic!("missing require completion `{label}`"));
      assert_eq!(entry.insert_text.as_deref(), Some(*insert_text));
    }
  }

  let mut fixture = ACBuiltinsFixture::default();
  fixture.base.base.file_resolver.source.insert(
    String::from("MainModule"),
    r#"
        local info = "MainModule serves as the root directory"
    "#,
  );

  fixture.base.base.file_resolver.source.insert(
    String::from("MainModule/Folder"),
    r#"
        local info = "MainModule/Folder serves as a subdirectory"
    "#,
  );

  fixture.base.base.file_resolver.source.insert(
    String::from("MainModule/Folder/Requirer"),
    r#"
        local res0 = require("@")

        local res1 = require(".")
        local res2 = require("./")
        local res3 = require("./Sib")

        local res4 = require("..")
        local res5 = require("../")
        local res6 = require("../Sib")
    "#,
  );

  fixture.base.base.file_resolver.source.insert(
    String::from("MainModule/Folder/SiblingDependency"),
    r#"
        return {"result"}
    "#,
  );

  fixture.base.base.file_resolver.source.insert(
    String::from("MainModule/ParentDependency"),
    r#"
        return {"result"}
    "#,
  );

  let module_name = ModuleName::from("MainModule/Folder/Requirer");

  let ac_result = fixture
    .base
    .autocomplete_module_name_position_string_completion_callback(
      &module_name,
      Position {
        line: 1,
        column: 31,
      },
      Box::new(null_callback),
    );
  check_entries(
    &ac_result.entry_map,
    &[
      ("@defaultalias", "@defaultalias"),
      ("./", "./"),
      ("../", "../"),
    ],
  );

  let ac_result = fixture
    .base
    .autocomplete_module_name_position_string_completion_callback(
      &module_name,
      Position {
        line: 3,
        column: 31,
      },
      Box::new(null_callback),
    );
  check_entries(
    &ac_result.entry_map,
    &[
      ("@defaultalias", "@defaultalias"),
      ("./", "./"),
      ("../", "../"),
    ],
  );

  let ac_result = fixture
    .base
    .autocomplete_module_name_position_string_completion_callback(
      &module_name,
      Position {
        line: 4,
        column: 32,
      },
      Box::new(null_callback),
    );
  check_entries(
    &ac_result.entry_map,
    &[
      ("..", "."),
      ("Requirer", "./Requirer"),
      ("SiblingDependency", "./SiblingDependency"),
    ],
  );

  let ac_result = fixture
    .base
    .autocomplete_module_name_position_string_completion_callback(
      &module_name,
      Position {
        line: 5,
        column: 35,
      },
      Box::new(null_callback),
    );
  check_entries(
    &ac_result.entry_map,
    &[
      ("..", "."),
      ("Requirer", "./Requirer"),
      ("SiblingDependency", "./SiblingDependency"),
    ],
  );

  let ac_result = fixture
    .base
    .autocomplete_module_name_position_string_completion_callback(
      &module_name,
      Position {
        line: 7,
        column: 32,
      },
      Box::new(null_callback),
    );
  check_entries(
    &ac_result.entry_map,
    &[
      ("@defaultalias", "@defaultalias"),
      ("./", "./"),
      ("../", "../"),
    ],
  );

  let ac_result = fixture
    .base
    .autocomplete_module_name_position_string_completion_callback(
      &module_name,
      Position {
        line: 8,
        column: 33,
      },
      Box::new(null_callback),
    );
  check_entries(
    &ac_result.entry_map,
    &[
      ("..", "../.."),
      ("Folder", "../Folder"),
      ("ParentDependency", "../ParentDependency"),
    ],
  );

  let ac_result = fixture
    .base
    .autocomplete_module_name_position_string_completion_callback(
      &module_name,
      Position {
        line: 9,
        column: 36,
      },
      Box::new(null_callback),
    );
  check_entries(
    &ac_result.entry_map,
    &[
      ("..", "../.."),
      ("Folder", "../Folder"),
      ("ParentDependency", "../ParentDependency"),
    ],
  );
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_require_tracing() {
  let mut fixture = ACBuiltinsFixture::default();
  fixture.base.base.file_resolver.source.insert(
    String::from("Module/A"),
    r#"
return { x = 0 }
    "#,
  );
  fixture.base.base.file_resolver.source.insert(
    String::from("Module/B"),
    r#"
local result = require(script.Parent.A)
local x = 1 + result.
    "#,
  );

  let ac = fixture
    .base
    .autocomplete_module_name_position_string_completion_callback(
      &ModuleName::from("Module/B"),
      Position {
        line: 2,
        column: 21,
      },
      Box::new(null_callback),
    );

  assert_eq!(ac.entry_map.len(), 1);
  assert!(ac.entry_map.contains_key("x"));
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_return_types() {
  let (_fixture, ac) = ac_check!(
    r#"
local function f(a: number): n@1
local b: string = "don't trip"
    "#,
    '1'
  );

  assert!(ac.entry_map.contains_key("nil"));
  assert!(ac.entry_map.contains_key("number"));
  assert_eq!(ac.context, AutocompleteContext::Type);
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_simple() {
  let mut fixture = AcFixture::default();
  fixture.base.check(
    r#"
local t = {}
function t:m() end
t:m()
    "#,
  );
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_skip_current_local() {
  let (mut fixture, ac1) = ac_check!(
    r#"
local other = 1
local name = na@1
    "#,
    '1'
  );

  assert!(!ac1.entry_map.contains_key("name"));
  assert!(ac1.entry_map.contains_key("other"));

  fixture.base.check(
    r#"
local other = 1
local name, test = na@1
    "#,
  );

  let ac2 = fixture.base.autocomplete_marker('1');

  assert!(!ac2.entry_map.contains_key("name"));
  assert!(!ac2.entry_map.contains_key("test"));
  assert!(ac2.entry_map.contains_key("other"));
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_sometimes_the_metatable_is_an_error() {
  let mut fixture = AcFixture::default();
  fixture.base.check(
    r#"
        local T = {}
        T.__index = T

        function T.new()
            return setmetatable({x=6}, X) -- oops!
        end
        local t = T.new()
        t.  @1
    "#,
  );

  fixture.base.autocomplete_marker('1');
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_source_module_preservation_and_invalidation() {
  use ulua_unit_test::records::ac_fixture::AcFixture;
  let mut fixture = AcFixture::default();
  fixture.base.check(
    r#"
local a = { x = 2, y = 4 }
a.@1
    "#,
  );

  fixture.base.get_frontend().clear();

  let ac = fixture.base.autocomplete_marker('1');

  assert_eq!(2, ac.entry_map.len());
  assert!(ac.entry_map.contains_key("x"));
  assert!(ac.entry_map.contains_key("y"));

  fixture
    .base
    .get_frontend()
    .check_module_name_optional_frontend_options(&ModuleName::from("MainModule"), None);

  let ac = fixture.base.autocomplete_marker('1');

  assert!(ac.entry_map.contains_key("x"));
  assert!(ac.entry_map.contains_key("y"));

  fixture
    .base
    .get_frontend()
    .mark_dirty(&ModuleName::from("MainModule"), None);

  let ac = fixture.base.autocomplete_marker('1');

  assert!(ac.entry_map.contains_key("x"));
  assert!(ac.entry_map.contains_key("y"));

  fixture
    .base
    .get_frontend()
    .check_module_name_optional_frontend_options(&ModuleName::from("MainModule"), None);

  let ac = fixture.base.autocomplete_marker('1');

  assert!(ac.entry_map.contains_key("x"));
  assert!(ac.entry_map.contains_key("y"));
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_statement_between_two_statements() {
  let (_fixture, ac) = ac_check!(
    r#"
        function getmyscripts() end

        g@1

        getmyscripts()
    "#,
    '1'
  );

  assert_ne!(0, ac.entry_map.len());
  assert!(ac.entry_map.contains_key("getmyscripts"));
  assert_eq!(ac.context, AutocompleteContext::Statement);
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_stop_at_first_stat_when_recommending_keywords() {
  let (_fixture, ac1) = ac_check!(
    r#"
        repeat
            for x @1
    "#,
    '1'
  );
  assert_eq!(1, ac1.entry_map.get("in").map_or(0, |_| 1));
  assert_eq!(0, ac1.entry_map.get("until").map_or(0, |_| 1));
  assert_eq!(ac1.context, AutocompleteContext::Keyword);
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_strict_mode_force() {
  let (_fixture, ac) = ac_check!(
    r#"
--!nonstrict
local a: {x: number} = {x=1}
local b = a
local c = b.@1
    "#,
    '1'
  );

  assert_eq!(1, ac.entry_map.len());
  assert!(ac.entry_map.contains_key("x"));
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_string_completion_outside_quotes() {
  use ulua_analysis::{
    records::autocomplete_entry::AutocompleteEntry,
    type_aliases::autocomplete_entry_map::AutocompleteEntryMap,
  };
  use ulua_unit_test::functions::autocomplete_attach_require_call_tag::autocomplete_attach_require_call_tag;

  let mut fixture = AcFixture::default();
  fixture.base.load_definition(
    r#"
        declare function require(path: string): any
    "#,
  );

  autocomplete_attach_require_call_tag(fixture.base.get_frontend());

  fixture.base.check(
    r#"
        local x = require(@1"@2"@3)
    "#,
  );

  let ac = fixture.base.autocomplete_marker_callback(
    '2',
    Box::new(|_tag, _extern_type, _contents| {
      let mut results = AutocompleteEntryMap::new();
      results.insert(
        String::from("test"),
        AutocompleteEntry {
          kind: AutocompleteEntryKind::String,
          ..Default::default()
        },
      );
      Some(results)
    }),
  );

  assert_eq!(ac.entry_map.len(), 1);
  assert!(ac.entry_map.contains_key("test"));

  let ac = fixture.base.autocomplete_marker_callback(
    '1',
    Box::new(|_tag, _extern_type, _contents| {
      let mut results = AutocompleteEntryMap::new();
      results.insert(
        String::from("test"),
        AutocompleteEntry {
          kind: AutocompleteEntryKind::String,
          ..Default::default()
        },
      );
      Some(results)
    }),
  );

  assert_eq!(ac.entry_map.len(), 0);

  let ac = fixture.base.autocomplete_marker_callback(
    '3',
    Box::new(|_tag, _extern_type, _contents| {
      let mut results = AutocompleteEntryMap::new();
      results.insert(
        String::from("test"),
        AutocompleteEntry {
          kind: AutocompleteEntryKind::String,
          ..Default::default()
        },
      );
      Some(results)
    }),
  );

  assert_eq!(ac.entry_map.len(), 0);
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_string_contents_is_available_to_callback() {
  use alloc::rc::Rc;
  use core::cell::Cell;

  use ulua_unit_test::functions::autocomplete_attach_require_call_tag::autocomplete_attach_require_call_tag;

  let mut fixture = AcFixture::default();
  fixture.base.load_definition(
    r#"
        declare function require(path: string): any
    "#,
  );

  autocomplete_attach_require_call_tag(fixture.base.get_frontend());

  fixture.base.check(
    r#"
        local x = require("testing/@1")
    "#,
  );

  let is_correct = Rc::new(Cell::new(false));
  let is_correct_for_callback = Rc::clone(&is_correct);
  fixture.base.autocomplete_marker_callback(
    '1',
    Box::new(move |_tag, _extern_type, contents| {
      is_correct_for_callback.set(contents.as_deref() == Some("testing/"));
      None
    }),
  );

  assert!(is_correct.get());
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_string_prim_non_self_calls_are_avoided() {
  let (_fixture, ac) = ac_check!(
    r#"
local s = "hello"
s.@1
    "#,
    '1'
  );

  assert!(ac.entry_map.contains_key("char"));
  assert!(!ac.entry_map["char"].wrong_index_type);
  assert!(!ac.entry_map["char"].indexed_with_self);
  assert!(ac.entry_map.contains_key("sub"));
  assert!(ac.entry_map["sub"].wrong_index_type);
  assert!(!ac.entry_map["sub"].indexed_with_self);
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_string_prim_self_calls_are_fine() {
  let (_fixture, ac) = ac_check!(
    r#"
local s = "hello"
s:@1
    "#,
    '1'
  );

  assert!(ac.entry_map.contains_key("byte"));
  assert!(!ac.entry_map["byte"].wrong_index_type);
  assert!(ac.entry_map["byte"].indexed_with_self);
  assert!(ac.entry_map.contains_key("char"));
  assert!(ac.entry_map["char"].wrong_index_type);
  assert!(ac.entry_map["char"].indexed_with_self);
  assert!(ac.entry_map.contains_key("sub"));
  assert!(!ac.entry_map["sub"].wrong_index_type);
  assert!(ac.entry_map["sub"].indexed_with_self);
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_string_singleton_as_table_key() {
  let (mut fixture, ac) = ac_check!(
    r#"
        type Direction = "up" | "down"

        local a: {[Direction]: boolean} = {[@1] = true}
        local b: {[Direction]: boolean} = {["@2"] = true}
        local c: {[Direction]: boolean} = {u@3 = true}
        local d: {[Direction]: boolean} = {[u@4] = true}

        local e: {[Direction]: boolean} = {[@5]}
        local f: {[Direction]: boolean} = {["@6"]}
        local g: {[Direction]: boolean} = {u@7}
        local h: {[Direction]: boolean} = {[u@8]}
    "#,
    '1'
  );
  assert!(ac.entry_map.contains_key("\"up\""));
  assert!(ac.entry_map.contains_key("\"down\""));

  let ac = fixture.base.autocomplete_marker('2');
  assert!(ac.entry_map.contains_key("up"));
  assert!(ac.entry_map.contains_key("down"));

  let ac = fixture.base.autocomplete_marker('3');
  assert!(ac.entry_map.contains_key("up"));
  assert!(ac.entry_map.contains_key("down"));

  let ac = fixture.base.autocomplete_marker('4');
  assert!(!ac.entry_map.contains_key("up"));
  assert!(!ac.entry_map.contains_key("down"));
  assert!(ac.entry_map.contains_key("\"up\""));
  assert!(ac.entry_map.contains_key("\"down\""));

  let ac = fixture.base.autocomplete_marker('5');
  assert!(ac.entry_map.contains_key("\"up\""));
  assert!(ac.entry_map.contains_key("\"down\""));

  let ac = fixture.base.autocomplete_marker('6');
  assert!(ac.entry_map.contains_key("up"));
  assert!(ac.entry_map.contains_key("down"));

  let ac = fixture.base.autocomplete_marker('7');
  assert!(ac.entry_map.contains_key("up"));
  assert!(ac.entry_map.contains_key("down"));

  let ac = fixture.base.autocomplete_marker('8');
  assert!(!ac.entry_map.contains_key("up"));
  assert!(!ac.entry_map.contains_key("down"));
  assert!(ac.entry_map.contains_key("\"up\""));
  assert!(ac.entry_map.contains_key("\"down\""));
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_string_singleton_as_table_key_iso() {
  let (_fixture, ac) = ac_check!(
    r#"
        type Direction = "up" | "down"
        local b: {[Direction]: boolean} = {["@2"] = true}
    "#,
    '2'
  );

  assert!(ac.entry_map.contains_key("up"));
  assert!(ac.entry_map.contains_key("down"));
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_string_singleton_in_if_statement() {
  let _new_solver = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);

  let mut fixture = AcFixture::default();
  fixture.base.check(
    r#"
        --!strict

        type Direction = "left" | "right"

        local dir: Direction = "left"

        if dir == @1"@2"@3 then end
        local a: {[Direction]: boolean} = {[@4"@5"@6]}

        if dir == @7`@8`@9 then end
        local a: {[Direction]: boolean} = {[@A`@B`@C]}
    "#,
  );

  let mut check_marker = |marker: u8, has_left: bool, has_right: bool| {
    let ac = fixture.base.autocomplete_marker(marker);
    assert_eq!(ac.entry_map.contains_key("left"), has_left);
    assert_eq!(ac.entry_map.contains_key("right"), has_right);
  };

  check_marker(b'1', false, false);
  check_marker(b'2', true, true);
  check_marker(b'3', false, false);
  check_marker(b'4', false, false);
  check_marker(b'5', true, true);
  check_marker(b'6', false, false);
  check_marker(b'7', false, false);
  check_marker(b'8', true, true);
  check_marker(b'9', false, false);
  check_marker(b'A', false, false);
  check_marker(b'B', true, true);
  check_marker(b'C', false, false);
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_string_singleton_in_if_statement_2() {
  let _new_solver = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);

  let mut fixture = AcFixture::default();
  fixture.base.check(
    r#"
        --!strict

        type Direction = "left" | "right"

        local dir: Direction
        -- typestate here means dir is actually typed as `"left"`
        dir = "left"

        if dir == @1"@2"@3 then end
        local a: {[Direction]: boolean} = {[@4"@5"@6]}

        if dir == @7`@8`@9 then end
        local a: {[Direction]: boolean} = {[@A`@B`@C]}
    "#,
  );

  let mut check_marker = |marker: u8, has_left: bool, has_right: bool| {
    let ac = fixture.base.autocomplete_marker(marker);
    assert_eq!(ac.entry_map.contains_key("left"), has_left);
    assert_eq!(ac.entry_map.contains_key("right"), has_right);
  };

  check_marker(b'1', false, false);
  check_marker(b'2', true, false);
  check_marker(b'3', false, false);
  check_marker(b'4', false, false);
  check_marker(b'5', true, true);
  check_marker(b'6', false, false);
  check_marker(b'7', false, false);
  check_marker(b'8', true, false);
  check_marker(b'9', false, false);
  check_marker(b'A', false, false);
  check_marker(b'B', true, true);
  check_marker(b'C', false, false);
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_suggest_exported_types() {
  let (_fixture, ac) = ac_check!(
    r#"
export type Type = {a: number}
local a: T@1
    "#,
    '1'
  );

  assert!(ac.entry_map.contains_key("Type"));
  assert_eq!(ac.context, AutocompleteContext::Type);
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_suggest_external_module_type() {
  let mut fixture = ACBuiltinsFixture::default();
  fixture.base.base.file_resolver.source.insert(
    String::from("Module/A"),
    r#"
export type done = { x: number, y: number }
local function a(a: (done) -> number) return a({x=1, y=2}) end
local function b(a: ((done) -> number) -> number) return a(function(done) return 1 end) end
return {a = a, b = b}
    "#,
  );

  let module_a = ModuleName::from("Module/A");
  let result = fixture
    .base
    .get_frontend()
    .check_module_name_optional_frontend_options(&module_a, None);
  assert!(result.errors.is_empty());

  fixture.base.base.file_resolver.source.insert(
    String::from("Module/B"),
    r#"
local ex = require(script.Parent.A)
ex.a(function(x:
    "#,
  );

  let module_b = ModuleName::from("Module/B");
  fixture
    .base
    .get_frontend()
    .check_module_name_optional_frontend_options(&module_b, None);

  let ac1 = fixture
    .base
    .autocomplete_module_name_position_string_completion_callback(
      &module_b,
      Position {
        line: 2,
        column: 16,
      },
      Box::new(null_callback),
    );

  assert!(!ac1.entry_map.contains_key("done"));
  assert!(ac1.entry_map.contains_key("ex.done"));
  assert_eq!(
    ac1.entry_map["ex.done"].type_correct,
    TypeCorrectKind::Correct
  );

  fixture.base.base.file_resolver.source.insert(
    String::from("Module/C"),
    r#"
local ex = require(script.Parent.A)
ex.b(function(x:
    "#,
  );

  let module_c = ModuleName::from("Module/C");
  fixture
    .base
    .get_frontend()
    .check_module_name_optional_frontend_options(&module_c, None);

  let ac2 = fixture
    .base
    .autocomplete_module_name_position_string_completion_callback(
      &module_c,
      Position {
        line: 2,
        column: 16,
      },
      Box::new(null_callback),
    );

  assert!(!ac2.entry_map.contains_key("(done) -> number"));
  assert!(ac2.entry_map.contains_key("(ex.done) -> number"));
  assert_eq!(
    ac2.entry_map["(ex.done) -> number"].type_correct,
    TypeCorrectKind::Correct
  );
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_suggest_table_keys() {
  if !fflag::DebugLuauForceOldSolver.get() {
    return;
  }

  let mut fixture = AcFixture::default();

  fixture.base.check(
    r#"
type Test = { first: number, second: number }
local t: Test = { f@1 }
    "#,
  );

  let ac1 = fixture.base.autocomplete_marker('1');
  assert!(ac1.entry_map.contains_key("first"));
  assert!(ac1.entry_map.contains_key("second"));
  assert_eq!(ac1.context, AutocompleteContext::Property);

  fixture.base.check(
    r#"
type Test = { first: number } & { second: number }
local t: Test = { f@1 }
    "#,
  );

  let ac2 = fixture.base.autocomplete_marker('1');
  assert!(ac2.entry_map.contains_key("first"));
  assert!(ac2.entry_map.contains_key("second"));
  assert_eq!(ac2.context, AutocompleteContext::Property);

  fixture.base.check(
    r#"
type Test = { first: number, second: number } | { second: number, third: number }
local t: Test = { s@1 }
    "#,
  );

  let ac3 = fixture.base.autocomplete_marker('1');
  assert!(ac3.entry_map.contains_key("second"));
  assert!(!ac3.entry_map.contains_key("first"));
  assert!(!ac3.entry_map.contains_key("third"));
  assert_eq!(ac3.context, AutocompleteContext::Property);

  fixture.base.check(
    r#"
type Test = { first: (number) -> number, second: number }
local t: Test = { f@1 }
    "#,
  );

  let ac4 = fixture.base.autocomplete_marker('1');
  assert!(ac4.entry_map.contains_key("first"));
  assert_eq!(
    ac4.entry_map["first"].parens,
    ParenthesesRecommendation::None
  );
  assert_eq!(ac4.context, AutocompleteContext::Property);

  fixture.base.check(
    r#"
type Test = { first: number, second: number }
local t: Test = { f@1 = 2 }
    "#,
  );

  let ac5 = fixture.base.autocomplete_marker('1');
  assert!(ac5.entry_map.contains_key("first"));
  assert!(ac5.entry_map.contains_key("second"));
  assert_eq!(ac5.context, AutocompleteContext::Property);

  fixture.base.check(
    r#"
type Test = { first: number, second: number }
local t: Test = { ["f@1"] }
    "#,
  );

  let ac6 = fixture.base.autocomplete_marker('1');
  assert!(ac6.entry_map.contains_key("first"));
  assert!(ac6.entry_map.contains_key("second"));
  assert_eq!(ac6.context, AutocompleteContext::Property);

  fixture.base.check(
    r#"
type Test = { first: number, second: number }
local t: Test = { "f@1" }
    "#,
  );

  let ac7 = fixture.base.autocomplete_marker('1');
  assert!(!ac7.entry_map.contains_key("first"));
  assert!(!ac7.entry_map.contains_key("second"));
  assert_eq!(ac7.context, AutocompleteContext::String);

  fixture.base.check(
    r#"
type Test = { first: number, second: number }
local t: Test = { first = 2, s@1 }
    "#,
  );

  let ac8 = fixture.base.autocomplete_marker('1');
  assert!(!ac8.entry_map.contains_key("first"));
  assert!(ac8.entry_map.contains_key("second"));
  assert_eq!(ac8.context, AutocompleteContext::Property);

  fixture.base.check(
    r#"
type Test = { first: number, second: number }
local t: Test = { first@1 }
    "#,
  );

  let ac9 = fixture.base.autocomplete_marker('1');
  assert!(ac9.entry_map.contains_key("first"));
  assert!(ac9.entry_map.contains_key("second"));
  assert_eq!(ac9.context, AutocompleteContext::Property);

  fixture.base.check(
    r#"
local t = {
    { first = 5, second = 10 },
    { f@1 }
}
    "#,
  );

  let ac10 = fixture.base.autocomplete_marker('1');
  assert!(ac10.entry_map.contains_key("first"));
  assert!(ac10.entry_map.contains_key("second"));
  assert_eq!(ac10.context, AutocompleteContext::Property);

  fixture.base.check(
    r#"
local t = {
    [2] = { first = 5, second = 10 },
    [5] = { f@1 }
}
    "#,
  );

  let ac11 = fixture.base.autocomplete_marker('1');
  assert!(ac11.entry_map.contains_key("first"));
  assert!(ac11.entry_map.contains_key("second"));
  assert_eq!(ac11.context, AutocompleteContext::Property);
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_suggest_table_keys_no_initial_character() {
  let (_fixture, ac) = ac_check!(
    r#"
type Test = { first: number, second: number }
local t: Test = { @1 }
    "#,
    '1'
  );
  assert!(ac.entry_map.contains_key("first"));
  assert!(ac.entry_map.contains_key("second"));
  assert_eq!(ac.context, AutocompleteContext::Property);
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_suggest_table_keys_no_initial_character_2() {
  let (_fixture, ac) = ac_check!(
    r#"
type Test = { first: number, second: number }
local t: Test = { first = 1, @1 }
    "#,
    '1'
  );
  assert!(!ac.entry_map.contains_key("first"));
  assert!(ac.entry_map.contains_key("second"));
  assert_eq!(ac.context, AutocompleteContext::Property);
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_suggest_table_keys_no_initial_character_3() {
  let (_fixture, ac) = ac_check!(
    r#"
type Properties = { TextScaled: boolean, Text: string }
local function create(props: Properties) end

create({ @1 })
    "#,
    '1'
  );
  assert!(!ac.entry_map.is_empty());
  assert!(ac.entry_map.contains_key("TextScaled"));
  assert!(ac.entry_map.contains_key("Text"));
  assert_eq!(ac.context, AutocompleteContext::Property);
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_table_intersection() {
  let (_fixture, ac) = ac_check!(
    r#"
        type t1 = { a1 : string, b2 : number }
        type t2 = { b2 : number, c3 : string }
        function func(abc : t1 & t2)
            abc.  @1
        end
    "#,
    '1'
  );

  assert_eq!(3, ac.entry_map.len());
  assert!(ac.entry_map.contains_key("a1"));
  assert!(ac.entry_map.contains_key("b2"));
  assert!(ac.entry_map.contains_key("c3"));
  assert_eq!(ac.context, AutocompleteContext::Property);
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_table_union() {
  let (_fixture, ac) = ac_check!(
    r#"
        type t1 = { a1 : string, b2 : number }
        type t2 = { b2 : string, c3 : string }
        function func(abc : t1 | t2)
            abc.  @1
        end
    "#,
    '1'
  );

  assert_eq!(1, ac.entry_map.len());
  assert!(ac.entry_map.contains_key("b2"));
  assert_eq!(ac.context, AutocompleteContext::Property);
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_type_correct_argument_type_suggestion() {
  let (mut fixture, ac1) = ac_check!(
    r#"
local function target(a: number, b: string) return a + #b end

local function d(a: n@1, b)
    return target(a, b)
end
    "#,
    '1'
  );

  assert!(ac1.entry_map.contains_key("number"));
  assert_eq!(
    ac1.entry_map["number"].type_correct,
    TypeCorrectKind::Correct
  );

  fixture.base.check(
    r#"
local function target(a: number, b: string) return a + #b end

local function d(a, b: s@1)
    return target(a, b)
end
    "#,
  );

  let ac2 = fixture.base.autocomplete_marker('1');

  assert!(ac2.entry_map.contains_key("string"));
  assert_eq!(
    ac2.entry_map["string"].type_correct,
    TypeCorrectKind::Correct
  );

  fixture.base.check(
    r#"
local function target(a: number, b: string) return a + #b end

local function d(a:@1 @2, b)
    return target(a, b)
end
    "#,
  );

  let ac3 = fixture.base.autocomplete_marker('1');

  assert!(ac3.entry_map.contains_key("number"));
  assert_eq!(
    ac3.entry_map["number"].type_correct,
    TypeCorrectKind::Correct
  );

  let ac4 = fixture.base.autocomplete_marker('2');

  assert!(ac4.entry_map.contains_key("number"));
  assert_eq!(
    ac4.entry_map["number"].type_correct,
    TypeCorrectKind::Correct
  );

  fixture.base.check(
    r#"
local function target(a: number, b: string) return a + #b end

local function d(a, b: @1)@2: number
    return target(a, b)
end
    "#,
  );

  let ac5 = fixture.base.autocomplete_marker('1');

  assert!(ac5.entry_map.contains_key("string"));
  assert_eq!(
    ac5.entry_map["string"].type_correct,
    TypeCorrectKind::Correct
  );

  let ac6 = fixture.base.autocomplete_marker('2');
  let string_type_correct = ac6
    .entry_map
    .get("string")
    .map(|entry| entry.type_correct)
    .unwrap_or(TypeCorrectKind::None);

  assert_eq!(string_type_correct, TypeCorrectKind::None);
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_type_correct_expected_argument_type_pack_suggestion() {
  let (mut fixture, ac1) = ac_check!(
    r#"
local function target(callback: (...number) -> number) return callback(1, 2, 3) end

local x = target(function(...:n@1)
    return a
end
    "#,
    '1'
  );

  assert!(ac1.entry_map.contains_key("number"));
  assert_eq!(
    ac1.entry_map["number"].type_correct,
    TypeCorrectKind::Correct
  );

  fixture.base.check(
    r#"
local function target(callback: (...number) -> number) return callback(1, 2, 3) end

local x = target(function(a:number, b:number, ...:@1)
    return a + b
end
    "#,
  );

  let ac2 = fixture.base.autocomplete_marker('1');

  assert!(ac2.entry_map.contains_key("number"));
  assert_eq!(
    ac2.entry_map["number"].type_correct,
    TypeCorrectKind::Correct
  );
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_type_correct_expected_argument_type_suggestion() {
  let (mut fixture, ac1) = ac_check!(
    r#"
local function target(callback: (a: number, b: string) -> number) return callback(4, "hello") end

local x = target(function(a: @1
    "#,
    '1'
  );

  assert!(ac1.entry_map.contains_key("number"));
  assert_eq!(
    ac1.entry_map["number"].type_correct,
    TypeCorrectKind::Correct
  );

  fixture.base.check(
    r#"
local function target(callback: (a: number, b: string) -> number) return callback(4, "hello") end

local x = target(function(a: n@1
    "#,
  );

  let ac2 = fixture.base.autocomplete_marker('1');

  assert!(ac2.entry_map.contains_key("number"));
  assert_eq!(
    ac2.entry_map["number"].type_correct,
    TypeCorrectKind::Correct
  );

  fixture.base.check(
    r#"
local function target(callback: (a: number, b: string) -> number) return callback(4, "hello") end

local x = target(function(a: n@1, b: @2)
    return a + #b
end)
    "#,
  );

  let ac3 = fixture.base.autocomplete_marker('1');

  assert!(ac3.entry_map.contains_key("number"));
  assert_eq!(
    ac3.entry_map["number"].type_correct,
    TypeCorrectKind::Correct
  );

  let ac4 = fixture.base.autocomplete_marker('2');

  assert!(ac4.entry_map.contains_key("string"));
  assert_eq!(
    ac4.entry_map["string"].type_correct,
    TypeCorrectKind::Correct
  );

  fixture.base.check(
    r#"
local function target(callback: (...number) -> number) return callback(1, 2, 3) end

local x = target(function(a: n@1)
    return a
end
    "#,
  );

  let ac5 = fixture.base.autocomplete_marker('1');

  assert!(ac5.entry_map.contains_key("number"));
  assert_eq!(
    ac5.entry_map["number"].type_correct,
    TypeCorrectKind::Correct
  );
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_type_correct_expected_argument_type_suggestion_optional() {
  let mut fixture = AcFixture::default();
  fixture.base.check(        r#"
local function target(callback: nil | (a: number, b: string) -> number) return callback(4, "hello") end

local x = target(function(a: @1
    "#,
  );

  let ac = fixture.base.autocomplete_marker('1');

  assert!(ac.entry_map.contains_key("number"));
  assert_eq!(
    ac.entry_map["number"].type_correct,
    TypeCorrectKind::Correct
  );
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_type_correct_expected_argument_type_suggestion_self() {
  let (mut fixture, ac1) = ac_check!(
    r#"
local t = {}
t.x = 5
function t:target(callback: (a: number, b: string) -> number) return callback(self.x, "hello") end

local x = t:target(function(a: @1, b:@2 ) end)
local y = t.target(t, function(a: number, b: @3) end)
    "#,
    '1'
  );

  assert!(ac1.entry_map.contains_key("number"));
  assert_eq!(
    ac1.entry_map["number"].type_correct,
    TypeCorrectKind::Correct
  );

  let ac2 = fixture.base.autocomplete_marker('2');

  assert!(ac2.entry_map.contains_key("string"));
  assert_eq!(
    ac2.entry_map["string"].type_correct,
    TypeCorrectKind::Correct
  );

  let ac3 = fixture.base.autocomplete_marker('3');

  assert!(ac3.entry_map.contains_key("string"));
  assert_eq!(
    ac3.entry_map["string"].type_correct,
    TypeCorrectKind::Correct
  );
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_type_correct_expected_return_type_pack_suggestion() {
  let (mut fixture, ac1) = ac_check!(
    r#"
local function target(callback: () -> ...number) return callback() end

local x = target(function(): ...n@1
    return 1, 2, 3
end
    "#,
    '1'
  );

  assert!(ac1.entry_map.contains_key("number"));
  assert_eq!(
    ac1.entry_map["number"].type_correct,
    TypeCorrectKind::Correct
  );

  fixture.base.check(
    r#"
local function target(callback: () -> ...number) return callback() end

local x = target(function(): (number, number, ...n@1
    return 1, 2, 3
end
    "#,
  );

  let ac2 = fixture.base.autocomplete_marker('1');

  assert!(ac2.entry_map.contains_key("number"));
  assert_eq!(
    ac2.entry_map["number"].type_correct,
    TypeCorrectKind::Correct
  );
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_type_correct_expected_return_type_suggestion() {
  let (mut fixture, ac1) = ac_check!(
    r#"
local function target(callback: () -> number) return callback() end

local x = target(function(): n@1
    return 1
end
    "#,
    '1'
  );

  assert!(ac1.entry_map.contains_key("number"));
  assert_eq!(
    ac1.entry_map["number"].type_correct,
    TypeCorrectKind::Correct
  );

  fixture.base.check(
    r#"
local function target(callback: () -> (number, number)) return callback() end

local x = target(function(): (number, n@1
    return 1, 2
end
    "#,
  );

  let ac2 = fixture.base.autocomplete_marker('1');

  assert!(ac2.entry_map.contains_key("number"));
  assert_eq!(
    ac2.entry_map["number"].type_correct,
    TypeCorrectKind::Correct
  );
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_type_correct_full_type_suggestion() {
  let (mut fixture, ac1) = ac_check!(
    r#"
local b:@1 @2= "str"
    "#,
    '1'
  );

  assert!(ac1.entry_map.contains_key("string"));
  assert_eq!(
    ac1.entry_map["string"].type_correct,
    TypeCorrectKind::Correct
  );

  let ac2 = fixture.base.autocomplete_marker('2');

  assert!(ac2.entry_map.contains_key("string"));
  assert_eq!(
    ac2.entry_map["string"].type_correct,
    TypeCorrectKind::Correct
  );

  fixture.base.check(
    r#"
local b: @1= function(a: number) return -a end
    "#,
  );

  let ac3 = fixture.base.autocomplete_marker('1');

  assert!(ac3.entry_map.contains_key("(number) -> number"));
  assert_eq!(
    ac3.entry_map["(number) -> number"].type_correct,
    TypeCorrectKind::Correct
  );
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_type_correct_function_no_parenthesis() {
  let (_fixture, ac) = ac_check!(
    r#"
local function target(a: (number) -> number) return a(4) end
local function bar1(a: number) return -a end
local function bar2(a: string) return a .. 'x' end

return target(b@1
    "#,
    '1'
  );

  assert!(ac.entry_map.contains_key("bar1"));
  assert_eq!(ac.entry_map["bar1"].type_correct, TypeCorrectKind::Correct);
  assert_eq!(ac.entry_map["bar1"].parens, ParenthesesRecommendation::None);
  assert_eq!(ac.entry_map["bar2"].type_correct, TypeCorrectKind::None);
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_type_correct_function_return_types() {
  let (mut fixture, ac1) = ac_check!(
    r#"
local function target(a: number, b: string) return a + #b end
local function bar1(a: number) return -a end
local function bar2(a: string) return a .. 'x' end

return target(b@1
    "#,
    '1'
  );

  assert!(ac1.entry_map.contains_key("bar1"));
  assert_eq!(
    ac1.entry_map["bar1"].type_correct,
    TypeCorrectKind::CorrectFunctionResult
  );
  assert_eq!(ac1.entry_map["bar2"].type_correct, TypeCorrectKind::None);

  fixture.base.check(
    r#"
local function target(a: number, b: string) return a + #b end
local function bar1(a: number) return -a end
local function bar2(a: string) return a .. 'x' end

return target(bar1, b@1
    "#,
  );

  let ac2 = fixture.base.autocomplete_marker('1');

  assert!(ac2.entry_map.contains_key("bar2"));
  assert_eq!(
    ac2.entry_map["bar2"].type_correct,
    TypeCorrectKind::CorrectFunctionResult
  );
  assert_eq!(ac2.entry_map["bar1"].type_correct, TypeCorrectKind::None);

  fixture.base.check(
    r#"
local function target(a: number, b: string) return a + #b end
local function bar1(a: number): (...number) return -a, a end
local function bar2(a: string) return a .. 'x' end

return target(b@1
    "#,
  );

  let ac3 = fixture.base.autocomplete_marker('1');

  assert!(ac3.entry_map.contains_key("bar1"));
  assert_eq!(
    ac3.entry_map["bar1"].type_correct,
    TypeCorrectKind::CorrectFunctionResult
  );
  assert_eq!(ac3.entry_map["bar2"].type_correct, TypeCorrectKind::None);
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_type_correct_function_type_suggestion() {
  let (mut fixture, ac1) = ac_check!(
    r#"
local b: (n@1) -> number = function(a: number, b: string) return a + #b end
    "#,
    '1'
  );

  assert!(ac1.entry_map.contains_key("number"));
  assert_eq!(
    ac1.entry_map["number"].type_correct,
    TypeCorrectKind::Correct
  );

  fixture.base.check(
    r#"
local b: (number, s@1 = function(a: number, b: string) return a + #b end
    "#,
  );

  let ac2 = fixture.base.autocomplete_marker('1');

  assert!(ac2.entry_map.contains_key("string"));
  assert_eq!(
    ac2.entry_map["string"].type_correct,
    TypeCorrectKind::Correct
  );

  fixture.base.check(
    r#"
local b: (number, string) -> b@1 = function(a: number, b: string): boolean return a + #b == 0 end
    "#,
  );

  let ac3 = fixture.base.autocomplete_marker('1');

  assert!(ac3.entry_map.contains_key("boolean"));
  assert_eq!(
    ac3.entry_map["boolean"].type_correct,
    TypeCorrectKind::Correct
  );

  fixture.base.check(
    r#"
local b: (number, ...s@1) = function(a: number, ...: string) return a end
    "#,
  );

  let ac4 = fixture.base.autocomplete_marker('1');

  assert!(ac4.entry_map.contains_key("string"));
  assert_eq!(
    ac4.entry_map["string"].type_correct,
    TypeCorrectKind::Correct
  );

  fixture.base.check(
    r#"
local b: (number) -> ...s@1 = function(a: number): ...string return "a", "b", "c" end
    "#,
  );

  let ac5 = fixture.base.autocomplete_marker('1');

  assert!(ac5.entry_map.contains_key("string"));
  assert_eq!(
    ac5.entry_map["string"].type_correct,
    TypeCorrectKind::Correct
  );
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_type_correct_keywords() {
  let (mut fixture, ac1) = ac_check!(
    r#"
local function a(x: boolean) end
local function b(x: number?) end
local function c(x: (number) -> string) end
local function d(x: ((number) -> string)?) end
local function e(x: ((number) -> string) & ((boolean) -> number)) end

local tru = {}
local ni = false

local ac = a(t@1)
local bc = b(n@2)
local cc = c(f@3)
local dc = d(f@4)
local ec = e(f@5)
    "#,
    '1'
  );
  assert!(ac1.entry_map.contains_key("tru"));
  assert_eq!(ac1.entry_map["tru"].type_correct, TypeCorrectKind::None);
  assert_eq!(ac1.entry_map["true"].type_correct, TypeCorrectKind::Correct);
  assert_eq!(
    ac1.entry_map["false"].type_correct,
    TypeCorrectKind::Correct
  );

  let ac2 = fixture.base.autocomplete_marker('2');
  assert!(ac2.entry_map.contains_key("ni"));
  assert_eq!(ac2.entry_map["ni"].type_correct, TypeCorrectKind::None);
  assert_eq!(ac2.entry_map["nil"].type_correct, TypeCorrectKind::Correct);

  let ac3 = fixture.base.autocomplete_marker('3');
  assert!(ac3.entry_map.contains_key("false"));
  assert_eq!(ac3.entry_map["false"].type_correct, TypeCorrectKind::None);
  assert_eq!(
    ac3.entry_map["function"].type_correct,
    TypeCorrectKind::Correct
  );

  let ac4 = fixture.base.autocomplete_marker('4');
  assert_eq!(
    ac4.entry_map["function"].type_correct,
    TypeCorrectKind::Correct
  );

  let ac5 = fixture.base.autocomplete_marker('5');
  assert_eq!(
    ac5.entry_map["function"].type_correct,
    TypeCorrectKind::Correct
  );
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_type_correct_local_type_suggestion() {
  let (mut fixture, ac1) = ac_check!(
    r#"
local b: s@1 = "str"
    "#,
    '1'
  );

  assert!(ac1.entry_map.contains_key("string"));
  assert_eq!(
    ac1.entry_map["string"].type_correct,
    TypeCorrectKind::Correct
  );

  fixture.base.check(
    r#"
local function f() return "str" end
local b: s@1 = f()
    "#,
  );

  let ac2 = fixture.base.autocomplete_marker('1');

  assert!(ac2.entry_map.contains_key("string"));
  assert_eq!(
    ac2.entry_map["string"].type_correct,
    TypeCorrectKind::Correct
  );

  fixture.base.check(
    r#"
local b: s@1, c: n@2 = "str", 2
    "#,
  );

  let ac3 = fixture.base.autocomplete_marker('1');

  assert!(ac3.entry_map.contains_key("string"));
  assert_eq!(
    ac3.entry_map["string"].type_correct,
    TypeCorrectKind::Correct
  );

  let ac4 = fixture.base.autocomplete_marker('2');

  assert!(ac4.entry_map.contains_key("number"));
  assert_eq!(
    ac4.entry_map["number"].type_correct,
    TypeCorrectKind::Correct
  );

  fixture.base.check(
    r#"
local function f() return 1, "str", 3 end
local a: b@1, b: n@2, c: s@3, d: n@4 = false, f()
    "#,
  );

  let ac5 = fixture.base.autocomplete_marker('1');

  assert!(ac5.entry_map.contains_key("boolean"));
  assert_eq!(
    ac5.entry_map["boolean"].type_correct,
    TypeCorrectKind::Correct
  );

  let ac6 = fixture.base.autocomplete_marker('2');

  assert!(ac6.entry_map.contains_key("number"));
  assert_eq!(
    ac6.entry_map["number"].type_correct,
    TypeCorrectKind::Correct
  );

  let ac7 = fixture.base.autocomplete_marker('3');

  assert!(ac7.entry_map.contains_key("string"));
  assert_eq!(
    ac7.entry_map["string"].type_correct,
    TypeCorrectKind::Correct
  );

  let ac8 = fixture.base.autocomplete_marker('4');

  assert!(ac8.entry_map.contains_key("number"));
  assert_eq!(
    ac8.entry_map["number"].type_correct,
    TypeCorrectKind::Correct
  );

  fixture.base.check(
    r#"
local function f(): ...number return 1, 2, 3 end
local a: boolean, b: n@1 = false, f()
    "#,
  );

  let ac9 = fixture.base.autocomplete_marker('1');

  assert!(ac9.entry_map.contains_key("number"));
  assert_eq!(
    ac9.entry_map["number"].type_correct,
    TypeCorrectKind::Correct
  );
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_type_correct_sealed_table() {
  use ulua_analysis::{
    enums::solver_mode::SolverMode, functions::to_string_to_string::to_string_type_id,
  };
  let (mut fixture, ac) = ac_check!(
    r#"
local function f(a: { x: number, y: number }) return a.x + a.y end
local fp: @1= f
    "#,
    '1'
  );

  // The skeleton fixture currently runs the translated old solver path.
  if !fflag::DebugLuauForceOldSolver.get()
    && fixture.base.get_frontend().get_luau_solver_mode() == SolverMode::New
  {
    assert_eq!(
      "({ x: number, y: number }) -> number",
      to_string_type_id(fixture.base.base.require_type_string("f"))
    );
  } else {
    assert_eq!(
      "({ x: number, y: number }) -> (...any)",
      to_string_type_id(fixture.base.base.require_type_string("f"))
    );
  }

  assert!(
    ac.entry_map
      .contains_key("({ x: number, y: number }) -> number")
  );
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_type_correct_suggestion_for_overloads() {
  if !fflag::DebugLuauForceOldSolver.get() {
    return;
  }

  let (mut fixture, ac1) = ac_check!(
    r#"
local target: ((number) -> string) & ((string) -> number))

local one = 4
local two = "hello"
return target(o@1)
    "#,
    '1'
  );

  assert!(ac1.entry_map.contains_key("one"));
  assert_eq!(ac1.entry_map["one"].type_correct, TypeCorrectKind::Correct);
  assert_eq!(ac1.entry_map["two"].type_correct, TypeCorrectKind::Correct);

  fixture.base.check(
    r#"
local target: ((number) -> string) & ((number) -> number))

local one = 4
local two = "hello"
return target(o@1)
    "#,
  );

  let ac2 = fixture.base.autocomplete_marker('1');

  assert!(ac2.entry_map.contains_key("one"));
  assert_eq!(ac2.entry_map["one"].type_correct, TypeCorrectKind::Correct);
  assert_eq!(ac2.entry_map["two"].type_correct, TypeCorrectKind::None);

  fixture.base.check(
    r#"
local target: ((number, number) -> string) & ((string) -> number))

local one = 4
local two = "hello"
return target(1, o@1)
    "#,
  );

  let ac3 = fixture.base.autocomplete_marker('1');

  assert!(ac3.entry_map.contains_key("one"));
  assert_eq!(ac3.entry_map["one"].type_correct, TypeCorrectKind::Correct);
  assert_eq!(ac3.entry_map["two"].type_correct, TypeCorrectKind::None);
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_type_correct_suggestion_in_argument() {
  let (mut fixture, ac1) = ac_check!(
    r#"
local function target(a: number, b: string) return a + #b end

local one = 4
local two = "hello"
return target(o@1
    "#,
    '1'
  );

  assert!(ac1.entry_map.contains_key("one"));
  assert_eq!(ac1.entry_map["one"].type_correct, TypeCorrectKind::Correct);
  assert_eq!(ac1.entry_map["two"].type_correct, TypeCorrectKind::None);

  fixture.base.check(
    r#"
local function target(a: number, b: string) return a + #b end

local one = 4
local two = "hello"
return target(one, t@1
    "#,
  );

  let ac2 = fixture.base.autocomplete_marker('1');

  assert!(ac2.entry_map.contains_key("two"));
  assert_eq!(ac2.entry_map["two"].type_correct, TypeCorrectKind::Correct);
  assert_eq!(ac2.entry_map["one"].type_correct, TypeCorrectKind::None);

  fixture.base.check(
    r#"
local function target(a: number, b: string) return a + #b end

local a = { one = 4, two = "hello" }
return target(a.@1
    "#,
  );

  let ac3 = fixture.base.autocomplete_marker('1');

  assert!(ac3.entry_map.contains_key("one"));
  assert_eq!(ac3.entry_map["one"].type_correct, TypeCorrectKind::Correct);
  assert_eq!(ac3.entry_map["two"].type_correct, TypeCorrectKind::None);

  fixture.base.check(
    r#"
local function target(a: number, b: string) return a + #b end

local a = { one = 4, two = "hello" }
return target(a.one, a.@1
    "#,
  );

  let ac4 = fixture.base.autocomplete_marker('1');

  assert!(ac4.entry_map.contains_key("two"));
  assert_eq!(ac4.entry_map["two"].type_correct, TypeCorrectKind::Correct);
  assert_eq!(ac4.entry_map["one"].type_correct, TypeCorrectKind::None);

  fixture.base.check(
    r#"
local function target(a: string?) return #b end

local a = { one = 4, two = "hello" }
return target(a.@1
    "#,
  );

  let ac5 = fixture.base.autocomplete_marker('1');

  assert!(ac5.entry_map.contains_key("two"));
  assert_eq!(ac5.entry_map["two"].type_correct, TypeCorrectKind::Correct);
  assert_eq!(ac5.entry_map["one"].type_correct, TypeCorrectKind::None);
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_type_correct_suggestion_in_table() {
  let (mut fixture, ac1) = ac_check!(
    r#"
type Foo = { a: number, b: string }
local a = { one = 4, two = "hello" }
local b: Foo = { a = a.@1
    "#,
    '1'
  );

  assert!(ac1.entry_map.contains_key("one"));
  assert_eq!(ac1.entry_map["one"].type_correct, TypeCorrectKind::Correct);
  assert_eq!(ac1.entry_map["two"].type_correct, TypeCorrectKind::None);
  assert_eq!(ac1.context, AutocompleteContext::Property);

  fixture.base.check(
    r#"
type Foo = { a: number, b: string }
local a = { one = 4, two = "hello" }
local b: Foo = { b = a.@1
    "#,
  );

  let ac2 = fixture.base.autocomplete_marker('1');

  assert!(ac2.entry_map.contains_key("two"));
  assert_eq!(ac2.entry_map["two"].type_correct, TypeCorrectKind::Correct);
  assert_eq!(ac2.entry_map["one"].type_correct, TypeCorrectKind::None);
  assert_eq!(ac2.context, AutocompleteContext::Property);
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_type_function_eval_in_autocomplete() {
  let _new_solver = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);

  let (_fixture, ac) = acb_check!(
    r#"
type function foo(x)
    local tbl = types.newtable(nil, nil, nil)
    tbl:setproperty(types.singleton("boolean"), x)
    tbl:setproperty(types.singleton("number"), types.number)
    return tbl
end

local function test(a: foo<string>)
    return a.@1
end
    "#,
    '1'
  );
  assert!(ac.entry_map.contains_key("boolean"));
  assert!(ac.entry_map.contains_key("number"));
}

// Source: `tests/Autocomplete.test.cpp:4894`（type_function_string_singleton_union）
#[test]
fn autocomplete_type_function_string_singleton_union() {
  // Type functions are only handled in the new solver（cpp 同款 flag 前置）
  let _new_solver = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);

  let (_fixture, ac) = acb_check!(
    r#"
type function test(ty: type)
    return types.unionof(types.singleton("test"), types.singleton("test2"))
end

local a: test<number> = "@1"
"#,
    '1'
  );
  assert_eq!(ac.context, AutocompleteContext::String);
  assert!(ac.entry_map.contains_key("test"));
  assert!(ac.entry_map.contains_key("test2"));
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_type_function_has_types_definitions() {
  let _new_solver = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);

  let (_fixture, ac) = acb_check!(
    r#"
type function foo()
    types.@1
end
    "#,
    '1'
  );
  assert!(ac.entry_map.contains_key("singleton"));
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_type_function_private_scope() {
  use ulua_analysis::{
    functions::add_global_binding_builtin_definitions::add_global_binding_value,
    records::binding::Binding,
  };
  use ulua_ast::records::location::Location;
  let _new_solver = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);

  let mut fixture = ACBuiltinsFixture::default();
  // (a) 类 `as *mut Frontend` + unsafe 解引用绕道已消除：直借
  // `&mut Frontend`，binding 值就地构造后成对登记（cpp 同款顺序变更）。
  {
    let frontend = fixture.base.get_frontend();
    let any_type = frontend.builtin_types_ref().any_type;
    let binding = || Binding {
      type_id: any_type,
      location: Location::default(),
      deprecated: false,
      deprecated_suggestion: String::new(),
      documentation_symbol: None,
    };

    add_global_binding_value(&mut frontend.globals, "thisAlsoShouldNotBeThere", binding());
    add_global_binding_value(
      &mut frontend.globals_for_autocomplete,
      "thisAlsoShouldNotBeThere",
      binding(),
    );
  }

  fixture.base.check(
    r#"
local function thisShouldNotBeThere() end

type function thisShouldBeThere() end

type function foo()
    this@1
end

this@2
    "#,
  );

  let ac = fixture.base.autocomplete_marker('1');
  assert!(!ac.entry_map.contains_key("thisShouldNotBeThere"));
  assert!(!ac.entry_map.contains_key("thisAlsoShouldNotBeThere"));
  assert!(ac.entry_map.contains_key("thisShouldBeThere"));

  let ac = fixture.base.autocomplete_marker('2');
  assert!(ac.entry_map.contains_key("thisShouldNotBeThere"));
  assert!(ac.entry_map.contains_key("thisAlsoShouldNotBeThere"));
  assert!(!ac.entry_map.contains_key("thisShouldBeThere"));
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_type_scoping_easy() {
  use ulua_analysis::{
    functions::{follow_type, get_type},
    records::table_type::TableType,
  };
  let (_fixture, ac) = ac_check!(
    r#"
type Table = { a: number, b: number }
do
    type Table = { x: string, y: string }
    local a: T@1
end
    "#
  );

  assert!(ac.entry_map.contains_key("Table"));
  let ty = ac.entry_map["Table"]
    .r#type
    .expect("Table entry should have a type");
  let ty = follow_type::follow(ty);
  let table = get_type::get::<TableType>(ty).expect("Table should be a table");
  assert!(table.props.contains_key("x"));
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_unsealed_table() {
  let (_fixture, ac) = ac_check!(
    r#"
        local tbl = {}
        tbl.prop = 5
        tbl.@1
    "#,
    '1'
  );

  assert_eq!(1, ac.entry_map.len());
  assert!(ac.entry_map.contains_key("prop"));
  assert_eq!(ac.context, AutocompleteContext::Property);
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_unsealed_table_2() {
  let (_fixture, ac) = ac_check!(
    r#"
        local tbl = {}
        local inner = { prop = 5 }
        tbl.inner = inner
        tbl.inner. @1
    "#,
    '1'
  );

  assert_eq!(1, ac.entry_map.len());
  assert!(ac.entry_map.contains_key("prop"));
  assert_eq!(ac.context, AutocompleteContext::Property);
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_user_defined_globals() {
  let mut fixture = AcFixture::default();
  fixture.base.check("local myLocal = 4; @1");

  let ac = fixture.base.autocomplete_marker('1');

  assert!(ac.entry_map.contains_key("myLocal"));
  assert!(ac.entry_map.contains_key("table"));
  assert!(ac.entry_map.contains_key("math"));
  assert_eq!(ac.context, AutocompleteContext::Statement);
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_user_defined_local_functions_in_own_definition() {
  let (mut fixture, ac) = ac_check!(
    r#"
        local function abc()
@1
        end
    "#,
    '1'
  );

  assert!(ac.entry_map.contains_key("abc"));
  assert!(ac.entry_map.contains_key("table"));
  assert!(ac.entry_map.contains_key("math"));

  fixture.base.check(
    r#"
        local abc = function()
@1
        end
    "#,
  );

  let ac = fixture.base.autocomplete_marker('1');

  assert!(ac.entry_map.contains_key("abc"));
  assert!(ac.entry_map.contains_key("table"));
  assert!(ac.entry_map.contains_key("math"));
}

// Source: `tests/Autocomplete.test.cpp`
#[test]
fn autocomplete_we_know_the_fields_of_a_class_instance() {
  let _new_solver = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);
  let _classes = ScopedFastFlag::new(&fflag::DebugLuauUserDefinedClasses, true);

  let (_fixture, ac) = ac_check!(
    r#"
        class Point2d
            public x: number
            public y: number
        end

        local p = Point2d.new { x=3, y=4 }

        local q = p.@1
    "#,
    '1'
  );
  assert!(ac.entry_map.contains_key("x"));
  assert!(ac.entry_map.contains_key("y"));
  assert!(!ac.entry_map.contains_key("z"));
}

// Source: `tests/Autocomplete.test.cpp:5844`（AutocompleteOnNonexistentTable）
//
// `typeof(mygame.interesting)` 里 `mygame` 无 `interesting` 成员，交集的另一侧
// （显式 `Humanoid.Animator` 表类型）字段必须仍在补全面前保留——即与
// error/unknown 类型的交集不吞掉已知字段。
#[test]
fn autocomplete_on_nonexistent_table() {
  let (_fixture, ac) = ac_check!(
    r#"
        local mygame = {}

        local char = (nil :: any) :: {
            Humanoid: {
                Animator: number
            }
        } & typeof(mygame.interesting)

        char.Humanoid.@1
    "#,
    '1'
  );
  assert!(
    ac.entry_map.contains_key("Animator"),
    "补全键: {:?}",
    ac.entry_map.keys().collect::<Vec<_>>()
  );
}

// 缺口（未移植）：`tests/Autocomplete.test.cpp:5671` 的
// `autocomplete_props_through_metatable_typed_metatable`（无 flag 依赖）——补全须
// 穿透 `setmetatable` 值元表链（`obj -> Meta -> Base`）看到 `baseProp`；本移植
// 实测 `entry_map` 为空，属 autocomplete 实现缺口而非测试侧不可表达，待实现
// 补齐后应补回该用例。
//
// 缺口（tst-r06 对照新增留证）：
// - `anonymous_autofilled_cursor_in_arglist_{empty,args,with_return,named_args,varargs}`
//   （:4664-4740，无 flag 依赖）——光标已在 `function(` 实参表内时只应补全参数表
//   （cpp `AutocompleteCore.cpp` 的 `makeAnonymousArgList` 分支：`AstExprFunction`
//   的 argLocation 存在时 `insertText` 只给参数表，如 `"a0: number, a1: string"`）；
//   本移植 `make_anonymous_autofilled` 无该分支、恒产全量 `function(...)  end` 文本，
//   属实现缺口，待补齐后应补回 5 例（`cursor_after_function_keyword` 一例无
//   argLocation、走全量路径，已于本轮先行补入）。
// - `autocomplete_{,not_}deprecated_on_*` 家族 9 例（:5329-5471 另含 :5884
//   `autocomplete_deprecated_on_recursive_intersection`）——依赖 FFlag
//   `LuauCheckTypeForDeprecated`（未同步），faithful 断言不可表达。
// - `colon_no_conversion_marker` / `dot_function_no_conversion` /
//   `dot_method_marks_for_conversion` / `extern_type_method_via_dot` /
//   `extern_type_first_arg_match_does_not_make_colon_compatible` /
//   `intersection_with_some_self_overloads`（:3638-3763）——依赖 FFlag
//   `LuauAutocompleteDotMethodConversion`（未同步）。
// - `if_local_*` / `elseif_local_*` / `if_const_*` 家族 16 例（:5905-6142）——依赖
//   `DebugLuauIfLocalSyntax` / `DebugLuauIfLocalAnalysis`（`if local`/`if const`
//   语法 parser 未接入，与 linter.rs 台账同源），用例源码无法解析。
// - `type_correct_suggestion_with_explicit_type_args_on_method_call`（:5862）——
//   依赖 FFlag `LuauUseExplicitTypeArgsInGenerics`（未同步）。
