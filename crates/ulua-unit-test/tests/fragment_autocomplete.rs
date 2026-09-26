extern crate alloc;

// 原 190 个测试 mod 各自 fn 体内重复声明的导入统一上提至此：
// 各 mod 末尾以 `use super::*;` 继承，消除 ~440 处逐 fn 重复 use。
use alloc::string::String;

// 新增上提：>=3 个 test fn 重复且与既有顶层导入零碰撞的名统一收口至文件顶（借 tst-r16/tst-r17 上提先例）；
// 另有与顶层同路径的 fn 内纯重复导入一并删除（FragmentAutocompleteStatusResult/Fixture/LUAU_ASSERT/NodePtr/BuiltinsFixture）。
use ulua_analysis::enums::autocomplete_context::AutocompleteContext;
use ulua_analysis::{
  enums::{
    autocomplete_entry_kind::AutocompleteEntryKind,
    fragment_autocomplete_status::FragmentAutocompleteStatus, solver_mode::SolverMode,
  },
  functions::{block_diff_start::block_diff_start, to_string_to_string::to_string_type_id},
  records::{
    fragment_autocomplete_status_result::FragmentAutocompleteStatusResult,
    source_module::SourceModule,
  },
  type_aliases::module_name_type::ModuleName,
};
use ulua_ast::{
  records::{
    ast_node::AstNode, ast_stat::AstStat, ast_stat_block::AstStatBlock, ast_stat_expr::AstStatExpr,
    ast_stat_for_in::AstStatForIn, ast_stat_function::AstStatFunction, ast_stat_if::AstStatIf,
    ast_stat_local::AstStatLocal, ast_stat_local_function::AstStatLocalFunction,
    location::Location, position::Position,
  },
  rtti::AstNodeClass,
};
use ulua_common::{fflag, fint, macros::luau_assert::LUAU_ASSERT};
use ulua_unit_test::{
  functions::{
    ast_node_ref::{NodePtr, PtrRef},
    linear_search_for_binding::linear_search_for_binding,
  },
  records::{
    fragment_autocomplete_builtins_fixture::FragmentAutocompleteBuiltinsFixture,
    fragment_autocomplete_fixture::FragmentAutocompleteFixture,
  },
  type_aliases::{scoped_fast_flag::ScopedFastFlag, scoped_fast_int::ScopedFastInt},
};

/// cpp 夹具 `dynamic_cast<T*>(nearestStatement) != nullptr` 惯用断言的收口：
/// `nearestStatement` 恒为 `*mut AstStat`（夹具 arena 存活基类指针），判型/下转
/// 走安全门面 `NodePtr::as_node`（null 与类型不符一并折叠为 `None`），调用点零
/// `unsafe`。语义与 cpp `node->as<T>() != nullptr` 逐条对应。
#[inline]
fn is_nearest<T: AstNodeClass>(node: *mut AstStat) -> bool {
  node.as_node::<T>().is_some()
}

/// cpp 夹具 `dynamic_cast<T*>(node) != nullptr` 之于 `*mut AstNode`（ancestry 元素）：
/// 同 [`is_nearest`]，安全门面 `as_node` 收口判型与判空，调用点零 `unsafe`。
#[inline]
fn is_node<T: AstNodeClass>(node: *mut AstNode) -> bool {
  node.as_node::<T>().is_some()
}

// 样板收口助手：原逐例重复的 FragmentAutocompleteFixture 构造 + get_autocomplete_region(
// String::from(src), &Position{..}) 恒等前件收口为宏，行为与原语句逐字一致（借 tst-r16/tst-r17 助手宏先例）。
macro_rules! fx_region {
  ($src:expr, $pos:expr) => {{
    let mut fixture = FragmentAutocompleteFixture::default();
    let region = fixture
      .base
      .get_autocomplete_region(String::from($src), &$pos);
    (fixture, region)
  }};
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_after_func_new_line() {
  let (_fixture, region) = fx_region!(
    r#"
function f()
end

"#,
    Position { line: 3, column: 0 }
  );

  assert_eq!(
    Location {
      begin: Position { line: 3, column: 0 },
      end: Position { line: 3, column: 0 },
    },
    region.fragment_location
  );
  assert!(!region.parent_block.is_null());
  assert!(crate::is_nearest::<AstStatFunction>(
    region.nearest_statement
  ));
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_after_func_same_line() {
  let (_fixture, region) = fx_region!(
    r#"
function f()
end
"#,
    Position { line: 2, column: 3 }
  );

  assert_eq!(
    Location {
      begin: Position { line: 2, column: 3 },
      end: Position { line: 2, column: 3 },
    },
    region.fragment_location
  );
  assert!(!region.parent_block.is_null());
  assert!(crate::is_nearest::<AstStatFunction>(
    region.nearest_statement
  ));
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_after_local_func_new_line() {
  let (_fixture, region) = fx_region!(
    r#"
local function f()
end

"#,
    Position { line: 3, column: 0 }
  );

  assert_eq!(
    Location {
      begin: Position { line: 3, column: 0 },
      end: Position { line: 3, column: 0 },
    },
    region.fragment_location
  );
  assert!(!region.parent_block.is_null());
  assert!(crate::is_nearest::<AstStatLocalFunction>(
    region.nearest_statement
  ));
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_after_local_func_same_line() {
  let (_fixture, region) = fx_region!(
    r#"
local function f()
end
"#,
    Position { line: 2, column: 3 }
  );

  assert_eq!(
    Location {
      begin: Position { line: 2, column: 3 },
      end: Position { line: 2, column: 3 },
    },
    region.fragment_location
  );
  assert!(!region.parent_block.is_null());
  assert!(crate::is_nearest::<AstStatLocalFunction>(
    region.nearest_statement
  ));
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_anonymous_autofilled_generic_named_arg() {
  use ulua_analysis::enums::type_correct_kind::TypeCorrectKind;

  // C++ `kGeneratedAnonymousFunctionEntryName` (AutocompleteTypes.h:92).
  const K_GENERATED_ANONYMOUS_FUNCTION_ENTRY_NAME: &str = "function (anonymous autofilled)";

  let source = String::from(
    "
local function foo<A>(f: (a: A) -> number, a: A)
\treturn f(a)
end
    ",
  );

  let dest = String::from(
    "
local function foo<A>(f: (a: A) -> number, a: A)
\treturn f(a)
end

foo(@1)
    ",
  );

  let mut fixture = FragmentAutocompleteFixture::default();
  fixture.base.autocomplete_fragment_in_both_solvers(
    &source,
    &dest,
    '1',
    |frag: &mut FragmentAutocompleteStatusResult| {
      let expected_insert = "function(a): number  end";
      LUAU_ASSERT!(frag.result.is_some());
      let ac_results = &frag.result.as_ref().unwrap().ac_results;
      LUAU_ASSERT!(
        ac_results
          .entry_map
          .contains_key(K_GENERATED_ANONYMOUS_FUNCTION_ENTRY_NAME)
      );
      let entry = &ac_results.entry_map[K_GENERATED_ANONYMOUS_FUNCTION_ENTRY_NAME];
      assert!(entry.kind == AutocompleteEntryKind::GeneratedFunction);
      assert!(entry.type_correct == TypeCorrectKind::Correct);
      LUAU_ASSERT!(entry.insert_text.is_some());
      assert_eq!(
        expected_insert,
        entry.insert_text.as_ref().unwrap().as_str()
      );
    },
    None,
  );
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_anonymous_autofilled_generic_return_type() {
  use ulua_analysis::enums::type_correct_kind::TypeCorrectKind;

  // C++ `kGeneratedAnonymousFunctionEntryName` (AutocompleteTypes.h:92).
  const K_GENERATED_ANONYMOUS_FUNCTION_ENTRY_NAME: &str = "function (anonymous autofilled)";

  let source = String::from(
    "
local function foo<A>(f: () -> A)
\treturn f()
end
    ",
  );

  let dest = String::from(
    "
local function foo<A>(f: () -> A)
\treturn f()
end

foo(@1)
    ",
  );

  let mut fixture = FragmentAutocompleteFixture::default();
  fixture.base.autocomplete_fragment_in_both_solvers(
    &source,
    &dest,
    '1',
    |frag: &mut FragmentAutocompleteStatusResult| {
      let expected_insert = "function()  end";
      assert!(frag.result.is_some());
      let ac = &frag.result.as_ref().unwrap().ac_results;
      assert_eq!(
        ac.entry_map
          .contains_key(K_GENERATED_ANONYMOUS_FUNCTION_ENTRY_NAME) as usize,
        1
      );
      let entry = &ac.entry_map[K_GENERATED_ANONYMOUS_FUNCTION_ENTRY_NAME];
      assert!(entry.kind == AutocompleteEntryKind::GeneratedFunction);
      assert!(entry.type_correct == TypeCorrectKind::Correct);
      assert!(entry.insert_text.is_some());
      assert_eq!(
        expected_insert,
        entry.insert_text.as_ref().unwrap().as_str()
      );
    },
    None,
  );
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_anonymous_autofilled_generic_type_pack_vararg() {
  use ulua_analysis::enums::type_correct_kind::TypeCorrectKind;

  // C++ `kGeneratedAnonymousFunctionEntryName` (AutocompleteTypes.h:92).
  const K_GENERATED_ANONYMOUS_FUNCTION_ENTRY_NAME: &str = "function (anonymous autofilled)";

  let source = String::from(
    r#"
local function foo<A>(a: (...A) -> number, ...: A)
	return a(...)
end
    "#,
  );

  let dest = String::from(
    r#"
local function foo<A>(a: (...A) -> number, ...: A)
	return a(...)
end

foo(@1)
    "#,
  );

  let mut fixture = FragmentAutocompleteFixture::default();
  fixture.base.autocomplete_fragment_in_both_solvers(
    &source,
    &dest,
    '1',
    |frag: &mut FragmentAutocompleteStatusResult| {
      let expected_insert = "function(...): number  end";
      assert!(frag.result.is_some());
      let ac = &frag.result.as_ref().unwrap().ac_results;
      assert!(
        ac.entry_map
          .contains_key(K_GENERATED_ANONYMOUS_FUNCTION_ENTRY_NAME)
      );
      let entry = &ac.entry_map[K_GENERATED_ANONYMOUS_FUNCTION_ENTRY_NAME];
      assert!(entry.kind == AutocompleteEntryKind::GeneratedFunction);
      assert!(entry.type_correct == TypeCorrectKind::Correct);
      assert!(entry.insert_text.is_some());
      assert_eq!(
        expected_insert,
        entry.insert_text.as_ref().unwrap().as_str()
      );
    },
    None,
  );
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_bad_range_1() {
  let source = String::from(
    r#"
local t = 1
"#,
  );
  let updated = String::from(
    r#"
t
@1"#,
  );

  let mut fixture = FragmentAutocompleteBuiltinsFixture::default();
  fixture.base.autocomplete_fragment_in_both_solvers(
    &source,
    &updated,
    '1',
    |frag: &mut FragmentAutocompleteStatusResult| {
      LUAU_ASSERT!(frag.result.is_some());
      // Safety: 成功结果的 fresh_scope 由夹具保有的 Scope 句柄给出（非空），
      // 夹具作用域覆盖本行；下游只读遍历绑定链。
      let result = frag.result.as_ref().unwrap();
      assert!(!result.fresh_scope.is_null());
      let opt = linear_search_for_binding(result.fresh_scope.as_ref_opt().unwrap(), "t");
      LUAU_ASSERT!(opt.is_some());
      assert_eq!("number", to_string_type_id(opt.unwrap()));
    },
    None,
  );
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_bad_range_2() {
  let source = String::from(
    r#"
local t = 1
"#,
  );
  let updated = String::from(
    r#"
local t = 1
t@1
"#,
  );

  let mut fixture = FragmentAutocompleteBuiltinsFixture::default();
  fixture.base.autocomplete_fragment_in_both_solvers(
    &source,
    &updated,
    '1',
    |frag: &mut FragmentAutocompleteStatusResult| {
      LUAU_ASSERT!(frag.result.is_some());
      // Safety: 成功结果的 fresh_scope 由夹具保有的 Scope 句柄给出（非空），
      // 夹具作用域覆盖本行；下游只读遍历绑定链。
      let result = frag.result.as_ref().unwrap();
      assert!(!result.fresh_scope.is_null());
      let opt = linear_search_for_binding(result.fresh_scope.as_ref_opt().unwrap(), "t");
      LUAU_ASSERT!(opt.is_some());
      assert_eq!("number", to_string_type_id(opt.unwrap()));
    },
    None,
  );
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_bad_range_3() {
  // This test makes less sense since we don't have an updated check that
  // includes l
  // instead this will recommend nothing useful because `local t` hasn't
  // been typechecked in the fresh module
  let source = String::from(
    r#"
l
"#,
  );
  let updated = String::from(
    r#"
local t = 1
l@1
"#,
  );

  let mut fixture = FragmentAutocompleteBuiltinsFixture::default();
  fixture.base.autocomplete_fragment_in_both_solvers(
    &source,
    &updated,
    '1',
    |frag: &mut FragmentAutocompleteStatusResult| {
      assert!(frag.status == FragmentAutocompleteStatus::Success);
      LUAU_ASSERT!(frag.result.is_some());
    },
    None,
  );
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_before_func() {
  let (_fixture, region) = fx_region!(
    r#"
function f()
end
"#,
    Position { line: 1, column: 0 }
  );

  assert_eq!(
    Location {
      begin: Position { line: 1, column: 0 },
      end: Position { line: 1, column: 0 },
    },
    region.fragment_location
  );
  assert!(!region.parent_block.is_null());
  assert!(crate::is_nearest::<AstStatFunction>(
    region.nearest_statement
  ));
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_before_local_func() {
  let (_fixture, region) = fx_region!(
    r#"
local function f()
end
"#,
    Position { line: 1, column: 0 }
  );

  assert_eq!(
    Location {
      begin: Position { line: 1, column: 0 },
      end: Position { line: 1, column: 0 },
    },
    region.fragment_location
  );
  assert!(!region.parent_block.is_null());
  assert!(crate::is_nearest::<AstStatLocalFunction>(
    region.nearest_statement
  ));
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_bidirectionally_inferred_table_member() {
  let source = String::from(
    r#"
type Foo = { foo1: string, bar1: number }
type Bar = { foo2: boolean, bar2: string }
type Baz = { foo3: number, bar3: boolean }

local X: Foo & Bar & Baz = {}
"#,
  );

  let dest = String::from(
    r#"
type Foo = { foo1: string, bar1: number }
type Bar = { foo2: boolean, bar2: string }
type Baz = { foo3: number, bar3: boolean }

local X: Foo & Bar & Baz = { f@1 }

"#,
  );

  let mut fixture = FragmentAutocompleteFixture::default();
  fixture.base.autocomplete_fragment_in_both_solvers(
    &source,
    &dest,
    '1',
    |result: &mut FragmentAutocompleteStatusResult| {
      LUAU_ASSERT!(result.result.is_some());
      let ac = &result.result.as_ref().unwrap().ac_results;
      assert!(!ac.entry_map.is_empty());
      assert!(ac.entry_map.contains_key("foo1"));
      assert!(ac.entry_map.contains_key("foo2"));
      assert!(ac.entry_map.contains_key("foo3"));
    },
    None,
  );
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_block_diff_added_locals_1() {
  let mut fixture = FragmentAutocompleteBuiltinsFixture::default();
  let mut stale = SourceModule::new();
  let mut fresh = SourceModule::new();
  let old = fixture.base.parse_helper_(&mut stale, String::from(""));
  let new = fixture.base.parse_helper_(
    &mut fresh,
    String::from(
      r#"local x = 4
local y = 3
local z = 3"#,
    ),
  );

  // Safety: old/new.root 为夹具刚解析出的 arena 根块，存活非空；
  // block_diff_start 仅只读遍历两棵树。
  let pos =
    unsafe { block_diff_start(old.root, new.root, (*new.root).body.as_slice()[2].as_ptr()) };
  assert_eq!(Some(Position { line: 0, column: 0 }), pos);
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_block_diff_added_locals_1_e_2_e() {
  let source = String::from(r#""#);
  let dest = String::from(
    r#"local f1 = 4
local f2 = "a"
local f3 = f@1
"#,
  );

  let mut fixture = FragmentAutocompleteBuiltinsFixture::default();
  fixture.base.autocomplete_fragment_in_both_solvers(
    &source,
    &dest,
    '1',
    |result: &mut FragmentAutocompleteStatusResult| {
      assert!(FragmentAutocompleteStatus::Success == result.status);
      assert!(result.result.is_some());
      let ac = &result.result.as_ref().unwrap().ac_results;
      assert!(!ac.entry_map.is_empty());
      assert!(ac.entry_map.contains_key("f1"));
      assert!(ac.entry_map.contains_key("f2"));
    },
    None,
  );
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_block_diff_added_locals_1_e_2_e_in_the_middle() {
  let source = String::from(r#""#);
  let dest = String::from(
    r#"local f1 = 4
local f2 = f@1
local f3 = f
"#,
  );

  let mut fixture = FragmentAutocompleteBuiltinsFixture::default();
  fixture.base.autocomplete_fragment_in_both_solvers(
    &source,
    &dest,
    '1',
    |result: &mut FragmentAutocompleteStatusResult| {
      assert_eq!(FragmentAutocompleteStatus::Success, result.status);
      LUAU_ASSERT!(result.result.is_some());
      let ac = &result.result.as_ref().unwrap().ac_results;
      assert!(!ac.entry_map.is_empty());
      assert!(ac.entry_map.contains_key("f1"));
      assert!(!ac.entry_map.contains_key("f3"));
    },
    None,
  );
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_block_diff_added_locals_2() {
  let mut fixture = FragmentAutocompleteBuiltinsFixture::default();
  let mut stale = SourceModule::new();
  let mut fresh = SourceModule::new();
  let old = fixture
    .base
    .parse_helper_(&mut stale, String::from("local x = 4"));
  let new = fixture.base.parse_helper_(
    &mut fresh,
    String::from(
      r#"local x = 4
local y = 3
local z = 3"#,
    ),
  );

  // Safety: old/new.root 为夹具刚解析出的 arena 根块，存活非空；
  // block_diff_start 仅只读遍历两棵树。
  let pos =
    unsafe { block_diff_start(old.root, new.root, (*new.root).body.as_slice()[1].as_ptr()) };
  assert_eq!(Some(Position { line: 1, column: 0 }), pos);
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_block_diff_added_locals_3() {
  let mut fixture = FragmentAutocompleteBuiltinsFixture::default();
  let mut stale = SourceModule::new();
  let mut fresh = SourceModule::new();
  let old = fixture.base.parse_helper_(
    &mut stale,
    String::from(
      r#"local x = 4
local y = 2 + 1"#,
    ),
  );
  let new = fixture.base.parse_helper_(
    &mut fresh,
    String::from(
      r#"local x = 4
local y = 3
local z = 3
local foo = 8"#,
    ),
  );

  // Safety: old/new.root 为夹具刚解析出的 arena 根块，存活非空；
  // block_diff_start 仅只读遍历两棵树。
  let pos =
    unsafe { block_diff_start(old.root, new.root, (*new.root).body.as_slice()[3].as_ptr()) };
  assert_eq!(Some(Position { line: 1, column: 0 }), pos);
}

mod fragment_autocomplete_block_diff_added_locals_3_fragment_autocomplete_test_case_2 {
  //! Source: `tests/FragmentAutocomplete.test.cpp`
  use super::*;

  #[test]
  fn fragment_autocomplete_block_diff_added_locals_3() {
    let source = String::from(
      r#"local f1 = 4
local f2 = 2 + 1"#,
    );
    let dest = String::from(
      r#"local f1 = 4
local f2 = 3
local f3 = 3
local foo = 8 + @1"#,
    );

    let mut fixture = FragmentAutocompleteBuiltinsFixture::default();
    fixture.base.autocomplete_fragment_in_both_solvers(
      &source,
      &dest,
      '1',
      |result: &mut FragmentAutocompleteStatusResult| {
        assert_eq!(FragmentAutocompleteStatus::Success, result.status);
        LUAU_ASSERT!(result.result.is_some());
        let ac_results = &result.result.as_ref().unwrap().ac_results;
        assert!(!ac_results.entry_map.is_empty());
        assert!(ac_results.entry_map.contains_key("f1"));
        assert!(ac_results.entry_map.contains_key("f2"));
        assert!(ac_results.entry_map.contains_key("f3"));
      },
      None,
    );
  }
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_block_diff_added_locals_fake_similarity() {
  let mut fixture = FragmentAutocompleteBuiltinsFixture::default();
  let mut stale = SourceModule::new();
  let mut fresh = SourceModule::new();
  let old = fixture.base.parse_helper_(
    &mut stale,
    String::from(
      r#"local x = 4
local y = true
local z = 2 + 1"#,
    ),
  );
  let new = fixture.base.parse_helper_(
    &mut fresh,
    String::from(
      r#"local x = 4
local y = "tr"
local z = 3
local foo = 8"#,
    ),
  );

  // Safety: old/new.root 为夹具刚解析出的 arena 根块，存活非空；
  // block_diff_start 仅只读遍历两棵树。
  let pos =
    unsafe { block_diff_start(old.root, new.root, (*new.root).body.as_slice()[2].as_ptr()) };
  assert_eq!(Some(Position { line: 2, column: 0 }), pos);
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_block_diff_test_both_empty() {
  use core::ptr::null_mut;

  let mut fixture = FragmentAutocompleteBuiltinsFixture::default();
  let mut stale = SourceModule::new();
  let mut fresh = SourceModule::new();
  let old = fixture.base.parse_helper_(&mut stale, String::from(""));
  let new = fixture.base.parse_helper_(&mut fresh, String::from(""));

  // 故意传 null：两个块皆空时无 nearest statement，null 是 API 的哨兵输入
  // （cpp `blockDiffStart(old, new, nullptr)` 原样），nearest 仅按地址比较、不解引用。
  let pos = block_diff_start(old.root, new.root, null_mut());
  assert!(pos.is_none());
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_block_diff_test_both_empty_e_2_e() {
  let source = String::from(r#"@1"#);

  let mut fixture = FragmentAutocompleteBuiltinsFixture::default();
  fixture.base.autocomplete_fragment_in_both_solvers(
    &source,
    &source,
    '1',
    |result: &mut FragmentAutocompleteStatusResult| {
      assert_eq!(FragmentAutocompleteStatus::Success, result.status);
    },
    None,
  );
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_can_autocomplete_nested_property_access() {
  let source = String::from(
    r#"
local tbl = { abc = { def = 1234, egh = false } }
"#,
  );
  let updated = String::from(
    r#"
local tbl = { abc = { def = 1234, egh = false } }
tbl.abc.@1
"#,
  );

  let mut fixture = FragmentAutocompleteFixture::default();
  fixture.base.autocomplete_fragment_in_both_solvers(
    &source,
    &updated,
    '1',
    |fragment: &mut FragmentAutocompleteStatusResult| {
      LUAU_ASSERT!(fragment.result.is_some());
      LUAU_ASSERT!(!fragment.result.as_ref().unwrap().fresh_scope.is_null());

      assert_eq!(
        2,
        fragment.result.as_ref().unwrap().ac_results.entry_map.len()
      );
      assert!(
        fragment
          .result
          .as_ref()
          .unwrap()
          .ac_results
          .entry_map
          .contains_key("def")
      );
      assert!(
        fragment
          .result
          .as_ref()
          .unwrap()
          .ac_results
          .entry_map
          .contains_key("egh")
      );
      assert_eq!(
        fragment.result.as_ref().unwrap().ac_results.context,
        AutocompleteContext::Property
      );
    },
    None,
  );
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_can_autocomplete_simple_property_access() {
  let source = String::from(
    r#"
local tbl = { abc = 1234}
"#,
  );
  let updated = String::from(
    r#"
local tbl = { abc = 1234}
tbl. @1
"#,
  );

  let mut fixture = FragmentAutocompleteFixture::default();
  fixture.base.autocomplete_fragment_in_both_solvers(
    &source,
    &updated,
    '1',
    |fragment: &mut FragmentAutocompleteStatusResult| {
      LUAU_ASSERT!(fragment.result.is_some());
      let ac_results = &fragment.result.as_ref().unwrap().ac_results;

      assert_eq!(1, ac_results.entry_map.len());
      assert!(ac_results.entry_map.contains_key("abc"));
      assert_eq!(AutocompleteContext::Property, ac_results.context);
    },
    None,
  );
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_can_parse_complete_fragments() {
  use ulua_ast::records::{
    ast_expr_binary::{AstExprBinary, AstExprBinaryOp},
    ast_expr_local::AstExprLocal,
  };
  let _sff = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);
  let mut fixture = FragmentAutocompleteFixture::default();
  let result = fixture.base.check_with_options(
    r#"
local x = 4
local y = 5
"#,
  );
  assert_eq!(0, result.errors.len(), "{:?}", result.errors);

  let fragment = fixture
    .base
    .parse_fragment(
      r#"
local x = 4
local y = 5
local z = x + y
"#,
      &Position {
        line: 3,
        column: 15,
      },
      None,
    )
    .expect("expected fragment parse result");

  assert_eq!("local z = x + y", fragment.fragment_to_parse);
  assert_eq!(4, fragment.ancestry.len());
  assert!(!fragment.root.is_null());
  // Safety: fragment.root 已判空，指向夹具 arena 存活根块，仅只读。
  let root = fragment.root.as_ref_opt().unwrap();
  assert_eq!(
    Location {
      begin: Position { line: 3, column: 0 },
      end: Position {
        line: 3,
        column: 15
      },
    },
    root.base.base.location
  );
  assert_eq!(1, root.body.len());

  // 门面一步「上转+判型+判空+物化引用」：原 `ast_node_as + is_null 断言 + &*` 三步样板收敛。
  // Safety: body/values/左右操作数指针均为夹具 arena 存活节点，只读至用例结束。
  let stat = (root.body.as_slice()[0])
    .as_node::<AstStatLocal>()
    .expect("首条语句应为 local 声明");
  assert_eq!(1, stat.vars.size);
  assert_eq!(1, stat.values.size);
  assert_eq!(
    "z",
    // Binding 指针槽读法走夹具门面 PtrRef（null → None），零 unsafe。
    PtrRef::as_ref_opt(&stat.vars.as_slice()[0])
      .expect("vars 元素是 arena 写入的存活 Binding 指针")
      .name
      .as_str()
      .unwrap()
  );

  let bin = (stat.values.as_slice()[0])
    .as_node::<AstExprBinary>()
    .expect("local 值应为二元表达式");
  assert_eq!(AstExprBinaryOp::Add, bin.op);

  let lhs = (bin.left)
    .as_node::<AstExprLocal>()
    .expect("左操作数应为 local 引用");
  let rhs = (bin.right)
    .as_node::<AstExprLocal>()
    .expect("右操作数应为 local 引用");
  // Safety: local 字段指向存活 Binding（夹具 arena 只读）。
  assert_eq!(
    "x",
    PtrRef::as_ref_opt(&lhs.local)
      .expect("存活 Binding 指针")
      .name
      .as_str()
      .unwrap()
  );
  assert_eq!(
    "y",
    PtrRef::as_ref_opt(&rhs.local)
      .expect("存活 Binding 指针")
      .name
      .as_str()
      .unwrap()
  );
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_can_parse_fragments_in_line() {
  use ulua_ast::records::{
    ast_expr_binary::{AstExprBinary, AstExprBinaryOp},
    ast_expr_global::AstExprGlobal,
    ast_expr_local::AstExprLocal,
  };
  let _sff = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);
  let mut fixture = FragmentAutocompleteFixture::default();
  let result = fixture.base.check_with_options(
    r#"
local x = 4
local y = 5
"#,
  );
  assert_eq!(0, result.errors.len(), "{:?}", result.errors);

  let fragment = fixture
    .base
    .parse_fragment(
      r#"
local x = 4
local z = x + y
local y = 5
"#,
      &Position {
        line: 2,
        column: 15,
      },
      None,
    )
    .expect("expected fragment parse result");

  assert_eq!("local z = x + y", fragment.fragment_to_parse);
  assert_eq!(4, fragment.ancestry.len());
  assert!(!fragment.root.is_null());
  // Safety: fragment.root 已判空，指向夹具 arena 存活根块，仅只读。
  let root = fragment.root.as_ref_opt().unwrap();
  assert_eq!(
    Location {
      begin: Position { line: 2, column: 0 },
      end: Position {
        line: 2,
        column: 15
      },
    },
    root.base.base.location
  );
  assert_eq!(1, root.body.len());

  let stat = (root.body.as_slice()[0])
    .as_node::<AstStatLocal>()
    .expect("首条语句应为 local 声明");
  assert_eq!(1, stat.vars.size);
  assert_eq!(1, stat.values.size);
  assert_eq!(
    "z",
    // Binding 指针槽读法走夹具门面 PtrRef（null → None），零 unsafe。
    PtrRef::as_ref_opt(&stat.vars.as_slice()[0])
      .expect("vars 元素是 arena 写入的存活 Binding 指针")
      .name
      .as_str()
      .unwrap()
  );

  let bin = (stat.values.as_slice()[0])
    .as_node::<AstExprBinary>()
    .expect("local 值应为二元表达式");
  assert_eq!(AstExprBinaryOp::Add, bin.op);

  let lhs = (bin.left)
    .as_node::<AstExprLocal>()
    .expect("左操作数应为 local 引用");
  let rhs = (bin.right)
    .as_node::<AstExprGlobal>()
    .expect("右操作数应为 global 引用");
  // Safety: local 字段指向存活 Binding（夹具 arena 只读）。
  assert_eq!(
    "x",
    PtrRef::as_ref_opt(&lhs.local)
      .expect("存活 Binding 指针")
      .name
      .as_str()
      .unwrap()
  );
  assert_eq!("y", rhs.name.as_str().unwrap());
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_can_parse_in_correct_scope() {
  let _sff = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);
  let mut fixture = FragmentAutocompleteFixture::default();
  fixture.base.check_with_options(
    r#"
        local myLocal = 4
        function abc()
             local myInnerLocal = 1

        end
"#,
  );

  let fragment = fixture
    .base
    .parse_fragment(
      r#"
        local myLocal = 4
        function abc()
             local myInnerLocal = 1

        end
"#,
      &Position { line: 6, column: 0 },
      None,
    )
    .expect("expected fragment parse result");

  assert_eq!("", fragment.fragment_to_parse);
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_can_parse_multi_line_fragment_override() {
  use ulua_ast::records::{
    ast_expr_call::AstExprCall, ast_expr_constant_string::AstExprConstantString,
  };
  let _sff = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);
  let mut fixture = FragmentAutocompleteFixture::default();
  let result = fixture
    .base
    .check_with_options("function abc(foo: string) end");
  assert_eq!(0, result.errors.len(), "{:?}", result.errors);

  let fragment = fixture
    .base
    .parse_fragment(
      r#"function abc(foo: string) end
abc(
"foo"
)
abc("bar")
"#,
      &Position { line: 2, column: 5 },
      Some(Position { line: 3, column: 1 }),
    )
    .expect("expected fragment parse result");

  assert_eq!("abc(\n\"foo\"\n)", fragment.fragment_to_parse);
  assert!(!fragment.nearest_statement.is_null());
  assert!(crate::is_nearest::<AstStatExpr>(fragment.nearest_statement));
  assert!(fragment.ancestry.len() >= 2);

  let back = *fragment.ancestry.last().unwrap();
  assert!(crate::is_node::<AstExprConstantString>(back));
  // Safety: 判型后的 arena 节点指针物化为只读引用，用例作用域内存活且不再写入。
  let back = back.as_ref_opt().unwrap();
  assert_eq!(Position { line: 2, column: 0 }, back.location.begin);
  assert_eq!(Position { line: 2, column: 5 }, back.location.end);

  let parent = fragment.ancestry[fragment.ancestry.len() - 2];
  assert!(crate::is_node::<AstExprCall>(parent));
  // Safety: 同上，判型后的存活 arena 节点只读引用。
  let parent = parent.as_ref_opt().unwrap();
  assert_eq!(Position { line: 1, column: 0 }, parent.location.begin);
  assert_eq!(Position { line: 3, column: 1 }, parent.location.end);
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_can_parse_single_line_fragment_override() {
  use core::str;

  use ulua_ast::{
    enums::quote_style_ast::QuoteStyle,
    records::{ast_expr_call::AstExprCall, ast_expr_constant_string::AstExprConstantString},
  };
  let _sff = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);
  let mut fixture = FragmentAutocompleteFixture::default();
  let result = fixture
    .base
    .check_with_options("function abc(foo: string) end");
  assert_eq!(0, result.errors.len(), "{:?}", result.errors);

  let source = String::from(
    r#"function abc(foo: string) end
abc("foo")
abc("bar")
"#,
  );

  let call_fragment = fixture
    .base
    .parse_fragment(
      &source,
      &Position { line: 1, column: 6 },
      Some(Position {
        line: 1,
        column: 10,
      }),
    )
    .expect("expected call fragment parse result");

  assert_eq!("abc(\"foo\")", call_fragment.fragment_to_parse);
  assert!(!call_fragment.nearest_statement.is_null());
  assert!(crate::is_nearest::<AstStatExpr>(
    call_fragment.nearest_statement
  ));
  assert!(call_fragment.ancestry.len() >= 2);

  let back = *call_fragment.ancestry.last().unwrap();
  assert!(crate::is_node::<AstExprConstantString>(back));
  // Safety: 判型后的 arena 节点指针物化为只读引用，用例作用域内存活且不再写入。
  let back = back.as_ref_opt().unwrap();
  assert_eq!(Position { line: 1, column: 4 }, back.location.begin);
  assert_eq!(Position { line: 1, column: 9 }, back.location.end);

  let parent = call_fragment.ancestry[call_fragment.ancestry.len() - 2];
  assert!(crate::is_node::<AstExprCall>(parent));
  // Safety: 同上，判型后的存活 arena 节点只读引用。
  let parent = parent.as_ref_opt().unwrap();
  assert_eq!(Position { line: 1, column: 0 }, parent.location.begin);
  assert_eq!(
    Position {
      line: 1,
      column: 10
    },
    parent.location.end
  );

  let string_fragment = fixture
    .base
    .parse_fragment(
      &source,
      &Position { line: 1, column: 6 },
      Some(Position { line: 1, column: 9 }),
    )
    .expect("expected string fragment parse result");

  assert_eq!("abc(\"foo\"", string_fragment.fragment_to_parse);
  assert!(!string_fragment.nearest_statement.is_null());
  assert!(crate::is_nearest::<AstStatExpr>(
    string_fragment.nearest_statement
  ));
  assert!(!string_fragment.ancestry.is_empty());

  let back = *string_fragment.ancestry.last().unwrap();
  // 门面一步下转+判型+物化（原 `ast_node_as + is_null + &*` 三步）。
  // Safety: ancestry 元素为夹具 arena 存活节点指针，只读至用例结束。
  let as_string = (back)
    .as_node::<AstExprConstantString>()
    .expect("ancestry 尾元素应为字符串字面量");
  assert_eq!(
    Position { line: 1, column: 4 },
    as_string.base.base.location.begin
  );
  assert_eq!(
    Position { line: 1, column: 9 },
    as_string.base.base.location.end
  );
  // AstArray<c_char> 自带 as_bytes()，无需手写 from_raw_parts。
  let value = as_string.value.as_bytes();
  assert_eq!("foo", str::from_utf8(value).unwrap());
  assert_eq!(QuoteStyle::QuotedSimple, as_string.quote_style);
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_can_typecheck_fragment_inserted_inline() {
  let _sff = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);
  let mut fixture = FragmentAutocompleteFixture::default();
  let res = fixture.base.check_with_options(
    r#"
local x = 4
local y = 5
"#,
  );

  assert_eq!(0, res.errors.len(), "{:?}", res.errors);

  let fragment = fixture.base.check_fragment(
    r#"
local x = 4
local z = x
local y = 5
"#,
    Position {
      line: 2,
      column: 11,
    },
    None,
  );

  let correct = linear_search_for_binding(&fragment.fresh_scope, "z");
  LUAU_ASSERT!(correct.is_some());
  assert_eq!("number", to_string_type_id(correct.unwrap()));
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_can_typecheck_simple_fragment() {
  let _sff = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);
  let mut fixture = FragmentAutocompleteFixture::default();
  let res = fixture.base.check_with_options(
    r#"
local x = 4
local y = 5
"#,
  );

  assert_eq!(0, res.errors.len(), "{:?}", res.errors);

  let fragment = fixture.base.check_fragment(
    r#"
local x = 4
local y = 5
local z = x + y
"#,
    Position {
      line: 3,
      column: 15,
    },
    None,
  );

  let opt = linear_search_for_binding(&fragment.fresh_scope, "z");
  LUAU_ASSERT!(opt.is_some());
  assert_eq!("number", to_string_type_id(opt.unwrap()));
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_class_autocomplete_between_definitions() {
  let _sff = ScopedFastFlag::new(&fflag::DebugLuauUserDefinedClasses, true);

  let source = String::from(
    r#"--!strict
class Bar
    public value: number
    function printvalue(self)
        print(self.value)
    end
end
"#,
  );

  let dest = String::from(
    r#"--!strict
class Bar
    public value: number
    function printvalue(self)
        print(self.value)
    end
    s@1
end
"#,
  );

  let mut fixture = FragmentAutocompleteFixture::default();
  fixture.base.autocomplete_fragment_in_new_solver(
    &source,
    &dest,
    '1',
    |frag: &mut FragmentAutocompleteStatusResult| {
      LUAU_ASSERT!(frag.result.is_some());
      assert!(
        !frag
          .result
          .as_ref()
          .unwrap()
          .ac_results
          .entry_map
          .contains_key("self")
      );
    },
    None,
  );
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_class_autocomplete_classname_inside_method() {
  let _sff = ScopedFastFlag::new(&fflag::DebugLuauUserDefinedClasses, true);

  let source = String::from(
    r#"--!strict
class Bar
    public value: number
    function new()
    end
end
"#,
  );

  let dest = String::from(
    r#"--!strict
class Bar
    public value: number
    function new()
        return B@1
    end
end
"#,
  );

  let mut fixture = FragmentAutocompleteFixture::default();
  fixture.base.autocomplete_fragment_in_new_solver(
    &source,
    &dest,
    '1',
    |frag: &mut FragmentAutocompleteStatusResult| {
      assert!(frag.result.is_some());
      let ac = &frag.result.as_ref().unwrap().ac_results;
      assert!(ac.entry_map.contains_key("Bar"));
    },
    None,
  );
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_class_instance_dot_includes_method_from_outside() {
  let _sff = ScopedFastFlag::new(&fflag::DebugLuauUserDefinedClasses, true);

  let source = String::from(
    r#"--!strict
class Bar
    public value: number
    function doThing(self)
    end
end
local bar = Bar.new { value = 1 }
"#,
  );

  let dest = String::from(
    r#"--!strict
class Bar
    public value: number
    function doThing(self)
    end
end
local bar = Bar.new { value = 1 }
bar.@1
"#,
  );

  let mut fixture = FragmentAutocompleteFixture::default();
  fixture.base.autocomplete_fragment_in_new_solver(
    &source,
    &dest,
    '1',
    |frag: &mut FragmentAutocompleteStatusResult| {
      LUAU_ASSERT!(frag.result.is_some());
      let ac_results = &frag.result.as_ref().unwrap().ac_results;
      assert!(ac_results.entry_map.contains_key("value"));
      assert!(ac_results.entry_map.contains_key("doThing"));
    },
    None,
  );
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_class_instance_dot_property_from_outside() {
  let _sff = ScopedFastFlag::new(&fflag::DebugLuauUserDefinedClasses, true);

  let source = String::from(
    r#"--!strict
class Bar
    public value: number
end
local bar = Bar.new { value = 1 }
"#,
  );

  let dest = String::from(
    r#"--!strict
class Bar
    public value: number
end
local bar = Bar.new { value = 1 }
bar.@1
"#,
  );

  let mut fixture = FragmentAutocompleteFixture::default();
  fixture.base.autocomplete_fragment_in_new_solver(
    &source,
    &dest,
    '1',
    |frag: &mut FragmentAutocompleteStatusResult| {
      LUAU_ASSERT!(frag.result.is_some());
      let ac = &frag.result.as_ref().unwrap().ac_results;
      assert!(ac.entry_map.contains_key("value"));
      assert!(!ac.entry_map.contains_key("z"));
    },
    None,
  );
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_class_instance_multiple_props_from_outside() {
  let _sff = ScopedFastFlag::new(&fflag::DebugLuauUserDefinedClasses, true);

  let source = String::from(
    r#"--!strict
class Point
    public x: number
    public y: number
    public z: number
end
local p = Point.new { x = 0, y = 0, z = 0 }
"#,
  );

  let dest = String::from(
    r#"--!strict
class Point
    public x: number
    public y: number
    public z: number
end
local p = Point.new { x = 0, y = 0, z = 0 }
p.@1
"#,
  );

  let mut fixture = FragmentAutocompleteFixture::default();
  fixture.base.autocomplete_fragment_in_new_solver(
    &source,
    &dest,
    '1',
    |frag: &mut FragmentAutocompleteStatusResult| {
      LUAU_ASSERT!(frag.result.is_some());
      let ac = &frag.result.as_ref().unwrap().ac_results;
      assert!(ac.entry_map.contains_key("x"));
      assert!(ac.entry_map.contains_key("y"));
      assert!(ac.entry_map.contains_key("z"));
    },
    None,
  );
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_class_method_args_not_in_scope_outside_class() {
  let _sff = ScopedFastFlag::new(&fflag::DebugLuauUserDefinedClasses, true);

  let mut fixture = FragmentAutocompleteFixture::default();
  let result = fixture.base.run_autocomplete_visitor(
    r#"
class Bar
    function method(self)
    end
end
local x = 4
"#,
    &Position {
      line: 6,
      column: 10,
    },
  );

  assert!(
    !result
      .local_map
      .iter()
      .any(|(name, _)| { name.as_bytes() == b"self" })
  );
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_class_method_extra_args_visible_in_body() {
  let _sff = ScopedFastFlag::new(&fflag::DebugLuauUserDefinedClasses, true);

  let source = String::from(
    r#"--!strict
class Counter
    public value: number
    function increment(self, count: number)
    end
end
"#,
  );

  let dest = String::from(
    r#"--!strict
class Counter
    public value: number
    function increment(self, count: number)
        @1
    end
end
"#,
  );

  let mut fixture = FragmentAutocompleteFixture::default();
  fixture.base.autocomplete_fragment_in_new_solver(
    &source,
    &dest,
    '1',
    |frag: &mut FragmentAutocompleteStatusResult| {
      assert!(frag.result.is_some());
      let ac = &frag.result.as_ref().unwrap().ac_results;
      assert!(ac.entry_map.contains_key("self"));
      assert!(ac.entry_map.contains_key("count"));
    },
    None,
  );
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_class_method_self_dot_autocomplete() {
  let _sff = ScopedFastFlag::new(&fflag::DebugLuauUserDefinedClasses, true);

  let source = String::from(
    r#"--!strict
class Bar
    public value: number
    function doThing(self)
    end
end
"#,
  );

  let dest = String::from(
    r#"--!strict
class Bar
    public value: number
    function doThing(self)
        self.@1
    end
end
"#,
  );

  let mut fixture = FragmentAutocompleteFixture::default();
  fixture.base.autocomplete_fragment_in_new_solver(
    &source,
    &dest,
    '1',
    |frag: &mut FragmentAutocompleteStatusResult| {
      assert!(frag.result.is_some());
      let ac = &frag.result.as_ref().unwrap().ac_results;
      assert!(ac.entry_map.contains_key("value"));
      assert!(!ac.entry_map.contains_key("z"));
    },
    None,
  );
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_class_method_self_dot_multiple_properties() {
  let _sff = ScopedFastFlag::new(&fflag::DebugLuauUserDefinedClasses, true);

  let source = String::from(
    r#"--!strict
class Vec3
    public x: number
    public y: number
    public z: number
    function length(self)
    end
end
"#,
  );

  let dest = String::from(
    r#"--!strict
class Vec3
    public x: number
    public y: number
    public z: number
    function length(self)
        self.@1
    end
end
"#,
  );

  let mut fixture = FragmentAutocompleteFixture::default();
  fixture.base.autocomplete_fragment_in_new_solver(
    &source,
    &dest,
    '1',
    |frag: &mut FragmentAutocompleteStatusResult| {
      assert!(frag.result.is_some());
      let ac = &frag.result.as_ref().unwrap().ac_results;
      assert!(ac.entry_map.contains_key("x"));
      assert!(ac.entry_map.contains_key("y"));
      assert!(ac.entry_map.contains_key("z"));
    },
    None,
  );
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_class_method_self_in_local_stack() {
  let _sff = ScopedFastFlag::new(&fflag::DebugLuauUserDefinedClasses, true);

  let mut fixture = FragmentAutocompleteFixture::default();
  let result = fixture.base.run_autocomplete_visitor(
    r#"
class Bar
    public value: number
    function doThing(self)
    end
end
"#,
    &Position { line: 4, column: 2 },
  );

  assert_eq!(1, result.local_stack.len());
  assert_eq!(result.local_map.size(), result.local_stack.len());
  let last = *result.local_stack.last().unwrap();
  // (下方 unsafe: local_stack 元素是 arena 写入的存活 Binding 指针，只读取名)
  assert_eq!("self", last.as_ref_opt().unwrap().name.as_str().unwrap());
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_class_second_method_self_dot() {
  let _sff = ScopedFastFlag::new(&fflag::DebugLuauUserDefinedClasses, true);

  let source = String::from(
    r#"--!strict
class Bar
    public value: number
    function first(self)
    end
    function second(self)
    end
end
"#,
  );

  let dest = String::from(
    r#"--!strict
class Bar
    public value: number
    function first(self)
    end
    function second(self)
        self.@1
    end
end
"#,
  );

  let mut fixture = FragmentAutocompleteFixture::default();
  fixture.base.autocomplete_fragment_in_new_solver(
    &source,
    &dest,
    '1',
    |frag: &mut FragmentAutocompleteStatusResult| {
      assert!(frag.result.is_some());
      let ac = &frag.result.as_ref().unwrap().ac_results;
      assert!(ac.entry_map.contains_key("value"));
    },
    None,
  );
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_class_static_method_dot_autocomplete_1() {
  let _sff = ScopedFastFlag::new(&fflag::DebugLuauUserDefinedClasses, true);

  let source = String::from(
    r#"--!strict
class Bar
    public value: number
    function new()
        return Bar { value = 0 }
    end
end
"#,
  );

  let dest = String::from(
    r#"--!strict
class Bar
    public value: number
    function new()
        return Bar { value = 0 }
    end
end

Bar.@1
"#,
  );

  let mut fixture = FragmentAutocompleteFixture::default();
  fixture.base.autocomplete_fragment_in_new_solver(
    &source,
    &dest,
    '1',
    |frag: &mut FragmentAutocompleteStatusResult| {
      LUAU_ASSERT!(frag.result.is_some());
      assert!(
        frag
          .result
          .as_ref()
          .unwrap()
          .ac_results
          .entry_map
          .contains_key("new")
      );
    },
    None,
  );
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_class_static_method_dot_autocomplete_2() {
  let _sff = ScopedFastFlag::new(&fflag::DebugLuauUserDefinedClasses, true);

  let source = String::from(
    r#"--!strict
class Bar
    public value: number
    function new()
        return Bar { value = 0 }
    end
end
"#,
  );

  let dest = String::from(
    r#"--!strict
class Bar
    public value: number
    function new()
        return Bar { value = 0 }
    end
end

local _ = Bar.new()

Bar.@1
"#,
  );

  let mut fixture = FragmentAutocompleteFixture::default();
  fixture.base.autocomplete_fragment_in_new_solver(
    &source,
    &dest,
    '1',
    |frag: &mut FragmentAutocompleteStatusResult| {
      LUAU_ASSERT!(frag.result.is_some());
      let ac = &frag.result.as_ref().unwrap().ac_results;
      assert!(ac.entry_map.contains_key("new"));
    },
    None,
  );
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_correctly_grab_innermost_refinement() {
  let source = String::from(
    r#"
--!strict
type Type1 = { Type: "Type1", CommonKey: string, Type1Key: string }
type Type2 = { Type: "Type2", CommonKey: string, Type2Key: string }
type UnionType = Type1 | Type2

local foo: UnionType? = nil
if foo then
    if foo.Type == "Type2" then
    end
end
    "#,
  );

  let dest = String::from(
    r#"
--!strict
type Type1 = { Type: "Type1", CommonKey: string, Type1Key: string }
type Type2 = { Type: "Type2", CommonKey: string, Type2Key: string }
type UnionType = Type1 | Type2

local foo: UnionType? = nil
if foo then
    if foo.Type == "Type2" then
        foo.@1
    end
end
    "#,
  );

  let mut fixture = FragmentAutocompleteBuiltinsFixture::default();
  fixture.base.autocomplete_fragment_in_both_solvers(
    &source,
    &dest,
    '1',
    |result: &mut FragmentAutocompleteStatusResult| {
      let ac = &result.result.as_ref().unwrap().ac_results;
      assert!(!ac.entry_map.is_empty());
      assert!(ac.entry_map.contains_key("Type2Key") as usize > 0);
      assert!(ac.entry_map.contains_key("Type1Key") as usize == 0);
    },
    None,
  );
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_cursor_that_comes_later_shouldnt_capture_locals_in_unavailable_scope() {
  let mut fixture = FragmentAutocompleteFixture::default();
  let result = fixture.base.run_autocomplete_visitor(
    r#"
local x = 4
local y = 5
if x == 4 then
    local e = y
end
local z = x + x
if y == 5 then
    local q = x + y + z
end
"#,
    &Position {
      line: 8,
      column: 23,
    },
  );

  assert_eq!(6, result.ancestry.len());
  assert_eq!(3, result.local_stack.len());
  assert_eq!(result.local_map.size(), result.local_stack.len());
  assert!(!result.nearest_statement.is_null());
  let last = *result.local_stack.last().unwrap();
  // (下方 unsafe: local_stack 元素是 arena 写入的存活 Binding 指针，只读取名)
  assert_eq!("z", last.as_ref_opt().unwrap().name.as_str().unwrap());

  // 门面一步下转+判型+物化（原 `ast_node_as + is_null + &*` 三步）。
  // Safety: nearest_statement 为夹具 arena 存活语句指针，只读至用例结束。
  let local = (result.nearest_statement)
    .as_node::<AstStatLocal>()
    .expect("nearest_statement 应为 local 声明");
  assert_eq!(1, local.vars.size);
  // Binding 指针槽读法走夹具门面 PtrRef（null → None），零 unsafe。
  let var =
    PtrRef::as_ref_opt(&local.vars.as_slice()[0]).expect("vars 元素是 arena 写入的存活节点指针");
  assert_eq!("q", var.name.as_str().unwrap());
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_cursor_within_scope_tracks_locals_from_previous_scope() {
  let mut fixture = FragmentAutocompleteFixture::default();
  let result = fixture.base.run_autocomplete_visitor(
    r#"
local x = 4
local y = 5
if x == 4 then
    local e = y
end
"#,
    &Position {
      line: 4,
      column: 15,
    },
  );

  assert_eq!(5, result.ancestry.len());
  assert_eq!(2, result.local_stack.len());
  assert_eq!(result.local_map.size(), result.local_stack.len());
  assert!(!result.nearest_statement.is_null());
  let last = *result.local_stack.last().unwrap();
  // (下方 unsafe: local_stack 元素是 arena 写入的存活 Binding 指针，只读取名)
  assert_eq!("y", last.as_ref_opt().unwrap().name.as_str().unwrap());

  // 门面一步下转+判型+物化（原 `ast_node_as + is_null + &*` 三步）。
  // Safety: nearest_statement 为夹具 arena 存活语句指针，只读至用例结束。
  let local = (result.nearest_statement)
    .as_node::<AstStatLocal>()
    .expect("nearest_statement 应为 local 声明");
  assert_eq!(1, local.vars.size);
  // Binding 指针槽读法走夹具门面 PtrRef（null → None），零 unsafe。
  let var =
    PtrRef::as_ref_opt(&local.vars.as_slice()[0]).expect("vars 元素是 arena 写入的存活节点指针");
  assert_eq!("e", var.name.as_str().unwrap());
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_cyclic_table() {
  let source = String::from(
    r#"
        local abc = {}
        local def = { abc = abc }
        abc.def = def
        abc.def.@1
    "#,
  );

  let mut fixture = FragmentAutocompleteFixture::default();
  fixture.base.autocomplete_fragment_in_both_solvers(
    &source,
    &source,
    '1',
    |frag: &mut FragmentAutocompleteStatusResult| {
      LUAU_ASSERT!(frag.result.is_some());
      let ac = &frag.result.as_ref().unwrap().ac_results;
      assert!(ac.entry_map.contains_key("abc"));
      assert_eq!(ac.context, AutocompleteContext::Property);
    },
    None,
  );
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_diff_multiple_blocks_on_same_line() {
  let source = String::from(
    r#"
do local function foo() end; local x = ""; end do local function bar() end"#,
  );
  let dest = String::from(
    r#"
do local function foo() end; local x = ""; end do local function bar() end local x = {a : number}; b @1end "#,
  );

  let mut fixture = FragmentAutocompleteBuiltinsFixture::default();
  fixture.base.autocomplete_fragment_in_both_solvers(
    &source,
    &dest,
    '1',
    |res: &mut FragmentAutocompleteStatusResult| {
      assert_eq!(FragmentAutocompleteStatus::Success, res.status);
      LUAU_ASSERT!(res.result.is_some());
      let ac = &res.result.as_ref().unwrap().ac_results;
      assert!(!ac.entry_map.is_empty());
      assert!(ac.entry_map.contains_key("bar"));
      assert!(!ac.entry_map.contains_key("foo"));
    },
    None,
  );
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_differ_1() {
  let source = String::from(r#""#);
  let dest = String::from(
    r#"local tbl = { foo = 1, bar = 2 };
tbl.b@1"#,
  );

  let mut fixture = FragmentAutocompleteBuiltinsFixture::default();
  fixture.base.autocomplete_fragment_in_both_solvers(
    &source,
    &dest,
    '1',
    |status: &mut FragmentAutocompleteStatusResult| {
      assert!(FragmentAutocompleteStatus::Success == status.status);
      LUAU_ASSERT!(status.result.is_some());
      let ac = &status.result.as_ref().unwrap().ac_results;
      assert!(!ac.entry_map.is_empty());
      assert!(ac.entry_map.contains_key("foo"));
      assert!(ac.entry_map.contains_key("bar"));
    },
    None,
  );
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_do_not_recommend_results_in_multiline_comment() {
  let source = String::from(
    r#"--[[
"#,
  );
  let dest = String::from(
    r#"--[[
a@1
"#,
  );

  let mut fixture = FragmentAutocompleteFixture::default();
  fixture.base.autocomplete_fragment_in_both_solvers(
    &source,
    &dest,
    '1',
    |frag: &mut FragmentAutocompleteStatusResult| {
      assert!(frag.result.is_none());
    },
    None,
  );
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_dont_suggest_local_before_its_definition() {
  let source = String::from(
    r#"
        local myLocal = 4
        function abc()
@1             local myInnerLocal = 1
@2
        end
@3    "#,
  );

  let mut fixture = FragmentAutocompleteFixture::default();

  // autocomplete after abc but before myInnerLocal
  fixture.base.autocomplete_fragment_in_both_solvers(
    &source,
    &source,
    '1',
    |fragment: &mut FragmentAutocompleteStatusResult| {
      LUAU_ASSERT!(fragment.result.is_some());
      let ac = &fragment.result.as_ref().unwrap().ac_results;
      assert!(ac.entry_map.contains_key("myLocal"));
      assert!(!ac.entry_map.contains_key("myInnerLocal"));
    },
    None,
  );
  // autocomplete after my inner local
  fixture.base.autocomplete_fragment_in_both_solvers(
    &source,
    &source,
    '2',
    |fragment: &mut FragmentAutocompleteStatusResult| {
      LUAU_ASSERT!(fragment.result.is_some());
      let ac = &fragment.result.as_ref().unwrap().ac_results;
      assert!(ac.entry_map.contains_key("myLocal"));
      assert!(ac.entry_map.contains_key("myInnerLocal"));
    },
    None,
  );

  // autocomplete after abc, but don't include myInnerLocal(in the hidden scope)
  fixture.base.autocomplete_fragment_in_both_solvers(
    &source,
    &source,
    '3',
    |fragment: &mut FragmentAutocompleteStatusResult| {
      LUAU_ASSERT!(fragment.result.is_some());
      let ac = &fragment.result.as_ref().unwrap().ac_results;
      assert!(ac.entry_map.contains_key("myLocal"));
      assert!(!ac.entry_map.contains_key("myInnerLocal"));
    },
    None,
  );
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_duped_alias() {
  use ulua_analysis::records::scope::Scope;
  let source = String::from(
    r#"
type a = typeof({})

"#,
  );
  let dest = String::from(
    r#"
type a = typeof({})
type a = typeof({})@1
"#,
  );

  // Re-parsing and typechecking a type alias in the fragment that was defined in the base module will assert in ConstraintGenerator::checkAliases
  // unless we don't clone it This will let the incremental pass re-generate the type binding, and we will expect to see it in the type bindings
  let mut fixture = FragmentAutocompleteBuiltinsFixture::default();
  fixture.base.autocomplete_fragment_in_both_solvers(
    &source,
    &dest,
    '1',
    |frag: &mut FragmentAutocompleteStatusResult| {
      LUAU_ASSERT!(frag.result.is_some());
      let sc: *mut Scope = frag.result.as_ref().unwrap().fresh_scope;
      // Safety: fresh_scope 是结果自带的存活 Scope 句柄，仅只读查找绑定。
      assert!(
        sc.as_ref_opt()
          .unwrap()
          .private_type_bindings
          .contains_key("a")
      );
    },
    None,
  );
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_empty_program() {
  let mut fixture = FragmentAutocompleteFixture::default();
  fixture.base.autocomplete_fragment_in_both_solvers(
    "",
    "@1",
    '1',
    |frag: &mut FragmentAutocompleteStatusResult| {
      assert!(frag.result.is_some());
      let ac = &frag.result.as_ref().unwrap().ac_results;
      assert!(ac.entry_map.contains_key("table"));
      assert!(ac.entry_map.contains_key("math"));
      assert_eq!(ac.context, AutocompleteContext::Statement);
    },
    None,
  );
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_empty_program_1() {
  let mut fixture = FragmentAutocompleteFixture::default();
  fixture.base.check_with_options("");
  let _sfi = ScopedFastInt::new(&fint::LuauParseErrorLimit, 1);
  let fragment = fixture
    .base
    .parse_fragment(
      "",
      &Position {
        line: 0,
        column: 39,
      },
      None,
    )
    .expect("expected fragment parse result");

  assert_eq!("", fragment.fragment_to_parse);
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_empty_program_2() {
  let mut fixture = FragmentAutocompleteFixture::default();
  let source = String::from(
    r#"

"#,
  );
  fixture.base.check_with_options(&source);
  let _sfi = ScopedFastInt::new(&fint::LuauParseErrorLimit, 1);
  let fragment = fixture
    .base
    .parse_fragment(
      &source,
      &Position {
        line: 1,
        column: 39,
      },
      None,
    )
    .expect("expected fragment parse result");

  assert_eq!("", fragment.fragment_to_parse);
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_end_multiline_call() {
  let (_fixture, region) = fx_region!(
    r#"
abc(
"foo"
)
"#,
    Position { line: 3, column: 1 }
  );

  assert_eq!(
    Location {
      begin: Position { line: 1, column: 0 },
      end: Position { line: 3, column: 1 },
    },
    region.fragment_location
  );
  assert!(!region.parent_block.is_null());
  assert!(!region.nearest_statement.is_null());
  assert!(crate::is_nearest::<AstStatExpr>(region.nearest_statement));
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_end_of_do() {
  let (_fixture, region) = fx_region!(
    r#"
local x = 4
do
end
"#,
    Position { line: 3, column: 3 }
  );

  assert_eq!(
    Location {
      begin: Position { line: 3, column: 3 },
      end: Position { line: 3, column: 3 },
    },
    region.fragment_location
  );
  assert!(!region.parent_block.is_null());
  assert!(crate::is_nearest::<AstStatBlock>(region.nearest_statement));
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_expr_function() {
  let source = String::from(
    r#"
local t = {}
type Input = {x : string}
function t.Do(fn : (Input) -> ())
    if t.x == "a" then
        return
    end
end

t.Do(function (f)
    f
end)
"#,
  );

  let dest = String::from(
    r#"
local t = {}
type Input = {x : string}
function t.Do(fn : (Input) -> ())
    if t.x == "a" then
        return
    end
end

t.Do(function (f)
    f.@1
end)
"#,
  );

  let mut fixture = FragmentAutocompleteBuiltinsFixture::default();
  fixture.base.autocomplete_fragment_in_both_solvers(
    &source,
    &dest,
    '1',
    |status: &mut FragmentAutocompleteStatusResult| {
      assert!(FragmentAutocompleteStatus::Success == status.status);
      LUAU_ASSERT!(status.result.is_some());
      assert!(
        !status
          .result
          .as_ref()
          .unwrap()
          .ac_results
          .entry_map
          .is_empty()
      );
      assert!(
        status
          .result
          .as_ref()
          .unwrap()
          .ac_results
          .entry_map
          .contains_key("x")
      );
    },
    None,
  );
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_for_expr_in_should_rec_no_do() {
  let source = String::from(
    r#"
type T = { x : {[number] : number}, y: number, z: number}
local x : T = ({} :: T)
for i =
end
"#,
  );
  let dest = String::from(
    r#"
type T = { x : {[number] : number}, y: number, z : number}
local x : T = ({} :: T)
for i = x.@1
end
"#,
  );

  let mut fixture = FragmentAutocompleteBuiltinsFixture::default();
  fixture.base.autocomplete_fragment_in_both_solvers(
    &source,
    &dest,
    '1',
    |result: &mut FragmentAutocompleteStatusResult| {
      let ac_results = &result.result.as_ref().unwrap().ac_results;
      assert!(!ac_results.entry_map.is_empty());
      assert!(ac_results.entry_map.contains_key("x"));
      assert!(ac_results.entry_map.contains_key("y"));
      assert!(ac_results.entry_map.contains_key("z"));
    },
    None,
  );
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_for_expr_in_should_rec_with_do_in_max_add() {
  let source = String::from(
    r#"
type T = { x : {[number] : number}, y: number, z: number}
local x : T = ({} :: T)
for i = x.y do
end
"#,
  );
  let dest = String::from(
    r#"
type T = { x : {[number] : number}, y: number, z : number}
local x : T = ({} :: T)
for i = x.y, x.@1 do
end
"#,
  );

  let mut fixture = FragmentAutocompleteBuiltinsFixture::default();
  fixture.base.autocomplete_fragment_in_both_solvers(
    &source,
    &dest,
    '1',
    |result: &mut FragmentAutocompleteStatusResult| {
      let ac = &result.result.as_ref().unwrap().ac_results;
      assert!(!ac.entry_map.is_empty());
      assert!(ac.entry_map.contains_key("x"));
      assert!(ac.entry_map.contains_key("y"));
      assert!(ac.entry_map.contains_key("z"));
    },
    None,
  );
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_for_expr_in_should_rec_with_do_in_max_delete() {
  let source = String::from(
    r#"
type T = { x : {[number] : number}, y: number, z: number}
local x : T = ({} :: T)
for i = x.y, x.z do
end
"#,
  );
  let dest = String::from(
    r#"
type T = { x : {[number] : number}, y: number, z : number}
local x : T = ({} :: T)
for i = x.y, x.@1 do
end
"#,
  );
  let mut fixture = FragmentAutocompleteBuiltinsFixture::default();
  fixture.base.autocomplete_fragment_in_both_solvers(
    &source,
    &dest,
    '1',
    |result: &mut FragmentAutocompleteStatusResult| {
      assert!(
        !result
          .result
          .as_ref()
          .unwrap()
          .ac_results
          .entry_map
          .is_empty()
      );
      assert!(
        result
          .result
          .as_ref()
          .unwrap()
          .ac_results
          .entry_map
          .contains_key("x")
      );
      assert!(
        result
          .result
          .as_ref()
          .unwrap()
          .ac_results
          .entry_map
          .contains_key("y")
      );
      assert!(
        result
          .result
          .as_ref()
          .unwrap()
          .ac_results
          .entry_map
          .contains_key("z")
      );
    },
    None,
  );
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_for_expr_in_should_rec_with_do_in_step() {
  let source = String::from(
    r#"
type T = { x : {[number] : number}, y: number, z: number}
local x : T = ({} :: T)
for i = x.y, 100 do
end
"#,
  );
  let dest = String::from(
    r#"
type T = { x : {[number] : number}, y: number, z : number}
local x : T = ({} :: T)
for i = x.y, 100, x.@1 do
end
"#,
  );

  let mut fixture = FragmentAutocompleteBuiltinsFixture::default();
  fixture.base.autocomplete_fragment_in_both_solvers(
    &source,
    &dest,
    '1',
    |result: &mut FragmentAutocompleteStatusResult| {
      let ac_results = &result.result.as_ref().unwrap().ac_results;
      assert!(!ac_results.entry_map.is_empty());
      assert!(ac_results.entry_map.contains_key("x"));
      assert!(ac_results.entry_map.contains_key("y"));
      assert!(ac_results.entry_map.contains_key("z"));
    },
    None,
  );
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_for_in_should_rec() {
  let source = String::from(
    r#"
type T = { x : {[number] : number}, y: number}
local x : T = ({} :: T)
for _,n in pairs(x.@1) do
end
"#,
  );
  let mut fixture = FragmentAutocompleteBuiltinsFixture::default();
  fixture.base.autocomplete_fragment_in_both_solvers(
    &source,
    &source,
    '1',
    |result: &mut FragmentAutocompleteStatusResult| {
      assert!(
        !result
          .result
          .as_ref()
          .unwrap()
          .ac_results
          .entry_map
          .is_empty()
      );
      assert!(
        result
          .result
          .as_ref()
          .unwrap()
          .ac_results
          .entry_map
          .contains_key("x")
      );
      assert!(
        result
          .result
          .as_ref()
          .unwrap()
          .ac_results
          .entry_map
          .contains_key("y")
      );
    },
    None,
  );
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_for_loop_recommends() {
  let source = String::from(
    r#"
local testArr: {{a: number, b: number}} = {
{a = 1, b = 2},
{a = 2, b = 4},
}

for _, v in testArr do

end
"#,
  );

  let dest = String::from(
    r#"
local testArr: {{a: number, b: number}} = {
{a = 1, b = 2},
{a = 2, b = 4},
}

for _, v in testArr do
    print(v.@1
end
"#,
  );

  let mut fixture = FragmentAutocompleteBuiltinsFixture::default();
  fixture.base.autocomplete_fragment_in_both_solvers(
    &source,
    &dest,
    '1',
    |result: &mut FragmentAutocompleteStatusResult| {
      assert!(result.status != FragmentAutocompleteStatus::InternalIce);
      assert!(result.result.is_some());
      let ac = &result.result.as_ref().unwrap().ac_results;
      assert!(!ac.entry_map.is_empty());
      assert!(ac.entry_map.contains_key("a"));
      assert!(ac.entry_map.contains_key("b"));
    },
    None,
  );
}

mod fragment_autocomplete_for_loop_recommends_fragment_autocomplete_test_case_2 {
  //! Source: `tests/FragmentAutocomplete.test.cpp`
  use super::*;

  #[test]
  fn fragment_autocomplete_for_loop_recommends() {
    let source = String::from(
      r#"
local testArr: {string} = {
"a",
"b",
}

for _, v in testArr do

end
"#,
    );

    let dest = String::from(
      r#"
local testArr: {string} = {
"a",
"b",
}

for _, v in testArr do
    print(v:@1)
end
"#,
    );

    let mut fixture = FragmentAutocompleteBuiltinsFixture::default();
    fixture.base.autocomplete_fragment_in_both_solvers(
      &source,
      &dest,
      '1',
      |result: &mut FragmentAutocompleteStatusResult| {
        assert!(result.status != FragmentAutocompleteStatus::InternalIce);
        assert!(result.result.is_some());
        let ac = &result.result.as_ref().unwrap().ac_results;
        assert!(!ac.entry_map.is_empty());
        assert!(ac.entry_map.contains_key("upper"));
        assert!(ac.entry_map.contains_key("sub"));
      },
      None,
    );
  }
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_fragment_ac_must_traverse_typeof_and_not_ice() {
  // This test ensures that we traverse typeof expressions for defs that are being referred to in the fragment
  // In this case, we want to ensure we populate the incremental environment with the reference to `m`
  // Without this, we would ice as we will refer to the local `m` before it's declaration
  let source = String::from(
    r#"
--!strict
local m = {}
-- and here
function m:m1() end
type nt = typeof(m)

return m
"#,
  );
  let updated = String::from(
    r#"
--!strict
local m = {}
-- and here
function m:m1() end
type nt = typeof(m)
l @1
return m
"#,
  );

  let mut fixture = FragmentAutocompleteBuiltinsFixture::default();
  fixture.base.autocomplete_fragment_in_both_solvers(
    &source,
    &updated,
    '1',
    |_: &mut FragmentAutocompleteStatusResult| {},
    None,
  );
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_fragment_autocomplete_ensures_memory_isolation() {
  use ulua_analysis::{
    functions::to_string_to_string::to_string_type_id_to_string_options_mut,
    records::to_string_options::ToStringOptions,
    type_aliases::{module_ptr_module::ModulePtr, type_id::TypeId},
  };
  use ulua_unit_test::functions::lookup_name::lookup_name;

  let opt = ToStringOptions {
    exhaustive: true,
    function_type_arguments: true,
    max_table_length: 0,
    max_type_length: 0,
    ..ToStringOptions::default()
  };

  fn get_type_from_module(module: &ModulePtr, name: &str) -> Option<TypeId> {
    if !module.has_module_scope() {
      return None;
    }
    let scope = module.get_module_scope();
    lookup_name(&scope, name)
  }

  let source = String::from(
    r#"local module = {}
f
return module"#,
  );

  let updated1 = String::from(
    r#"local module = {}
function module.a
return module"#,
  );

  let updated2 = String::from(
    r#"local module = {}
function module.ab
return module"#,
  );

  let mut fixture = FragmentAutocompleteFixture::default();

  let check_and_examine =
    |fixture: &mut FragmentAutocompleteFixture, src: &str, id_name: &str, id_string: &str| {
      fixture.base.check_with_options(src);
      let id = fixture.base.base.base.get_type(id_name, true);
      LUAU_ASSERT!(id.is_some());
      assert_eq!(
        to_string_type_id_to_string_options_mut(id.unwrap(), opt.clone()),
        String::from(id_string)
      );
    };

  let fragment_ac_and_check =
    |fixture: &mut FragmentAutocompleteFixture, updated: &str, pos: Position, id_name: &str| {
      let frag = fixture.base.autocomplete_fragment(updated, pos, None);
      LUAU_ASSERT!(frag.result.is_some());
      let frag_id =
        get_type_from_module(&frag.result.as_ref().unwrap().incremental_module, id_name);
      LUAU_ASSERT!(frag_id.is_some());

      let src_id = fixture.base.base.base.get_type(id_name, true);
      LUAU_ASSERT!(src_id.is_some());

      let frag_id = frag_id.unwrap();
      let src_id = src_id.unwrap();
      // Safety: frag_id/src_id 为存活 arena 类型句柄（TypeId 即被测侧 `*const Type`
      // 别名，(c) 类无安全解引用门面），本处只读 owning_arena 标量字段。
      let frag_arena = frag_id.as_ref_opt().unwrap().owning_arena;
      let src_arena = src_id.as_ref_opt().unwrap().owning_arena;
      let module_arena = frag
        .result
        .as_ref()
        .unwrap()
        .incremental_module
        .internal_types
        .arena_id;
      assert_ne!(frag_arena, src_arena);
      assert_eq!(frag_arena, module_arena);
    };

  {
    let _sff = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, true);
    fixture
      .base
      .base
      .get_frontend()
      .set_luau_solver_mode(SolverMode::Old);
    check_and_examine(&mut fixture, &source, "module", "{|  |}");
    // [TODO] CLI-140762 we shouldn't mutate stale module in autocompleteFragment
    // early return since the following checking will fail, which it shouldn't!
    fragment_ac_and_check(
      &mut fixture,
      &updated1,
      Position {
        line: 1,
        column: 17,
      },
      "module",
    );
    fragment_ac_and_check(
      &mut fixture,
      &updated2,
      Position {
        line: 1,
        column: 18,
      },
      "module",
    );
  }

  {
    let _sff = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);
    fixture
      .base
      .base
      .get_frontend()
      .set_luau_solver_mode(SolverMode::New);
    check_and_examine(&mut fixture, &source, "module", "{  }");
    // [TODO] CLI-140762 we shouldn't mutate stale module in autocompleteFragment
    // early return since the following checking will fail, which it shouldn't!
    fragment_ac_and_check(
      &mut fixture,
      &updated1,
      Position {
        line: 1,
        column: 17,
      },
      "module",
    );
    fragment_ac_and_check(
      &mut fixture,
      &updated2,
      Position {
        line: 1,
        column: 18,
      },
      "module",
    );
  }
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_fragment_autocomplete_handles_parse_errors() {
  let _sfi = ScopedFastInt::new(&fint::LuauParseErrorLimit, 1);
  let source = String::from(
    r#"

"#,
  );
  let updated = String::from(
    r#"
type A = <>random non code text here  @1
"#,
  );

  let mut fixture = FragmentAutocompleteFixture::default();
  fixture.base.autocomplete_fragment_in_both_solvers(
    &source,
    &updated,
    '1',
    |frag: &mut FragmentAutocompleteStatusResult| {
      LUAU_ASSERT!(frag.result.is_some());
      assert!(
        frag
          .result
          .as_ref()
          .unwrap()
          .ac_results
          .entry_map
          .is_empty()
      );
    },
    None,
  );
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_fragment_autocomplete_react_narrow_fragment() {
  let src = String::from(
    "
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
        \treturn nil
        end

        createElement(MyComponent, { })
    ",
  );

  let dest = String::from(
    "
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
        \treturn nil
        end

        createElement(MyComponent, { f@1 })
    ",
  );

  let mut fixture = FragmentAutocompleteFixture::default();
  fixture.base.autocomplete_fragment_in_both_solvers(
    &src,
    &dest,
    '1',
    |ac: &mut FragmentAutocompleteStatusResult| {
      assert!(ac.result.is_some());
      let ac_results = &ac.result.as_ref().unwrap().ac_results;
      assert!(ac_results.entry_map.contains_key("foobar"));
    },
    None,
  );
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_fragment_autocomplete_react_properties() {
  let src = String::from(
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

    "#,
  );

  let dest = String::from(
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
  );

  let mut fixture = FragmentAutocompleteFixture::default();
  fixture.base.autocomplete_fragment_in_both_solvers(
    &src,
    &dest,
    '1',
    |ac: &mut FragmentAutocompleteStatusResult| {
      LUAU_ASSERT!(ac.result.is_some());
      assert!(
        (ac
          .result
          .as_ref()
          .unwrap()
          .ac_results
          .entry_map
          .contains_key("foobar") as usize)
          > 0
      );
    },
    None,
  );

  fixture.base.autocomplete_fragment_in_both_solvers(
    &src,
    &dest,
    '2',
    |ac: &mut FragmentAutocompleteStatusResult| {
      LUAU_ASSERT!(ac.result.is_some());
      assert!(
        (ac
          .result
          .as_ref()
          .unwrap()
          .ac_results
          .entry_map
          .contains_key("bazquxx") as usize)
          > 0
      );
    },
    None,
  );

  fixture.base.autocomplete_fragment_in_both_solvers(
    &src,
    &dest,
    '3',
    |ac: &mut FragmentAutocompleteStatusResult| {
      LUAU_ASSERT!(ac.result.is_some());
      assert!(
        (ac
          .result
          .as_ref()
          .unwrap()
          .ac_results
          .entry_map
          .contains_key("barbaz") as usize)
          > 0
      );
    },
    None,
  );
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_fragment_autocomplete_shouldnt_crash_on_cross_module_mutation() {
  let source = String::from(
    r#"local module = {}
function module.
return module
"#,
  );

  let updated = String::from(
    r#"local module = {}
function module.f@1
return module
"#,
  );

  let mut fixture = FragmentAutocompleteFixture::default();
  fixture.base.autocomplete_fragment_in_both_solvers(
    &source,
    &updated,
    '1',
    |_result: &mut FragmentAutocompleteStatusResult| {},
    None,
  );
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_fragment_autocomplete_string_singleton_intersection_param() {
  let _sff = ScopedFastFlag::new(&fflag::LuauAutocompleteStringSingletonIntersection, true);

  let source = String::from(
    r#"
        local function C(_: "Example"&"Example") end
    "#,
  );

  let dest = String::from(
    r#"
        local function C(_: "Example"&"Example") end
        C(@1
    "#,
  );

  let mut fixture = FragmentAutocompleteFixture::default();
  fixture.base.autocomplete_fragment_in_both_solvers(
    &source,
    &dest,
    '1',
    |frag: &mut FragmentAutocompleteStatusResult| {
      LUAU_ASSERT!(frag.result.is_some());
      assert!(
        frag
          .result
          .as_ref()
          .unwrap()
          .ac_results
          .entry_map
          .contains_key("\"Example\"")
      );
    },
    None,
  );
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_fragment_autocomplete_string_singleton_intersection_variable_annotation() {
  let _sff = ScopedFastFlag::new(&fflag::LuauAutocompleteStringSingletonIntersection, true);

  let source = String::from(
    r#"
        local _: "foo"&"foo"
    "#,
  );
  let dest = String::from(
    r#"
        local _: "foo"&"foo" = "@1"
    "#,
  );

  let mut fixture = FragmentAutocompleteFixture::default();
  fixture.base.autocomplete_fragment_in_both_solvers(
    &source,
    &dest,
    '1',
    |frag: &mut FragmentAutocompleteStatusResult| {
      LUAU_ASSERT!(frag.result.is_some());
      let ac_results = &frag.result.as_ref().unwrap().ac_results;
      assert!(ac_results.entry_map.contains_key("foo"));
      assert_eq!(AutocompleteContext::String, ac_results.context);
    },
    Some(Position {
      line: 1,
      column: 33,
    }),
  );
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_fragment_autocomplete_table_insert() {
  let src = String::from(
    r#"
        local function addToTable(t: {{ foobar: number }})
            table.insert(t, {})
        end
    "#,
  );

  let dest = String::from(
    r#"
        local function addToTable(t: {{ foobar: number }})
            table.insert(t, { f@1 })
        end
    "#,
  );

  let mut fixture = FragmentAutocompleteBuiltinsFixture::default();
  fixture.base.autocomplete_fragment_in_both_solvers(
    &src,
    &dest,
    '1',
    |ac: &mut FragmentAutocompleteStatusResult| {
      LUAU_ASSERT!(ac.result.is_some());
      assert!(
        (ac
          .result
          .as_ref()
          .unwrap()
          .ac_results
          .entry_map
          .contains_key("foobar") as usize)
          > 0
      );
    },
    None,
  );
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_fragment_autocomplete_using_function_call_with_variadic_args() {
  let source = String::from(
    r#"
        local function foo(...: "Val1" | "Val2") end
    "#,
  );

  let dest = String::from(
    r#"
        local function foo(...: "Val1" | "Val2") end
        foo(@1
    "#,
  );

  let mut fixture = FragmentAutocompleteFixture::default();
  fixture.base.autocomplete_fragment_in_both_solvers(
    &source,
    &dest,
    '1',
    |frag: &mut FragmentAutocompleteStatusResult| {
      LUAU_ASSERT!(frag.result.is_some());
      assert!(
        frag
          .result
          .as_ref()
          .unwrap()
          .ac_results
          .entry_map
          .contains_key("\"Val1\"") as usize
          == 1
      );
      assert!(
        frag
          .result
          .as_ref()
          .unwrap()
          .ac_results
          .entry_map
          .contains_key("\"Val2\"") as usize
          == 1
      );
    },
    None,
  );
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_fragment_autocomplete_using_indexer_with_singleton_keys() {
  let source = String::from(
    r#"
        type List = "Val1" | "Val2" | "Val3"
        local Table: { [List]: boolean }
    "#,
  );

  let dest = String::from(
    r#"
        type List = "Val1" | "Val2" | "Val3"
        local Table: { [List]: boolean }
        local _ = Table.@1
    "#,
  );

  let mut fixture = FragmentAutocompleteFixture::default();
  fixture.base.autocomplete_fragment_in_both_solvers(
    &source,
    &dest,
    '1',
    |frag: &mut FragmentAutocompleteStatusResult| {
      LUAU_ASSERT!(frag.result.is_some());
      let ac = &frag.result.as_ref().unwrap().ac_results;
      assert!(ac.entry_map.contains_key("Val1"));
      assert!(ac.entry_map.contains_key("Val2"));
      assert!(ac.entry_map.contains_key("Val3"));
    },
    None,
  );
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_free_type_in_old_solver_shouldnt_trigger_not_null_assertion() {
  let source = String::from(
    r#"--!strict
local foo
local a, z = foo()

local e = foo().x

local f = foo().y

z
"#,
  );

  let dest = String::from(
    r#"--!strict
local foo
local a, z = foo()

local e = foo().x

local f = foo().y

z:a@1
"#,
  );

  let mut fixture = FragmentAutocompleteBuiltinsFixture::default();
  fixture.base.autocomplete_fragment_in_both_solvers(
    &source,
    &dest,
    '1',
    |_: &mut FragmentAutocompleteStatusResult| {},
    None,
  );
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_function_parameter_not_recommending_out_of_scope_argument() {
  let source = String::from(
    r#"
--!strict
local function foo(abd: FakeVec)
end
local function bar(abc : FakeVec)
   a@1
end
"#,
  );

  let mut fixture = FragmentAutocompleteBuiltinsFixture::default();
  fixture.base.autocomplete_fragment_in_both_solvers(
    &source,
    &source,
    '1',
    |frag: &mut FragmentAutocompleteStatusResult| {
      LUAU_ASSERT!(frag.result.is_some());
      assert!(
        frag
          .result
          .as_ref()
          .unwrap()
          .ac_results
          .entry_map
          .contains_key("abc")
      );
      assert!(
        !frag
          .result
          .as_ref()
          .unwrap()
          .ac_results
          .entry_map
          .contains_key("abd")
      );
    },
    None,
  );
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_function_parameters() {
  let source = String::from(
    r#"
        function abc(test)

@1        end
    "#,
  );

  let mut fixture = FragmentAutocompleteFixture::default();
  fixture.base.autocomplete_fragment_in_both_solvers(
    &source,
    &source,
    '1',
    |frag: &mut FragmentAutocompleteStatusResult| {
      LUAU_ASSERT!(frag.result.is_some());
      let ac = &frag.result.as_ref().unwrap().ac_results;
      assert!(ac.entry_map.contains_key("test"));
    },
    None,
  );
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_generalization_crash_when_old_solver_freetypes_have_no_bounds_set() {
  let source = String::from(
    r#"
local UserInputService = game:GetService("UserInputService");

local Camera = workspace.CurrentCamera;

UserInputService.InputBegan:Connect(function(Input)
    if (Input.KeyCode == Enum.KeyCode.One) then
        local Up = Input.Foo
        local Vector = -(Up:Unit)
    end
end)
"#,
  );

  let dest = String::from(
    r#"
local UserInputService = game:GetService("UserInputService");

local Camera = workspace.CurrentCamera;

UserInputService.InputBegan:Connect(function(Input)
    if (Input.KeyCode == Enum.KeyCode.One) then
        local Up = Input.Foo
        local Vector = -(Up:Unit()) @1
    end
end)
"#,
  );

  let mut fixture = FragmentAutocompleteBuiltinsFixture::default();
  fixture.base.autocomplete_fragment_in_both_solvers(
    &source,
    &dest,
    '1',
    |_frag: &mut FragmentAutocompleteStatusResult| {},
    None,
  );
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_get_suggestions_for_the_very_start_of_the_script() {
  let source = String::from(
    r#"@1

        function aaa() end
    "#,
  );

  let mut fixture = FragmentAutocompleteFixture::default();
  fixture.base.autocomplete_fragment_in_both_solvers(
    &source,
    &source,
    '1',
    |frag: &mut FragmentAutocompleteStatusResult| {
      LUAU_ASSERT!(frag.result.is_some());
      let ac = &frag.result.as_ref().unwrap().ac_results;
      assert!(ac.entry_map.contains_key("table"));
      assert_eq!(ac.context, AutocompleteContext::Statement);
    },
    None,
  );
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_global_functions_are_not_scoped_lexically() {
  let source = String::from(
    r#"
        if true then
            function abc()

            end
        end
@1      "#,
  );

  let mut fixture = FragmentAutocompleteFixture::default();
  fixture.base.autocomplete_fragment_in_both_solvers(
    &source,
    &source,
    '1',
    |frag: &mut FragmentAutocompleteStatusResult| {
      assert!(frag.result.is_some());
      let ac = &frag.result.as_ref().unwrap().ac_results;
      assert!(!ac.entry_map.is_empty());
      assert!(ac.entry_map.contains_key("abc"));
      assert!(ac.entry_map.contains_key("table"));
      assert!(ac.entry_map.contains_key("math"));
    },
    None,
  );
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_hot_comment_should_rec() {
  let source = String::from(r#"--!@1"#);

  let mut fixture = FragmentAutocompleteBuiltinsFixture::default();
  fixture.base.autocomplete_fragment_in_both_solvers(
    &source,
    &source,
    '1',
    |result: &mut FragmentAutocompleteStatusResult| {
      let ac = &result.result.as_ref().unwrap().ac_results;
      assert!(!ac.entry_map.is_empty());
      assert!(ac.entry_map.contains_key("strict"));
      assert!(ac.entry_map.contains_key("nonstrict"));
      assert!(ac.entry_map.contains_key("nocheck"));
      assert!(ac.entry_map.contains_key("native"));
      assert!(ac.entry_map.contains_key("nolint"));
      assert!(ac.entry_map.contains_key("optimize"));
    },
    None,
  );
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_ice_caused_by_mixed_mode_use() {
  // C++ builds this source by concatenating string literals with explicit
  // escape sequences (`\n`, `\t`). Transcribed below with a literal tab and
  // newlines so the bytes match exactly.
  let source = String::from(
    "--[[\n\tPackage link auto-generated by Rotriever\n]]\nlocal PackageIndex = script.Parent._Index\n\nlocal Package = ",
  ) + "require(PackageIndex[\"ReactOtter\"][\"ReactOtter\"])\n\nexport type Goal = Package.Goal\nexport type SpringOptions "
    + "= Package.SpringOptions\n\n\nreturn Pa@1";

  let mut fixture = FragmentAutocompleteBuiltinsFixture::default();
  fixture.base.autocomplete_fragment_in_both_solvers(
    &source,
    &source,
    '1',
    |_: &mut FragmentAutocompleteStatusResult| {},
    None,
  );
  fixture.base.autocomplete_fragment_in_both_solvers(
    &source,
    &source,
    '1',
    |_: &mut FragmentAutocompleteStatusResult| {},
    None,
  );
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_if_complete_inside_scope_line() {
  let (_fixture, region) = fx_region!(
    r#"
if true then
    local x =
end

"#,
    Position {
      line: 2,
      column: 13,
    }
  );

  assert_eq!(
    Location {
      begin: Position { line: 2, column: 4 },
      end: Position {
        line: 2,
        column: 13
      },
    },
    region.fragment_location
  );
  assert!(!region.parent_block.is_null());
  assert!(crate::is_nearest::<AstStatLocal>(region.nearest_statement));
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_if_cond_no_then_recs_then() {
  let source = String::from(
    r#"

    "#,
  );

  let dest = String::from(
    r#"
if x t@1
    "#,
  );

  let mut fixture = FragmentAutocompleteFixture::default();
  fixture.base.autocomplete_fragment_in_both_solvers(
    &source,
    &dest,
    '1',
    |result: &mut FragmentAutocompleteStatusResult| {
      LUAU_ASSERT!(result.result.is_some());
      let ac_results = &result.result.as_ref().unwrap().ac_results;
      assert!(ac_results.entry_map.contains_key("then"));
    },
    None,
  );
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_if_else_if() {
  let (_fixture, region) = fx_region!(
    r#"
if true then
elseif
end

"#,
    Position { line: 2, column: 8 }
  );

  assert_eq!(
    Location {
      begin: Position { line: 2, column: 8 },
      end: Position { line: 2, column: 8 },
    },
    region.fragment_location
  );
  assert!(!region.parent_block.is_null());
  assert!(crate::is_nearest::<AstStatIf>(region.nearest_statement));
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_if_else_if_after_then() {
  let (_fixture, region) = fx_region!(
    r#"
if true then
elseif false then
end

"#,
    Position {
      line: 2,
      column: 17,
    }
  );

  assert_eq!(
    Location {
      begin: Position {
        line: 2,
        column: 17
      },
      end: Position {
        line: 2,
        column: 17
      },
    },
    region.fragment_location
  );
  assert!(!region.parent_block.is_null());
  assert!(crate::is_nearest::<AstStatIf>(region.nearest_statement));
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_if_else_if_after_then_new_line() {
  let (_fixture, region) = fx_region!(
    r#"
if true then
elseif false then

end

"#,
    Position { line: 3, column: 0 }
  );

  assert_eq!(
    Location {
      begin: Position { line: 3, column: 0 },
      end: Position { line: 3, column: 0 },
    },
    region.fragment_location
  );
  assert!(!region.parent_block.is_null());
  assert!(crate::is_nearest::<AstStatIf>(region.nearest_statement));
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_if_else_if_no_end() {
  let (_fixture, region) = fx_region!(
    r#"
if true then
elseif
"#,
    Position { line: 2, column: 8 }
  );

  assert_eq!(
    Location {
      begin: Position { line: 2, column: 8 },
      end: Position { line: 2, column: 8 },
    },
    region.fragment_location
  );
  assert!(!region.parent_block.is_null());
  assert!(crate::is_nearest::<AstStatIf>(region.nearest_statement));
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_if_else_if_table_prop_recs_no_then() {
  let source = String::from(
    r#"
type T = {xa : number, y : number}
local t : T = {xa = 3, y = 3}

if t.x then
elseif
end
"#,
  );
  let dest = String::from(
    r#"
type T = {xa : number, y : number}
local t : T = {xa = 3, y = 3}

if t.x then
elseif t.xa t@1
end
    "#,
  );

  let mut fixture = FragmentAutocompleteFixture::default();
  fixture.base.autocomplete_fragment_in_both_solvers(
    &source,
    &dest,
    '1',
    |result: &mut FragmentAutocompleteStatusResult| {
      LUAU_ASSERT!(result.result.is_some());
      let ac_results = &result.result.as_ref().unwrap().ac_results;
      assert!(!ac_results.entry_map.is_empty());
      assert!(!ac_results.entry_map.contains_key("xa"));
      assert!(!ac_results.entry_map.contains_key("y"));
      assert!(ac_results.entry_map.contains_key("then"));
    },
    None,
  );
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_if_else_if_table_prop_recs_with_then() {
  let source = String::from(
    r#"
type T = {xa : number, y : number}
local t : T = {xa = 3, y = 3}

if t.x then
elseif  then
end
"#,
  );

  let dest = String::from(
    r#"
type T = {xa : number, y : number}
local t : T = {xa = 3, y = 3}

if t.x then
elseif t.@1  then
end
"#,
  );

  let mut fixture = FragmentAutocompleteFixture::default();
  fixture.base.autocomplete_fragment_in_both_solvers(
    &source,
    &dest,
    '1',
    |result: &mut FragmentAutocompleteStatusResult| {
      LUAU_ASSERT!(result.result.is_some());
      let ac = &result.result.as_ref().unwrap().ac_results;
      assert!(!ac.entry_map.is_empty());
      assert!(ac.entry_map.contains_key("xa"));
      assert!(ac.entry_map.contains_key("y"));
      assert!(!ac.entry_map.contains_key("then"));
    },
    None,
  );
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_if_partial() {
  let (_fixture, region) = fx_region!(
    r#"
if"#,
    Position { line: 1, column: 2 }
  );

  assert_eq!(
    Location {
      begin: Position { line: 1, column: 2 },
      end: Position { line: 1, column: 2 },
    },
    region.fragment_location
  );
  assert!(!region.parent_block.is_null());
  assert!(crate::is_nearest::<AstStatIf>(region.nearest_statement));
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_if_partial_after_condition() {
  let (_fixture, region) = fx_region!(
    r#"
if true then
"#,
    Position {
      line: 1,
      column: 12,
    }
  );

  assert_eq!(
    Location {
      begin: Position {
        line: 1,
        column: 12
      },
      end: Position {
        line: 1,
        column: 12
      },
    },
    region.fragment_location
  );
  assert!(!region.parent_block.is_null());
  assert!(crate::is_nearest::<AstStatIf>(region.nearest_statement));
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_if_partial_in_condition_after() {
  let (_fixture, region) = fx_region!(
    r#"
if true
"#,
    Position { line: 1, column: 8 }
  );

  assert_eq!(
    Location {
      begin: Position { line: 1, column: 3 },
      end: Position { line: 1, column: 8 },
    },
    region.fragment_location
  );
  assert!(!region.parent_block.is_null());
  assert!(crate::is_nearest::<AstStatIf>(region.nearest_statement));
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_if_partial_in_condition_at() {
  let (_fixture, region) = fx_region!(
    r#"
if true
"#,
    Position { line: 1, column: 7 }
  );

  assert_eq!(
    Location {
      begin: Position { line: 1, column: 3 },
      end: Position { line: 1, column: 7 },
    },
    region.fragment_location
  );
  assert!(!region.parent_block.is_null());
  assert!(crate::is_nearest::<AstStatIf>(region.nearest_statement));
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_if_partial_new_line() {
  let (_fixture, region) = fx_region!(
    r#"
if true then

"#,
    Position { line: 2, column: 0 }
  );

  assert_eq!(
    Location {
      begin: Position { line: 2, column: 0 },
      end: Position { line: 2, column: 0 },
    },
    region.fragment_location
  );
  assert!(!region.parent_block.is_null());
  assert!(crate::is_nearest::<AstStatIf>(region.nearest_statement));
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_if_then_recs_else() {
  let source = String::from(
    r#"
if x then

end
    "#,
  );

  let dest = String::from(
    r#"
if x then
e@1
end
    "#,
  );

  let mut fixture = FragmentAutocompleteFixture::default();
  fixture.base.autocomplete_fragment_in_both_solvers(
    &source,
    &dest,
    '1',
    |result: &mut FragmentAutocompleteStatusResult| {
      LUAU_ASSERT!(result.result.is_some());
      let ac = &result.result.as_ref().unwrap().ac_results;
      assert!(ac.entry_map.contains_key("else"));
      assert!(ac.entry_map.contains_key("elseif"));
    },
    None,
  );
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_in_place_edit_of_for_loop_before_in_keyword_returns_fragment_starting_from_for()
 {
  let source = String::from(
    r#"
local x = {}
for i, value in x do
    print(i)
end
"#,
  );

  let dest = String::from(
    r#"
local x = {}
for @1, value in x do
    print(i)
end
"#,
  );

  let mut fixture = FragmentAutocompleteBuiltinsFixture::default();
  fixture.base.autocomplete_fragment_in_both_solvers(
    &source,
    &dest,
    '1',
    |result: &mut FragmentAutocompleteStatusResult| {
      let ac = &result.result.as_ref().unwrap().ac_results;
      assert!(!ac.entry_map.is_empty());
    },
    None,
  );
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_inline_autocomplete_picks_the_right_scope_1() {
  use ulua_analysis::{
    functions::{follow_type, get_type},
    records::table_type::TableType,
  };

  let source = String::from(
    r#"
type Table = { a: number, b: number }
do
    type Table = { x: string, y: string }
end
"#,
  );

  let updated = String::from(
    r#"
type Table = { a: number, b: number }
do
    type Table = { x: string, y: string }
    local a : T@1
end
"#,
  );

  let mut fixture = FragmentAutocompleteFixture::default();
  fixture.base.autocomplete_fragment_in_both_solvers(
    &source,
    &updated,
    '1',
    |fragment: &mut FragmentAutocompleteStatusResult| {
      LUAU_ASSERT!(fragment.result.is_some());
      let result = fragment.result.as_ref().unwrap();
      LUAU_ASSERT!(!result.fresh_scope.is_null());
      LUAU_ASSERT!(result.ac_results.entry_map.contains_key("Table"));
      LUAU_ASSERT!(result.ac_results.entry_map["Table"].r#type.is_some());
      let ty = follow_type::follow(result.ac_results.entry_map["Table"].r#type.unwrap());
      let tv = get_type::get::<TableType>(ty).expect("Table should be TableType");
      assert!(tv.props.contains_key("x"));
    },
    None,
  );
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_inline_autocomplete_picks_the_right_scope_2() {
  use ulua_analysis::{
    functions::{follow_type, get_type},
    records::table_type::TableType,
  };

  let source = String::from(
    r#"
type Table = { a: number, b: number }
do
    type Table = { x: string, y: string }
end
"#,
  );

  let updated = String::from(
    r#"
type Table = { a: number, b: number }
do
    type Table = { x: string, y: string }
end
local a : T@1
"#,
  );

  let mut fixture = FragmentAutocompleteFixture::default();
  fixture.base.autocomplete_fragment_in_both_solvers(
    &source,
    &updated,
    '1',
    |fragment: &mut FragmentAutocompleteStatusResult| {
      assert!(fragment.result.is_some());
      let result = fragment.result.as_ref().unwrap();
      LUAU_ASSERT!(!result.fresh_scope.is_null());
      assert!(result.ac_results.entry_map.contains_key("Table"));
      assert!(result.ac_results.entry_map["Table"].r#type.is_some());
      let ty = follow_type::follow(result.ac_results.entry_map["Table"].r#type.unwrap());
      let tv = get_type::get::<TableType>(ty).expect("Table should be TableType");
      assert!(tv.props.contains_key("a"));
      assert!(tv.props.contains_key("b"));
    },
    None,
  );
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_inline_prop_read_on_requires_provides_results() {
  use ulua_unit_test::functions::get_options::get_options;

  let module_a = String::from(
    r#"
local mod = { prop1 = true}
mod.prop2 = "a"
function mod.foo(a: number)
    return a
end
return mod
"#,
  );

  let main_module = String::from(
    r#"

"#,
  );

  let mut fixture = FragmentAutocompleteBuiltinsFixture::default();
  fixture
    .base
    .base
    .base
    .file_resolver
    .source
    .insert(String::from("MainModule"), main_module);
  fixture
    .base
    .base
    .base
    .file_resolver
    .source
    .insert(String::from("MainModule/A"), module_a);
  fixture
    .get_frontend()
    .check_module_name_optional_frontend_options(
      &ModuleName::from("MainModule/A"),
      Some(get_options()),
    );
  fixture
    .get_frontend()
    .check_module_name_optional_frontend_options(
      &ModuleName::from("MainModule"),
      Some(get_options()),
    );

  let updated_main = String::from(
    r#"
require(script.A).
"#,
  );

  let result = fixture.base.autocomplete_fragment(
    &updated_main,
    Position {
      line: 1,
      column: 18,
    },
    None,
  );
  let ac_results = &result.result.as_ref().unwrap().ac_results;
  assert!(!ac_results.entry_map.is_empty());
  assert!(ac_results.entry_map.contains_key("prop1"));
  assert!(ac_results.entry_map.contains_key("prop2"));
  assert!(ac_results.entry_map.contains_key("foo"));
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_inside_do() {
  let (_fixture, region) = fx_region!(
    r#"
local x = 4
do

end
"#,
    Position { line: 3, column: 3 }
  );

  assert_eq!(
    Location {
      begin: Position { line: 3, column: 3 },
      end: Position { line: 3, column: 3 },
    },
    region.fragment_location
  );
  assert!(!region.parent_block.is_null());
  assert!(crate::is_nearest::<AstStatBlock>(region.nearest_statement));
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_inside_incomplete_do() {
  let (_fixture, region) = fx_region!(
    r#"
local x = 4
do
"#,
    Position { line: 2, column: 2 }
  );

  assert_eq!(
    Location {
      begin: Position { line: 2, column: 2 },
      end: Position { line: 2, column: 2 },
    },
    region.fragment_location
  );
  assert!(!region.parent_block.is_null());
  assert!(crate::is_nearest::<AstStatBlock>(region.nearest_statement));
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_interior_free_types_assertion_caused_by_free_type_inheriting_null_scope_from_table()
 {
  let source = String::from(
    r#"--!strict
local foo
local a = foo()

local e = foo().x

local f = foo().y


"#,
  );

  let dest = String::from(
    r#"--!strict
local foo
local a = foo()

local e = foo().x

local f = foo().y

z = a.P.E@1
"#,
  );

  let mut fixture = FragmentAutocompleteBuiltinsFixture::default();
  fixture.base.autocomplete_fragment_in_both_solvers(
    &source,
    &dest,
    '1',
    |_: &mut FragmentAutocompleteStatusResult| {},
    None,
  );
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_isinstance_refines_for_autocomplete() {
  let _sff0 = ScopedFastFlag::new(&fflag::DebugLuauUserDefinedClasses, true);
  let _sff1 = ScopedFastFlag::new(&fflag::LuauAllowGlobalDeclarationToBeCalledClass, true);

  let source = String::from(
    r#"
class Point
    public x
    public y
end

local function f(v: Point | string)
    if class.isinstance(v, Point) then

    end
end
"#,
  );

  let dest = String::from(
    r#"
class Point
    public x
    public y
end

local function f(v: Point | string)
    if class.isinstance(v, Point) then
        v.@1
    end
end
"#,
  );

  let mut fixture = FragmentAutocompleteBuiltinsFixture::default();
  fixture.base.autocomplete_fragment_in_new_solver(
    &source,
    &dest,
    '1',
    |frag: &mut FragmentAutocompleteStatusResult| {
      LUAU_ASSERT!(frag.result.is_some());
      let ac = &frag.result.as_ref().unwrap().ac_results;
      assert!(ac.entry_map.contains_key("x"));
      assert!(ac.entry_map.contains_key("y"));
    },
    None,
  );
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_just_two_locals() {
  let (_fixture, region) = fx_region!(
    r#"
local x = 4
local y = 5
"#,
    Position {
      line: 2,
      column: 11,
    }
  );

  assert_eq!(
    Location {
      begin: Position { line: 2, column: 0 },
      end: Position {
        line: 2,
        column: 11
      },
    },
    region.fragment_location
  );
  assert!(!region.parent_block.is_null());
  assert!(!region.nearest_statement.is_null());
  assert!(crate::is_nearest::<AstStatLocal>(region.nearest_statement));
}

mod fragment_autocomplete_just_two_locals_fragment_autocomplete_test_case_2 {
  //! Source: `tests/FragmentAutocomplete.test.cpp`
  use super::*;

  #[test]
  fn fragment_autocomplete_just_two_locals() {
    let mut fixture = FragmentAutocompleteFixture::default();
    let result = fixture.base.run_autocomplete_visitor(
      r#"
local x = 4
local y = 5
"#,
      &Position {
        line: 2,
        column: 11,
      },
    );

    assert_eq!(3, result.ancestry.len());
    assert_eq!(1, result.local_stack.len());
    assert_eq!(result.local_map.size(), result.local_stack.len());
    assert!(!result.nearest_statement.is_null());

    // 门面一步下转+判型（原 `ast_node_as + is_null` 两步）。
    // Safety: nearest_statement 为夹具 arena 存活语句指针，只读。
    let local = (result.nearest_statement)
      .as_node::<AstStatLocal>()
      .expect("nearest_statement 应为 local 声明");
    assert_eq!(1, local.vars.size);
    let var =
      PtrRef::as_ref_opt(&local.vars.as_slice()[0]).expect("vars 元素是 arena 写入的存活节点指针");
    assert_eq!("y", var.name.as_str().unwrap());
  }
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_leave_numbers_alone() {
  let source = String::from("local a = 3.@1");

  let mut fixture = FragmentAutocompleteFixture::default();
  fixture.base.autocomplete_fragment_in_both_solvers(
    &source,
    &source,
    '1',
    |frag: &mut FragmentAutocompleteStatusResult| {
      LUAU_ASSERT!(frag.result.is_some());
      let ac = &frag.result.as_ref().unwrap().ac_results;
      assert!(ac.entry_map.is_empty());
      assert_eq!(ac.context, AutocompleteContext::Unknown);
    },
    None,
  );
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_len_operator_needs_to_provide_autocomplete_results() {
  let source = String::from(
    r#"
type Pool = { numbers: { number }}

local function foobar(p)
    local pool = p :: Pool
    if #pool
end
"#,
  );
  let dest = String::from(
    r#"
type Pool = { numbers: { number }}

local function foobar(p)
    local pool = p :: Pool
    if #pool.@1
end
"#,
  );

  let mut fixture = FragmentAutocompleteBuiltinsFixture::default();
  fixture.base.autocomplete_fragment_in_both_solvers(
    &source,
    &dest,
    '1',
    |result: &mut FragmentAutocompleteStatusResult| {
      let ac_results = &result.result.as_ref().unwrap().ac_results;
      assert!(!ac_results.entry_map.is_empty());
      assert!(ac_results.entry_map.contains_key("numbers"));
    },
    None,
  );
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_local_funcs_show_up_in_local_stack() {
  use ulua_ast::records::ast_stat_return::AstStatReturn;
  let mut fixture = FragmentAutocompleteFixture::default();
  let result = fixture.base.run_autocomplete_visitor(
    r#"
local function foo() return 4 end
local x = foo()
local function bar() return x + foo() end
"#,
    &Position {
      line: 3,
      column: 32,
    },
  );

  assert_eq!(8, result.ancestry.len());
  assert_eq!(3, result.local_stack.len());
  assert_eq!(result.local_map.size(), result.local_stack.len());
  let last = *result.local_stack.last().unwrap();
  // (下方 unsafe: local_stack 元素是 arena 写入的存活 Binding 指针，只读取名)
  assert_eq!("bar", last.as_ref_opt().unwrap().name.as_str().unwrap());
  assert!(crate::is_nearest::<AstStatReturn>(result.nearest_statement));
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_local_functions_fall_out_of_scope() {
  let source = String::from(
    r#"
        if true then
            local function abc()

            end
        end
@1      "#,
  );

  let mut fixture = FragmentAutocompleteFixture::default();
  fixture.base.autocomplete_fragment_in_both_solvers(
    &source,
    &source,
    '1',
    |frag: &mut FragmentAutocompleteStatusResult| {
      LUAU_ASSERT!(frag.result.is_some());
      let ac = &frag.result.as_ref().unwrap().ac_results;
      assert_ne!(0, ac.entry_map.len());
      assert!(!ac.entry_map.contains_key("abc"));
    },
    None,
  );
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_local_initializer() {
  let _sff = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);
  let mut fixture = FragmentAutocompleteFixture::default();
  fixture.base.check_with_options("local a =");
  let fragment = fixture
    .base
    .parse_fragment("local a =", &Position { line: 0, column: 9 }, None)
    .expect("expected fragment parse result");

  assert_eq!("local a =", fragment.fragment_to_parse);
  assert_eq!(
    Location {
      begin: Position { line: 0, column: 0 },
      end: Position { line: 0, column: 9 },
    },
    fragment.root.as_ref_opt().unwrap().base.base.location
  );
}

mod fragment_autocomplete_local_initializer_fragment_autocomplete_test_case_2 {
  //! Source: `tests/FragmentAutocomplete.test.cpp`
  use super::*;

  #[test]
  fn fragment_autocomplete_local_initializer() {
    let source = String::from("local a =@1");
    let mut fixture = FragmentAutocompleteFixture::default();
    fixture.base.autocomplete_fragment_in_both_solvers(
      &source,
      &source,
      '1',
      |frag: &mut FragmentAutocompleteStatusResult| {
        assert!(frag.result.is_some());
        let ac = frag.result.as_ref().unwrap().ac_results.clone();

        assert!(ac.entry_map.contains_key("table"));
        assert!(ac.entry_map.contains_key("math"));
        assert_eq!(ac.context, AutocompleteContext::Expression);
      },
      None,
    );
  }
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_method_call_inside_function_body() {
  let source = String::from(
    r#"
        local game = { GetService=function(s) return 'hello' end }

        function a()

        end
    "#,
  );

  let updated = String::from(
    r#"
        local game = { GetService=function(s) return 'hello' end }

        function a()
            game:@1
        end
    "#,
  );

  let mut fixture = FragmentAutocompleteFixture::default();
  fixture.base.autocomplete_fragment_in_both_solvers(
    &source,
    &updated,
    '1',
    |frag: &mut FragmentAutocompleteStatusResult| {
      LUAU_ASSERT!(frag.result.is_some());
      let ac = &frag.result.as_ref().unwrap().ac_results;
      assert_ne!(0, ac.entry_map.len());

      assert!(!ac.entry_map.contains_key("math"));
      assert_eq!(ac.context, AutocompleteContext::Property);
    },
    None,
  );
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_method_in_unfinished_repeat_body_eof() {
  let source = String::from(
    r#"
local t = {}
function t:Foo() end
repeat"#,
  );

  let dest = String::from(
    r#"
local t = {}
function t:Foo() end
repeat
t:@1"#,
  );

  let mut fixture = FragmentAutocompleteBuiltinsFixture::default();
  fixture.base.autocomplete_fragment_in_both_solvers(
    &source,
    &dest,
    '1',
    |result: &mut FragmentAutocompleteStatusResult| {
      let ac_results = &result.result.as_ref().unwrap().ac_results;
      assert!(!ac_results.entry_map.is_empty());
      assert!(ac_results.entry_map.contains_key("Foo"));
    },
    None,
  );
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_method_in_unfinished_repeat_body_not_eof() {
  let source = String::from(
    r#"
local t = {}
function t:Foo() end
repeat
t

local function whatever() end
"#,
  );

  let dest = String::from(
    r#"
local t = {}
function t:Foo() end
repeat
t:@1

local function whatever() end
"#,
  );

  let mut fixture = FragmentAutocompleteBuiltinsFixture::default();
  fixture.base.autocomplete_fragment_in_both_solvers(
    &source,
    &dest,
    '1',
    |result: &mut FragmentAutocompleteStatusResult| {
      let ac = &result.result.as_ref().unwrap().ac_results;
      assert!(!ac.entry_map.is_empty());
      assert!(ac.entry_map.contains_key("Foo"));
    },
    None,
  );
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_midway_multiline_call() {
  let (_fixture, region) = fx_region!(
    r#"
abc(
"foo"
)
"#,
    Position { line: 2, column: 4 }
  );

  assert_eq!(
    Location {
      begin: Position { line: 1, column: 0 },
      end: Position { line: 2, column: 4 },
    },
    region.fragment_location
  );
  assert!(!region.parent_block.is_null());
  assert!(!region.nearest_statement.is_null());
  assert!(crate::is_nearest::<AstStatExpr>(region.nearest_statement));
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_midway_through_call() {
  let (_fixture, region) = fx_region!(
    r#"
abc("foo")
"#,
    Position { line: 1, column: 6 }
  );

  assert_eq!(
    Location {
      begin: Position { line: 1, column: 0 },
      end: Position { line: 1, column: 6 },
    },
    region.fragment_location
  );
  assert!(!region.parent_block.is_null());
  assert!(!region.nearest_statement.is_null());
  assert!(crate::is_nearest::<AstStatExpr>(region.nearest_statement));
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_mixed_mode_basic_example_append() {
  let _sff = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, true);
  let mut fixture = FragmentAutocompleteFixture::default();
  fixture
    .base
    .base
    .get_frontend()
    .set_luau_solver_mode(if !fflag::DebugLuauForceOldSolver.get() {
      SolverMode::New
    } else {
      SolverMode::Old
    });
  let res = fixture.base.check_old_solver(
    r#"
local x = 4
local y = 5
"#,
  );

  assert_eq!(0, res.errors.len(), "{:?}", res.errors);

  let fragment = fixture.base.check_fragment(
    r#"
local x = 4
local y = 5
local z = x + y
"#,
    Position {
      line: 3,
      column: 15,
    },
    None,
  );

  let opt = linear_search_for_binding(&fragment.fresh_scope, "z");
  LUAU_ASSERT!(opt.is_some());
  assert_eq!("number", to_string_type_id(opt.unwrap()));
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_mixed_mode_basic_example_inlined() {
  let _sff = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, true);
  let mut fixture = FragmentAutocompleteFixture::default();
  fixture
    .base
    .base
    .get_frontend()
    .set_luau_solver_mode(if !fflag::DebugLuauForceOldSolver.get() {
      SolverMode::New
    } else {
      SolverMode::Old
    });
  let _res = fixture.base.check_old_solver(
    r#"
local x = 4
local y = 5
"#,
  );

  let fragment = fixture.base.check_fragment(
    r#"
local x = 4
local z = x
local y = 5
"#,
    Position {
      line: 2,
      column: 11,
    },
    None,
  );

  let correct = linear_search_for_binding(&fragment.fresh_scope, "z");
  LUAU_ASSERT!(correct.is_some());
  assert_eq!("number", to_string_type_id(correct.unwrap()));
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_mixed_mode_can_autocomplete_simple_property_access() {
  let _sff = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, true);
  let mut fixture = FragmentAutocompleteFixture::default();
  fixture
    .base
    .base
    .get_frontend()
    .set_luau_solver_mode(if !fflag::DebugLuauForceOldSolver.get() {
      SolverMode::New
    } else {
      SolverMode::Old
    });
  let res = fixture.base.check_old_solver(
    r#"
local tbl = { abc = 1234}
"#,
  );

  assert_eq!(0, res.errors.len(), "{:?}", res.errors);

  let fragment = fixture.base.autocomplete_fragment(
    r#"
local tbl = { abc = 1234}
tbl.
"#,
    Position { line: 2, column: 5 },
    None,
  );
  LUAU_ASSERT!(fragment.result.is_some());
  let result = fragment.result.as_ref().unwrap();
  LUAU_ASSERT!(!result.fresh_scope.is_null());

  assert_eq!(1, result.ac_results.entry_map.len());
  assert!(result.ac_results.entry_map.contains_key("abc"));
  assert_eq!(AutocompleteContext::Property, result.ac_results.context);
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_multiple_fragment_autocomplete() {
  use ulua_analysis::{
    functions::to_string_to_string::to_string_type_id_to_string_options_mut,
    records::to_string_options::ToStringOptions,
    type_aliases::{module_ptr_module::ModulePtr, type_id::TypeId},
  };
  use ulua_unit_test::functions::lookup_name::lookup_name;

  let opt = ToStringOptions {
    exhaustive: true,
    function_type_arguments: true,
    max_table_length: 0,
    max_type_length: 0,
    ..Default::default()
  };

  fn get_type_from_module(module: &ModulePtr, name: &str) -> Option<TypeId> {
    if !module.has_module_scope() {
      return None;
    }
    let scope = module.get_module_scope();
    lookup_name(&scope, name)
  }

  let source = String::from(
    r#"local module = {}
f
return module"#,
  );

  let updated1 = String::from(
    r#"local module = {}
function module.a
return module"#,
  );

  let updated2 = String::from(
    r#"local module = {}
function module.ab
return module"#,
  );

  let mut fixture = FragmentAutocompleteFixture::default();

  {
    let _sff = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, true);
    fixture
      .base
      .base
      .get_frontend()
      .set_luau_solver_mode(SolverMode::Old);

    // checkAndExamine(source, "module", "{|  |}")
    fixture.base.check_with_options(&source);
    let id = fixture.base.base.base.get_type("module", true);
    LUAU_ASSERT!(id.is_some());
    assert_eq!(
      to_string_type_id_to_string_options_mut(id.unwrap(), opt.clone()),
      String::from("{|  |}")
    );

    // fragmentACAndCheck(updated1, Position{1, 17}, "module", "{|  |}", "{| a: (%error-id%: unknown) -> () |}")
    {
      let frag = fixture.base.autocomplete_fragment(
        &updated1,
        Position {
          line: 1,
          column: 17,
        },
        None,
      );
      LUAU_ASSERT!(frag.result.is_some());
      let frag_id =
        get_type_from_module(&frag.result.as_ref().unwrap().incremental_module, "module");
      LUAU_ASSERT!(frag_id.is_some());
      assert_eq!(
        to_string_type_id_to_string_options_mut(frag_id.unwrap(), opt.clone()),
        String::from("{| a: (%error-id%: unknown) -> () |}")
      );

      let src_id = fixture.base.base.base.get_type("module", true);
      LUAU_ASSERT!(src_id.is_some());
      assert_eq!(
        to_string_type_id_to_string_options_mut(src_id.unwrap(), opt.clone()),
        String::from("{|  |}")
      );
    }

    // fragmentACAndCheck(updated2, Position{1, 18}, "module", "{|  |}", "{| ab: (%error-id%: unknown) -> () |}")
    {
      let frag = fixture.base.autocomplete_fragment(
        &updated2,
        Position {
          line: 1,
          column: 18,
        },
        None,
      );
      LUAU_ASSERT!(frag.result.is_some());
      let frag_id =
        get_type_from_module(&frag.result.as_ref().unwrap().incremental_module, "module");
      LUAU_ASSERT!(frag_id.is_some());
      assert_eq!(
        to_string_type_id_to_string_options_mut(frag_id.unwrap(), opt.clone()),
        String::from("{| ab: (%error-id%: unknown) -> () |}")
      );

      let src_id = fixture.base.base.base.get_type("module", true);
      LUAU_ASSERT!(src_id.is_some());
      assert_eq!(
        to_string_type_id_to_string_options_mut(src_id.unwrap(), opt.clone()),
        String::from("{|  |}")
      );
    }
  }
  {
    let _sff = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);
    fixture
      .base
      .base
      .get_frontend()
      .set_luau_solver_mode(SolverMode::New);

    // checkAndExamine(source, "module", "{  }")
    fixture.base.check_with_options(&source);
    let id = fixture.base.base.base.get_type("module", true);
    LUAU_ASSERT!(id.is_some());
    assert_eq!(
      to_string_type_id_to_string_options_mut(id.unwrap(), opt.clone()),
      String::from("{  }")
    );
    // [TODO] CLI-140762 Fragment autocomplete still doesn't return correct result when LuauSolverV2 is on
    // #if 0 (fragmentACAndCheck calls disabled in C++)
  }
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_multiple_functions_complex() {
  let text = String::from(
    r#"@1 local function f1(a1)@2
    local l1 = 1;@3
    g1 = 1;@4
end
@5
local function f2(a2)
    local l2 = 1;@6
    g2 = 1;
end @7
"#,
  );

  let mut fixture = FragmentAutocompleteFixture::default();

  fixture.base.autocomplete_fragment_in_both_solvers(
    &text,
    &text,
    '1',
    |fragment: &mut FragmentAutocompleteStatusResult| {
      LUAU_ASSERT!(fragment.result.is_some());
      let strings = &fragment.result.as_ref().unwrap().ac_results.entry_map;
      assert!(!strings.contains_key("f1"));
      assert!(!strings.contains_key("a1"));
      assert!(!strings.contains_key("l1"));
      assert!(strings.contains_key("g1"));
      assert!(!strings.contains_key("f2"));
      assert!(!strings.contains_key("a2"));
      assert!(!strings.contains_key("l2"));
      assert!(strings.contains_key("g2"));
    },
    None,
  );

  fixture.base.autocomplete_fragment_in_both_solvers(
    &text,
    &text,
    '2',
    |fragment: &mut FragmentAutocompleteStatusResult| {
      LUAU_ASSERT!(fragment.result.is_some());
      let strings = &fragment.result.as_ref().unwrap().ac_results.entry_map;
      assert!(strings.contains_key("f1"));
      assert!(strings.contains_key("a1"));
      assert!(!strings.contains_key("l1"));
      assert!(strings.contains_key("g1"));
      assert!(!strings.contains_key("f2"));
      assert!(!strings.contains_key("a2"));
      assert!(!strings.contains_key("l2"));
      assert!(strings.contains_key("g2"));
    },
    None,
  );

  fixture.base.autocomplete_fragment_in_both_solvers(
    &text,
    &text,
    '3',
    |fragment: &mut FragmentAutocompleteStatusResult| {
      LUAU_ASSERT!(fragment.result.is_some());
      let strings = &fragment.result.as_ref().unwrap().ac_results.entry_map;
      assert!(strings.contains_key("f1"));
      assert!(strings.contains_key("a1"));
      assert!(strings.contains_key("l1"));
      assert!(strings.contains_key("g1"));
      assert!(!strings.contains_key("f2"));
      assert!(!strings.contains_key("a2"));
      assert!(!strings.contains_key("l2"));
      assert!(strings.contains_key("g2"));
    },
    None,
  );

  fixture.base.autocomplete_fragment_in_both_solvers(
    &text,
    &text,
    '4',
    |fragment: &mut FragmentAutocompleteStatusResult| {
      LUAU_ASSERT!(fragment.result.is_some());
      let strings = &fragment.result.as_ref().unwrap().ac_results.entry_map;
      assert!(strings.contains_key("f1"));
      assert!(strings.contains_key("a1"));
      assert!(strings.contains_key("l1"));
      assert!(strings.contains_key("g1"));
      assert!(!strings.contains_key("f2"));
      assert!(!strings.contains_key("a2"));
      assert!(!strings.contains_key("l2"));
      assert!(strings.contains_key("g2"));
    },
    None,
  );

  fixture.base.autocomplete_fragment_in_both_solvers(
    &text,
    &text,
    '5',
    |fragment: &mut FragmentAutocompleteStatusResult| {
      LUAU_ASSERT!(fragment.result.is_some());
      let strings = &fragment.result.as_ref().unwrap().ac_results.entry_map;
      assert!(strings.contains_key("f1"));
      assert!(!strings.contains_key("a1"));
      assert!(!strings.contains_key("l1"));
      assert!(strings.contains_key("g1"));
      assert!(!strings.contains_key("f2"));
      assert!(!strings.contains_key("a2"));
      assert!(!strings.contains_key("l2"));
      assert!(strings.contains_key("g2"));
    },
    None,
  );

  fixture.base.autocomplete_fragment_in_both_solvers(
    &text,
    &text,
    '6',
    |fragment: &mut FragmentAutocompleteStatusResult| {
      LUAU_ASSERT!(fragment.result.is_some());
      let strings = &fragment.result.as_ref().unwrap().ac_results.entry_map;
      assert!(strings.contains_key("f1"));
      assert!(!strings.contains_key("a1"));
      assert!(!strings.contains_key("l1"));
      assert!(strings.contains_key("g1"));
      assert!(strings.contains_key("f2"));
      assert!(strings.contains_key("a2"));
      assert!(strings.contains_key("l2"));
      assert!(strings.contains_key("g2"));
    },
    None,
  );

  fixture.base.autocomplete_fragment_in_both_solvers(
    &text,
    &text,
    '7',
    |fragment: &mut FragmentAutocompleteStatusResult| {
      LUAU_ASSERT!(fragment.result.is_some());
      let strings = &fragment.result.as_ref().unwrap().ac_results.entry_map;
      assert!(strings.contains_key("f1"));
      assert!(!strings.contains_key("a1"));
      assert!(!strings.contains_key("l1"));
      assert!(strings.contains_key("g1"));
      assert!(strings.contains_key("f2"));
      assert!(!strings.contains_key("a2"));
      assert!(!strings.contains_key("l2"));
      assert!(strings.contains_key("g2"));
    },
    None,
  );
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_mutually_recursive_alias() {
  use ulua_analysis::records::scope::Scope;
  let source = String::from(
    r#"
type U = {f : number, g : U}

"#,
  );
  let dest = String::from(
    r#"
type U = {f : number, g : V}
type V = {h : number, i : U?} @1
"#,
  );

  // Re-parsing and typechecking a type alias in the fragment that was defined in the base module will assert in ConstraintGenerator::checkAliases
  // unless we don't clone it This will let the incremental pass re-generate the type binding, and we will expect to see it in the type bindings
  let mut fixture = FragmentAutocompleteBuiltinsFixture::default();
  fixture.base.autocomplete_fragment_in_both_solvers(
    &source,
    &dest,
    '1',
    |frag: &mut FragmentAutocompleteStatusResult| {
      LUAU_ASSERT!(!frag.result.as_ref().unwrap().fresh_scope.is_null());
      let scope: *mut Scope = frag.result.as_ref().unwrap().fresh_scope;
      assert!(
        1 == scope
          .as_ref_opt()
          .unwrap()
          .private_type_bindings
          .contains_key("U") as usize
      );
      assert!(
        1 == scope
          .as_ref_opt()
          .unwrap()
          .private_type_bindings
          .contains_key("V") as usize
      );
    },
    None,
  );
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_nearest_enclosing_statement_can_be_non_local() {
  let mut fixture = FragmentAutocompleteFixture::default();
  let result = fixture.base.run_autocomplete_visitor(
    r#"
local x = 4
local y = 5
if x == 4 then
"#,
    &Position { line: 3, column: 4 },
  );

  assert_eq!(4, result.ancestry.len());
  assert_eq!(2, result.local_stack.len());
  assert_eq!(result.local_map.size(), result.local_stack.len());
  assert!(!result.nearest_statement.is_null());
  let last = *result.local_stack.last().unwrap();
  // (下方 unsafe: local_stack 元素是 arena 写入的存活 Binding 指针，只读取名)
  assert_eq!("y", last.as_ref_opt().unwrap().name.as_str().unwrap());

  assert!(crate::is_nearest::<AstStatIf>(result.nearest_statement));
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_nested_blocks_else_difficult_2() {
  let source = String::from(
    r#"
local function foo(t : {foo : number})
    do
        if t then
        end
    end
end
"#,
  );
  let dest = String::from(
    r#"
local function foo(t : {foo : number})
    do
        if t then
        else
            local x = 4
            return x + t@1.
        end
    end
end
"#,
  );

  let mut fixture = FragmentAutocompleteBuiltinsFixture::default();
  fixture.base.autocomplete_fragment_in_both_solvers(
    &source,
    &dest,
    '1',
    |res: &mut FragmentAutocompleteStatusResult| {
      assert_eq!(FragmentAutocompleteStatus::Success, res.status);
      LUAU_ASSERT!(res.result.is_some());
      let ac_results = &res.result.as_ref().unwrap().ac_results;
      assert!(!ac_results.entry_map.is_empty());
      assert!(ac_results.entry_map.contains_key("foo"));
    },
    None,
  );
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_nested_blocks_else_simple() {
  let source = String::from(
    r#"
local function foo(t : {foo : string})
    local x = t.foo
    do
        if t then
        end
    end
end
"#,
  );
  let dest = String::from(
    r#"
local function foo(t : {foo : string})
    local x = t.foo
    do
        if t then
            x:@1
        end
    end
end
"#,
  );

  let mut fixture = FragmentAutocompleteBuiltinsFixture::default();
  fixture.base.autocomplete_fragment_in_both_solvers(
    &source,
    &dest,
    '1',
    |res: &mut FragmentAutocompleteStatusResult| {
      assert!(FragmentAutocompleteStatus::Success == res.status);
      assert!(res.result.is_some());
      let ac = &res.result.as_ref().unwrap().ac_results;
      assert!(!ac.entry_map.is_empty());
      assert!(ac.entry_map.contains_key("gsub"));
      assert!(ac.entry_map.contains_key("len"));
    },
    None,
  );
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_nested_recursive_function() {
  let source = String::from(
    r#"
function foo()
@1end
"#,
  );

  let mut fixture = FragmentAutocompleteFixture::default();
  fixture.base.autocomplete_fragment_in_both_solvers(
    &source,
    &source,
    '1',
    |fragment: &mut FragmentAutocompleteStatusResult| {
      LUAU_ASSERT!(fragment.result.is_some());
      let ac_results = &fragment.result.as_ref().unwrap().ac_results;
      assert!(ac_results.entry_map.contains_key("foo"));
      assert_eq!(AutocompleteContext::Statement, ac_results.context);
    },
    None,
  );
}

mod fragment_autocomplete_nested_recursive_function_fragment_autocomplete_test_case_2 {
  //! Source: `tests/FragmentAutocomplete.test.cpp`
  use super::*;

  #[test]
  fn fragment_autocomplete_nested_recursive_function() {
    let source = String::from(
      r#"
        local function outer()
            local function inner()
@1            end
        end
    "#,
    );

    let mut fixture = FragmentAutocompleteFixture::default();
    fixture.base.autocomplete_fragment_in_both_solvers(
      &source,
      &source,
      '1',
      |frag: &mut FragmentAutocompleteStatusResult| {
        LUAU_ASSERT!(frag.result.is_some());
        let ac = &frag.result.as_ref().unwrap().ac_results;
        assert!(ac.entry_map.contains_key("inner"));
        assert!(ac.entry_map.contains_key("outer"));
      },
      None,
    );
  }
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_no_recs_for_comments() {
  let source = String::from(
    r#"
-- sel @1
-- retur @2
-- fo @3
--[[ sel @4]]
local @5 -- hell@6o
"#,
  );

  let mut fixture = FragmentAutocompleteFixture::default();
  fixture.base.autocomplete_fragment_in_both_solvers(
    &source,
    &source,
    '1',
    |frag: &mut FragmentAutocompleteStatusResult| {
      assert!(frag.result.is_none());
    },
    None,
  );

  fixture.base.autocomplete_fragment_in_both_solvers(
    &source,
    &source,
    '2',
    |frag: &mut FragmentAutocompleteStatusResult| {
      assert!(frag.result.is_none());
    },
    None,
  );

  fixture.base.autocomplete_fragment_in_both_solvers(
    &source,
    &source,
    '3',
    |frag: &mut FragmentAutocompleteStatusResult| {
      assert!(frag.result.is_none());
    },
    None,
  );

  fixture.base.autocomplete_fragment_in_both_solvers(
    &source,
    &source,
    '4',
    |frag: &mut FragmentAutocompleteStatusResult| {
      assert!(frag.result.is_none());
    },
    None,
  );

  fixture.base.autocomplete_fragment_in_both_solvers(
    &source,
    &source,
    '5',
    |frag: &mut FragmentAutocompleteStatusResult| {
      LUAU_ASSERT!(frag.result.is_some());
      assert!(
        !frag
          .result
          .as_ref()
          .unwrap()
          .ac_results
          .entry_map
          .is_empty()
      );
    },
    None,
  );

  fixture.base.autocomplete_fragment_in_both_solvers(
    &source,
    &source,
    '6',
    |frag: &mut FragmentAutocompleteStatusResult| {
      assert!(frag.result.is_none());
    },
    None,
  );
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_no_recs_for_comments_blocks() {
  let source = String::from(
    r#"
--[[
comment 1
@1]]@2 local
-- [[ comment 2]]
--
-- sdfsdfsdf
--[[comment 3]]
--[[  @3
foo
@4bar
baz
]]
"#,
  );

  let mut fixture = FragmentAutocompleteFixture::default();
  fixture.base.autocomplete_fragment_in_both_solvers(
    &source,
    &source,
    '1',
    |frag: &mut FragmentAutocompleteStatusResult| {
      assert!(frag.result.is_none());
    },
    None,
  );

  fixture.base.autocomplete_fragment_in_both_solvers(
    &source,
    &source,
    '2',
    |frag: &mut FragmentAutocompleteStatusResult| {
      LUAU_ASSERT!(frag.result.is_some());
      assert!(
        !frag
          .result
          .as_ref()
          .unwrap()
          .ac_results
          .entry_map
          .is_empty()
      );
    },
    None,
  );

  fixture.base.autocomplete_fragment_in_both_solvers(
    &source,
    &source,
    '3',
    |frag: &mut FragmentAutocompleteStatusResult| {
      assert!(frag.result.is_none());
    },
    None,
  );

  fixture.base.autocomplete_fragment_in_both_solvers(
    &source,
    &source,
    '4',
    |frag: &mut FragmentAutocompleteStatusResult| {
      assert!(frag.result.is_none());
    },
    None,
  );
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_no_recs_for_comments_in_incremental_fragment() {
  let source = String::from(
    r#"
local x = 5
if x == 5
"#,
  );
  let updated = String::from(
    r#"
local x = 5
if x == 5 then -- a comment @1
"#,
  );

  let mut fixture = FragmentAutocompleteFixture::default();
  fixture.base.autocomplete_fragment_in_both_solvers(
    &source,
    &updated,
    '1',
    |frag: &mut FragmentAutocompleteStatusResult| {
      assert!(frag.result.is_none());
    },
    None,
  );
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_no_recs_for_comments_simple() {
  let source = String::from(
    r#"
-- sel
-- retur
-- fo
-- if @1
-- end
-- the
"#,
  );

  let mut fixture = FragmentAutocompleteFixture::default();
  fixture.base.autocomplete_fragment_in_both_solvers(
    &source,
    &source,
    '1',
    |frag: &mut FragmentAutocompleteStatusResult| {
      assert!(frag.result.is_none());
    },
    None,
  );
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_not_null_assertion_caused_by_leaking_free_type_from_stale_module() {
  let source = String::from(
    r#"
local Players = game:GetService("Players")

Players.PlayerAdded:Connect(function(Player)
    for_,v in script.PlayerValue:GetChildren()do
        v
    end
end)
"#,
  );

  let dest = String::from(
    r#"
local Players = game:GetService("Players")

Players.PlayerAdded:Connect(function(Player)
    for_,v in script.PlayerValue:GetChildren()do
        v:l@1
    end
end)
"#,
  );

  let mut fixture = FragmentAutocompleteBuiltinsFixture::default();
  fixture.base.autocomplete_fragment_in_both_solvers(
    &source,
    &dest,
    '1',
    |_result: &mut FragmentAutocompleteStatusResult| {},
    None,
  );
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_not_null_nil_scope_assertion_caused_by_free_type_inheriting_null_scope_from_table()
 {
  let source = String::from(
    r#"--!strict
local foo
local a = foo()

local e = foo().x

local f = foo().y


"#,
  );

  let dest = String::from(
    r#"--!strict
local foo
local a = foo()

local e = foo().x

local f = foo().y

z = a.P.E@1
"#,
  );

  let mut fixture = FragmentAutocompleteBuiltinsFixture::default();
  fixture.base.autocomplete_fragment_in_both_solvers(
    &source,
    &dest,
    '1',
    |_frag: &mut FragmentAutocompleteStatusResult| {},
    None,
  );
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_oss_1850() {
  let source = String::from(
    r#"
type t = { name: "t", } | { name: "ts", person: "dog" }

local t:t
if t.name == "ts" then
end
    "#,
  );
  let dest = String::from(
    r#"
type t = { name: "t", } | { name: "ts", person: "dog" }

local t:t
if t.name == "ts" then
    t.@1
end
    "#,
  );

  let mut fixture = FragmentAutocompleteFixture::default();
  fixture.base.autocomplete_fragment_in_both_solvers(
    &source,
    &dest,
    '1',
    |result: &mut FragmentAutocompleteStatusResult| {
      let ac_results = &result.result.as_ref().unwrap().ac_results;
      assert!(!ac_results.entry_map.is_empty());
      assert!(ac_results.entry_map.contains_key("name"));
      assert!(ac_results.entry_map.contains_key("person"));
    },
    None,
  );
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_partial_for_in_in_body() {
  let (_fixture, region) = fx_region!(
    r#"
for i,v in {1,2,3} do
"#,
    Position {
      line: 1,
      column: 21,
    }
  );

  assert_eq!(
    Location {
      begin: Position {
        line: 1,
        column: 21
      },
      end: Position {
        line: 1,
        column: 21
      },
    },
    region.fragment_location
  );
  assert!(!region.parent_block.is_null());
  assert!(crate::is_nearest::<AstStatForIn>(region.nearest_statement));
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_partial_for_in_in_condition_1() {
  let (_fixture, region) = fx_region!(
    r#"
for i,v in {1,2,3}
"#,
    Position {
      line: 1,
      column: 18,
    }
  );

  assert_eq!(
    Location {
      begin: Position { line: 1, column: 0 },
      end: Position {
        line: 1,
        column: 18
      },
    },
    region.fragment_location
  );
  assert!(!region.parent_block.is_null());
  assert!(crate::is_nearest::<AstStatForIn>(region.nearest_statement));
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_partial_for_in_in_condition_2() {
  let (_fixture, region) = fx_region!(
    r#"
for i,v in
"#,
    Position {
      line: 1,
      column: 10,
    }
  );

  assert_eq!(
    Location {
      begin: Position { line: 1, column: 0 },
      end: Position {
        line: 1,
        column: 10
      },
    },
    region.fragment_location
  );
  assert!(!region.parent_block.is_null());
  assert!(crate::is_nearest::<AstStatForIn>(region.nearest_statement));
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_partial_for_in_in_condition_3() {
  let (_fixture, region) = fx_region!(
    r#"
for i,
"#,
    Position { line: 1, column: 6 }
  );

  assert_eq!(
    Location {
      begin: Position { line: 1, column: 0 },
      end: Position { line: 1, column: 6 },
    },
    region.fragment_location
  );
  assert!(!region.parent_block.is_null());
  assert!(crate::is_nearest::<AstStatForIn>(region.nearest_statement));
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_partial_for_numeric_in_body() {
  use ulua_ast::records::ast_stat_for::AstStatFor;

  let (_fixture, region) = fx_region!(
    r#"
for c = 1,3 do
"#,
    Position {
      line: 1,
      column: 14,
    }
  );

  assert_eq!(
    Location {
      begin: Position {
        line: 1,
        column: 14
      },
      end: Position {
        line: 1,
        column: 14
      },
    },
    region.fragment_location
  );
  assert!(!region.parent_block.is_null());
  assert!(crate::is_nearest::<AstStatFor>(region.nearest_statement));
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_partial_for_numeric_in_condition() {
  use ulua_ast::records::ast_stat_for::AstStatFor;

  let (_fixture, region) = fx_region!(
    r#"
for c = 1,3
"#,
    Position {
      line: 1,
      column: 11,
    }
  );

  assert_eq!(
    Location {
      begin: Position {
        line: 1,
        column: 10
      },
      end: Position {
        line: 1,
        column: 11
      },
    },
    region.fragment_location
  );
  assert!(!region.parent_block.is_null());
  assert!(crate::is_nearest::<AstStatFor>(region.nearest_statement));
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_partial_statement_after_do() {
  let (_fixture, region) = fx_region!(
    r#"
local x = 4
do

end
local x =
"#,
    Position { line: 5, column: 9 }
  );

  assert_eq!(
    Location {
      begin: Position { line: 5, column: 0 },
      end: Position { line: 5, column: 9 },
    },
    region.fragment_location
  );
  assert!(!region.parent_block.is_null());
  assert!(crate::is_nearest::<AstStatLocal>(region.nearest_statement));
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_partial_statement_inside_do() {
  let (_fixture, region) = fx_region!(
    r#"
local x = 4
do
    local x =
end
"#,
    Position {
      line: 3,
      column: 13,
    }
  );

  assert_eq!(
    Location {
      begin: Position { line: 3, column: 4 },
      end: Position {
        line: 3,
        column: 13
      },
    },
    region.fragment_location
  );
  assert!(!region.parent_block.is_null());
  assert!(crate::is_nearest::<AstStatLocal>(region.nearest_statement));
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_partial_while_in_condition() {
  use ulua_ast::records::ast_stat_while::AstStatWhile;

  let (_fixture, region) = fx_region!(
    r#"
while t
"#,
    Position { line: 1, column: 7 }
  );

  assert_eq!(
    Location {
      begin: Position { line: 1, column: 0 },
      end: Position { line: 1, column: 7 },
    },
    region.fragment_location
  );
  assert!(!region.parent_block.is_null());
  assert!(crate::is_nearest::<AstStatWhile>(region.nearest_statement));
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_require_tracing() {
  let mut fixture = FragmentAutocompleteBuiltinsFixture::default();

  fixture.base.base.base.file_resolver.source.insert(
    String::from("MainModule/A"),
    String::from(
      r#"
return { x = 0 }
    "#,
    ),
  );

  fixture.base.base.base.file_resolver.source.insert(
    String::from("MainModule"),
    String::from(
      r#"
local result = require(script.A)
local x = 1 + result.@1
    "#,
    ),
  );

  let main_module = fixture
    .base
    .base
    .base
    .file_resolver
    .source
    .get("MainModule")
    .unwrap()
    .clone();

  fixture.base.autocomplete_fragment_in_both_solvers(
    &main_module,
    &main_module,
    '1',
    |frag: &mut FragmentAutocompleteStatusResult| {
      LUAU_ASSERT!(frag.result.is_some());
      assert!(frag.result.as_ref().unwrap().ac_results.entry_map.len() == 1);
      assert!(
        frag
          .result
          .as_ref()
          .unwrap()
          .ac_results
          .entry_map
          .contains_key("x")
      );
    },
    None,
  );
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_respects_frontend_options() {
  use ulua_analysis::{
    functions::try_fragment_autocomplete::try_fragment_autocomplete,
    records::{fragment_context::FragmentContext, frontend_options::FrontendOptions},
  };
  use ulua_unit_test::functions::null_callback_autocomplete_test::null_callback;

  // NOTE: This does not pass the new solver because it is exercising behavior
  // that is only meaningful under the old solver (whether the correct
  // module resolver is used).
  //
  // C++ `DOES_NOT_PASS_NEW_SOLVER_GUARD()` =>
  // `ScopedFastFlag{FFlag::DebugLuauForceOldSolver, !FFlag::DebugLuauForceAllNewSolverTests}`.
  // 上游旗标 `DebugLuauForceAllNewSolverTests` 已按 r7 deadcode 仲裁摘除（全仓无任何路径置
  // true，恒 false），故 `!false` 内联为常量 `true`：本用例始终钉在 old solver 下跑。
  let _guard = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, true);

  let source = String::from(
    r#"
local tbl = { abc = 1234}
t
"#,
  );

  let mut fixture = FragmentAutocompleteFixture::default();
  fixture
    .base
    .base
    .base
    .file_resolver
    .source
    .insert(String::from("game/A"), source.clone());

  let opts = FrontendOptions {
    for_autocomplete: true,
    ..Default::default()
  };

  {
    let frontend = fixture.base.base.get_frontend();
    frontend.set_luau_solver_mode(if !fflag::DebugLuauForceOldSolver.get() {
      SolverMode::New
    } else {
      SolverMode::Old
    });
    frontend
      .check_module_name_optional_frontend_options(&ModuleName::from("game/A"), Some(opts.clone()));
    assert!(
      frontend
        .module_resolver_for_autocomplete
        .modules
        .contains_key("game/A")
    );
    assert!(!frontend.module_resolver.modules.contains_key("game/A"));
  }

  let parse_result = fixture.base.parse_helper(source.clone());
  let context =
    FragmentContext::new_with_options(source.as_str(), &parse_result, Some(opts.clone()), None);

  let frontend = fixture.base.base.get_frontend();
  let frag = try_fragment_autocomplete(
    frontend,
    &ModuleName::from("game/A"),
    Position { line: 2, column: 1 },
    context,
    Box::new(null_callback),
  );

  LUAU_ASSERT!(frag.result.is_some());
  let result = frag.result.as_ref().unwrap();
  assert_eq!("game/A", result.incremental_module.name);

  let frontend = fixture.base.base.get_frontend();
  assert!(
    frontend
      .module_resolver_for_autocomplete
      .modules
      .contains_key("game/A")
  );
  assert!(!frontend.module_resolver.modules.contains_key("game/A"));
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_self_types_provide_rich_autocomplete() {
  let source = String::from(
    r#"
type Service = {
    Start: (self: Service) -> (),
    Prop: number
}

local Service: Service = {}

function Service:Start()

end
"#,
  );
  let dest = String::from(
    r#"
type Service = {
    Start: (self: Service) -> (),
    Prop: number
}

local Service: Service = {}

function Service:Start()
    self.@1
end
"#,
  );

  let mut fixture = FragmentAutocompleteBuiltinsFixture::default();
  fixture.base.autocomplete_fragment_in_both_solvers(
    &source,
    &dest,
    '1',
    |result: &mut FragmentAutocompleteStatusResult| {
      let ac_results = &result.result.as_ref().unwrap().ac_results;
      assert!(!ac_results.entry_map.is_empty());
      assert!(ac_results.entry_map.contains_key("Prop"));
      assert!(ac_results.entry_map.contains_key("Start"));
    },
    None,
  );
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_self_with_colon_good_recommendations() {
  let source = String::from(
    r#"
type Service = {
    Start: (self: Service) -> (),
    Prop: number
}

local Service: Service = {}

function Service:Start()

end
"#,
  );
  let dest = String::from(
    r#"
type Service = {
    Start: (self: Service) -> (),
    Prop: number
}

local Service: Service = {}

function Service:Start()
    self:@1
end
"#,
  );

  let mut fixture = FragmentAutocompleteBuiltinsFixture::default();
  fixture.base.autocomplete_fragment_in_both_solvers(
    &source,
    &dest,
    '1',
    |result: &mut FragmentAutocompleteStatusResult| {
      let ac = &result.result.as_ref().unwrap().ac_results;
      assert!(!ac.entry_map.is_empty());
      assert!(ac.entry_map.contains_key("Prop"));
      assert!(ac.entry_map["Prop"].wrong_index_type);
      assert!(ac.entry_map.contains_key("Start"));
      assert!(!ac.entry_map["Start"].wrong_index_type);
    },
    None,
  );
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_self_with_fancy_metatable_setting_new_solver() {
  let source = String::from(
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
            print("My balance is: " .. )
        end
"#,
  );

  let dest = String::from(
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
            print("My balance is: " .. self.@1 )
        end
"#,
  );

  let mut fixture = FragmentAutocompleteBuiltinsFixture::default();
  fixture.base.autocomplete_fragment_in_new_solver(
    &source,
    &dest,
    '1',
    |result: &mut FragmentAutocompleteStatusResult| {
      let ac = &result.result.as_ref().unwrap().ac_results;
      assert!(!ac.entry_map.is_empty());
      assert!(ac.entry_map.contains_key("new"));
      assert!(ac.entry_map.contains_key("report"));
    },
    None,
  );
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_single_line_local_and_annot() {
  use ulua_ast::records::ast_stat_error::AstStatError;

  let (_fixture, region) = fx_region!(
    r#"
type Part = {x : number}
local part : Part = {x = 3}; pa
"#,
    Position {
      line: 2,
      column: 32,
    }
  );

  assert_eq!(
    Location {
      begin: Position {
        line: 2,
        column: 29
      },
      end: Position {
        line: 2,
        column: 32
      },
    },
    region.fragment_location
  );
  assert!(!region.parent_block.is_null());
  assert!(crate::is_nearest::<AstStatError>(region.nearest_statement));
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_singleline_call() {
  let (_fixture, region) = fx_region!(
    r#"
abc("foo")
"#,
    Position {
      line: 1,
      column: 10,
    }
  );

  assert_eq!(
    Location {
      begin: Position { line: 1, column: 0 },
      end: Position {
        line: 1,
        column: 10
      },
    },
    region.fragment_location
  );
  assert!(!region.parent_block.is_null());
  assert!(!region.nearest_statement.is_null());
  assert!(crate::is_nearest::<AstStatExpr>(region.nearest_statement));
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_statement_in_empty_fragment_is_non_null() {
  let _sff = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);
  let mut fixture = FragmentAutocompleteFixture::default();
  let source = String::from(
    r#"

"#,
  );
  let result = fixture.base.check_with_options(&source);
  assert_eq!(0, result.errors.len(), "{:?}", result.errors);

  let fragment = fixture
    .base
    .parse_fragment(&source, &Position { line: 1, column: 0 }, None)
    .expect("expected fragment parse result");

  assert_eq!("", fragment.fragment_to_parse);
  assert_eq!(1, fragment.ancestry.len());
  assert!(!fragment.root.is_null());
  assert_eq!(0, fragment.root.as_ref_opt().unwrap().body.len());
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_str_metata_table_finished_defining() {
  let source = String::from(
    r#"local function foobar(): string return "" end
local foo = f"#,
  );
  let dest = String::from(
    r#"local function foobar(): string return "" end
local foo = foobar()
foo:@1"#,
  );

  let mut fixture = FragmentAutocompleteBuiltinsFixture::default();
  fixture.base.autocomplete_fragment_in_both_solvers(
    &source,
    &dest,
    '1',
    |res: &mut FragmentAutocompleteStatusResult| {
      assert_eq!(FragmentAutocompleteStatus::Success, res.status);
      LUAU_ASSERT!(res.result.is_some());
      let ac_results = &res.result.as_ref().unwrap().ac_results;
      assert!(!ac_results.entry_map.is_empty());
      assert!(ac_results.entry_map.contains_key("len"));
      assert!(ac_results.entry_map.contains_key("gsub"));
    },
    None,
  );
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_str_metata_table_redef() {
  let source = String::from(r#"local x = 42"#);
  let dest = String::from(
    r#"local x = 42
local x = ""
x:@1"#,
  );

  let mut fixture = FragmentAutocompleteBuiltinsFixture::default();
  fixture.base.autocomplete_fragment_in_both_solvers(
    &source,
    &dest,
    '1',
    |res: &mut FragmentAutocompleteStatusResult| {
      assert_eq!(FragmentAutocompleteStatus::Success, res.status);
      assert!(res.result.is_some());
      let ac = &res.result.as_ref().unwrap().ac_results;
      assert!(!ac.entry_map.is_empty());
      assert!(ac.entry_map.contains_key("len"));
      assert!(ac.entry_map.contains_key("gsub"));
    },
    None,
  );
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_string_interpolation_format_provides_autocomplete_results() {
  let source = String::from(
    r#"
type Foo = {x : number, x1 : string, x2 : boolean}
local e: Foo = {x = 1, x1 = "1", x2 = true}
local s =
"#,
  );

  let dest = String::from(
    r#"
type Foo = {x : number, x1 : string, x2 : boolean}
local e : Foo = {x = 1, x1 = "1", x2 = true}
local s = `{e.@1 }`
"#,
  );

  let mut fixture = FragmentAutocompleteBuiltinsFixture::default();
  fixture.base.autocomplete_fragment_in_both_solvers(
    &source,
    &dest,
    '1',
    |result: &mut FragmentAutocompleteStatusResult| {
      let ac = &result.result.as_ref().unwrap().ac_results;
      assert!(!ac.entry_map.is_empty());
      assert!(ac.entry_map.contains_key("x"));
      assert!(ac.entry_map.contains_key("x1"));
      assert!(ac.entry_map.contains_key("x2"));
    },
    None,
  );
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_string_interpolation_format_provides_results_inside_of_function_call() {
  let source = String::from(
    r#"
type T = {x : number, y : number, z : number}
local e = {x = 1, y = 2, z = 3}
print(`{e.x}`)
"#,
  );

  let dest = String::from(
    r#"
type T = {x : number, y : number, z : number}
local e = {x = 1, y = 2, z = 3}
print(`{e.x} {e.@1}`)
"#,
  );

  let mut fixture = FragmentAutocompleteBuiltinsFixture::default();
  fixture.base.autocomplete_fragment_in_both_solvers(
    &source,
    &dest,
    '1',
    |result: &mut FragmentAutocompleteStatusResult| {
      let ac = &result.result.as_ref().unwrap().ac_results;
      assert!(!ac.entry_map.is_empty());
      assert!(ac.entry_map.contains_key("x"));
      assert!(ac.entry_map.contains_key("y"));
      assert!(ac.entry_map.contains_key("z"));
    },
    None,
  );
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_string_literal_with_override() {
  let source = String::from(
    r#"
function foo(bar: string) end
foo("a@1bc")
"#,
  );

  let mut fixture = FragmentAutocompleteFixture::default();
  fixture.base.autocomplete_fragment_in_both_solvers(
    &source,
    &source,
    '1',
    |fragment: &mut FragmentAutocompleteStatusResult| {
      assert!(fragment.result.is_some());
      let ac_results = &fragment.result.as_ref().unwrap().ac_results;
      assert!(ac_results.entry_map.is_empty());
      assert_eq!(AutocompleteContext::String, ac_results.context);
    },
    Some(Position { line: 2, column: 9 }),
  );
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_studio_ice_1() {
  let source = String::from(
    r#"
--Woop
\@native
local function test()

end
"#,
  );

  let updated = String::from(
    r#"
--Woop
\@native
local function test()

end
function a@1
"#,
  );

  let mut fixture = FragmentAutocompleteFixture::default();
  fixture.base.autocomplete_fragment_in_both_solvers(
    &source,
    &updated,
    '1',
    |_result: &mut FragmentAutocompleteStatusResult| {},
    None,
  );
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_table_intersection() {
  let source = String::from(
    r#"
        type t1 = { a1 : string, b2 : number }
        type t2 = { b2 : number, c3 : string }
        function func(abc : t1 & t2)

        end
    "#,
  );
  let updated = String::from(
    r#"
        type t1 = { a1 : string, b2 : number }
        type t2 = { b2 : number, c3 : string }
        function func(abc : t1 & t2)
            abc.@1
        end
    "#,
  );

  let mut fixture = FragmentAutocompleteFixture::default();
  fixture.base.autocomplete_fragment_in_both_solvers(
    &source,
    &updated,
    '1',
    |frag: &mut FragmentAutocompleteStatusResult| {
      LUAU_ASSERT!(frag.result.is_some());
      let ac = &frag.result.as_ref().unwrap().ac_results;
      assert_eq!(3, ac.entry_map.len());
      assert!(ac.entry_map.contains_key("a1"));
      assert!(ac.entry_map.contains_key("b2"));
      assert!(ac.entry_map.contains_key("c3"));
      assert_eq!(AutocompleteContext::Property, ac.context);
    },
    None,
  );
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_table_union() {
  let source = String::from(
    r#"
        type t1 = { a1 : string, b2 : number }
        type t2 = { b2 : string, c3 : string }
        function func(abc : t1 | t2)

        end
    "#,
  );
  let updated = String::from(
    r#"
        type t1 = { a1 : string, b2 : number }
        type t2 = { b2 : string, c3 : string }
        function func(abc : t1 | t2)
            abc.@1
        end
    "#,
  );

  let mut fixture = FragmentAutocompleteFixture::default();
  fixture.base.autocomplete_fragment_in_both_solvers(
    &source,
    &updated,
    '1',
    |frag: &mut FragmentAutocompleteStatusResult| {
      assert!(frag.result.is_some());
      let ac = &frag.result.as_ref().unwrap().ac_results;
      assert_eq!(1, ac.entry_map.len());
      assert!(ac.entry_map.contains_key("b2"));
      assert_eq!(ac.context, AutocompleteContext::Property);
    },
    None,
  );
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_tagged_union_completion_first_branch_of_union_new_solver() {
  // TODO: CLI-155619 - Fragment autocomplete needs to use stale refinement information for modules typechecked in the new solver as well
  let source = String::from(
    r#"
type Ok<T> = { type: "ok", value: T}
type Err<E> = { type : "err", error : E}
type Result<T,E> = Ok<T> | Err<E>

local result = {} :: Result<number, string>

if result.type == "ok" then

end
"#,
  );

  let dest = String::from(
    r#"
type Ok<T> = { type: "ok", value: T}
type Err<E> = { type : "err", error : E}
type Result<T,E> = Ok<T> | Err<E>

local result = {} :: Result<number, string>

if result.type == "ok" then
    result.@1
end
"#,
  );

  let mut fixture = FragmentAutocompleteFixture::default();
  fixture.base.autocomplete_fragment_in_new_solver(
    &source,
    &dest,
    '1',
    |result: &mut FragmentAutocompleteStatusResult| {
      assert!(result.result.is_some());
      let ac = &result.result.as_ref().unwrap().ac_results;
      assert_eq!(ac.entry_map.contains_key("type") as usize, 1);
      assert_eq!(ac.entry_map.contains_key("value") as usize, 1);
    },
    None,
  );
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_tagged_union_completion_first_branch_of_union_old_solver() {
  let source = String::from(
    r#"
type Ok<T> = { type: "ok", value: T}
type Err<E> = { type : "err", error : E}
type Result<T,E> = Ok<T> | Err<E>

local result = {} :: Result<number, string>

if result.type == "ok" then

end
"#,
  );

  let dest = String::from(
    r#"
type Ok<T> = { type: "ok", value: T}
type Err<E> = { type : "err", error : E}
type Result<T,E> = Ok<T> | Err<E>

local result = {} :: Result<number, string>

if result.type == "ok" then
    result.@1
end
"#,
  );

  let mut fixture = FragmentAutocompleteFixture::default();
  fixture.base.autocomplete_fragment_in_old_solver(
    &source,
    &dest,
    '1',
    |result: &mut FragmentAutocompleteStatusResult| {
      assert!(result.result.is_some());
      let ac = &result.result.as_ref().unwrap().ac_results;
      assert_eq!(ac.entry_map.contains_key("type") as usize, 1);
      assert_eq!(ac.entry_map.contains_key("value") as usize, 1);
    },
    None,
  );
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_tagged_union_completion_second_branch_of_union_new_solver() {
  let source = String::from(
    r#"
type Ok<T> = { type: "ok", value: T}
type Err<E> = { type : "err", error : E}
type Result<T,E> = Ok<T> | Err<E>

local result = {} :: Result<number, string>

if result.type == "err" then

end
"#,
  );

  let dest = String::from(
    r#"
type Ok<T> = { type: "ok", value: T}
type Err<E> = { type : "err", error : E}
type Result<T,E> = Ok<T> | Err<E>

local result = {} :: Result<number, string>

if result.type == "err" then
    result.@1
end
"#,
  );

  let mut fixture = FragmentAutocompleteFixture::default();
  fixture.base.autocomplete_fragment_in_new_solver(
    &source,
    &dest,
    '1',
    |result: &mut FragmentAutocompleteStatusResult| {
      LUAU_ASSERT!(result.result.is_some());
      let ac = &result.result.as_ref().unwrap().ac_results;
      assert!(ac.entry_map.contains_key("type"));
      assert!(ac.entry_map.contains_key("error"));
    },
    None,
  );
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_tagged_union_completion_second_branch_of_union_old_solver() {
  let source = String::from(
    r#"
type Ok<T> = { type: "ok", value: T}
type Err<E> = { type : "err", error : E}
type Result<T,E> = Ok<T> | Err<E>

local result = {} :: Result<number, string>

if result.type == "err" then

end
"#,
  );

  let dest = String::from(
    r#"
type Ok<T> = { type: "ok", value: T}
type Err<E> = { type : "err", error : E}
type Result<T,E> = Ok<T> | Err<E>

local result = {} :: Result<number, string>

if result.type == "err" then
    result.@1
end
"#,
  );

  let mut fixture = FragmentAutocompleteFixture::default();
  fixture.base.autocomplete_fragment_in_old_solver(
    &source,
    &dest,
    '1',
    |result: &mut FragmentAutocompleteStatusResult| {
      assert!(result.result.is_some());
      let ac = &result.result.as_ref().unwrap().ac_results;
      assert_eq!(ac.entry_map.contains_key("type") as usize, 1);
      assert_eq!(ac.entry_map.contains_key("error") as usize, 1);
    },
    None,
  );
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_tbl_function_parameter() {
  let source = String::from(
    r#"
--!strict
type Foo = {x : number, y : number}
local function func(abc : Foo)
   abc.@1
end
"#,
  );

  let mut fixture = FragmentAutocompleteFixture::default();
  fixture.base.autocomplete_fragment_in_both_solvers(
    &source,
    &source,
    '1',
    |frag: &mut FragmentAutocompleteStatusResult| {
      LUAU_ASSERT!(frag.result.is_some());
      let ac_results = &frag.result.as_ref().unwrap().ac_results;
      assert_eq!(2, ac_results.entry_map.len());
      assert!(ac_results.entry_map.contains_key("x"));
      assert!(ac_results.entry_map.contains_key("y"));
    },
    None,
  );
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_tbl_local_function_parameter() {
  let source = String::from(
    r#"
--!strict
type Foo = {x : number, y : number}
local function func(abc : Foo)
   abc.@1
end
"#,
  );

  let mut fixture = FragmentAutocompleteFixture::default();
  fixture.base.autocomplete_fragment_in_both_solvers(
    &source,
    &source,
    '1',
    |frag: &mut FragmentAutocompleteStatusResult| {
      LUAU_ASSERT!(frag.result.is_some());
      let ac = &frag.result.as_ref().unwrap().ac_results;
      assert_eq!(2, ac.entry_map.len());
      assert!(ac.entry_map.contains_key("x"));
      assert!(ac.entry_map.contains_key("y"));
    },
    None,
  );
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_thrown_parse_error_leads_to_null_root() {
  let mut fixture = FragmentAutocompleteFixture::default();
  fixture.base.check_with_options("type A =  ");
  let _sfi = ScopedFastInt::new(&fint::LuauParseErrorLimit, 1);
  let fragment = fixture.base.parse_fragment(
    "type A = <>function<> more garbage here",
    &Position {
      line: 0,
      column: 39,
    },
    None,
  );

  assert!(fragment.is_none());
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_type_correct_local_rank_assert() {
  let source = String::from(r#""#);
  let dest = String::from(
    r#"local function target(a: number, b: string) return a + #b end
local bar1 = 'hello'
local bar2 = 4
return target(bar@1"#,
  );

  let mut fixture = FragmentAutocompleteBuiltinsFixture::default();
  fixture.base.autocomplete_fragment_in_both_solvers(
    &source,
    &dest,
    '1',
    |status: &mut FragmentAutocompleteStatusResult| {
      assert!(FragmentAutocompleteStatus::Success == status.status);
      LUAU_ASSERT!(status.result.is_some());
      assert!(
        !status
          .result
          .as_ref()
          .unwrap()
          .ac_results
          .entry_map
          .is_empty()
      );
      assert!(
        status
          .result
          .as_ref()
          .unwrap()
          .ac_results
          .entry_map
          .contains_key("bar1")
      );
      assert!(
        status
          .result
          .as_ref()
          .unwrap()
          .ac_results
          .entry_map
          .contains_key("bar2")
      );
    },
    None,
  );
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_type_correct_local_return_assert() {
  let source = String::from(r#""#);
  let dest = String::from(
    r#"local function target(a: number, b: string) return a + #b end
local function bar1(a: string) reutrn a .. 'x' end
local function bar2(a: number) return -a end
return target(bar@1"#,
  );

  let mut fixture = FragmentAutocompleteBuiltinsFixture::default();
  fixture.base.autocomplete_fragment_in_both_solvers(
    &source,
    &dest,
    '1',
    |status: &mut FragmentAutocompleteStatusResult| {
      assert!(FragmentAutocompleteStatus::Success == status.status);
      LUAU_ASSERT!(status.result.is_some());
      let ac = &status.result.as_ref().unwrap().ac_results;
      assert!(!ac.entry_map.is_empty());
      assert!(ac.entry_map.contains_key("bar1"));
      assert!(ac.entry_map.contains_key("bar2"));
    },
    None,
  );
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_typecheck_fragment_handles_unusable_module() {
  use ulua_analysis::enums::fragment_type_check_status::FragmentTypeCheckStatus;
  use ulua_unit_test::functions::get_options::get_options;

  let source_a = ModuleName::from("MainModule");
  let source_b = ModuleName::from("game/Gui/Modules/B");

  let source_a_text = String::from(
    r#"
local Modules = game:GetService('Gui').Modules
local B = require(Modules.B)
return { hello = B }
"#,
  );
  let source_b_text = String::from(r#"return {hello = "hello"}"#);

  let mut fixture = FragmentAutocompleteBuiltinsFixture::default();
  fixture
    .base
    .base
    .base
    .file_resolver
    .source
    .insert(source_a.clone(), source_a_text.clone());
  fixture
    .base
    .base
    .base
    .file_resolver
    .source
    .insert(source_b.clone(), source_b_text.clone());

  let for_autocomplete = get_options().for_autocomplete;

  {
    let frontend = fixture.base.base.get_frontend();
    let _result =
      frontend.check_module_name_optional_frontend_options(&source_a, Some(get_options()));
    assert!(!frontend.is_dirty(&source_a, for_autocomplete));
  }

  // C++ `getModuleResolver(frontend)` selects `moduleResolver` under the new
  // solver and `moduleResolverForAutocomplete` under the old solver. The
  // weak_ptr / expired() semantics map to `Arc::downgrade` / `upgrade`.
  let weak_module = {
    let frontend = fixture.base.base.get_frontend();
    let resolver = if !fflag::DebugLuauForceOldSolver.get() {
      &frontend.module_resolver
    } else {
      &frontend.module_resolver_for_autocomplete
    };
    let module = resolver.modules.get(&source_b);
    LUAU_ASSERT!(module.is_some());
    use alloc::sync::Arc;
    Arc::downgrade(module.unwrap())
  };
  // `REQUIRE(!weakModule.expired())`
  assert!(weak_module.upgrade().is_some());

  {
    let frontend = fixture.base.base.get_frontend();
    frontend.mark_dirty(&source_b, None);
    assert!(frontend.is_dirty(&source_a, for_autocomplete));

    frontend.check_module_name_optional_frontend_options(&source_b, Some(get_options()));
  }
  // `CHECK(weakModule.expired())`
  assert!(weak_module.upgrade().is_none());

  let (status, _) = fixture.base.typecheck_fragment_for_module(
    &source_a,
    &source_a_text,
    Position { line: 0, column: 0 },
    None,
  );
  assert_eq!(FragmentTypeCheckStatus::SkipAutocomplete, status);

  let (status2, _) = fixture.base.typecheck_fragment_for_module(
    &source_b,
    &source_b_text,
    Position {
      line: 3,
      column: 20,
    },
    None,
  );
  assert_eq!(FragmentTypeCheckStatus::Success, status2);
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_unary_minus_operator_needs_to_provide_autocomplete_results() {
  let source = String::from(
    r#"
type Pool = { x : number }

local function foobar(p)
    local pool = p :: Pool
    if -pool
end
"#,
  );
  let dest = String::from(
    r#"
type Pool = { x : number }

local function foobar(p)
    local pool = p :: Pool
    if -pool.@1
end
"#,
  );

  let mut fixture = FragmentAutocompleteBuiltinsFixture::default();
  fixture.base.autocomplete_fragment_in_both_solvers(
    &source,
    &dest,
    '1',
    |result: &mut FragmentAutocompleteStatusResult| {
      let ac_results = &result.result.as_ref().unwrap().ac_results;
      assert!(!ac_results.entry_map.is_empty());
      assert!(ac_results.entry_map.contains_key("x"));
    },
    None,
  );
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_unsealed_table() {
  let source = String::from(
    r#"
        local tbl = {}
        tbl.prop = 5
        tbl.@1
    "#,
  );

  let mut fixture = FragmentAutocompleteFixture::default();
  fixture.base.autocomplete_fragment_in_both_solvers(
    &source,
    &source,
    '1',
    |frag: &mut FragmentAutocompleteStatusResult| {
      LUAU_ASSERT!(frag.result.is_some());
      let ac = &frag.result.as_ref().unwrap().ac_results;
      assert_eq!(1, ac.entry_map.len());
      assert!(ac.entry_map.contains_key("prop"));
      assert_eq!(AutocompleteContext::Property, ac.context);
    },
    None,
  );
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_unsealed_table_2() {
  let source = String::from(
    r#"
        local tbl = {}
        local inner = { prop = 5 }
        tbl.inner = inner
        tbl.inner.@1
    "#,
  );

  let mut fixture = FragmentAutocompleteFixture::default();
  fixture.base.autocomplete_fragment_in_both_solvers(
    &source,
    &source,
    '1',
    |frag: &mut FragmentAutocompleteStatusResult| {
      LUAU_ASSERT!(frag.result.is_some());
      let ac = &frag.result.as_ref().unwrap().ac_results;
      assert_eq!(1, ac.entry_map.len());
      assert!(ac.entry_map.contains_key("prop"));
      assert_eq!(ac.context, AutocompleteContext::Property);
    },
    None,
  );
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_user_defined_globals() {
  let source = String::from("local myLocal = 4;@1 ");

  let mut fixture = FragmentAutocompleteFixture::default();
  fixture.base.autocomplete_fragment_in_both_solvers(
    &source,
    &source,
    '1',
    |frag: &mut FragmentAutocompleteStatusResult| {
      assert!(frag.result.is_some());
      let ac = &frag.result.as_ref().unwrap().ac_results;

      assert!(ac.entry_map.contains_key("myLocal"));
      assert!(ac.entry_map.contains_key("table"));
      assert!(ac.entry_map.contains_key("math"));
      assert_eq!(ac.context, AutocompleteContext::Statement);
    },
    None,
  );
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_user_defined_local_functions_in_own_definition() {
  let source = String::from(
    r#"
        local function abc()
@1
        end
    "#,
  );

  let mut fixture = FragmentAutocompleteFixture::default();
  // Autocomplete inside of abc
  fixture.base.autocomplete_fragment_in_both_solvers(
    &source,
    &source,
    '1',
    |frag: &mut FragmentAutocompleteStatusResult| {
      assert!(frag.result.is_some());
      let ac = &frag.result.as_ref().unwrap().ac_results;
      assert!(ac.entry_map.contains_key("abc"));
      assert!(ac.entry_map.contains_key("table"));
      assert!(ac.entry_map.contains_key("math"));
    },
    None,
  );
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_user_defined_type_function_local() {
  let source = String::from(
    r#"--!strict
type function foo(x: type): type
    if x.tag == "singleton" then
        local t = x:value()

        return types.unionof(types.singleton(t), types.singleton(nil))
    end

    return types.number
end
"#,
  );

  let dest = String::from(
    r#"--!strict
type function foo(x: type): type
    if x.tag == "singleton" then
        local t = x:value()
        x
        return types.unionof(types.singleton(t), types.singleton(nil))
    end

    return types.number
end
"#,
  );

  let _new_solver = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);
  let mut fixture = FragmentAutocompleteBuiltinsFixture::default();
  fixture.get_frontend();
  let result = fixture.base.check_with_options(&source);
  assert_eq!(0, result.errors.len(), "{:?}", result.errors);

  let result = fixture
    .base
    .autocomplete_fragment(&dest, Position { line: 4, column: 9 }, None);
  assert_ne!(FragmentAutocompleteStatus::InternalIce, result.status);
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_vec_3_function_parameter() {
  let source = String::from(
    r#"
--!strict
local function func(abc : FakeVec)
   abc.@1
end
"#,
  );

  let mut fixture = FragmentAutocompleteBuiltinsFixture::default();
  // C++ `FragmentAutocompleteBuiltinsFixture::getFrontend()` virtually loads the `FakeVec`
  // class declaration into the (auto)globals on first frontend access; prime it here so the
  // shared frontend used by `autocomplete_fragment_in_both_solvers` has `FakeVec` available.
  fixture.get_frontend();
  fixture.base.autocomplete_fragment_in_both_solvers(
    &source,
    &source,
    '1',
    |frag: &mut FragmentAutocompleteStatusResult| {
      LUAU_ASSERT!(frag.result.is_some());
      let ac = &frag.result.as_ref().unwrap().ac_results;
      assert_eq!(2, ac.entry_map.len());
      assert!(ac.entry_map.contains_key("zero"));
      assert!(ac.entry_map.contains_key("dot"));
    },
    None,
  );
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_vec_3_local_function_parameter() {
  let source = String::from(
    r#"
--!strict
local function func(abc : FakeVec)
   abc.@1
end
"#,
  );

  let mut fixture = FragmentAutocompleteBuiltinsFixture::default();
  fixture.base.autocomplete_fragment_in_both_solvers(
    &source,
    &source,
    '1',
    |frag: &mut FragmentAutocompleteStatusResult| {
      assert!(frag.result.is_some());
      let ac_results = &frag.result.as_ref().unwrap().ac_results;
      assert_eq!(2, ac_results.entry_map.len());
      assert!(ac_results.entry_map.contains_key("zero"));
      assert!(ac_results.entry_map.contains_key("dot"));
    },
    None,
  );
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_while_inside_condition_same_line() {
  use ulua_ast::records::ast_stat_while::AstStatWhile;

  let (_fixture, region) = fx_region!(
    r#"
while true do
end
"#,
    Position {
      line: 1,
      column: 13,
    }
  );

  assert_eq!(
    Location {
      begin: Position {
        line: 1,
        column: 13
      },
      end: Position {
        line: 1,
        column: 13
      },
    },
    region.fragment_location
  );
  assert!(!region.parent_block.is_null());
  assert!(crate::is_nearest::<AstStatWhile>(region.nearest_statement));
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_while_writing_func() {
  let (_fixture, region) = fx_region!(
    r#"
function f(arg1,
"#,
    Position {
      line: 1,
      column: 17,
    }
  );

  assert_eq!(
    Location {
      begin: Position { line: 1, column: 0 },
      end: Position {
        line: 1,
        column: 17
      },
    },
    region.fragment_location
  );
  assert!(!region.parent_block.is_null());
  assert!(crate::is_nearest::<AstStatFunction>(
    region.nearest_statement
  ));
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_while_writing_local_func() {
  let (_fixture, region) = fx_region!(
    r#"
local function f(arg1,
"#,
    Position {
      line: 1,
      column: 22,
    }
  );

  assert_eq!(
    Location {
      begin: Position { line: 1, column: 0 },
      end: Position {
        line: 1,
        column: 22
      },
    },
    region.fragment_location
  );
  assert!(!region.parent_block.is_null());
  assert!(crate::is_nearest::<AstStatLocalFunction>(
    region.nearest_statement
  ));
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_writing_func_annotation() {
  let (_fixture, region) = fx_region!(
    r#"
function f(arg1 : T
"#,
    Position {
      line: 1,
      column: 19,
    }
  );

  assert_eq!(
    Location {
      begin: Position { line: 1, column: 0 },
      end: Position {
        line: 1,
        column: 19
      },
    },
    region.fragment_location
  );
  assert!(!region.parent_block.is_null());
  assert!(crate::is_nearest::<AstStatFunction>(
    region.nearest_statement
  ));
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_writing_func_return() {
  let (_fixture, region) = fx_region!(
    r#"
function f(arg1 : T) :
"#,
    Position {
      line: 1,
      column: 22,
    }
  );

  assert_eq!(
    Location {
      begin: Position { line: 1, column: 0 },
      end: Position {
        line: 1,
        column: 22
      },
    },
    region.fragment_location
  );
  assert!(!region.parent_block.is_null());
  assert!(crate::is_nearest::<AstStatFunction>(
    region.nearest_statement
  ));
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_writing_func_return_pack() {
  let (_fixture, region) = fx_region!(
    r#"
function f(arg1 : T) : T...
"#,
    Position {
      line: 1,
      column: 27,
    }
  );

  assert_eq!(
    Location {
      begin: Position { line: 1, column: 0 },
      end: Position {
        line: 1,
        column: 27
      },
    },
    region.fragment_location
  );
  assert!(!region.parent_block.is_null());
  assert!(crate::is_nearest::<AstStatFunction>(
    region.nearest_statement
  ));
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_writing_local_func_annotation() {
  let (_fixture, region) = fx_region!(
    r#"
local function f(arg1 : T
"#,
    Position {
      line: 1,
      column: 25,
    }
  );

  assert_eq!(
    Location {
      begin: Position { line: 1, column: 0 },
      end: Position {
        line: 1,
        column: 25
      },
    },
    region.fragment_location
  );
  assert!(!region.parent_block.is_null());
  assert!(crate::is_nearest::<AstStatLocalFunction>(
    region.nearest_statement
  ));
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_writing_local_func_return() {
  let (_fixture, region) = fx_region!(
    r#"
local function f(arg1 : T) :
"#,
    Position {
      line: 1,
      column: 28,
    }
  );

  assert_eq!(
    Location {
      begin: Position { line: 1, column: 0 },
      end: Position {
        line: 1,
        column: 28
      },
    },
    region.fragment_location
  );
  assert!(!region.parent_block.is_null());
  assert!(crate::is_nearest::<AstStatLocalFunction>(
    region.nearest_statement
  ));
}

// Source: `tests/FragmentAutocomplete.test.cpp`
#[test]
fn fragment_autocomplete_writing_local_func_return_pack() {
  let (_fixture, region) = fx_region!(
    r#"
local function f(arg1 : T) : T...
"#,
    Position {
      line: 1,
      column: 33,
    }
  );

  assert_eq!(
    Location {
      begin: Position { line: 1, column: 0 },
      end: Position {
        line: 1,
        column: 33
      },
    },
    region.fragment_location
  );
  assert!(!region.parent_block.is_null());
  assert!(crate::is_nearest::<AstStatLocalFunction>(
    region.nearest_statement
  ));
}

// Source: `tests/FragmentAutocomplete.test.cpp:5534-5569`
#[test]
fn fragment_autocomplete_fragment_ac_on_nonexistent_table() {
  let source = String::from(
    r#"
        local mygame = {}

        local char = (nil :: any) :: {
            Humanoid: {
                Animator: number
            }
        } & typeof(mygame.interesting)
    "#,
  );

  let updated = String::from(
    r#"
        local mygame = {}

        local char = (nil :: any) :: {
            Humanoid: {
                Animator: number
            }
        } & typeof(mygame.interesting)

        char.Humanoid.@1
    "#,
  );

  let mut fixture = FragmentAutocompleteFixture::default();
  fixture.base.autocomplete_fragment_in_new_solver(
    &source,
    &updated,
    '1',
    |frag: &mut FragmentAutocompleteStatusResult| {
      LUAU_ASSERT!(frag.result.is_some());
      let ac = &frag.result.as_ref().unwrap().ac_results;
      assert!(ac.entry_map.contains_key("Animator"));
    },
    None,
  );
}

// 灭失申报（对照 cpp `FragmentAutocomplete.test.cpp`，tw-4 复核；本文件缺的
// 另 11 例全部挂账如下，非测试侧可表达）：
// - `autocomplete_props_through_metatable_typed_metatable`（:1610，无 flag 依赖）
//   ——与 autocomplete.rs 台账同根因：补全须穿透 `setmetatable` 值元表链
//   （obj -> Meta -> Base）看到 `baseProp`，本移植实测 `entry_map` 为空，属
//   autocomplete 实现缺口，待实现补齐后应补回。
// - `local_inside_of_function_parameter`（:2555）——依赖 FFlag
//   `LuauFragmentACLocalAutocompleteFix`（全仓 0 引用、未同步），faithful
//   前置不可表达。
// - `fragment_autocomplete_type_function_string_singleton_union`（:5571）——
//   依赖 FFlag `LuauFragmentACEnableTypeFunctionEvaluation`；产品侧未注册该
//   可设旗标，`allow_evaluation` 恒 false 硬编码
//   （typecheck_fragment_fragment_autocomplete.rs:188-189），用例无法按 cpp
//   置位，待 sync 该旗标后补回。
// - `if_local_optional_binding_member_completion_in_then_body`（:5607）、
//   `if_local_optional_binding_is_in_scope_in_then_body`（:5643）、
//   `if_local_and_elseif_local_bindings_are_scoped_to_their_own_branch`（:5678）、
//   `elseif_local_binding_offers_member_completion`（:5730）、
//   `nested_if_local_bindings_are_both_in_scope`（:5768）、
//   `if_local_binding_is_not_in_scope_in_else_branch`（:5808）、
//   `if_local_binding_is_not_in_scope_after_if_statement`（:5845）、
//   `if_const_binding_offers_member_completion`（:5880）——8 例均前置
//   `DebugLuauIfLocalSyntax` + `DebugLuauIfLocalAnalysis` 且源码用 `if local`/
//   `if const` 语法；本端口 parser 未接入（ast_expr_if_else.rs:23
//   `condition_local` 构造端恒 None，与 compiler.rs/linter.rs 台账同源），
//   用例源码无法解析，待语法接入后补回。
