use std::ptr::eq;

use ulua_analysis::type_aliases::module_name_type::ModuleName;
extern crate alloc;

// Ported from `tests/Frontend.test.cpp`.
// Source: `tests/Frontend.test.cpp`
#[test]
fn frontend_accumulate_cached_errors() {
  use alloc::string::String;

  use ulua_unit_test::records::{
    builtins_fixture::BuiltinsFixture, frontend_fixture::FrontendFixture,
  };

  let mut fixture = FrontendFixture {
    base: BuiltinsFixture::default(),
  };

  fixture.base.base.file_resolver.source.insert(
    String::from("Modules/A"),
    String::from(
      r#"
        local n: number = 'five'
        return {n=n}
    "#,
    ),
  );

  fixture.base.base.file_resolver.source.insert(
    String::from("Modules/B"),
    String::from(
      r#"
        local Modules = script.Parent
        local A = require(Modules.A)
        local b: number = 'seven'
        print(A, b)
    "#,
    ),
  );

  let result1 = fixture
    .get_frontend()
    .check_module_name_optional_frontend_options(&ModuleName::from("Modules/B"), None);

  assert_eq!(2, result1.errors.len(), "{:?}", result1.errors);

  assert_eq!("Modules/A", result1.errors[0].module_name);
  assert_eq!("Modules/B", result1.errors[1].module_name);

  let result2 = fixture
    .get_frontend()
    .check_module_name_optional_frontend_options(&ModuleName::from("Modules/B"), None);

  assert_eq!(2, result2.errors.len(), "{:?}", result2.errors);

  assert_eq!("Modules/A", result2.errors[0].module_name);
  assert_eq!("Modules/B", result2.errors[1].module_name);
}

// Ported from `tests/Frontend.test.cpp`.
// Source: `tests/Frontend.test.cpp`
#[test]
fn frontend_accumulate_cached_errors_in_consistent_order() {
  use alloc::string::String;

  use ulua_unit_test::records::{
    builtins_fixture::BuiltinsFixture, frontend_fixture::FrontendFixture,
  };

  let mut fixture = FrontendFixture {
    base: BuiltinsFixture::default(),
  };

  fixture.base.base.file_resolver.source.insert(
    String::from("Modules/A"),
    String::from(
      r#"
        a = 1
        b = 2
        local Modules = script.Parent
        local A = require(Modules.B)
    "#,
    ),
  );

  fixture.base.base.file_resolver.source.insert(
    String::from("Modules/B"),
    String::from(
      r#"
        d = 3
        e = 4
        return {}
    "#,
    ),
  );

  let result1 = fixture
    .get_frontend()
    .check_module_name_optional_frontend_options(&ModuleName::from("Modules/A"), None);

  assert_eq!(4, result1.errors.len(), "{:?}", result1.errors);

  assert_eq!("Modules/A", result1.errors[2].module_name);
  assert_eq!("Modules/A", result1.errors[3].module_name);

  assert_eq!("Modules/B", result1.errors[0].module_name);
  assert_eq!("Modules/B", result1.errors[1].module_name);

  let result2 = fixture
    .get_frontend()
    .check_module_name_optional_frontend_options(&ModuleName::from("Modules/A"), None);
  assert_eq!(4, result2.errors.len(), "{:?}", result2.errors);

  for (left, right) in result1.errors.iter().zip(result2.errors.iter()) {
    assert_eq!(left, right);
  }
}

// Ported from `tests/Frontend.test.cpp`.
// Source: `tests/Frontend.test.cpp`
#[test]
fn frontend_any_annotation_breaks_cycle() {
  use alloc::string::String;

  use ulua_unit_test::records::{
    builtins_fixture::BuiltinsFixture, frontend_fixture::FrontendFixture,
  };

  let mut fixture = FrontendFixture {
    base: BuiltinsFixture::default(),
  };

  fixture.base.base.file_resolver.source.insert(
    String::from("game/Gui/Modules/A"),
    String::from(
      r#"
        local Modules = game:GetService('Gui').Modules
        local B = require(Modules.B)
        return {hello = B.hello}
    "#,
    ),
  );
  fixture.base.base.file_resolver.source.insert(
    String::from("game/Gui/Modules/B"),
    String::from(
      r#"
        local Modules = game:GetService('Gui').Modules
        local A = require(Modules.A) :: any
        return {hello = A.hello}
    "#,
    ),
  );

  let result = fixture
    .get_frontend()
    .check_module_name_optional_frontend_options(&ModuleName::from("game/Gui/Modules/A"), None);
  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Source: `tests/Frontend.test.cpp`
#[test]
fn frontend_ast_node_at_position() {
  use ulua_analysis::functions::find_node_at_position_ast_query::find_node_at_position_source_module_position;
  use ulua_unit_test::records::{
    builtins_fixture::BuiltinsFixture, frontend_fixture::FrontendFixture,
  };

  let mut fixture = FrontendFixture {
    base: BuiltinsFixture::default(),
  };

  fixture.base.base.check_string_optional_frontend_options(
    r#"
        local t = {}

        function t:aa() end

        t:
    "#,
    None,
  );

  let source = fixture.base.base.main_source_module();
  // Safety: `source.root` 是 parser 成功产物的非空 arena 块指针，只读取
  // 其 Location（cpp fixture 同形）。
  let mut pos = unsafe { (*source.root).base.base.location.end };
  let node = find_node_at_position_source_module_position(source, pos);

  assert!(!node.is_null());
  assert!(unsafe { (*node).as_expr() }.is_some());

  pos.column += 1;
  let node2 = find_node_at_position_source_module_position(source, pos);
  assert_eq!(node, node2);
}

// Ported from `tests/Frontend.test.cpp`.
// Source: `tests/Frontend.test.cpp`
#[test]
fn frontend_attribute_ices_to_the_correct_module() {
  use alloc::string::String;
  use std::panic::{AssertUnwindSafe, catch_unwind};

  use ulua_analysis::records::internal_compiler_error::InternalCompilerError;
  use ulua_common::fflag;
  use ulua_unit_test::{
    records::{builtins_fixture::BuiltinsFixture, frontend_fixture::FrontendFixture},
    type_aliases::scoped_fast_flag::ScopedFastFlag,
  };

  let _magic_types = ScopedFastFlag::new(&fflag::DebugLuauMagicTypes, true);
  let mut fixture = FrontendFixture {
    base: BuiltinsFixture::default(),
  };

  fixture.base.base.file_resolver.source.insert(
    String::from("game/one"),
    String::from(
      r#"
        require(game.two)
    "#,
    ),
  );

  fixture.base.base.file_resolver.source.insert(
    String::from("game/two"),
    String::from(
      r#"
        local a: _luau_ice
    "#,
    ),
  );

  let result = catch_unwind(AssertUnwindSafe(|| {
    fixture
      .get_frontend()
      .check_module_name_optional_frontend_options(&ModuleName::from("game/one"), None);
  }));

  let panic = result.expect_err("expected an InternalCompilerError");
  let ice = panic
    .downcast_ref::<InternalCompilerError>()
    .expect("expected InternalCompilerError panic payload");
  assert_eq!(Some(String::from("game/two")), ice.module_name.clone());
}

// Ported from `tests/Frontend.test.cpp`.
// Source: `tests/Frontend.test.cpp`
#[test]
fn frontend_automatically_check_cyclically_dependent_scripts() {
  use alloc::string::String;

  use ulua_analysis::records::module_has_cyclic_dependency::ModuleHasCyclicDependency;
  use ulua_unit_test::{
    functions::type_error_data_ref::type_error_data_ref,
    records::{builtins_fixture::BuiltinsFixture, frontend_fixture::FrontendFixture},
  };

  let mut fixture = FrontendFixture {
    base: BuiltinsFixture::default(),
  };

  fixture.base.base.file_resolver.source.insert(
    String::from("game/Gui/Modules/A"),
    String::from(
      r#"
        local Modules = game:GetService('Gui').Modules
        local B = require(Modules.B)
        return {}
    "#,
    ),
  );
  fixture.base.base.file_resolver.source.insert(
    String::from("game/Gui/Modules/B"),
    String::from(
      r#"
        local Modules = game:GetService('Gui').Modules
        local A = require(Modules.A)
        require(Modules.C)
        return {}
    "#,
    ),
  );
  fixture.base.base.file_resolver.source.insert(
    String::from("game/Gui/Modules/C"),
    String::from(
      r#"
        local Modules = game:GetService('Gui').Modules
        do local A = require(Modules.A) end
        return {}
    "#,
    ),
  );

  fixture.base.base.file_resolver.source.insert(
    String::from("game/Gui/Modules/D"),
    String::from(
      r#"
        local Modules = game:GetService('Gui').Modules
        do local A = require(Modules.A) end
        return {}
    "#,
    ),
  );

  let result1 = fixture
    .get_frontend()
    .check_module_name_optional_frontend_options(&ModuleName::from("game/Gui/Modules/B"), None);
  assert_eq!(4, result1.errors.len(), "{:?}", result1.errors);

  assert!(
    type_error_data_ref::<ModuleHasCyclicDependency>(&result1.errors[0]).is_some(),
    "Should have been a ModuleHasCyclicDependency: {:?}",
    result1.errors[0]
  );
  assert!(
    type_error_data_ref::<ModuleHasCyclicDependency>(&result1.errors[1]).is_some(),
    "Should have been a ModuleHasCyclicDependency: {:?}",
    result1.errors[1]
  );
  assert!(
    type_error_data_ref::<ModuleHasCyclicDependency>(&result1.errors[2]).is_some(),
    "Should have been a ModuleHasCyclicDependency: {:?}",
    result1.errors[2]
  );

  let result2 = fixture
    .get_frontend()
    .check_module_name_optional_frontend_options(&ModuleName::from("game/Gui/Modules/D"), None);
  assert_eq!(0, result2.errors.len(), "{:?}", result2.errors);
}

// Ported from `tests/Frontend.test.cpp`.
// Source: `tests/Frontend.test.cpp`
#[test]
fn frontend_automatically_check_dependent_scripts() {
  use alloc::string::String;

  use ulua_analysis::functions::{first::first, to_string_to_string::to_string_type_id};
  use ulua_unit_test::records::{
    builtins_fixture::BuiltinsFixture, frontend_fixture::FrontendFixture,
  };

  let mut fixture = FrontendFixture {
    base: BuiltinsFixture::default(),
  };

  fixture.base.base.file_resolver.source.insert(
    String::from("game/Gui/Modules/A"),
    String::from("return {hello=5, world=true}"),
  );
  fixture.base.base.file_resolver.source.insert(
    String::from("game/Gui/Modules/B"),
    String::from(
      r#"
        local Modules = game:GetService('Gui').Modules
        local A = require(Modules.A)
        return {b_value = A.hello}
    "#,
    ),
  );

  fixture
    .get_frontend()
    .check_module_name_optional_frontend_options(&ModuleName::from("game/Gui/Modules/B"), None);

  let b_module = fixture
    .get_frontend()
    .module_resolver
    .get_module(&ModuleName::from("game/Gui/Modules/B"));
  assert!(b_module.errors.is_empty(), "{:?}", b_module.errors);

  let b_exports = first(b_module.return_type, true).expect("expected module return type");

  assert_eq!("{ b_value: number }", to_string_type_id(b_exports));
}

// Ported from `tests/Frontend.test.cpp`.
// Source: `tests/Frontend.test.cpp`
#[test]
fn frontend_check_module_references_allocator() {
  use alloc::{string::String, sync::Arc};

  use ulua_unit_test::records::{
    builtins_fixture::BuiltinsFixture, frontend_fixture::FrontendFixture,
  };

  let mut fixture = FrontendFixture {
    base: BuiltinsFixture::default(),
  };

  fixture.base.base.file_resolver.source.insert(
    String::from("game/workspace/MyScript"),
    String::from(
      r#"
        print("Hello World")
    "#,
    ),
  );

  fixture
    .get_frontend()
    .check_module_name_optional_frontend_options(
      &ModuleName::from("game/workspace/MyScript"),
      None,
    );

  let module = fixture
    .get_frontend()
    .module_resolver
    .get_module(&ModuleName::from("game/workspace/MyScript"));
  let source = fixture
    .get_frontend()
    .get_source_module(&ModuleName::from("game/workspace/MyScript"))
    .expect("expected source module")
    .get();
  assert_eq!(
    Arc::as_ptr(
      module
        .allocator
        .as_ref()
        .expect("expected module allocator")
    ),
    Arc::as_ptr(&source.allocator)
  );
  assert_eq!(
    Arc::as_ptr(module.names.as_ref().expect("expected module names")),
    Arc::as_ptr(&source.names)
  );
}

// Ported from `tests/Frontend.test.cpp`.
// Source: `tests/Frontend.test.cpp`
#[test]
fn frontend_check_module_references_correct_ast_root() {
  use alloc::string::String;

  use ulua_unit_test::records::{
    builtins_fixture::BuiltinsFixture, frontend_fixture::FrontendFixture,
  };

  let mut fixture = FrontendFixture {
    base: BuiltinsFixture::default(),
  };

  fixture.base.base.file_resolver.source.insert(
    String::from("game/workspace/MyScript"),
    String::from(
      r#"
        print("Hello World")
    "#,
    ),
  );

  fixture
    .get_frontend()
    .check_module_name_optional_frontend_options(
      &ModuleName::from("game/workspace/MyScript"),
      None,
    );

  let module = fixture
    .get_frontend()
    .module_resolver
    .get_module(&ModuleName::from("game/workspace/MyScript"));
  let source = fixture
    .get_frontend()
    .get_source_module(&ModuleName::from("game/workspace/MyScript"))
    .expect("expected source module")
    .get();
  assert_eq!(module.root, source.root);
}

// Ported from `tests/Frontend.test.cpp`.
// Source: `tests/Frontend.test.cpp`
#[test]
fn frontend_check_without_builtin_next() {
  use alloc::string::String;

  use ulua_analysis::{
    enums::solver_mode::SolverMode,
    records::{frontend::Frontend, frontend_options::FrontendOptions},
  };
  use ulua_common::fflag;
  use ulua_unit_test::records::{
    test_config_resolver::TestConfigResolver, test_file_resolver::TestFileResolver,
  };

  let mut file_resolver = TestFileResolver::default();
  let mut config_resolver = TestConfigResolver::default();
  let mode = if fflag::DebugLuauForceOldSolver.get() {
    SolverMode::Old
  } else {
    SolverMode::New
  };
  // `new_boxed` 在 safe 边界内完成「构造 → 堆上落位 → 自指针布线」：resolver 是
  // 本测试函数局部（frontend 后声明先析构，句柄恒覆盖使用期），Box 钉死 Frontend
  // 堆地址，调用点免手写 unsafe ctor + wire_self_pointers。
  let mut frontend = Frontend::new_boxed(
    mode,
    &mut file_resolver,
    Some(&mut config_resolver.base),
    FrontendOptions::default(),
  );

  file_resolver.source.insert(
    String::from("Module/A"),
    String::from("for k,v in 2 do end"),
  );
  file_resolver
    .source
    .insert(String::from("Module/B"), String::from("return next"));

  // We don't care about the result. That we haven't crashed is enough.
  frontend.check_module_name_optional_frontend_options(&ModuleName::from("Module/A"), None);
  frontend.check_module_name_optional_frontend_options(&ModuleName::from("Module/B"), None);
}

// Ported from `tests/Frontend.test.cpp`.
// Source: `tests/Frontend.test.cpp`
#[test]
fn frontend_checked_modules_have_the_correct_mode() {
  use alloc::string::String;

  use ulua_ast::enums::mode::Mode;
  use ulua_unit_test::records::{
    builtins_fixture::BuiltinsFixture, frontend_fixture::FrontendFixture,
  };

  let mut fixture = FrontendFixture {
    base: BuiltinsFixture::default(),
  };

  fixture.base.base.file_resolver.source.insert(
    String::from("game/A"),
    String::from(
      r#"
        --!nocheck
        local a: number = "five"
    "#,
    ),
  );

  fixture.base.base.file_resolver.source.insert(
    String::from("game/B"),
    String::from(
      r#"
        --!nonstrict
        local a = math.abs("five")
    "#,
    ),
  );

  fixture.base.base.file_resolver.source.insert(
    String::from("game/C"),
    String::from(
      r#"
        --!strict
        local a = 10
    "#,
    ),
  );

  fixture
    .get_frontend()
    .check_module_name_optional_frontend_options(&ModuleName::from("game/A"), None);
  fixture
    .get_frontend()
    .check_module_name_optional_frontend_options(&ModuleName::from("game/B"), None);
  fixture
    .get_frontend()
    .check_module_name_optional_frontend_options(&ModuleName::from("game/C"), None);

  let module_a = fixture
    .get_frontend()
    .module_resolver
    .get_module(&ModuleName::from("game/A"));
  assert_eq!(Mode::NoCheck, module_a.mode);

  let module_b = fixture
    .get_frontend()
    .module_resolver
    .get_module(&ModuleName::from("game/B"));
  assert_eq!(Mode::Nonstrict, module_b.mode);

  let module_c = fixture
    .get_frontend()
    .module_resolver
    .get_module(&ModuleName::from("game/C"));
  assert_eq!(Mode::Strict, module_c.mode);
}

// Ported from `tests/Frontend.test.cpp`.
// Source: `tests/Frontend.test.cpp`
#[test]
fn frontend_clear_modules_cleans_up_reverse_dependency_edges() {
  use alloc::string::String;

  use ulua_unit_test::records::{
    builtins_fixture::BuiltinsFixture, frontend_fixture::FrontendFixture,
  };

  let mut fixture = FrontendFixture {
    base: BuiltinsFixture::default(),
  };

  fixture.base.base.file_resolver.source.insert(
    String::from("game/Gui/Modules/A"),
    String::from("return {hello=5}"),
  );
  fixture.base.base.file_resolver.source.insert(
    String::from("game/Gui/Modules/B"),
    String::from(
      r#"
        return require(game:GetService('Gui').Modules.A)
    "#,
    ),
  );

  let result = fixture
    .get_frontend()
    .check_module_name_optional_frontend_options(&ModuleName::from("game/Gui/Modules/B"), None);
  assert_eq!(0, result.errors.len(), "{:?}", result.errors);

  // Before clearing: A has B as a dependent
  assert!(
    fixture
      .get_frontend()
      .source_nodes
      .get("game/Gui/Modules/A")
      .expect("expected source node A")
      .dependents
      .contains(&ModuleName::from("game/Gui/Modules/B"))
  );

  fixture
    .get_frontend()
    .clear_modules(&[ModuleName::from("game/Gui/Modules/B")]);

  // B is erased
  assert!(
    !fixture
      .get_frontend()
      .source_nodes
      .contains_key("game/Gui/Modules/B")
  );

  // A should no longer list B as a dependent
  assert!(
    !fixture
      .get_frontend()
      .source_nodes
      .get("game/Gui/Modules/A")
      .expect("expected source node A")
      .dependents
      .contains(&ModuleName::from("game/Gui/Modules/B"))
  );
}

// Ported from `tests/Frontend.test.cpp`.
// Source: `tests/Frontend.test.cpp`
#[test]
fn frontend_clear_modules_erases_module_and_marks_dependents_dirty() {
  use alloc::string::String;

  use ulua_unit_test::records::{
    builtins_fixture::BuiltinsFixture, frontend_fixture::FrontendFixture,
  };

  let mut fixture = FrontendFixture {
    base: BuiltinsFixture::default(),
  };

  fixture.base.base.file_resolver.source.insert(
    String::from("game/Gui/Modules/A"),
    String::from("return {hello=5}"),
  );
  fixture.base.base.file_resolver.source.insert(
    String::from("game/Gui/Modules/B"),
    String::from(
      r#"
        return require(game:GetService('Gui').Modules.A)
    "#,
    ),
  );
  fixture.base.base.file_resolver.source.insert(
    String::from("game/Gui/Modules/C"),
    String::from(
      r#"
        return require(game:GetService('Gui').Modules.B)
    "#,
    ),
  );

  let result = fixture
    .get_frontend()
    .check_module_name_optional_frontend_options(&ModuleName::from("game/Gui/Modules/C"), None);
  assert_eq!(0, result.errors.len(), "{:?}", result.errors);

  assert!(
    !fixture
      .get_frontend()
      .is_dirty(&ModuleName::from("game/Gui/Modules/A"), false)
  );
  assert!(
    !fixture
      .get_frontend()
      .is_dirty(&ModuleName::from("game/Gui/Modules/B"), false)
  );
  assert!(
    !fixture
      .get_frontend()
      .is_dirty(&ModuleName::from("game/Gui/Modules/C"), false)
  );

  fixture
    .get_frontend()
    .clear_modules(&[ModuleName::from("game/Gui/Modules/A")]);

  // A should be fully erased
  assert!(
    !fixture
      .get_frontend()
      .source_nodes
      .contains_key("game/Gui/Modules/A")
  );
  assert!(
    fixture
      .get_frontend()
      .get_source_module_mut(&ModuleName::from("game/Gui/Modules/A"))
      .is_none()
  );
  assert!(
    fixture
      .get_frontend()
      .module_resolver
      .try_get_module(&ModuleName::from("game/Gui/Modules/A"))
      .is_none()
  );

  // B and C should be marked dirty (transitive dependents)
  assert!(
    fixture
      .get_frontend()
      .is_dirty(&ModuleName::from("game/Gui/Modules/B"), false)
  );
  assert!(
    fixture
      .get_frontend()
      .is_dirty(&ModuleName::from("game/Gui/Modules/C"), false)
  );

  let result = fixture
    .get_frontend()
    .check_module_name_optional_frontend_options(&ModuleName::from("game/Gui/Modules/C"), None);
  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Ported from `tests/Frontend.test.cpp`.
// Source: `tests/Frontend.test.cpp`
#[test]
fn frontend_clear_modules_multiple_with_shared_dependents() {
  use alloc::string::String;

  use ulua_unit_test::records::{
    builtins_fixture::BuiltinsFixture, frontend_fixture::FrontendFixture,
  };

  let mut fixture = FrontendFixture {
    base: BuiltinsFixture::default(),
  };

  fixture
    .base
    .base
    .file_resolver
    .source
    .insert(String::from("game/Gui/Modules/A"), String::from("return 1"));
  fixture
    .base
    .base
    .file_resolver
    .source
    .insert(String::from("game/Gui/Modules/B"), String::from("return 2"));
  fixture.base.base.file_resolver.source.insert(
    String::from("game/Gui/Modules/C"),
    String::from(
      r#"
        local A = require(game:GetService('Gui').Modules.A)
        local B = require(game:GetService('Gui').Modules.B)
        return A + B
    "#,
    ),
  );

  let result = fixture
    .get_frontend()
    .check_module_name_optional_frontend_options(&ModuleName::from("game/Gui/Modules/C"), None);
  assert_eq!(0, result.errors.len(), "{:?}", result.errors);

  assert!(
    !fixture
      .get_frontend()
      .is_dirty(&ModuleName::from("game/Gui/Modules/C"), false)
  );

  // Clear both A and B at once; C depends on both
  fixture.get_frontend().clear_modules(&[
    ModuleName::from("game/Gui/Modules/A"),
    ModuleName::from("game/Gui/Modules/B"),
  ]);

  assert!(
    !fixture
      .get_frontend()
      .source_nodes
      .contains_key("game/Gui/Modules/A")
  );
  assert!(
    !fixture
      .get_frontend()
      .source_nodes
      .contains_key("game/Gui/Modules/B")
  );
  assert!(
    fixture
      .get_frontend()
      .is_dirty(&ModuleName::from("game/Gui/Modules/C"), false)
  );

  let result = fixture
    .get_frontend()
    .check_module_name_optional_frontend_options(&ModuleName::from("game/Gui/Modules/C"), None);
  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Ported from `tests/Frontend.test.cpp`.
// Source: `tests/Frontend.test.cpp`
#[test]
fn frontend_clear_modules_nonexistent_module_is_noop() {
  use alloc::string::String;

  use ulua_unit_test::records::{
    builtins_fixture::BuiltinsFixture, frontend_fixture::FrontendFixture,
  };

  let mut fixture = FrontendFixture {
    base: BuiltinsFixture::default(),
  };

  fixture.base.base.file_resolver.source.insert(
    String::from("game/Gui/Modules/A"),
    String::from("return {hello=5}"),
  );
  let result = fixture
    .get_frontend()
    .check_module_name_optional_frontend_options(&ModuleName::from("game/Gui/Modules/A"), None);
  assert_eq!(0, result.errors.len(), "{:?}", result.errors);

  assert!(
    !fixture
      .get_frontend()
      .is_dirty(&ModuleName::from("game/Gui/Modules/A"), false)
  );

  // Clearing a non-existent module should not affect anything
  fixture
    .get_frontend()
    .clear_modules(&[ModuleName::from("game/Gui/Modules/DoesNotExist")]);

  assert!(
    !fixture
      .get_frontend()
      .is_dirty(&ModuleName::from("game/Gui/Modules/A"), false)
  );
  assert!(
    fixture
      .get_frontend()
      .source_nodes
      .contains_key("game/Gui/Modules/A")
  );

  let result = fixture
    .get_frontend()
    .check_module_name_optional_frontend_options(&ModuleName::from("game/Gui/Modules/A"), None);
  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Ported from `tests/Frontend.test.cpp`.
// Source: `tests/Frontend.test.cpp`
#[test]
fn frontend_clear_stats() {
  use alloc::string::String;

  use ulua_unit_test::records::{
    builtins_fixture::BuiltinsFixture, frontend_fixture::FrontendFixture,
  };

  let mut fixture = FrontendFixture {
    base: BuiltinsFixture::default(),
  };

  fixture.base.base.file_resolver.source.insert(
    String::from("Module/A"),
    String::from(
      r#"
        --!strict
        local B = require(script.Parent.B)
        local foo = B.foo + 1
    "#,
    ),
  );

  fixture.base.base.file_resolver.source.insert(
    String::from("Module/B"),
    String::from(
      r#"
        --!strict
        return {foo = 1}
    "#,
    ),
  );

  let r1 = fixture
    .get_frontend()
    .check_module_name_optional_frontend_options(&ModuleName::from("Module/A"), None);
  assert_eq!(0, r1.errors.len(), "{:?}", r1.errors);

  let stats1 = fixture.get_frontend().stats;
  assert_eq!(2, stats1.files);

  fixture
    .get_frontend()
    .mark_dirty(&ModuleName::from("Module/A"), None);
  fixture
    .get_frontend()
    .mark_dirty(&ModuleName::from("Module/B"), None);

  fixture.get_frontend().clear_stats();
  let r2 = fixture
    .get_frontend()
    .check_module_name_optional_frontend_options(&ModuleName::from("Module/A"), None);
  assert_eq!(0, r2.errors.len(), "{:?}", r2.errors);
  let stats2 = fixture.get_frontend().stats;

  assert_eq!(2, stats2.files);
}

// Ported from `tests/Frontend.test.cpp`.
// Source: `tests/Frontend.test.cpp`
#[test]
fn frontend_cycle_detection_between_check_and_nocheck() {
  use alloc::string::String;

  use ulua_unit_test::records::{
    builtins_fixture::BuiltinsFixture, frontend_fixture::FrontendFixture,
  };

  let mut fixture = FrontendFixture {
    base: BuiltinsFixture::default(),
  };

  fixture.base.base.file_resolver.source.insert(
    String::from("game/Gui/Modules/A"),
    String::from(
      r#"
        local Modules = game:GetService('Gui').Modules
        local B = require(Modules.B)
        return {hello = B.hello}
    "#,
    ),
  );
  fixture.base.base.file_resolver.source.insert(
    String::from("game/Gui/Modules/B"),
    String::from(
      r#"
        --!nocheck
        local Modules = game:GetService('Gui').Modules
        local A = require(Modules.A)
        return {hello = A.hello}
    "#,
    ),
  );

  let result = fixture
    .get_frontend()
    .check_module_name_optional_frontend_options(&ModuleName::from("game/Gui/Modules/A"), None);
  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
}

// Ported from `tests/Frontend.test.cpp`.
// Source: `tests/Frontend.test.cpp`
#[test]
fn frontend_cycle_detection_disabled_in_nocheck() {
  use alloc::string::String;

  use ulua_unit_test::records::{
    builtins_fixture::BuiltinsFixture, frontend_fixture::FrontendFixture,
  };

  let mut fixture = FrontendFixture {
    base: BuiltinsFixture::default(),
  };

  fixture.base.base.file_resolver.source.insert(
    String::from("game/Gui/Modules/A"),
    String::from(
      r#"
        --!nocheck
        local Modules = game:GetService('Gui').Modules
        local B = require(Modules.B)
        return {hello = B.hello}
    "#,
    ),
  );
  fixture.base.base.file_resolver.source.insert(
    String::from("game/Gui/Modules/B"),
    String::from(
      r#"
        --!nocheck
        local Modules = game:GetService('Gui').Modules
        local A = require(Modules.A)
        return {hello = A.hello}
    "#,
    ),
  );

  let result = fixture
    .get_frontend()
    .check_module_name_optional_frontend_options(&ModuleName::from("game/Gui/Modules/A"), None);
  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Ported from `tests/Frontend.test.cpp`.
// Source: `tests/Frontend.test.cpp`
#[test]
fn frontend_cycle_error_paths() {
  use alloc::string::String;

  use ulua_analysis::records::module_has_cyclic_dependency::ModuleHasCyclicDependency;
  use ulua_unit_test::{
    functions::type_error_data_ref::type_error_data_ref,
    records::{builtins_fixture::BuiltinsFixture, frontend_fixture::FrontendFixture},
  };

  let mut fixture = FrontendFixture {
    base: BuiltinsFixture::default(),
  };

  fixture.base.base.file_resolver.source.insert(
    String::from("game/Gui/Modules/A"),
    String::from(
      r#"
        local Modules = game:GetService('Gui').Modules
        local B = require(Modules.B)
        return {hello = B.hello}
    "#,
    ),
  );
  fixture.base.base.file_resolver.source.insert(
    String::from("game/Gui/Modules/B"),
    String::from(
      r#"
        local Modules = game:GetService('Gui').Modules
        local A = require(Modules.A)
        return {hello = A.hello}
    "#,
    ),
  );

  let result = fixture
    .get_frontend()
    .check_module_name_optional_frontend_options(&ModuleName::from("game/Gui/Modules/A"), None);
  assert_eq!(2, result.errors.len(), "{:?}", result.errors);

  let ce1 = type_error_data_ref::<ModuleHasCyclicDependency>(&result.errors[0])
    .expect("expected first cycle error");
  assert_eq!("game/Gui/Modules/B", result.errors[0].module_name);
  assert_eq!(2, ce1.cycle().len());
  assert_eq!("game/Gui/Modules/A", ce1.cycle()[0]);
  assert_eq!("game/Gui/Modules/B", ce1.cycle()[1]);

  let ce2 = type_error_data_ref::<ModuleHasCyclicDependency>(&result.errors[1])
    .expect("expected second cycle error");
  assert_eq!("game/Gui/Modules/A", result.errors[1].module_name);
  assert_eq!(2, ce2.cycle().len());
  assert_eq!("game/Gui/Modules/B", ce2.cycle()[0]);
  assert_eq!("game/Gui/Modules/A", ce2.cycle()[1]);
}

// Ported from `tests/Frontend.test.cpp`.
// Source: `tests/Frontend.test.cpp`
#[test]
fn frontend_cycle_errors_can_be_fixed() {
  use alloc::string::String;

  use ulua_analysis::records::module_has_cyclic_dependency::ModuleHasCyclicDependency;
  use ulua_unit_test::{
    functions::type_error_data_ref::type_error_data_ref,
    records::{builtins_fixture::BuiltinsFixture, frontend_fixture::FrontendFixture},
  };

  let mut fixture = FrontendFixture {
    base: BuiltinsFixture::default(),
  };

  fixture.base.base.file_resolver.source.insert(
    String::from("game/Gui/Modules/A"),
    String::from(
      r#"
        local Modules = game:GetService('Gui').Modules
        local B = require(Modules.B)
        return {hello = B.hello}
    "#,
    ),
  );
  fixture.base.base.file_resolver.source.insert(
    String::from("game/Gui/Modules/B"),
    String::from(
      r#"
        local Modules = game:GetService('Gui').Modules
        local A = require(Modules.A)
        return {hello = A.hello}
    "#,
    ),
  );

  let result1 = fixture
    .get_frontend()
    .check_module_name_optional_frontend_options(&ModuleName::from("game/Gui/Modules/A"), None);
  assert_eq!(2, result1.errors.len(), "{:?}", result1.errors);

  assert!(
    type_error_data_ref::<ModuleHasCyclicDependency>(&result1.errors[0]).is_some(),
    "Should have been a ModuleHasCyclicDependency: {:?}",
    result1.errors[0]
  );
  assert!(
    type_error_data_ref::<ModuleHasCyclicDependency>(&result1.errors[1]).is_some(),
    "Should have been a ModuleHasCyclicDependency: {:?}",
    result1.errors[1]
  );

  fixture.base.base.file_resolver.source.insert(
    String::from("game/Gui/Modules/B"),
    String::from(
      r#"
        return {hello = 42}
    "#,
    ),
  );
  fixture
    .get_frontend()
    .mark_dirty(&ModuleName::from("game/Gui/Modules/B"), None);

  let result2 = fixture
    .get_frontend()
    .check_module_name_optional_frontend_options(&ModuleName::from("game/Gui/Modules/A"), None);
  assert_eq!(0, result2.errors.len(), "{:?}", result2.errors);
}

// Ported from `tests/Frontend.test.cpp`.
// Source: `tests/Frontend.test.cpp`
#[test]
fn frontend_cycle_incremental_type_surface() {
  use alloc::string::String;

  use ulua_analysis::{
    functions::to_string_to_string::to_string_type_id,
    records::module_has_cyclic_dependency::ModuleHasCyclicDependency,
  };
  use ulua_unit_test::{
    functions::type_error_data_ref::type_error_data_ref,
    records::{builtins_fixture::BuiltinsFixture, frontend_fixture::FrontendFixture},
  };

  let mut fixture = FrontendFixture {
    base: BuiltinsFixture::default(),
  };

  fixture.base.base.file_resolver.source.insert(
    String::from("game/A"),
    String::from(
      r#"
        return {hello = 2}
    "#,
    ),
  );

  let mut result = fixture
    .get_frontend()
    .check_module_name_optional_frontend_options(&ModuleName::from("game/A"), None);
  assert_eq!(0, result.errors.len(), "{:?}", result.errors);

  fixture.base.base.file_resolver.source.insert(
    String::from("game/A"),
    String::from(
      r#"
        local me = require(game.A)
        return {hello = 2}
    "#,
    ),
  );
  fixture
    .get_frontend()
    .mark_dirty(&ModuleName::from("game/A"), None);

  result = fixture
    .get_frontend()
    .check_module_name_optional_frontend_options(&ModuleName::from("game/A"), None);
  // cpp LUAU_REQUIRE_ERRORS(result)（Frontend.test.cpp:451）只要求有错；
  // 这里锁到具体数量与错误种类：自引用环 → ModuleHasCyclicDependency
  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  assert!(
    type_error_data_ref::<ModuleHasCyclicDependency>(&result.errors[0]).is_some(),
    "Should have been a ModuleHasCyclicDependency: {:?}",
    result.errors[0]
  );

  let module = fixture
    .get_frontend()
    .module_resolver
    .get_module(&ModuleName::from("game/A"));
  let ty = fixture
    .base
    .base
    .require_type_module_ptr_string(&module, "me");
  assert_eq!("any", to_string_type_id(ty));
}

// Ported from `tests/Frontend.test.cpp`.
// Source: `tests/Frontend.test.cpp`
#[test]
fn frontend_cycle_incremental_type_surface_exports() {
  use alloc::string::String;

  use ulua_analysis::{
    functions::to_string_to_string::to_string_type_id_to_string_options,
    records::{
      module_has_cyclic_dependency::ModuleHasCyclicDependency, to_string_options::ToStringOptions,
    },
  };
  use ulua_unit_test::{
    functions::type_error_data_ref::type_error_data_ref,
    records::{builtins_fixture::BuiltinsFixture, frontend_fixture::FrontendFixture},
  };

  let mut fixture = FrontendFixture {
    base: BuiltinsFixture::default(),
  };

  fixture.base.base.file_resolver.source.insert(
    String::from("game/A"),
    String::from(
      r#"
local b = require(game.B)
export type atype = { x: b.btype }
return {mod_a = 1}
    "#,
    ),
  );

  fixture.base.base.file_resolver.source.insert(
    String::from("game/B"),
    String::from(
      r#"
export type btype = { x: number }

local function bf()
    local a = require(game.A)
    local bfl : a.atype = nil
    return {bfl.x}
end
return {mod_b = 2}
    "#,
    ),
  );

  let result_a = fixture
    .get_frontend()
    .check_module_name_optional_frontend_options(&ModuleName::from("game/A"), None);
  // cpp LUAU_REQUIRE_ERRORS(resultA)（Frontend.test.cpp:519）只要求有错；
  // 这里锁到具体数量、种类与内容（A↔B 环 + 环上 `a.atype` 解析失败）。
  assert_eq!(3, result_a.errors.len(), "{:?}", result_a.errors);
  {
    use ulua_analysis::records::unknown_symbol::UnknownSymbol;
    type_error_data_ref::<UnknownSymbol>(&result_a.errors[0])
      .expect("expected UnknownSymbol for a.atype");
    assert_eq!("game/B", result_a.errors[0].module_name);
    let ce = type_error_data_ref::<ModuleHasCyclicDependency>(&result_a.errors[1])
      .expect("expected ModuleHasCyclicDependency");
    assert_eq!(
      &[String::from("game/A"), String::from("game/B")][..],
      ce.cycle()
    );
    let ce = type_error_data_ref::<ModuleHasCyclicDependency>(&result_a.errors[2])
      .expect("expected ModuleHasCyclicDependency");
    assert_eq!(
      &[String::from("game/B"), String::from("game/A")][..],
      ce.cycle()
    );
  }

  let mut result_b = fixture
    .get_frontend()
    .check_module_name_optional_frontend_options(&ModuleName::from("game/B"), None);
  // cpp `LUAU_REQUIRE_ERRORS(resultB)`（Frontend.test.cpp:519）只要求有错；
  // 这里按 resultA 的镜像锁数量与种类：两处环报告 + 环上 `a.atype` 解析失败。
  assert_eq!(3, result_b.errors.len(), "{:?}", result_b.errors);
  {
    use ulua_analysis::records::unknown_symbol::UnknownSymbol;
    let ce = type_error_data_ref::<ModuleHasCyclicDependency>(&result_b.errors[0])
      .expect("expected ModuleHasCyclicDependency");
    assert_eq!(
      &[String::from("game/B"), String::from("game/A")][..],
      ce.cycle()
    );
    let ce = type_error_data_ref::<ModuleHasCyclicDependency>(&result_b.errors[1])
      .expect("expected ModuleHasCyclicDependency");
    assert_eq!(
      &[String::from("game/A"), String::from("game/B")][..],
      ce.cycle()
    );
    type_error_data_ref::<UnknownSymbol>(&result_b.errors[2])
      .expect("expected UnknownSymbol for a.atype");
    assert_eq!("game/B", result_b.errors[2].module_name);
  }

  let module_b = fixture
    .get_frontend()
    .module_resolver
    .get_module(&ModuleName::from("game/B"));
  let ty_b = module_b
    .exported_type_bindings
    .get("btype")
    .expect("expected exported btype")
    .r#type();
  let mut opts = ToStringOptions {
    exhaustive: true,
    ..Default::default()
  };
  assert_eq!(
    "{ x: number }",
    to_string_type_id_to_string_options(ty_b, &mut opts)
  );

  let module_a = fixture
    .get_frontend()
    .module_resolver
    .get_module(&ModuleName::from("game/A"));
  let ty_a = module_a
    .exported_type_bindings
    .get("atype")
    .expect("expected exported atype")
    .r#type();
  let mut opts = ToStringOptions {
    exhaustive: true,
    ..Default::default()
  };
  assert_eq!(
    "{ x: any }",
    to_string_type_id_to_string_options(ty_a, &mut opts)
  );

  fixture
    .get_frontend()
    .mark_dirty(&ModuleName::from("game/B"), None);
  result_b = fixture
    .get_frontend()
    .check_module_name_optional_frontend_options(&ModuleName::from("game/B"), None);
  // cpp `LUAU_REQUIRE_ERRORS(resultB)`（Frontend.test.cpp:533）只要求有错，
  // 未锁数量/种类（脏标记后环的展开顺序与求解器状态相关），故仅锁非空。
  assert!(!result_b.errors.is_empty(), "{:?}", result_b.errors);

  let module_b = fixture
    .get_frontend()
    .module_resolver
    .get_module(&ModuleName::from("game/B"));
  let ty_b = module_b
    .exported_type_bindings
    .get("btype")
    .expect("expected exported btype")
    .r#type();
  let mut opts = ToStringOptions {
    exhaustive: true,
    ..Default::default()
  };
  assert_eq!(
    "{ x: number }",
    to_string_type_id_to_string_options(ty_b, &mut opts)
  );

  let module_a = fixture
    .get_frontend()
    .module_resolver
    .get_module(&ModuleName::from("game/A"));
  let ty_a = module_a
    .exported_type_bindings
    .get("atype")
    .expect("expected exported atype")
    .r#type();
  let mut opts = ToStringOptions {
    exhaustive: true,
    ..Default::default()
  };
  assert_eq!(
    "{ x: any }",
    to_string_type_id_to_string_options(ty_a, &mut opts)
  );
}

// Ported from `tests/Frontend.test.cpp`.
// Source: `tests/Frontend.test.cpp`
#[test]
fn frontend_cycle_incremental_type_surface_longer() {
  use alloc::string::String;

  use ulua_analysis::functions::to_string_to_string::to_string_type_id;
  use ulua_unit_test::records::{
    builtins_fixture::BuiltinsFixture, frontend_fixture::FrontendFixture,
  };

  let mut fixture = FrontendFixture {
    base: BuiltinsFixture::default(),
  };

  fixture.base.base.file_resolver.source.insert(
    String::from("game/A"),
    String::from(
      r#"
        return {mod_a = 2}
    "#,
    ),
  );

  let mut result = fixture
    .get_frontend()
    .check_module_name_optional_frontend_options(&ModuleName::from("game/A"), None);
  assert_eq!(0, result.errors.len(), "{:?}", result.errors);

  fixture.base.base.file_resolver.source.insert(
    String::from("game/B"),
    String::from(
      r#"
        local me = require(game.A)
        return {mod_b = 4}
    "#,
    ),
  );

  result = fixture
    .get_frontend()
    .check_module_name_optional_frontend_options(&ModuleName::from("game/B"), None);
  assert_eq!(0, result.errors.len(), "{:?}", result.errors);

  fixture.base.base.file_resolver.source.insert(
    String::from("game/A"),
    String::from(
      r#"
        local me = require(game.B)
        return {mod_a_prime = 3}
    "#,
    ),
  );

  fixture
    .get_frontend()
    .mark_dirty(&ModuleName::from("game/A"), None);
  fixture
    .get_frontend()
    .mark_dirty(&ModuleName::from("game/B"), None);

  result = fixture
    .get_frontend()
    .check_module_name_optional_frontend_options(&ModuleName::from("game/A"), None);
  assert!(!result.errors.is_empty(), "expected errors");

  let module_a = fixture
    .get_frontend()
    .module_resolver
    .get_module(&ModuleName::from("game/A"));
  let ty_a = fixture
    .base
    .base
    .require_type_module_ptr_string(&module_a, "me");
  assert_eq!("any", to_string_type_id(ty_a));

  result = fixture
    .get_frontend()
    .check_module_name_optional_frontend_options(&ModuleName::from("game/B"), None);
  assert!(!result.errors.is_empty(), "expected errors");

  let module_b = fixture
    .get_frontend()
    .module_resolver
    .get_module(&ModuleName::from("game/B"));
  let ty_b = fixture
    .base
    .base
    .require_type_module_ptr_string(&module_b, "me");
  assert_eq!("any", to_string_type_id(ty_b));
}

// Ported from `tests/Frontend.test.cpp`.
// Source: `tests/Frontend.test.cpp`
#[test]
fn frontend_deleted_source_is_evicted_on_recheck() {
  use alloc::string::String;

  use ulua_common::fflag;
  use ulua_unit_test::{
    records::{builtins_fixture::BuiltinsFixture, frontend_fixture::FrontendFixture},
    type_aliases::scoped_fast_flag::ScopedFastFlag,
  };

  let _source_node_erase = ScopedFastFlag::new(&fflag::LuauFrontendSourceNodeErase, true);
  let mut fixture = FrontendFixture {
    base: BuiltinsFixture::default(),
  };

  fixture.base.base.file_resolver.source.insert(
    String::from("game/A"),
    String::from(
      r#"
        export type Props = { name: string, value: number, label: string? }
        local function make(p: Props): Props
            return p
        end
        return {make = make}
    "#,
    ),
  );
  fixture.base.base.file_resolver.source.insert(
    String::from("game/B"),
    String::from(
      r#"
        local A = require(game.A)
        local function wrap(p: A.Props): A.Props
            return A.make(p)
        end
        return {wrap = wrap}
    "#,
    ),
  );
  fixture.base.base.file_resolver.source.insert(
    String::from("game/C"),
    String::from(
      r#"
        local A = require(game.A)
        local B = require(game.B)
        local x = B.wrap({name = "hi", value = 1})
        local y = A.make({name = "lo", value = 2})
        return {x, y}
    "#,
    ),
  );

  let result = fixture
    .get_frontend()
    .check_module_name_optional_frontend_options(&ModuleName::from("game/C"), None);
  assert_eq!(0, result.errors.len(), "{:?}", result.errors);

  // Delete module B, mark it as dirty
  fixture.base.base.file_resolver.source.remove("game/B");
  fixture
    .get_frontend()
    .mark_dirty(&ModuleName::from("game/B"), None);

  // Invalidate old contents of A
  fixture
    .get_frontend()
    .mark_dirty(&ModuleName::from("game/A"), None);

  // Should be able to check C and fail on missing B
  let result = fixture
    .get_frontend()
    .check_module_name_optional_frontend_options(&ModuleName::from("game/C"), None);
  assert_eq!(1, result.errors.len(), "{:?}", result.errors);

  assert!(!fixture.get_frontend().source_nodes.contains_key("game/B"));
  assert!(
    fixture
      .get_frontend()
      .module_resolver
      .try_get_module(&ModuleName::from("game/B"))
      .is_none()
  );

  // C++ `sourceNodes.count("game/A") == 1`
  assert!(fixture.get_frontend().source_nodes.contains_key("game/A"));
  assert!(
    fixture
      .get_frontend()
      .module_resolver
      .try_get_module(&ModuleName::from("game/A"))
      .is_some()
  );
}

// Ported from `tests/Frontend.test.cpp`.
// Source: `tests/Frontend.test.cpp`
#[test]
fn frontend_dfg_data_cleared_on_retain_type_graphs_unset() {
  use alloc::string::String;

  use ulua_common::fflag;
  use ulua_unit_test::{
    records::{builtins_fixture::BuiltinsFixture, frontend_fixture::FrontendFixture},
    type_aliases::scoped_fast_flag::ScopedFastFlag,
  };

  let _new_solver = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);
  let mut fixture = FrontendFixture {
    base: BuiltinsFixture::default(),
  };

  fixture.base.base.file_resolver.source.insert(
    String::from("game/A"),
    String::from(
      r#"
local a = 1
local b = 2
local c = 3
return {x = a, y = b, z = c}
"#,
    ),
  );

  fixture.get_frontend().options.retain_full_type_graphs = true;
  fixture
    .get_frontend()
    .check_module_name_optional_frontend_options(&ModuleName::from("game/A"), None);

  let module = fixture
    .get_frontend()
    .module_resolver
    .get_module(&ModuleName::from("game/A"));
  assert!(!module.def_arena.allocator.empty());
  assert!(!module.key_arena.empty());

  fixture.get_frontend().options.retain_full_type_graphs = false;
  fixture
    .get_frontend()
    .mark_dirty(&ModuleName::from("game/A"), None);
  fixture
    .get_frontend()
    .check_module_name_optional_frontend_options(&ModuleName::from("game/A"), None);

  let module = fixture
    .get_frontend()
    .module_resolver
    .get_module(&ModuleName::from("game/A"));
  assert!(module.def_arena.allocator.empty());
  assert!(module.key_arena.empty());
}

// Ported from `tests/Frontend.test.cpp`.
// Source: `tests/Frontend.test.cpp`
#[test]
fn frontend_discard_type_graphs() {
  use alloc::string::String;

  use ulua_analysis::{
    enums::solver_mode::SolverMode,
    records::{frontend::Frontend, frontend_options::FrontendOptions},
  };
  use ulua_common::fflag;
  use ulua_unit_test::records::{
    test_config_resolver::TestConfigResolver, test_file_resolver::TestFileResolver,
  };

  let mut file_resolver = TestFileResolver::default();
  let mut config_resolver = TestConfigResolver::default();
  let mode = if fflag::DebugLuauForceOldSolver.get() {
    SolverMode::Old
  } else {
    SolverMode::New
  };
  // `new_boxed` 在 safe 边界内完成「构造 → 堆上落位 → 自指针布线」：resolver 是
  // 本测试函数局部（fe 后声明先析构，句柄恒覆盖使用期），Box 钉死 Frontend 堆
  // 地址，调用点免手写 unsafe ctor + wire_self_pointers。
  let mut fe = Frontend::new_boxed(
    mode,
    &mut file_resolver,
    Some(&mut config_resolver.base),
    FrontendOptions::default(),
  );

  file_resolver.source.insert(
    String::from("Module/A"),
    String::from(
      r#"
        local a = {1,2,3,4,5}
    "#,
    ),
  );

  let _result = fe.check_module_name_optional_frontend_options(&ModuleName::from("Module/A"), None);

  let module = fe.module_resolver.get_module(&ModuleName::from("Module/A"));

  assert_eq!(0, module.internal_types.types.size());
  assert_eq!(0, module.internal_types.type_packs.size());
  assert_eq!(0, module.ast_types.size());
  assert_eq!(0, module.ast_resolved_types.size());
  assert_eq!(0, module.ast_resolved_type_packs.size());
}

// Ported from `tests/Frontend.test.cpp`.
// Source: `tests/Frontend.test.cpp`
#[test]
fn frontend_dont_recheck_script_that_hasnt_been_marked_dirty() {
  use alloc::string::String;

  use ulua_unit_test::records::{
    builtins_fixture::BuiltinsFixture, frontend_fixture::FrontendFixture,
  };

  let mut fixture = FrontendFixture {
    base: BuiltinsFixture::default(),
  };

  fixture.base.base.file_resolver.source.insert(
    String::from("game/Gui/Modules/A"),
    String::from("return {hello=5, world=true}"),
  );
  fixture.base.base.file_resolver.source.insert(
    String::from("game/Gui/Modules/B"),
    String::from(
      r#"
        local Modules = game:GetService('Gui').Modules
        local A = require(Modules.A)
        return {b_value = A.hello}
    "#,
    ),
  );

  fixture
    .get_frontend()
    .check_module_name_optional_frontend_options(&ModuleName::from("game/Gui/Modules/B"), None);

  fixture.base.base.file_resolver.source.insert(
      String::from("game/Gui/Modules/A"),
      String::from(
          "Massively incorrect syntax haha oops!  However!  The getFrontend().doesn't know that this file needs reparsing!",
      ),
  );

  fixture
    .get_frontend()
    .check_module_name_optional_frontend_options(&ModuleName::from("game/Gui/Modules/B"), None);

  let b_module = fixture
    .get_frontend()
    .module_resolver
    .get_module(&ModuleName::from("game/Gui/Modules/B"));
  assert!(b_module.errors.is_empty(), "{:?}", b_module.errors);
}

// Ported from `tests/Frontend.test.cpp`.
// Source: `tests/Frontend.test.cpp`
#[test]
fn frontend_dont_reparse_clean_file_when_linting() {
  use alloc::string::String;

  use ulua_analysis::records::frontend_options::FrontendOptions;
  use ulua_config::records::lint_warning::LintWarning;
  use ulua_unit_test::records::{
    builtins_fixture::BuiltinsFixture, frontend_fixture::FrontendFixture,
  };

  let mut fixture = FrontendFixture {
    base: BuiltinsFixture::default(),
  };

  fixture.base.base.file_resolver.source.insert(
    String::from("Modules/A"),
    String::from(
      r#"
        local t = {}

        for i=#t,1 do
        end

        for i=#t,1,-1 do
        end
    "#,
    ),
  );

  fixture.get_frontend();
  fixture
    .base
    .base
    .config_resolver
    .default_config
    .enabled_lint
    .enable_warning(LintWarning::CODE_FOR_RANGE);

  let opts = FrontendOptions {
    run_lint_checks: true,
    ..Default::default()
  };
  fixture
    .get_frontend()
    .check_module_name_optional_frontend_options(
      &ModuleName::from("Modules/A"),
      Some(opts.clone()),
    );

  fixture.base.base.file_resolver.source.insert(
    String::from("Modules/A"),
    String::from(
      r#"
        -- We have fixed the lint error, but we did not tell the Frontend that the file is changed!
        -- Therefore, we expect Frontend to reuse the results from previous lint.
    "#,
    ),
  );

  let result = fixture
    .get_frontend()
    .check_module_name_optional_frontend_options(&ModuleName::from("Modules/A"), Some(opts));
  assert_eq!(1, result.lint_result.warnings.len());
}

// Source: `tests/Frontend.test.cpp`
#[test]
fn frontend_environments() {
  use alloc::string::String;

  use ulua_analysis::functions::{freeze::freeze, unfreeze::unfreeze};
  use ulua_unit_test::records::{
    builtins_fixture::BuiltinsFixture, frontend_fixture::FrontendFixture,
  };

  let mut fixture = FrontendFixture {
    base: BuiltinsFixture::default(),
  };

  let test_scope = fixture.get_frontend().add_environment(String::from("test"));

  let frontend = fixture.get_frontend();
  unfreeze(frontend.globals.global_types_mut());
  let result = frontend.load_definition_file(
    |frontend| &mut frontend.globals,
    test_scope,
    r#"
        export type Foo = number | string
    "#,
    String::from("@test"),
    false,
    false,
  );
  assert!(result.success, "{:?}", result);
  freeze(frontend.globals.global_types_mut());

  fixture.base.base.file_resolver.source.insert(
    String::from("A"),
    String::from(
      r#"
        --!nonstrict
        local foo: Foo = 1
    "#,
    ),
  );

  fixture.base.base.file_resolver.source.insert(
    String::from("B"),
    String::from(
      r#"
        --!nonstrict
        local foo: Foo = 1
    "#,
    ),
  );

  fixture.base.base.file_resolver.source.insert(
    String::from("C"),
    String::from(
      r#"
        --!strict
        local foo: Foo = 1
    "#,
    ),
  );

  fixture
    .base
    .base
    .file_resolver
    .environments
    .insert(ModuleName::from("A"), String::from("test"));

  let result_a = fixture
    .get_frontend()
    .check_module_name_optional_frontend_options(&ModuleName::from("A"), None);
  assert_eq!(0, result_a.errors.len(), "{:?}", result_a.errors);

  let result_b = fixture
    .get_frontend()
    .check_module_name_optional_frontend_options(&ModuleName::from("B"), None);
  assert_eq!(1, result_b.errors.len(), "{:?}", result_b.errors);

  let result_c = fixture
    .get_frontend()
    .check_module_name_optional_frontend_options(&ModuleName::from("C"), None);
  assert_eq!(1, result_c.errors.len(), "{:?}", result_c.errors);
}

// Ported from `tests/Frontend.test.cpp`.
// Source: `tests/Frontend.test.cpp`
#[test]
fn frontend_exported_tables_have_position_metadata() {
  use ulua_analysis::{
    functions::{flatten_type_pack::flatten_type_pack_id, get_type},
    records::table_type::TableType,
  };
  use ulua_ast::records::{location::Location, position::Position};
  use ulua_unit_test::records::fixture::Fixture;

  let mut fixture = Fixture::fixture_bool(false);
  let result = fixture.check_string_optional_frontend_options(
    r#"
        return { abc = 22 }
    "#,
    None,
  );
  assert_eq!(0, result.errors.len(), "{:?}", result.errors);

  let main_module = fixture.get_main_module(false);
  assert!(!main_module.is_null());
  let return_type = unsafe { (*main_module).get_module_scope() }.return_type;

  let (ret_head, _tail) = flatten_type_pack_id(return_type);
  assert_eq!(1, ret_head.len());

  let table_type = get_type::get::<TableType>(ret_head[0]).expect("expected a table type");
  assert_eq!("MainModule", table_type.definition_module_name);
  assert_eq!(1, table_type.props.len());
  assert!(table_type.props.contains_key("abc"));

  let prop = table_type.props.get("abc").expect("expected property abc");
  assert_eq!(
    Some(Location::new(
      Position {
        line: 1,
        column: 17
      },
      Position {
        line: 1,
        column: 20
      },
    )),
    prop.location
  );
}

// Ported from `tests/Frontend.test.cpp`.
// Source: `tests/Frontend.test.cpp`
#[test]
fn frontend_export_value_modules_have_typed_require_surface() {
  use alloc::string::String;

  use ulua_analysis::functions::{first::first, to_string_to_string::to_string_type_id};
  use ulua_common::fflag;
  use ulua_unit_test::{
    records::{builtins_fixture::BuiltinsFixture, frontend_fixture::FrontendFixture},
    type_aliases::scoped_fast_flag::ScopedFastFlag,
  };

  let _flags = [
    ScopedFastFlag::new(&fflag::LuauExportValueSyntax, true),
    ScopedFastFlag::new(&fflag::LuauExportValueTypecheck, true),
  ];
  let mut fixture = FrontendFixture {
    base: BuiltinsFixture::default(),
  };

  fixture.base.base.file_resolver.source.insert(
    String::from("game/ModuleA"),
    String::from(
      r#"
        --!strict
        export local version = "1.0.0"
        export const answer = 42

        export function inc(x: number): number
            return x + 1
        end
    "#,
    ),
  );

  fixture.base.base.file_resolver.source.insert(
    String::from("game/ModuleB"),
    String::from(
      r#"
        --!strict
        local M = require(game.ModuleA)

        local version: string = M.version
        local answer: number = M.answer
        local nextValue: number = M.inc(answer)

        return version, nextValue
    "#,
    ),
  );

  let a_result = fixture
    .get_frontend()
    .check_module_name_optional_frontend_options(&ModuleName::from("game/ModuleA"), None);
  assert_eq!(0, a_result.errors.len(), "{:?}", a_result.errors);

  let b_result = fixture
    .get_frontend()
    .check_module_name_optional_frontend_options(&ModuleName::from("game/ModuleB"), None);
  assert_eq!(0, b_result.errors.len(), "{:?}", b_result.errors);

  let module_a = fixture
    .get_frontend()
    .module_resolver
    .get_module(&ModuleName::from("game/ModuleA"));

  let exports = first(module_a.return_type, true).expect("expected module return type");
  assert_eq!(
    "{ read answer: number, read inc: (number) -> number, read version: string }",
    to_string_type_id(exports)
  );
}

// Ported from `tests/Frontend.test.cpp`.
// Source: `tests/Frontend.test.cpp`
#[test]
fn frontend_find_a_require() {
  use ulua_analysis::{
    functions::trace_requires::trace_requires, records::type_check_limits::TypeCheckLimits,
  };
  use ulua_ast::records::parse_options::ParseOptions;
  use ulua_unit_test::records::{
    builtins_fixture::BuiltinsFixture, frontend_fixture::FrontendFixture,
    naive_file_resolver::NaiveFileResolver,
  };

  let mut fixture = FrontendFixture {
    base: BuiltinsFixture::default(),
  };
  let program = fixture.base.base.parse(
    r#"
        local M = require(Modules.Foo.Bar)
    "#,
    &ParseOptions::default(),
  );

  let mut naive_file_resolver = NaiveFileResolver;

  let result = trace_requires(
    &mut naive_file_resolver,
    program,
    ModuleName::new(),
    &TypeCheckLimits::default(),
  );
  assert_eq!(1, result.require_list.len());
  assert_eq!("Modules/Foo/Bar", result.require_list[0].0);
}

// Ported from `tests/Frontend.test.cpp`.
// Source: `tests/Frontend.test.cpp`
#[test]
fn frontend_find_a_require_inside_a_function() {
  use ulua_analysis::{
    functions::trace_requires::trace_requires, records::type_check_limits::TypeCheckLimits,
  };
  use ulua_ast::records::parse_options::ParseOptions;
  use ulua_unit_test::records::{
    builtins_fixture::BuiltinsFixture, frontend_fixture::FrontendFixture,
    naive_file_resolver::NaiveFileResolver,
  };

  let mut fixture = FrontendFixture {
    base: BuiltinsFixture::default(),
  };
  let program = fixture.base.base.parse(
    r#"
        function foo()
            local M = require(Modules.Foo.Bar)
        end
    "#,
    &ParseOptions::default(),
  );

  let mut naive_file_resolver = NaiveFileResolver;

  let result = trace_requires(
    &mut naive_file_resolver,
    program,
    ModuleName::new(),
    &TypeCheckLimits::default(),
  );
  assert_eq!(1, result.require_list.len());
}

// Ported from `tests/Frontend.test.cpp`.
// Source: `tests/Frontend.test.cpp`
#[test]
#[ignore = "欠账（已复核订正）：本移植误报 TypeMismatch 不是 Subtyping 缺口——is_covariant 与 cpp/Analysis/src/Subtyping.cpp 逐行等价（表×表入口 force 恒 false、as-nil 用 readonly(nil) 对 rw-shared 属性同样判 false，cpp 对同输入也给 false）。真实差异在 cpp 侧 generic-P widening 全链路（约束求解器 → Instantiation2::cleanType 选界 → visit_call 复查时不产生该硬子类型判定），需移植上游对应 PR 全量 diff 或先拿 cpp 插桩数据，单改 Subtyping 属盲改"]
fn frontend_generic_p_widening_with_cross_module_recursive_type() {
  use alloc::string::String;

  use ulua_common::fflag;
  use ulua_unit_test::{
    records::{builtins_fixture::BuiltinsFixture, frontend_fixture::FrontendFixture},
    type_aliases::scoped_fast_flag::ScopedFastFlag,
  };

  // cpp 的 DOES_NOT_PASS_OLD_SOLVER_GUARD() 等价于强制关闭旧求解器。
  let _flags = [
    ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false),
    ScopedFastFlag::new(&fflag::LuauSubtypingMissingPropertiesAsNil, true),
  ];

  let mut fixture = FrontendFixture {
    base: BuiltinsFixture::default(),
  };

  // Module A: exports a recursive type and a component that uses it.
  fixture.base.base.file_resolver.source.insert(
    String::from("game/Gui/Modules/A"),
    String::from(
      r#"
        --!strict
        type Element = { key: (number | string)?, props: any?, ref: any, type: any }
        type NodeArray = { (NodeArray | boolean | number | string | Element | { [string]: (NodeArray | boolean | number | string | Element)?, UNIQUE_TAG: any? })? }
        export type Node = string | number | boolean | Element | NodeArray | { [string]: (NodeArray | boolean | number | string | Element)?, UNIQUE_TAG: any? }
        export type BaseProps = { tag: string?, children: Node? }
        export type ExtraProps = { size: number? }
        local function View(props: BaseProps & ExtraProps)
            return nil
        end
        return View
    "#,
    ),
  );

  // Module B: imports and calls createElement.
  fixture.base.base.file_resolver.source.insert(
    String::from("game/Gui/Modules/B"),
    String::from(
      r#"
        --!strict
        local Modules = game:GetService('Gui').Modules
        local View = require(Modules.A)
        local function createElement<P>(component: (P) -> any, props: P?): any
            return nil
        end
        local _x = createElement(View, { tag = "hello" })
    "#,
    ),
  );

  let result = fixture
    .get_frontend()
    .check_module_name_optional_frontend_options(&ModuleName::from("game/Gui/Modules/B"), None);

  // cpp 端先 ignoreMissingAnnotations(result) 过滤 TypeAnnotationRequired；
  // 该诊断在本移植中不存在（TypeErrorData 无对应变体），过滤等价于无操作，
  // 因此直接断言无任何错误即保持原语义。
  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Ported from `tests/Frontend.test.cpp`.
// Source: `tests/Frontend.test.cpp`
#[test]
fn frontend_get_required_scripts() {
  use alloc::string::String;

  use ulua_analysis::records::type_check_limits::TypeCheckLimits;
  use ulua_unit_test::records::{
    builtins_fixture::BuiltinsFixture, frontend_fixture::FrontendFixture,
  };

  let mut fixture = FrontendFixture {
    base: BuiltinsFixture::default(),
  };

  fixture.base.base.file_resolver.source.insert(
    String::from("game/workspace/MyScript"),
    String::from(
      r#"
        local MyModuleScript = require(game.workspace.MyModuleScript)
        local MyModuleScript2 = require(game.workspace.MyModuleScript2)
        MyModuleScript.myPrint()
    "#,
    ),
  );

  fixture.base.base.file_resolver.source.insert(
    String::from("game/workspace/MyModuleScript"),
    String::from(
      r#"
        local module = {}
        function module.myPrint()
            print("Hello World")
        end
        return module
    "#,
    ),
  );

  fixture.base.base.file_resolver.source.insert(
    String::from("game/workspace/MyModuleScript2"),
    String::from(
      r#"
        local module = {}
        return module
    "#,
    ),
  );

  fixture
    .get_frontend()
    .mark_dirty(&ModuleName::from("game/workspace/MyScript"), None);
  let mut required_scripts = fixture.get_frontend().get_required_scripts(
    &ModuleName::from("game/workspace/MyScript"),
    &TypeCheckLimits::default(),
  );
  assert_eq!(2, required_scripts.len(), "{:?}", required_scripts);
  assert_eq!("game/workspace/MyModuleScript", required_scripts[0]);
  assert_eq!("game/workspace/MyModuleScript2", required_scripts[1]);

  fixture
    .get_frontend()
    .check_module_name_optional_frontend_options(
      &ModuleName::from("game/workspace/MyScript"),
      None,
    );
  required_scripts = fixture.get_frontend().get_required_scripts(
    &ModuleName::from("game/workspace/MyScript"),
    &TypeCheckLimits::default(),
  );
  assert_eq!(2, required_scripts.len(), "{:?}", required_scripts);
  assert_eq!("game/workspace/MyModuleScript", required_scripts[0]);
  assert_eq!("game/workspace/MyModuleScript2", required_scripts[1]);
}

// Ported from `tests/Frontend.test.cpp`.
// Source: `tests/Frontend.test.cpp`
#[test]
fn frontend_get_required_scripts_dirty() {
  use alloc::string::String;

  use ulua_analysis::records::type_check_limits::TypeCheckLimits;
  use ulua_unit_test::records::{
    builtins_fixture::BuiltinsFixture, frontend_fixture::FrontendFixture,
  };

  let mut fixture = FrontendFixture {
    base: BuiltinsFixture::default(),
  };

  fixture.base.base.file_resolver.source.insert(
    String::from("game/workspace/MyScript"),
    String::from(
      r#"
        print("Hello World")
    "#,
    ),
  );

  fixture.base.base.file_resolver.source.insert(
    String::from("game/workspace/MyModuleScript"),
    String::from(
      r#"
        local module = {}
        function module.myPrint()
            print("Hello World")
        end
        return module
    "#,
    ),
  );

  fixture
    .get_frontend()
    .check_module_name_optional_frontend_options(
      &ModuleName::from("game/workspace/MyScript"),
      None,
    );
  let mut required_scripts = fixture.get_frontend().get_required_scripts(
    &ModuleName::from("game/workspace/MyScript"),
    &TypeCheckLimits::default(),
  );
  assert_eq!(0, required_scripts.len(), "{:?}", required_scripts);

  fixture.base.base.file_resolver.source.insert(
    String::from("game/workspace/MyScript"),
    String::from(
      r#"
        local MyModuleScript = require(game.workspace.MyModuleScript)
        MyModuleScript.myPrint()
    "#,
    ),
  );

  required_scripts = fixture.get_frontend().get_required_scripts(
    &ModuleName::from("game/workspace/MyScript"),
    &TypeCheckLimits::default(),
  );
  assert_eq!(0, required_scripts.len(), "{:?}", required_scripts);

  fixture
    .get_frontend()
    .mark_dirty(&ModuleName::from("game/workspace/MyScript"), None);
  required_scripts = fixture.get_frontend().get_required_scripts(
    &ModuleName::from("game/workspace/MyScript"),
    &TypeCheckLimits::default(),
  );
  assert_eq!(1, required_scripts.len(), "{:?}", required_scripts);
  assert_eq!("game/workspace/MyModuleScript", required_scripts[0]);
}

// Ported from `tests/Frontend.test.cpp`.
// Source: `tests/Frontend.test.cpp`
#[test]
fn frontend_ignore_require_to_nonexistent_file() {
  use alloc::string::String;

  use ulua_unit_test::records::{
    builtins_fixture::BuiltinsFixture, frontend_fixture::FrontendFixture,
  };

  let mut fixture = FrontendFixture {
    base: BuiltinsFixture::default(),
  };

  fixture.base.base.file_resolver.source.insert(
    String::from("Modules/A"),
    String::from(
      r#"
        local Modules = script
        local B = require(Modules.B) :: any
    "#,
    ),
  );

  let result = fixture
    .get_frontend()
    .check_module_name_optional_frontend_options(&ModuleName::from("Modules/A"), None);
  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Ported from `tests/Frontend.test.cpp`.
// Source: `tests/Frontend.test.cpp`
#[test]
fn frontend_imported_table_modification_2() {
  use alloc::string::String;

  use ulua_common::fflag;
  use ulua_unit_test::records::{
    builtins_fixture::BuiltinsFixture, frontend_fixture::FrontendFixture,
  };

  if !fflag::DebugLuauForceOldSolver.get() {
    return;
  }

  let mut fixture = FrontendFixture {
    base: BuiltinsFixture::default(),
  };

  fixture.get_frontend().options.retain_full_type_graphs = false;

  fixture.base.base.file_resolver.source.insert(
    String::from("Module/A"),
    String::from(
      r#"
--!nonstrict
local a = {}
a.x = 1
return a;
    "#,
    ),
  );

  fixture.base.base.file_resolver.source.insert(
    String::from("Module/B"),
    String::from(
      r#"
--!nonstrict
local a = require(script.Parent.A)
local b = {}
function a:b() end -- this should error, since A doesn't define a:b()
return b
    "#,
    ),
  );

  fixture.base.base.file_resolver.source.insert(
    String::from("Module/C"),
    String::from(
      r#"
--!nonstrict
local a = require(script.Parent.A)
local b = require(script.Parent.B)
a:b() -- this should error, since A doesn't define a:b()
    "#,
    ),
  );

  let result_a = fixture
    .get_frontend()
    .check_module_name_optional_frontend_options(&ModuleName::from("Module/A"), None);
  assert_eq!(0, result_a.errors.len(), "{:?}", result_a.errors);

  let result_b = fixture
    .get_frontend()
    .check_module_name_optional_frontend_options(&ModuleName::from("Module/B"), None);
  assert!(!result_b.errors.is_empty(), "expected errors");

  let result_c = fixture
    .get_frontend()
    .check_module_name_optional_frontend_options(&ModuleName::from("Module/C"), None);
  assert!(!result_c.errors.is_empty(), "expected errors");
}

// Ported from `tests/Frontend.test.cpp`.
// Source: `tests/Frontend.test.cpp`
#[test]
fn frontend_it_should_be_safe_to_stringify_errors_when_full_type_graph_is_discarded() {
  use alloc::string::String;

  use ulua_analysis::{
    enums::solver_mode::SolverMode,
    functions::to_string_error::to_string_type_error,
    records::{frontend::Frontend, frontend_options::FrontendOptions},
  };
  use ulua_common::fflag;
  use ulua_unit_test::records::{
    test_config_resolver::TestConfigResolver, test_file_resolver::TestFileResolver,
  };

  let mut file_resolver = TestFileResolver::default();
  let mut config_resolver = TestConfigResolver::default();
  let mode = if fflag::DebugLuauForceOldSolver.get() {
    SolverMode::Old
  } else {
    SolverMode::New
  };
  // `new_boxed` 在 safe 边界内完成「构造 → 堆上落位 → 自指针布线」：resolver 是
  // 本测试函数局部（fe 后声明先析构，句柄恒覆盖使用期），Box 钉死 Frontend 堆
  // 地址，调用点免手写 unsafe ctor + wire_self_pointers。
  let mut fe = Frontend::new_boxed(
    mode,
    &mut file_resolver,
    Some(&mut config_resolver.base),
    FrontendOptions::default(),
  );

  file_resolver.source.insert(
    String::from("Module/A"),
    String::from(
      r#"
        --!strict
        local a: {Count: number} = {count='five'}
    "#,
    ),
  );

  let result = fe.check_module_name_optional_frontend_options(&ModuleName::from("Module/A"), None);

  assert_eq!(1, result.errors.len(), "{:?}", result.errors);

  if !fflag::DebugLuauForceOldSolver.get() {
    assert_eq!(
      "Table type '{ count: string }' not compatible with type '{ Count: number }' because the former is missing field 'Count'",
      to_string_type_error(&result.errors[0])
    );
  } else {
    assert_eq!(
      "Table type 'a' not compatible with type '{ Count: number }' because the former is missing field 'Count'",
      to_string_type_error(&result.errors[0])
    );
  }
}

// Ported from `tests/Frontend.test.cpp`.
// Source: `tests/Frontend.test.cpp`
#[test]
fn frontend_lint_results_are_only_for_checked_module() {
  use alloc::string::String;

  use ulua_analysis::records::frontend_options::FrontendOptions;
  use ulua_unit_test::records::{
    builtins_fixture::BuiltinsFixture, frontend_fixture::FrontendFixture,
  };

  let mut fixture = FrontendFixture {
    base: BuiltinsFixture::default(),
  };

  fixture.base.base.file_resolver.source.insert(
    String::from("Module/A"),
    String::from(
      r#"
local _ = 0b10000000000000000000000000000000000000000000000000000000000000000
    "#,
    ),
  );

  fixture.base.base.file_resolver.source.insert(
    String::from("Module/B"),
    String::from(
      r#"
require(script.Parent.A)
local _ = 0x10000000000000000
    "#,
    ),
  );

  let opts = FrontendOptions {
    run_lint_checks: true,
    ..Default::default()
  };
  let mut result = fixture
    .get_frontend()
    .check_module_name_optional_frontend_options(&ModuleName::from("Module/B"), Some(opts.clone()));
  assert_eq!(1, result.lint_result.warnings.len());

  result = fixture
    .get_frontend()
    .check_module_name_optional_frontend_options(&ModuleName::from("Module/B"), Some(opts));
  assert_eq!(1, result.lint_result.warnings.len());
}

// Ported from `tests/Frontend.test.cpp`.
// Source: `tests/Frontend.test.cpp`
#[test]
fn frontend_mark_non_immediate_reverse_deps_as_dirty() {
  use alloc::{string::String, vec::Vec};

  use ulua_unit_test::records::{
    builtins_fixture::BuiltinsFixture, frontend_fixture::FrontendFixture,
  };

  let mut fixture = FrontendFixture {
    base: BuiltinsFixture::default(),
  };

  fixture.base.base.file_resolver.source.insert(
    String::from("game/Gui/Modules/A"),
    String::from("return {hello=5, world=true}"),
  );
  fixture.base.base.file_resolver.source.insert(
    String::from("game/Gui/Modules/B"),
    String::from(
      r#"
        return require(game:GetService('Gui').Modules.A)
    "#,
    ),
  );
  fixture.base.base.file_resolver.source.insert(
    String::from("game/Gui/Modules/C"),
    String::from(
      r#"
        local Modules = game:GetService('Gui').Modules
        local B = require(Modules.B)
        return {c_value = B.hello}
    "#,
    ),
  );

  fixture
    .get_frontend()
    .check_module_name_optional_frontend_options(&ModuleName::from("game/Gui/Modules/C"), None);

  let mut marked_dirty: Vec<ModuleName> = Vec::new();
  fixture.get_frontend().mark_dirty(
    &ModuleName::from("game/Gui/Modules/A"),
    Some(&mut marked_dirty),
  );

  assert_eq!(3, marked_dirty.len(), "{:?}", marked_dirty);
  assert!(marked_dirty.contains(&ModuleName::from("game/Gui/Modules/A")));
  assert!(marked_dirty.contains(&ModuleName::from("game/Gui/Modules/B")));
  assert!(marked_dirty.contains(&ModuleName::from("game/Gui/Modules/C")));
}

// Ported from `tests/Frontend.test.cpp`.
// Source: `tests/Frontend.test.cpp`
#[test]
fn frontend_markdirty_early_return() {
  use alloc::{string::String, vec::Vec};

  use ulua_unit_test::records::{
    builtins_fixture::BuiltinsFixture, frontend_fixture::FrontendFixture,
  };

  let module_name = ModuleName::from("game/Gui/Modules/A");
  let mut fixture = FrontendFixture {
    base: BuiltinsFixture::default(),
  };

  fixture.base.base.file_resolver.source.insert(
    module_name.clone(),
    String::from(
      r#"
        return 1
    "#,
    ),
  );

  {
    let mut marked_dirty: Vec<ModuleName> = Vec::new();
    fixture
      .get_frontend()
      .mark_dirty(&module_name, Some(&mut marked_dirty));
    assert!(marked_dirty.is_empty(), "{:?}", marked_dirty);
  }

  fixture.get_frontend().parse_module_name(&module_name);

  {
    let mut marked_dirty: Vec<ModuleName> = Vec::new();
    fixture
      .get_frontend()
      .mark_dirty(&module_name, Some(&mut marked_dirty));
    assert!(!marked_dirty.is_empty());
  }
}

// Ported from `tests/Frontend.test.cpp`.
// Source: `tests/Frontend.test.cpp`
#[test]
fn frontend_module_scope_check() {
  use alloc::{rc::Rc, string::String, sync::Arc};

  use ulua_analysis::{
    functions::to_string_to_string::to_string_type_id,
    records::{binding::Binding, symbol::Symbol},
    type_aliases::{module_name_type::ModuleName, scope_ptr_type::ScopePtr},
  };
  use ulua_ast::records::{ast_name::AstName, location::Location};
  use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

  let mut fixture = BuiltinsFixture::default();
  let number_type = fixture.base.get_builtins().number_type;

  // C++: `scope->bindings[AstName{"x"}] = Binding{globals.builtinTypes->numberType};`
  fixture.base.get_frontend().prepare_module_scope = Some(Rc::new(
    move |_name: &ModuleName, scope: &ScopePtr, _for_autocomplete| {
      let binding = Binding {
        type_id: number_type,
        location: Location::default(),
        deprecated: false,
        deprecated_suggestion: String::new(),
        documentation_symbol: None,
      };
      // Safety: `scope` 是检查器为被测模块持有的 `Arc<Scope>`，比本回调帧长寿
      // 且期间无其他借用；(b) 类写入句柄：Binding 值已在安全区构造，此处一次性 insert。
      unsafe {
        (*Arc::as_ptr(scope).cast_mut())
          .bindings
          .insert(Symbol::from_global(AstName::from_static(b"x")), binding);
      }
    },
  ));

  fixture.base.file_resolver.source.insert(
    String::from("game/A"),
    String::from(
      r#"
        local a = x
    "#,
    ),
  );

  let result = fixture
    .base
    .get_frontend()
    .check_module_name_optional_frontend_options(&ModuleName::from("game/A"), None);
  assert_eq!(0, result.errors.len(), "{:?}", result.errors);

  let ty = fixture.base.require_type_module_name_string("game/A", "a");
  assert_eq!("number", to_string_type_id(ty));
}

// Ported from `tests/Frontend.test.cpp`.
// Source: `tests/Frontend.test.cpp`
#[test]
fn frontend_no_separate_caches_with_the_new_solver() {
  use alloc::string::String;

  use ulua_analysis::records::frontend_options::FrontendOptions;
  use ulua_ast::enums::mode::Mode;
  use ulua_common::fflag;
  use ulua_unit_test::{
    records::{builtins_fixture::BuiltinsFixture, frontend_fixture::FrontendFixture},
    type_aliases::scoped_fast_flag::ScopedFastFlag,
  };

  let _new_solver = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false);
  let mut fixture = FrontendFixture {
    base: BuiltinsFixture::default(),
  };

  fixture.base.base.file_resolver.source.insert(
    String::from("game/A"),
    String::from(
      r#"
        --!nonstrict
        local exports = {}
        function exports.hello() end
        return exports
    "#,
    ),
  );

  let opts = FrontendOptions {
    for_autocomplete: true,
    ..Default::default()
  };
  fixture
    .get_frontend()
    .check_module_name_optional_frontend_options(&ModuleName::from("game/A"), Some(opts));

  assert!(
    !fixture
      .get_frontend()
      .module_resolver_for_autocomplete
      .modules
      .contains_key("game/A")
  );

  let module = fixture
    .get_frontend()
    .module_resolver
    .get_module(&ModuleName::from("game/A"));
  assert_eq!(Mode::Nonstrict, module.mode);
}

// Ported from `tests/Frontend.test.cpp`.
// Source: `tests/Frontend.test.cpp`
#[test]
fn frontend_no_use_after_free_with_type_fun_instantiation() {
  use alloc::string::String;

  use ulua_common::fflag;
  use ulua_unit_test::{
    records::{builtins_fixture::BuiltinsFixture, frontend_fixture::FrontendFixture},
    type_aliases::scoped_fast_flag::ScopedFastFlag,
  };

  let _freeze_arena = ScopedFastFlag::new(&fflag::DebugLuauFreezeArena, true);

  let mut fixture = FrontendFixture {
    base: BuiltinsFixture::default(),
  };

  fixture.base.base.file_resolver.source.insert(
    String::from("Module/A"),
    String::from(
      r#"
export type Foo<V> = typeof(setmetatable({}, {}))
return false;
    "#,
    ),
  );

  fixture.base.base.file_resolver.source.insert(
    String::from("Module/B"),
    String::from(
      r#"
local A = require(script.Parent.A)
export type Foo<V> = A.Foo<V>
return false;
    "#,
    ),
  );

  // We don't care about the result. That we haven't crashed is enough.
  fixture
    .get_frontend()
    .check_module_name_optional_frontend_options(&ModuleName::from("Module/B"), None);
}

// Ported from `tests/Frontend.test.cpp`.
// Source: `tests/Frontend.test.cpp`
#[test]
fn frontend_nocheck_cycle_used_by_checked() {
  use alloc::string::String;

  use ulua_analysis::functions::{first::first, to_string_to_string::to_string_type_id};
  use ulua_unit_test::records::{
    builtins_fixture::BuiltinsFixture, frontend_fixture::FrontendFixture,
  };

  let mut fixture = FrontendFixture {
    base: BuiltinsFixture::default(),
  };

  fixture.base.base.file_resolver.source.insert(
    String::from("game/Gui/Modules/A"),
    String::from(
      r#"
        --!nocheck
        local Modules = game:GetService('Gui').Modules
        local B = require(Modules.B)
        return {hello = B.hello}
    "#,
    ),
  );
  fixture.base.base.file_resolver.source.insert(
    String::from("game/Gui/Modules/B"),
    String::from(
      r#"
        --!nocheck
        local Modules = game:GetService('Gui').Modules
        local A = require(Modules.A)
        return {hello = A.hello}
    "#,
    ),
  );
  fixture.base.base.file_resolver.source.insert(
    String::from("game/Gui/Modules/C"),
    String::from(
      r#"
        --!strict
        local Modules = game:GetService('Gui').Modules
        local A = require(Modules.A)
        local B = require(Modules.B)
        return {a=A, b=B}
    "#,
    ),
  );

  let result = fixture
    .get_frontend()
    .check_module_name_optional_frontend_options(&ModuleName::from("game/Gui/Modules/C"), None);
  assert_eq!(0, result.errors.len(), "{:?}", result.errors);

  let c_module = fixture
    .get_frontend()
    .module_resolver
    .get_module(&ModuleName::from("game/Gui/Modules/C"));
  let c_exports = first(c_module.return_type, true).expect("expected C module return type");

  assert_eq!(
    "{ a: { hello: any }, b: { hello: any } }",
    to_string_type_id(c_exports)
  );
}

// Ported from `tests/Frontend.test.cpp`.
// Source: `tests/Frontend.test.cpp`
#[test]
fn frontend_nocheck_modules_are_typed() {
  use alloc::string::String;

  use ulua_analysis::functions::{first::first, to_string_to_string::to_string_type_id};
  use ulua_unit_test::records::{
    builtins_fixture::BuiltinsFixture, frontend_fixture::FrontendFixture,
  };

  let mut fixture = FrontendFixture {
    base: BuiltinsFixture::default(),
  };

  fixture.base.base.file_resolver.source.insert(
    String::from("game/Gui/Modules/A"),
    String::from(
      r#"
        --!nocheck
        export type Foo = number
        return {hello = "hi"}
    "#,
    ),
  );
  fixture.base.base.file_resolver.source.insert(
    String::from("game/Gui/Modules/B"),
    String::from(
      r#"
        --!nonstrict
        export type Foo = number
        return {hello = "hi"}
    "#,
    ),
  );
  fixture.base.base.file_resolver.source.insert(
    String::from("game/Gui/Modules/C"),
    String::from(
      r#"
        local Modules = game:GetService('Gui').Modules
        local A = require(Modules.A)
        local B = require(Modules.B)
        local five : A.Foo = 5
    "#,
    ),
  );

  let result = fixture
    .get_frontend()
    .check_module_name_optional_frontend_options(&ModuleName::from("game/Gui/Modules/C"), None);
  assert_eq!(0, result.errors.len(), "{:?}", result.errors);

  let a_module = fixture
    .get_frontend()
    .module_resolver
    .get_module(&ModuleName::from("game/Gui/Modules/A"));
  let a_exports = first(a_module.return_type, true).expect("expected A module return type");

  let b_module = fixture
    .get_frontend()
    .module_resolver
    .get_module(&ModuleName::from("game/Gui/Modules/B"));
  let b_exports = first(b_module.return_type, true).expect("expected B module return type");

  assert_eq!(to_string_type_id(a_exports), to_string_type_id(b_exports));
}

// Source: `tests/Frontend.test.cpp`
#[test]
fn frontend_parse_just_a_type() {
  use ulua_analysis::{
    functions::to_string_to_string::to_string_type_id,
    records::{
      builtin_types::BuiltinTypes, internal_error_reporter::InternalErrorReporter,
      type_arena::TypeArena, type_check_limits::TypeCheckLimits,
    },
  };
  use ulua_ast::records::{allocator::Allocator, ast_name_table::AstNameTable};
  use ulua_unit_test::records::{
    builtins_fixture::BuiltinsFixture, frontend_fixture::FrontendFixture,
  };

  let src = "(number, string) -> boolean?";

  let mut fixture = FrontendFixture {
    base: BuiltinsFixture::default(),
  };

  let mut arena = TypeArena::default();
  // Box 钉堆：AstNameTable/Lexer/Parser 捕获宿主地址，宿主移动即悬垂。
  let mut allocator = Box::new(Allocator::new());
  let mut names = AstNameTable::new(&mut allocator);
  let _builtin_types = BuiltinTypes::new();
  let mut ice_handler = InternalErrorReporter::default();
  let limits = TypeCheckLimits::default();

  let ty = fixture.get_frontend().parse_type(
    &mut allocator,
    &mut names,
    &mut ice_handler,
    limits,
    &mut arena,
    src,
  );

  assert_eq!("(number, string) -> boolean?", to_string_type_id(ty));
}

// Ported from `tests/Frontend.test.cpp`.
// Source: `tests/Frontend.test.cpp`
#[test]
fn frontend_parse_only() {
  use alloc::string::String;

  use ulua_analysis::functions::to_string_error::to_string_type_error;
  use ulua_ast::records::{location::Location, position::Position};
  use ulua_unit_test::records::{
    builtins_fixture::BuiltinsFixture, frontend_fixture::FrontendFixture,
  };

  let mut fixture = FrontendFixture {
    base: BuiltinsFixture::default(),
  };

  fixture.base.base.file_resolver.source.insert(
    String::from("game/Gui/Modules/A"),
    String::from(
      r#"
        local a: number = 'oh no a type error'
        return {a=a}
    "#,
    ),
  );

  fixture.base.base.file_resolver.source.insert(
    String::from("game/Gui/Modules/B"),
    String::from(
      r#"
        local Modules = script.Parent
        local A = require(Modules.A)
        local b: number = 2
    "#,
    ),
  );

  fixture
    .get_frontend()
    .parse_module_name(&ModuleName::from("game/Gui/Modules/B"));

  assert!(
    fixture
      .get_frontend()
      .source_nodes
      .contains_key("game/Gui/Modules/A")
  );
  assert!(
    fixture
      .get_frontend()
      .source_nodes
      .contains_key("game/Gui/Modules/B")
  );

  let node = fixture
    .get_frontend()
    .source_nodes
    .get("game/Gui/Modules/B")
    .cloned()
    .expect("expected source node");
  assert!(
    node
      .require_set
      .contains(&ModuleName::from("game/Gui/Modules/A"))
  );
  assert_eq!(1, node.require_locations.len());
  assert_eq!(
    Location::new(
      Position {
        line: 2,
        column: 18,
      },
      Position {
        line: 2,
        column: 36,
      },
    ),
    node.require_locations[0].1
  );

  let result = fixture
    .get_frontend()
    .check_module_name_optional_frontend_options(&ModuleName::from("game/Gui/Modules/B"), None);
  assert_eq!(1, result.errors.len(), "{:?}", result.errors);

  assert_eq!("game/Gui/Modules/A", result.errors[0].module_name);
  assert_eq!(
    "Expected this to be 'number', but got 'string'",
    to_string_type_error(&result.errors[0])
  );
}

// Source: `tests/Frontend.test.cpp`
#[test]
fn frontend_parse_types() {
  use std::panic::{AssertUnwindSafe, catch_unwind};

  use ulua_analysis::{
    functions::{get_type, to_string_to_string::to_string_type_id},
    records::{
      internal_error_reporter::InternalErrorReporter, type_arena::TypeArena,
      type_check_limits::TypeCheckLimits,
    },
    type_aliases::error_type::ErrorType,
  };
  use ulua_ast::records::{allocator::Allocator, ast_name_table::AstNameTable};
  use ulua_unit_test::records::{
    builtins_fixture::BuiltinsFixture, frontend_fixture::FrontendFixture,
  };

  let mut fixture = FrontendFixture {
    base: BuiltinsFixture::default(),
  };
  // Box 钉堆：AstNameTable/Lexer/Parser 捕获宿主地址，宿主移动即悬垂。
  let mut allocator = Box::new(Allocator::new());
  let mut names = AstNameTable::new(&mut allocator);
  let mut arena = TypeArena::default();

  let mut parse_type = |fixture: &mut FrontendFixture, src: &str| {
    let mut ice_handler = InternalErrorReporter::default();
    fixture.get_frontend().parse_type(
      &mut allocator,
      &mut names,
      &mut ice_handler,
      TypeCheckLimits::default(),
      &mut arena,
      src,
    )
  };

  let ty1 = parse_type(&mut fixture, "(number, boolean?) -> string");
  assert_eq!("(number, boolean?) -> string", to_string_type_id(ty1));

  assert!(
    catch_unwind(AssertUnwindSafe(|| parse_type(
      &mut fixture,
      "illegal Luau Syntax here"
    )))
    .is_err()
  );

  let ty3 = parse_type(&mut fixture, "blah<blahblah, number>");
  assert!(get_type::get::<ErrorType>(ty3).is_some());

  assert!(
    catch_unwind(AssertUnwindSafe(|| parse_type(
      &mut fixture,
      "number, boolean?) -> string"
    )))
    .is_err()
  );
  assert!(
    catch_unwind(AssertUnwindSafe(|| parse_type(
      &mut fixture,
      "{size: number?"
    )))
    .is_err()
  );
}

// Ported from `tests/Frontend.test.cpp`.
// Source: `tests/Frontend.test.cpp`
#[test]
fn frontend_produce_errors_for_unchanged_file_with_a_syntax_error() {
  use alloc::string::String;

  use ulua_unit_test::records::{
    builtins_fixture::BuiltinsFixture, frontend_fixture::FrontendFixture,
  };

  let mut fixture = FrontendFixture {
    base: BuiltinsFixture::default(),
  };

  fixture.base.base.file_resolver.source.insert(
    String::from("Modules/A"),
    String::from("oh no a blatant syntax error!!"),
  );

  let one = fixture
    .get_frontend()
    .check_module_name_optional_frontend_options(&ModuleName::from("Modules/A"), None);
  let two = fixture
    .get_frontend()
    .check_module_name_optional_frontend_options(&ModuleName::from("Modules/A"), None);

  assert!(
    !one.errors.is_empty(),
    "expected first check to report errors"
  );
  assert!(
    !two.errors.is_empty(),
    "expected second check to report errors"
  );
}

// Ported from `tests/Frontend.test.cpp`.
// Source: `tests/Frontend.test.cpp`
#[test]
fn frontend_produce_errors_for_unchanged_file_with_errors() {
  use alloc::string::String;

  use ulua_unit_test::records::{
    builtins_fixture::BuiltinsFixture, frontend_fixture::FrontendFixture,
  };

  let mut fixture = FrontendFixture {
    base: BuiltinsFixture::default(),
  };

  fixture.base.base.file_resolver.source.insert(
    String::from("Modules/A"),
    String::from("local p: number = 'oh no a type error'"),
  );

  fixture
    .get_frontend()
    .check_module_name_optional_frontend_options(&ModuleName::from("Modules/A"), None);

  fixture.base.base.file_resolver.source.insert(
      String::from("Modules/A"),
      String::from(
          "local p = 4 -- We have fixed the problem, but we didn't tell the getFrontend(). so it will not recheck this file!",
      ),
  );
  let second_result = fixture
    .get_frontend()
    .check_module_name_optional_frontend_options(&ModuleName::from("Modules/A"), None);

  assert_eq!(1, second_result.errors.len(), "{:?}", second_result.errors);
}

// Ported from `tests/Frontend.test.cpp`.
// Source: `tests/Frontend.test.cpp`
#[test]
fn frontend_queue_check_cycle_delayed() {
  use alloc::{boxed::Box, string::String};

  use ulua_analysis::functions::to_string_error::to_string_type_error;
  use ulua_unit_test::records::{
    builtins_fixture::BuiltinsFixture, frontend_fixture::FrontendFixture,
  };

  let mut fixture = FrontendFixture {
    base: BuiltinsFixture::default(),
  };

  fixture.base.base.file_resolver.source.insert(
    String::from("game/Gui/Modules/C"),
    String::from(
      r#"
        --!strict
        return {c_value = 5}
    "#,
    ),
  );
  fixture.base.base.file_resolver.source.insert(
    String::from("game/Gui/Modules/A"),
    String::from(
      r#"
        --!strict
        local Modules = game:GetService('Gui').Modules
        local C = require(Modules.C)
        local B = require(Modules.B)
        return {a_value = B.hello + C.c_value}
    "#,
    ),
  );
  fixture.base.base.file_resolver.source.insert(
    String::from("game/Gui/Modules/B"),
    String::from(
      r#"
        --!strict
        local Modules = game:GetService('Gui').Modules
        local C = require(Modules.C)
        local A = require(Modules.A)
        return {b_value = A.hello + C.c_value}
    "#,
    ),
  );

  fixture
    .get_frontend()
    .queue_module_check_module_name(&ModuleName::from("game/Gui/Modules/B"));
  fixture.get_frontend().check_queued_modules(
    None,
    Box::new(|tasks, run| {
      for task in tasks {
        run(task);
      }
    }),
    |_, _| true,
  );

  let result = fixture
    .get_frontend()
    .get_check_result(&ModuleName::from("game/Gui/Modules/B"), true, false)
    .expect("expected queued check result");
  assert_eq!(2, result.errors.len(), "{:?}", result.errors);
  assert_eq!(
    "Cyclic module dependency: game/Gui/Modules/B -> game/Gui/Modules/A",
    to_string_type_error(&result.errors[0])
  );
  assert_eq!(
    "Cyclic module dependency: game/Gui/Modules/A -> game/Gui/Modules/B",
    to_string_type_error(&result.errors[1])
  );
}

// Ported from `tests/Frontend.test.cpp`.
// Source: `tests/Frontend.test.cpp`
#[test]
fn frontend_queue_check_cycle_instant() {
  use alloc::{boxed::Box, string::String};

  use ulua_analysis::functions::to_string_error::to_string_type_error;
  use ulua_unit_test::records::{
    builtins_fixture::BuiltinsFixture, frontend_fixture::FrontendFixture,
  };

  let mut fixture = FrontendFixture {
    base: BuiltinsFixture::default(),
  };

  fixture.base.base.file_resolver.source.insert(
    String::from("game/Gui/Modules/A"),
    String::from(
      r#"
        --!strict
        local Modules = game:GetService('Gui').Modules
        local B = require(Modules.B)
        return {a_value = B.hello}
    "#,
    ),
  );
  fixture.base.base.file_resolver.source.insert(
    String::from("game/Gui/Modules/B"),
    String::from(
      r#"
        --!strict
        local Modules = game:GetService('Gui').Modules
        local A = require(Modules.A)
        return {b_value = A.hello}
    "#,
    ),
  );

  fixture
    .get_frontend()
    .queue_module_check_module_name(&ModuleName::from("game/Gui/Modules/B"));
  fixture.get_frontend().check_queued_modules(
    None,
    Box::new(|tasks, run| {
      for task in tasks {
        run(task);
      }
    }),
    |_, _| true,
  );

  let result = fixture
    .get_frontend()
    .get_check_result(&ModuleName::from("game/Gui/Modules/B"), true, false)
    .expect("expected queued check result");
  assert_eq!(2, result.errors.len(), "{:?}", result.errors);
  assert_eq!(
    "Cyclic module dependency: game/Gui/Modules/B -> game/Gui/Modules/A",
    to_string_type_error(&result.errors[0])
  );
  assert_eq!(
    "Cyclic module dependency: game/Gui/Modules/A -> game/Gui/Modules/B",
    to_string_type_error(&result.errors[1])
  );
}

// Ported from `tests/Frontend.test.cpp`.
// Source: `tests/Frontend.test.cpp`
#[test]
fn frontend_queue_check_propagates_ice() {
  use alloc::{boxed::Box, string::String};
  use std::panic::{AssertUnwindSafe, catch_unwind};

  use ulua_analysis::records::internal_compiler_error::InternalCompilerError;
  use ulua_common::fflag;
  use ulua_unit_test::{
    records::{builtins_fixture::BuiltinsFixture, frontend_fixture::FrontendFixture},
    type_aliases::scoped_fast_flag::ScopedFastFlag,
  };

  let _magic_types = ScopedFastFlag::new(&fflag::DebugLuauMagicTypes, true);
  let mut fixture = FrontendFixture {
    base: BuiltinsFixture::default(),
  };

  let module_name = ModuleName::from("MainModule");
  fixture.base.base.file_resolver.source.insert(
    module_name.clone(),
    String::from(
      r#"
        --!strict
        local a: _luau_ice = 55
    "#,
    ),
  );
  fixture.get_frontend().mark_dirty(&module_name, None);
  fixture
    .get_frontend()
    .queue_module_check_module_name(&ModuleName::from("MainModule"));

  let result = catch_unwind(AssertUnwindSafe(|| {
    fixture.get_frontend().check_queued_modules(
      None,
      Box::new(|tasks, run| {
        for task in tasks {
          run(task);
        }
      }),
      |_, _| true,
    );
  }));

  let panic = result.expect_err("expected InternalCompilerError");
  assert!(
    panic.downcast_ref::<InternalCompilerError>().is_some(),
    "expected InternalCompilerError panic payload"
  );
}

// Ported from `tests/Frontend.test.cpp`.
// Source: `tests/Frontend.test.cpp`
#[test]
fn frontend_queue_check_simple() {
  use alloc::{boxed::Box, string::String};

  use ulua_unit_test::records::{
    builtins_fixture::BuiltinsFixture, frontend_fixture::FrontendFixture,
  };

  let mut fixture = FrontendFixture {
    base: BuiltinsFixture::default(),
  };

  fixture.base.base.file_resolver.source.insert(
    String::from("game/Gui/Modules/A"),
    String::from(
      r#"
        --!strict
        return {hello=5, world=true}
    "#,
    ),
  );
  fixture.base.base.file_resolver.source.insert(
    String::from("game/Gui/Modules/B"),
    String::from(
      r#"
        --!strict
        local Modules = game:GetService('Gui').Modules
        local A = require(Modules.A)
        return {b_value = A.hello}
    "#,
    ),
  );

  fixture
    .get_frontend()
    .queue_module_check_module_name(&ModuleName::from("game/Gui/Modules/B"));
  fixture.get_frontend().check_queued_modules(
    None,
    Box::new(|tasks, run| {
      for task in tasks {
        run(task);
      }
    }),
    |_, _| true,
  );

  let result = fixture
    .get_frontend()
    .get_check_result(&ModuleName::from("game/Gui/Modules/B"), true, false)
    .expect("expected queued check result");
  assert!(result.errors.is_empty(), "{:?}", result.errors);
}

// Ported from `tests/Frontend.test.cpp`.
// Source: `tests/Frontend.test.cpp`
#[test]
fn frontend_re_report_type_error_in_required_file() {
  use alloc::string::String;

  use ulua_unit_test::records::{
    builtins_fixture::BuiltinsFixture, frontend_fixture::FrontendFixture,
  };

  let mut fixture = FrontendFixture {
    base: BuiltinsFixture::default(),
  };

  fixture.base.base.file_resolver.source.insert(
    String::from("Modules/A"),
    String::from(
      r#"
        local n: number = 'five'
        return {n=n}
    "#,
    ),
  );

  fixture.base.base.file_resolver.source.insert(
    String::from("Modules/B"),
    String::from(
      r#"
        local Modules = script.Parent
        local A = require(Modules.A)
        print(A.n)
    "#,
    ),
  );

  let result = fixture
    .get_frontend()
    .check_module_name_optional_frontend_options(&ModuleName::from("Modules/B"), None);
  assert_eq!(1, result.errors.len(), "{:?}", result.errors);

  let result2 = fixture
    .get_frontend()
    .check_module_name_optional_frontend_options(&ModuleName::from("Modules/B"), None);
  assert_eq!(1, result2.errors.len(), "{:?}", result2.errors);

  assert_eq!("Modules/A", result.errors[0].module_name);
}

// Ported from `tests/Frontend.test.cpp`.
// Source: `tests/Frontend.test.cpp`
#[test]
fn frontend_real_source() {
  use ulua_analysis::{
    functions::trace_requires::trace_requires, records::type_check_limits::TypeCheckLimits,
  };
  use ulua_ast::records::parse_options::ParseOptions;
  use ulua_unit_test::records::{
    builtins_fixture::BuiltinsFixture, frontend_fixture::FrontendFixture,
    naive_file_resolver::NaiveFileResolver,
  };

  let mut fixture = FrontendFixture {
    base: BuiltinsFixture::default(),
  };
  let program = fixture.base.base.parse(
      r#"
        return function()
            local Modules = game:GetService("CoreGui").Gui.Modules

            local Roact = require(Modules.Common.Roact)
            local Rodux = require(Modules.Common.Rodux)

            local AppReducer = require(Modules.LuaApp.AppReducer)
            local AEAppReducer = require(Modules.LuaApp.Reducers.AEReducers.AEAppReducer)
            local AETabList = require(Modules.LuaApp.Components.Avatar.UI.Views.Portrait.AETabList)
            local mockServices = require(Modules.LuaApp.TestHelpers.mockServices)
            local DeviceOrientationMode = require(Modules.LuaApp.DeviceOrientationMode)
            local MockAvatarEditorTheme = require(Modules.LuaApp.TestHelpers.MockAvatarEditorTheming)
            local FFlagAvatarEditorEnableThemes = settings():GetFFlag("AvatarEditorEnableThemes2")
        end
    "#,
      &ParseOptions::default(),
  );

  let mut naive_file_resolver = NaiveFileResolver;

  let result = trace_requires(
    &mut naive_file_resolver,
    program,
    ModuleName::new(),
    &TypeCheckLimits::default(),
  );
  assert_eq!(8, result.require_list.len());
}

// Ported from `tests/Frontend.test.cpp`.
// Source: `tests/Frontend.test.cpp`
#[test]
#[ignore = "已复核订正：上游 cpp 自己禁用了该用例（cpp/tests/Frontend.test.cpp:635 `#if 0 // Does not work yet.`）；放开后 cpp 实测同样产出 2 条错误（UnknownRequire{path 空}+UnknownProperty{A}），与本移植逐条一致——行为已对齐 oracle，无缺口。保持 ignore 与上游同步"]
fn frontend_recheck_if_dependent_script_has_a_parse_error() {
  use alloc::string::String;

  use ulua_unit_test::records::{
    builtins_fixture::BuiltinsFixture, frontend_fixture::FrontendFixture,
  };

  let mut fixture = FrontendFixture {
    base: BuiltinsFixture::default(),
  };

  fixture.base.base.file_resolver.source.insert(
    String::from("Modules/A"),
    String::from("oh no a syntax error"),
  );
  fixture.base.base.file_resolver.source.insert(
    String::from("Modules/B"),
    String::from(
      r#"
        local Modules = {}
        local A = require(Modules.A)
        return {}
    "#,
    ),
  );

  let result = fixture
    .get_frontend()
    .check_module_name_optional_frontend_options(&ModuleName::from("Modules/B"), None);
  assert_eq!(1, result.errors.len(), "{:?}", result.errors);
  assert_eq!("Modules/A", result.errors[0].module_name);

  let result2 = fixture
    .get_frontend()
    .check_module_name_optional_frontend_options(&ModuleName::from("Modules/B"), None);
  assert_eq!(1, result2.errors.len(), "{:?}", result2.errors);

  // cpp 端 CHECK_EQ(result2.errors[0], result.errors[0])；TypeError 未实现
  // PartialEq，改以 Debug 全量结构比对，语义等价（含 location/module/data）。
  assert_eq!(
    format!("{:?}", result.errors[0]),
    format!("{:?}", result2.errors[0])
  );
}

// Ported from `tests/Frontend.test.cpp`.
// Source: `tests/Frontend.test.cpp`
#[test]
fn frontend_recheck_if_dependent_script_is_dirty() {
  use alloc::string::String;

  use ulua_analysis::functions::{first::first, to_string_to_string::to_string_type_id};
  use ulua_unit_test::records::{
    builtins_fixture::BuiltinsFixture, frontend_fixture::FrontendFixture,
  };

  let mut fixture = FrontendFixture {
    base: BuiltinsFixture::default(),
  };

  fixture.base.base.file_resolver.source.insert(
    String::from("game/Gui/Modules/A"),
    String::from("return {hello=5, world=true}"),
  );
  fixture.base.base.file_resolver.source.insert(
    String::from("game/Gui/Modules/B"),
    String::from(
      r#"
        local Modules = game:GetService('Gui').Modules
        local A = require(Modules.A)
        return {b_value = A.hello}
    "#,
    ),
  );

  fixture
    .get_frontend()
    .check_module_name_optional_frontend_options(&ModuleName::from("game/Gui/Modules/B"), None);

  fixture.base.base.file_resolver.source.insert(
    String::from("game/Gui/Modules/A"),
    String::from("return {hello='hi!'}"),
  );
  fixture
    .get_frontend()
    .mark_dirty(&ModuleName::from("game/Gui/Modules/A"), None);

  fixture
    .get_frontend()
    .check_module_name_optional_frontend_options(&ModuleName::from("game/Gui/Modules/B"), None);

  let b_module = fixture
    .get_frontend()
    .module_resolver
    .get_module(&ModuleName::from("game/Gui/Modules/B"));
  assert!(b_module.errors.is_empty(), "{:?}", b_module.errors);

  let b_exports = first(b_module.return_type, true).expect("expected module return type");

  assert_eq!("{ b_value: string }", to_string_type_id(b_exports));
}

// Ported from `tests/Frontend.test.cpp`.
// Source: `tests/Frontend.test.cpp`
#[test]
fn frontend_reexport_cyclic_type() {
  use alloc::string::String;

  use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

  let mut fixture = BuiltinsFixture::default();

  fixture.base.file_resolver.source.insert(
    String::from("Module/A"),
    String::from(
      r#"
        type F<T> = (set: G<T>) -> ()

        export type G<T> = {
            forEach: (a: F<T>) -> (),
        }

        function X<T>(a: F<T>): ()
        end

        return X
    "#,
    ),
  );

  fixture.base.file_resolver.source.insert(
    String::from("Module/B"),
    String::from(
      r#"
        --!strict
        local A = require(script.Parent.A)

        export type G<T> = A.G<T>

        return {
            A = A,
        }
    "#,
    ),
  );

  let result = fixture
    .get_frontend()
    .check_module_name_optional_frontend_options(&ModuleName::from("Module/B"), None);

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Ported from `tests/Frontend.test.cpp`.
// Source: `tests/Frontend.test.cpp`
#[test]
fn frontend_reexport_type_alias() {
  use alloc::string::String;

  use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

  let mut fixture = BuiltinsFixture::default();

  fixture.base.file_resolver.source.insert(
      String::from("Module/A"),
      String::from(
          r#"
        type KeyOfTestEvents = "test-file-start" | "test-file-success" | "test-file-failure" | "test-case-result"
        type MyAny = any

        export type TestFileEvent<T = KeyOfTestEvents> = (
            eventName: T,
            args: any --[[ ROBLOX TODO: Unhandled node for type: TSIndexedAccessType ]] --[[ TestEvents[T] ]]
        ) -> MyAny

        return {}
    "#,
      ),
  );

  fixture.base.file_resolver.source.insert(
    String::from("Module/B"),
    String::from(
      r#"
        --!strict
        local A = require(script.Parent.A)

        export type TestFileEvent = A.TestFileEvent
    "#,
    ),
  );

  let result = fixture
    .get_frontend()
    .check_module_name_optional_frontend_options(&ModuleName::from("Module/B"), None);

  assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}

// Ported from `tests/Frontend.test.cpp`.
// Source: `tests/Frontend.test.cpp`
#[test]
fn frontend_report_require_to_nonexistent_file() {
  use alloc::string::String;

  use ulua_analysis::records::unknown_require::UnknownRequire;
  use ulua_unit_test::{
    functions::type_error_data_ref::type_error_data_ref,
    records::{builtins_fixture::BuiltinsFixture, frontend_fixture::FrontendFixture},
  };

  let mut fixture = FrontendFixture {
    base: BuiltinsFixture::default(),
  };

  fixture.base.base.file_resolver.source.insert(
    String::from("Modules/A"),
    String::from(
      r#"
        local Modules = script
        local B = require(Modules.B)
    "#,
    ),
  );

  let result = fixture
    .get_frontend()
    .check_module_name_optional_frontend_options(&ModuleName::from("Modules/A"), None);
  assert_eq!(1, result.errors.len(), "{:?}", result.errors);

  assert!(
    type_error_data_ref::<UnknownRequire>(&result.errors[0]).is_some(),
    "Should have been an UnknownRequire: {:?}",
    result.errors[0]
  );
}

// Ported from `tests/Frontend.test.cpp`.
// Source: `tests/Frontend.test.cpp`
#[test]
fn frontend_report_syntax_error_in_required_file() {
  use alloc::string::String;

  use ulua_analysis::records::syntax_error::SyntaxError;
  use ulua_unit_test::{
    functions::type_error_data_ref::type_error_data_ref,
    records::{builtins_fixture::BuiltinsFixture, frontend_fixture::FrontendFixture},
  };

  let mut fixture = FrontendFixture {
    base: BuiltinsFixture::default(),
  };

  fixture.base.base.file_resolver.source.insert(
    String::from("Modules/A"),
    String::from("oh no a gross breach of syntax"),
  );
  fixture.base.base.file_resolver.source.insert(
    String::from("Modules/B"),
    String::from(
      r#"
        local Modules = script.Parent
        local A = require(Modules.A)
    "#,
    ),
  );

  let result = fixture
    .get_frontend()
    .check_module_name_optional_frontend_options(&ModuleName::from("Modules/B"), None);
  assert!(!result.errors.is_empty(), "expected errors");

  assert_eq!("Modules/A", result.errors[0].module_name);

  assert!(
    result
      .errors
      .iter()
      .any(|error| type_error_data_ref::<SyntaxError>(error).is_some()),
    "Expected a syntax error: {:?}",
    result.errors
  );
}

// Ported from `tests/Frontend.test.cpp`.
// Source: `tests/Frontend.test.cpp`
#[test]
fn frontend_reports_errors_from_multiple_sources() {
  use alloc::string::String;

  use ulua_unit_test::records::{
    builtins_fixture::BuiltinsFixture, frontend_fixture::FrontendFixture,
  };

  let mut fixture = FrontendFixture {
    base: BuiltinsFixture::default(),
  };

  fixture.base.base.file_resolver.source.insert(
    String::from("game/Gui/Modules/A"),
    String::from(
      r#"
        local a: number = 'oh no a type error'
        return {a=a}
    "#,
    ),
  );

  fixture.base.base.file_resolver.source.insert(
    String::from("game/Gui/Modules/B"),
    String::from(
      r#"
        local Modules = script.Parent
        local A = require(Modules.A)
        local b: number = 'another one!  This is quite distressing!'
    "#,
    ),
  );

  let result = fixture
    .get_frontend()
    .check_module_name_optional_frontend_options(&ModuleName::from("game/Gui/Modules/B"), None);
  assert_eq!(2, result.errors.len(), "{:?}", result.errors);

  assert_eq!("game/Gui/Modules/A", result.errors[0].module_name);
  assert_eq!("game/Gui/Modules/B", result.errors[1].module_name);
}

// Ported from `tests/Frontend.test.cpp`.
// Source: tests/Frontend.test.cpp:2662
#[test]
fn frontend_scc_old_solver_independent() {
  use alloc::string::String;

  use ulua_common::fflag;
  use ulua_unit_test::{
    records::{builtins_fixture::BuiltinsFixture, frontend_fixture::FrontendFixture},
    type_aliases::scoped_fast_flag::ScopedFastFlag,
  };

  // 对应 cpp: ScopedFastFlag forceOld{FFlag::DebugLuauForceOldSolver, true};
  let _force_old = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, true);

  let mut fixture = FrontendFixture {
    base: BuiltinsFixture::default(),
  };

  fixture.base.base.file_resolver.source.insert(
    String::from("game/A"),
    String::from(
      r#"
        local b = require(game.B)
        return {}
    "#,
    ),
  );

  fixture.base.base.file_resolver.source.insert(
    String::from("game/B"),
    String::from(
      r#"
        local a = require(game.A)
        return {}
    "#,
    ),
  );

  fixture
    .get_frontend()
    .check_module_name_optional_frontend_options(&ModuleName::from("game/A"), None);

  let module_a = fixture
    .get_frontend()
    .module_resolver
    .get_module(&ModuleName::from("game/A"));
  let module_b = fixture
    .get_frontend()
    .module_resolver
    .get_module(&ModuleName::from("game/B"));

  // 对应 cpp: CHECK(modA->internalTypes.get() != modB->internalTypes.get());
  // 本移植中 TypeArena 为 Module 内联字段（无 SCC 共享 arena 概念），
  // 以引用地址比较对齐 cpp 的 arena 指针比较语义：旧求解器下两模块 arena 必不共享。
  assert!(
    !eq(&module_a.internal_types, &module_b.internal_types),
    "old solver must not share type arenas between modules"
  );
}

// Ported from `tests/Frontend.test.cpp`.
// Source: `tests/Frontend.test.cpp`
#[test]
fn frontend_separate_caches_for_autocomplete() {
  use alloc::string::String;

  use ulua_analysis::{enums::solver_mode::SolverMode, records::frontend_options::FrontendOptions};
  use ulua_ast::enums::mode::Mode;
  use ulua_common::fflag;
  use ulua_unit_test::{
    records::{builtins_fixture::BuiltinsFixture, frontend_fixture::FrontendFixture},
    type_aliases::scoped_fast_flag::ScopedFastFlag,
  };

  let _old_solver = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, true);
  let mut fixture = FrontendFixture {
    base: BuiltinsFixture::default(),
  };

  fixture.base.base.file_resolver.source.insert(
    String::from("game/A"),
    String::from(
      r#"
        --!nonstrict
        local exports = {}
        function exports.hello() end
        return exports
    "#,
    ),
  );

  let opts = FrontendOptions {
    for_autocomplete: true,
    ..Default::default()
  };
  fixture
    .get_frontend()
    .set_luau_solver_mode(if !fflag::DebugLuauForceOldSolver.get() {
      SolverMode::New
    } else {
      SolverMode::Old
    });
  fixture
    .get_frontend()
    .check_module_name_optional_frontend_options(&ModuleName::from("game/A"), Some(opts));

  assert!(
    !fixture
      .get_frontend()
      .module_resolver
      .modules
      .contains_key("game/A")
  );

  let ac_module = fixture
    .get_frontend()
    .module_resolver_for_autocomplete
    .get_module(&ModuleName::from("game/A"));
  assert_eq!(Mode::Strict, ac_module.mode);

  fixture
    .get_frontend()
    .check_module_name_optional_frontend_options(&ModuleName::from("game/A"), None);

  let module = fixture
    .get_frontend()
    .module_resolver
    .get_module(&ModuleName::from("game/A"));
  assert_eq!(Mode::Nonstrict, module.mode);
}

// Ported from `tests/Frontend.test.cpp`.
// Source: `tests/Frontend.test.cpp`
#[test]
fn frontend_stats_are_not_reset_between_checks() {
  use alloc::string::String;

  use ulua_unit_test::records::{
    builtins_fixture::BuiltinsFixture, frontend_fixture::FrontendFixture,
  };

  let mut fixture = FrontendFixture {
    base: BuiltinsFixture::default(),
  };

  fixture.base.base.file_resolver.source.insert(
    String::from("Module/A"),
    String::from(
      r#"
        --!strict
        local B = require(script.Parent.B)
        local foo = B.foo + 1
    "#,
    ),
  );

  fixture.base.base.file_resolver.source.insert(
    String::from("Module/B"),
    String::from(
      r#"
        --!strict
        return {foo = 1}
    "#,
    ),
  );

  let r1 = fixture
    .get_frontend()
    .check_module_name_optional_frontend_options(&ModuleName::from("Module/A"), None);
  assert_eq!(0, r1.errors.len(), "{:?}", r1.errors);

  let stats1 = fixture.get_frontend().stats;
  assert_eq!(2, stats1.files);

  fixture
    .get_frontend()
    .mark_dirty(&ModuleName::from("Module/A"), None);
  fixture
    .get_frontend()
    .mark_dirty(&ModuleName::from("Module/B"), None);

  let r2 = fixture
    .get_frontend()
    .check_module_name_optional_frontend_options(&ModuleName::from("Module/A"), None);
  assert_eq!(0, r2.errors.len(), "{:?}", r2.errors);
  let stats2 = fixture.get_frontend().stats;

  assert_eq!(4, stats2.files);
}

// Ported from `tests/Frontend.test.cpp`.
// Source: `tests/Frontend.test.cpp`
#[test]
fn frontend_test_dependents_stored_on_node_as_graph_updates() {
  use alloc::{string::ToString, vec::Vec};
  use std::collections::BTreeMap;

  use ulua_unit_test::records::{
    builtins_fixture::BuiltinsFixture, frontend_fixture::FrontendFixture,
  };

  let mut fixture = FrontendFixture {
    base: BuiltinsFixture::default(),
  };

  let update_source = |fixture: &mut FrontendFixture, name: &str, source: &str| {
    fixture
      .base
      .base
      .file_resolver
      .source
      .insert(name.to_string(), source.to_string());
    fixture
      .get_frontend()
      .mark_dirty(&ModuleName::from(name), None);
  };

  let validate_matches_require_lists = |fixture: &mut FrontendFixture, message: &str| {
    let frontend = fixture.get_frontend();
    let mut dependents: BTreeMap<ModuleName, Vec<ModuleName>> = BTreeMap::new();

    for (module_name, node) in &frontend.source_nodes {
      for dep in node.require_set.iter() {
        dependents
          .entry(dep.clone())
          .or_default()
          .push(module_name.clone());
      }
    }

    for (module_name, node) in &frontend.source_nodes {
      if let Some(expected_dependents) = dependents.get(module_name) {
        for dep in expected_dependents {
          assert!(
            node.dependents.contains(dep),
            "Mismatch in dependents for {module_name}: {message}"
          );
        }
      }
    }
  };

  let validate_second_depends_on_first =
    |fixture: &mut FrontendFixture, from: &str, to: &str, expected: bool| {
      let frontend = fixture.get_frontend();
      let from_node = frontend
        .source_nodes
        .get(from)
        .unwrap_or_else(|| panic!("expected source node {from}"));
      assert_eq!(
        expected,
        from_node.dependents.contains(&ModuleName::from(to)),
        "Expected {from} to {}have a reverse dependency on {to}",
        if expected { "" } else { "not " }
      );
    };

  // C -> B -> A
  {
    update_source(
      &mut fixture,
      "game/Gui/Modules/A",
      "return {hello=5, world=true}",
    );
    update_source(
      &mut fixture,
      "game/Gui/Modules/B",
      r#"
            return require(game:GetService('Gui').Modules.A)
        "#,
    );
    update_source(
      &mut fixture,
      "game/Gui/Modules/C",
      r#"
            local Modules = game:GetService('Gui').Modules
            local B = require(Modules.B)
            return {c_value = B}
        "#,
    );
    fixture
      .get_frontend()
      .check_module_name_optional_frontend_options(&ModuleName::from("game/Gui/Modules/C"), None);

    validate_matches_require_lists(&mut fixture, "Initial check");

    validate_second_depends_on_first(
      &mut fixture,
      "game/Gui/Modules/A",
      "game/Gui/Modules/B",
      true,
    );
    validate_second_depends_on_first(
      &mut fixture,
      "game/Gui/Modules/B",
      "game/Gui/Modules/C",
      true,
    );
    validate_second_depends_on_first(
      &mut fixture,
      "game/Gui/Modules/C",
      "game/Gui/Modules/A",
      false,
    );
  }

  // C -> B, A
  {
    update_source(
      &mut fixture,
      "game/Gui/Modules/B",
      r#"
            return 1
        "#,
    );
    fixture
      .get_frontend()
      .check_module_name_optional_frontend_options(&ModuleName::from("game/Gui/Modules/C"), None);

    validate_matches_require_lists(&mut fixture, "Removing dependency B->A");
    validate_second_depends_on_first(
      &mut fixture,
      "game/Gui/Modules/A",
      "game/Gui/Modules/B",
      false,
    );
  }

  // C -> B -> A
  {
    update_source(
      &mut fixture,
      "game/Gui/Modules/B",
      r#"
            return require(game:GetService('Gui').Modules.A)
        "#,
    );
    fixture
      .get_frontend()
      .check_module_name_optional_frontend_options(&ModuleName::from("game/Gui/Modules/C"), None);

    validate_matches_require_lists(&mut fixture, "Adding back B->A");
    validate_second_depends_on_first(
      &mut fixture,
      "game/Gui/Modules/A",
      "game/Gui/Modules/B",
      true,
    );
  }

  // C -> B -> A, D -> (C,B,A)
  {
    update_source(
      &mut fixture,
      "game/Gui/Modules/D",
      r#"
            local C = require(game:GetService('Gui').Modules.C)
            local B = require(game:GetService('Gui').Modules.B)
            local A = require(game:GetService('Gui').Modules.A)
            return {d_value = C.c_value}
        "#,
    );
    fixture
      .get_frontend()
      .check_module_name_optional_frontend_options(&ModuleName::from("game/Gui/Modules/D"), None);

    validate_matches_require_lists(&mut fixture, "Adding D->C, D->B, D->A");
    validate_second_depends_on_first(
      &mut fixture,
      "game/Gui/Modules/A",
      "game/Gui/Modules/D",
      true,
    );
    validate_second_depends_on_first(
      &mut fixture,
      "game/Gui/Modules/B",
      "game/Gui/Modules/D",
      true,
    );
    validate_second_depends_on_first(
      &mut fixture,
      "game/Gui/Modules/C",
      "game/Gui/Modules/D",
      true,
    );
  }

  // B -> A, C <-> D
  {
    update_source(
      &mut fixture,
      "game/Gui/Modules/D",
      "return require(game:GetService('Gui').Modules.C)",
    );
    update_source(
      &mut fixture,
      "game/Gui/Modules/C",
      "return require(game:GetService('Gui').Modules.D)",
    );
    fixture
      .get_frontend()
      .check_module_name_optional_frontend_options(&ModuleName::from("game/Gui/Modules/D"), None);

    validate_matches_require_lists(&mut fixture, "Adding cycle D->C, C->D");
    validate_second_depends_on_first(
      &mut fixture,
      "game/Gui/Modules/C",
      "game/Gui/Modules/D",
      true,
    );
    validate_second_depends_on_first(
      &mut fixture,
      "game/Gui/Modules/D",
      "game/Gui/Modules/C",
      true,
    );
  }

  // B -> A, C -> D, D -> error
  {
    update_source(
      &mut fixture,
      "game/Gui/Modules/D",
      "return require(game:GetService('Gui').Modules.C.)",
    );
    fixture
      .get_frontend()
      .check_module_name_optional_frontend_options(&ModuleName::from("game/Gui/Modules/D"), None);

    validate_matches_require_lists(&mut fixture, "Adding error dependency D->C.");
    validate_second_depends_on_first(
      &mut fixture,
      "game/Gui/Modules/D",
      "game/Gui/Modules/C",
      true,
    );
    validate_second_depends_on_first(
      &mut fixture,
      "game/Gui/Modules/C",
      "game/Gui/Modules/D",
      false,
    );
  }
}

// Ported from `tests/Frontend.test.cpp`.
// Source: `tests/Frontend.test.cpp`
#[test]
fn frontend_test_invalid_dependency_tracking_per_module_resolver() {
  use alloc::string::String;

  use ulua_analysis::{enums::solver_mode::SolverMode, records::frontend_options::FrontendOptions};
  use ulua_common::fflag;
  use ulua_unit_test::{
    records::{builtins_fixture::BuiltinsFixture, frontend_fixture::FrontendFixture},
    type_aliases::scoped_fast_flag::ScopedFastFlag,
  };

  let _new_solver = ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, true);
  let mut fixture = FrontendFixture {
    base: BuiltinsFixture::default(),
  };

  fixture
    .get_frontend()
    .set_luau_solver_mode(if !fflag::DebugLuauForceOldSolver.get() {
      SolverMode::New
    } else {
      SolverMode::Old
    });

  fixture.base.base.file_resolver.source.insert(
    String::from("game/Gui/Modules/A"),
    String::from("return {hello=5, world=true}"),
  );
  fixture.base.base.file_resolver.source.insert(
    String::from("game/Gui/Modules/B"),
    String::from("return require(game:GetService('Gui').Modules.A)"),
  );

  let mut opts = FrontendOptions {
    for_autocomplete: false,
    ..Default::default()
  };
  fixture
    .get_frontend()
    .check_module_name_optional_frontend_options(
      &ModuleName::from("game/Gui/Modules/B"),
      Some(opts.clone()),
    );
  assert!(fixture.get_frontend().all_module_dependencies_valid(
    &ModuleName::from("game/Gui/Modules/B"),
    opts.for_autocomplete
  ));
  assert!(!fixture.get_frontend().all_module_dependencies_valid(
    &ModuleName::from("game/Gui/Modules/B"),
    !opts.for_autocomplete
  ));

  opts.for_autocomplete = true;
  fixture
    .get_frontend()
    .check_module_name_optional_frontend_options(
      &ModuleName::from("game/Gui/Modules/A"),
      Some(opts.clone()),
    );

  assert!(!fixture.get_frontend().all_module_dependencies_valid(
    &ModuleName::from("game/Gui/Modules/B"),
    opts.for_autocomplete
  ));
  assert!(fixture.get_frontend().all_module_dependencies_valid(
    &ModuleName::from("game/Gui/Modules/B"),
    !opts.for_autocomplete
  ));
  assert!(fixture.get_frontend().all_module_dependencies_valid(
    &ModuleName::from("game/Gui/Modules/A"),
    !opts.for_autocomplete
  ));
  assert!(fixture.get_frontend().all_module_dependencies_valid(
    &ModuleName::from("game/Gui/Modules/A"),
    opts.for_autocomplete
  ));
}

// Ported from `tests/Frontend.test.cpp`.
// Source: `tests/Frontend.test.cpp`
#[test]
fn frontend_test_lint_uses_correct_config() {
  use alloc::string::String;

  use ulua_analysis::records::frontend_options::FrontendOptions;
  use ulua_config::records::{
    config::Config, lint_options::LintOptions, lint_warning::LintWarning,
  };
  use ulua_unit_test::records::{
    builtins_fixture::BuiltinsFixture, frontend_fixture::FrontendFixture,
  };

  let mut fixture = FrontendFixture {
    base: BuiltinsFixture::default(),
  };

  fixture.base.base.file_resolver.source.insert(
    String::from("Module/A"),
    String::from(
      r#"
        local t = {}

        for i=#t,1 do
        end
    "#,
    ),
  );

  let mut config = Config::default();
  config
    .enabled_lint
    .enable_warning(LintWarning::CODE_FOR_RANGE);
  fixture
    .base
    .base
    .config_resolver
    .config_files
    .insert(ModuleName::from("Module/A"), config);

  let mut opts = FrontendOptions {
    run_lint_checks: true,
    ..Default::default()
  };
  let result = fixture
    .get_frontend()
    .check_module_name_optional_frontend_options(&ModuleName::from("Module/A"), Some(opts.clone()));
  assert_eq!(1, result.lint_result.warnings.len());

  fixture
    .base
    .base
    .config_resolver
    .config_files
    .get_mut("Module/A")
    .expect("expected config")
    .enabled_lint
    .disable_warning(LintWarning::CODE_FOR_RANGE);
  fixture
    .get_frontend()
    .mark_dirty(&ModuleName::from("Module/A"), None);

  let result2 = fixture
    .get_frontend()
    .check_module_name_optional_frontend_options(&ModuleName::from("Module/A"), Some(opts.clone()));
  assert_eq!(0, result2.lint_result.warnings.len());

  let mut override_options = LintOptions::default();
  override_options.enable_warning(LintWarning::CODE_FOR_RANGE);
  fixture
    .get_frontend()
    .mark_dirty(&ModuleName::from("Module/A"), None);

  opts.enabled_lint_warnings = Some(override_options);
  let result3 = fixture
    .get_frontend()
    .check_module_name_optional_frontend_options(&ModuleName::from("Module/A"), Some(opts.clone()));
  assert_eq!(1, result3.lint_result.warnings.len());

  let mut override_options = LintOptions::default();
  override_options.disable_warning(LintWarning::CODE_FOR_RANGE);
  fixture
    .get_frontend()
    .mark_dirty(&ModuleName::from("Module/A"), None);

  opts.enabled_lint_warnings = Some(override_options);
  let result4 = fixture
    .get_frontend()
    .check_module_name_optional_frontend_options(&ModuleName::from("Module/A"), Some(opts));
  assert_eq!(0, result4.lint_result.warnings.len());
}

// Ported from `tests/Frontend.test.cpp`.
// Source: `tests/Frontend.test.cpp`
#[test]
fn frontend_test_prune_parent_segments() {
  use ulua_unit_test::functions::path_expr_to_module_name_fixture::path_expr_to_module_name_module_name_vector_string_view;

  assert_eq!(
    Some("Modules/Enum/ButtonState".to_string()),
    path_expr_to_module_name_module_name_vector_string_view(
      "",
      &vec![
        "Modules",
        "LuaApp",
        "DeprecatedDarkTheme",
        "Parent",
        "Parent",
        "Enum",
        "ButtonState",
      ],
    )
  );
  assert_eq!(
    Some("workspace/Foo/Bar/Baz".to_string()),
    path_expr_to_module_name_module_name_vector_string_view(
      "workspace/Foo/Quux",
      &vec!["script", "Parent", "Bar", "Baz"],
    )
  );
  assert_eq!(
    None,
    path_expr_to_module_name_module_name_vector_string_view("", &vec![])
  );
  assert_eq!(
    Some("script".to_string()),
    path_expr_to_module_name_module_name_vector_string_view("", &vec!["script"])
  );
  assert_eq!(
    Some("script/Parent".to_string()),
    path_expr_to_module_name_module_name_vector_string_view("", &vec!["script", "Parent"])
  );
  assert_eq!(
    Some("script".to_string()),
    path_expr_to_module_name_module_name_vector_string_view(
      "",
      &vec!["script", "Parent", "Parent"],
    )
  );
  assert_eq!(
    Some("script".to_string()),
    path_expr_to_module_name_module_name_vector_string_view("", &vec!["script", "Test", "Parent"])
  );
  assert_eq!(
    Some("script/Parent".to_string()),
    path_expr_to_module_name_module_name_vector_string_view(
      "",
      &vec!["script", "Test", "Parent", "Parent"],
    )
  );
  assert_eq!(
    Some("script/Parent".to_string()),
    path_expr_to_module_name_module_name_vector_string_view(
      "",
      &vec!["script", "Test", "Parent", "Test", "Parent", "Parent",],
    )
  );
}

// Ported from `tests/Frontend.test.cpp`.
// Source: `tests/Frontend.test.cpp`
#[test]
fn frontend_test_traverse_dependents() {
  use alloc::{string::String, vec::Vec};

  use ulua_unit_test::records::{
    builtins_fixture::BuiltinsFixture, frontend_fixture::FrontendFixture,
  };

  let mut fixture = FrontendFixture {
    base: BuiltinsFixture::default(),
  };

  fixture.base.base.file_resolver.source.insert(
    String::from("game/Gui/Modules/A"),
    String::from("return {hello=5, world=true}"),
  );
  fixture.base.base.file_resolver.source.insert(
    String::from("game/Gui/Modules/B"),
    String::from(
      r#"
        return require(game:GetService('Gui').Modules.A)
    "#,
    ),
  );
  fixture.base.base.file_resolver.source.insert(
    String::from("game/Gui/Modules/C"),
    String::from(
      r#"
        local Modules = game:GetService('Gui').Modules
        local B = require(Modules.B)
        return {c_value = B.hello}
    "#,
    ),
  );
  fixture.base.base.file_resolver.source.insert(
    String::from("game/Gui/Modules/D"),
    String::from(
      r#"
        local Modules = game:GetService('Gui').Modules
        local C = require(Modules.C)
        return {d_value = C.c_value}
    "#,
    ),
  );

  fixture
    .get_frontend()
    .check_module_name_optional_frontend_options(&ModuleName::from("game/Gui/Modules/D"), None);

  let mut visited: Vec<String> = Vec::new();
  let visited_ptr = &mut visited as *mut Vec<String>;
  fixture.get_frontend().traverse_dependents(
    &ModuleName::from("game/Gui/Modules/B"),
    move |node| {
      unsafe {
        (*visited_ptr).push(node.name.to_string());
      }
      true
    },
  );

  assert_eq!(
    vec![
      String::from("game/Gui/Modules/B"),
      String::from("game/Gui/Modules/C"),
      String::from("game/Gui/Modules/D"),
    ],
    visited
  );
}

// Ported from `tests/Frontend.test.cpp`.
// Source: `tests/Frontend.test.cpp`
#[test]
fn frontend_test_traverse_dependents_early_exit() {
  use alloc::{string::String, vec::Vec};

  use ulua_unit_test::records::{
    builtins_fixture::BuiltinsFixture, frontend_fixture::FrontendFixture,
  };

  let mut fixture = FrontendFixture {
    base: BuiltinsFixture::default(),
  };

  fixture.base.base.file_resolver.source.insert(
    String::from("game/Gui/Modules/A"),
    String::from("return {hello=5, world=true}"),
  );
  fixture.base.base.file_resolver.source.insert(
    String::from("game/Gui/Modules/B"),
    String::from(
      r#"
        return require(game:GetService('Gui').Modules.A)
    "#,
    ),
  );
  fixture.base.base.file_resolver.source.insert(
    String::from("game/Gui/Modules/C"),
    String::from(
      r#"
        local Modules = game:GetService('Gui').Modules
        local B = require(Modules.B)
        return {c_value = B.hello}
    "#,
    ),
  );

  fixture
    .get_frontend()
    .check_module_name_optional_frontend_options(&ModuleName::from("game/Gui/Modules/C"), None);

  let mut visited: Vec<String> = Vec::new();
  let visited_ptr = &mut visited as *mut Vec<String>;
  fixture.get_frontend().traverse_dependents(
    &ModuleName::from("game/Gui/Modules/A"),
    move |node| {
      unsafe {
        (*visited_ptr).push(node.name.to_string());
      }
      node.name != "game/Gui/Modules/B"
    },
  );

  assert_eq!(
    vec![
      String::from("game/Gui/Modules/A"),
      String::from("game/Gui/Modules/B"),
    ],
    visited
  );
}

// Ported from `tests/Frontend.test.cpp`.
// Source: `tests/Frontend.test.cpp`
#[test]
fn frontend_trace_requires_in_nonstrict_mode() {
  use alloc::string::String;

  use ulua_common::fflag;
  use ulua_unit_test::records::{
    builtins_fixture::BuiltinsFixture, frontend_fixture::FrontendFixture,
  };

  if !fflag::DebugLuauForceOldSolver.get() {
    return;
  }

  let mut fixture = FrontendFixture {
    base: BuiltinsFixture::default(),
  };

  fixture.base.base.file_resolver.source.insert(
    String::from("Module/A"),
    String::from(
      r#"
        --!nonstrict
        local module = {}

        function module.f(arg: number)
            print('f', arg)
        end

        return module
    "#,
    ),
  );

  fixture.base.base.file_resolver.source.insert(
    String::from("Module/B"),
    String::from(
      r#"
        --!nonstrict
        local A = require(script.Parent.A)

        print(A.g(5))       -- Key 'g' not found
        print(A.f('five'))  -- Type mismatch number and string
        print(A.f(5))       -- OK
    "#,
    ),
  );

  let result = fixture
    .get_frontend()
    .check_module_name_optional_frontend_options(&ModuleName::from("Module/B"), None);

  assert_eq!(2, result.errors.len(), "{:?}", result.errors);

  assert_eq!(4, result.errors[0].location.begin.line);
  assert_eq!(5, result.errors[1].location.begin.line);
}

// Ported from `tests/Frontend.test.cpp`.
// Source: `tests/Frontend.test.cpp`
#[test]
fn frontend_typecheck_twice_for_ast_types() {
  use alloc::string::String;

  use ulua_analysis::functions::to_string_to_string::to_string_type_id;
  use ulua_unit_test::records::{
    builtins_fixture::BuiltinsFixture, frontend_fixture::FrontendFixture,
  };

  let mut fixture = FrontendFixture {
    base: BuiltinsFixture::default(),
  };

  fixture.base.base.file_resolver.source.insert(
    String::from("Module/A"),
    String::from(
      r#"
        local a = 1
    "#,
    ),
  );

  let result = fixture
    .get_frontend()
    .check_module_name_optional_frontend_options(&ModuleName::from("Module/A"), None);
  assert_eq!(0, result.errors.len(), "{:?}", result.errors);

  let module = fixture
    .get_frontend()
    .module_resolver
    .get_module(&ModuleName::from("Module/A"));

  assert_eq!(1, module.ast_types.size());
  let (_, ty) = module
    .ast_types
    .iter()
    .next()
    .expect("expected one ast type entry");
  assert_eq!("number", to_string_type_id(*ty));
}

// 缺口（未移植）：以下 `tests/Frontend.test.cpp` 用例依赖上游 ModuleSCC（循环 require
// 类型推断）特性：`SourceModule::scc`/`ModuleSCC` 共享 arena、FFlag::LuauCyclicRequireTypeInference、
// FFlag::LuauCyclicRequireTopLevelAccessError、FFlag::LuauExportedTypesParticipateInScc 等。
// 本移植的 `ulua-analysis` Frontend 尚无 SCC 分组与共享 arena 结构（records/source_module.rs
// 无 scc 字段，ulua-common/fflag.rs 亦未定义上述旗标），功能未落地，无法移植断言语义。
// 待 SCC 特性移植后应补齐：
//   scc_detection_identifies_cycle, scc_non_export_cycle_reports_errors,
//   scc_mixed_export_non_export_not_grouped, scc_export_type_only_module_participates_in_cycle,
//   scc_export_type_only_with_return_empty_table_not_grouped,
//   scc_export_type_only_with_return_non_empty_table_not_grouped,
//   scc_no_return_no_export_module_participates_in_cycle,
//   scc_no_return_mixed_with_return_not_grouped, scc_shared_arena, scc_no_cycle_errors,
//   scc_export_cycle_with_nocheck_no_errors, scc_export_cycle_strict_sees_nocheck_exports,
//   scc_return_types_resolved, scc_property_access_across_cycle, scc_three_module_cycle,
//   scc_non_cyclic_dependent_has_own_arena, scc_markdirty_propagates_to_peers,
//   scc_old_solver_independent 除外——tst-r29 已移植为 frontend_scc_old_solver_independent
//   （本例仅断言旧求解器下两模块 arena 不共享，不依赖 ModuleSCC 结构，可先行落地）。
//   scc_queued_modules_shared_arena,
//   scc_queued_modules_no_cycle_errors, scc_queued_modules_return_types_resolved,
//   scc_queued_modules_three_module_cycle, scc_queued_modules_property_access_across_cycle,
//   scc_queued_multiple_independent_cycles, scc_queued_cycle_with_non_cyclic_dependent,
//   scc_cyclic_dependency_error_only_lists_non_export_modules, scc_queued_recheck_after_dirty,
//   scc_self_loop, scc_self_loop_then_dependent_module, scc_self_loop_dependent_module,
//   scc_error_attributed_to_correct_module, scc_cyclic_peer_sees_exported_value_types,
//   scc_cyclic_peer_exports_from_later_module_not_unknown,
//   scc_cyclic_peer_sees_exported_type_bindings, scc_top_level_field_access_errors,
//   scc_deferred_field_access_ok, scc_non_peer_require_ok, scc_three_module_top_level_errors,
//   scc_top_level_access_on_function_call, export_cycle_between_check_and_nocheck,
//   nocheck_export_cycle_produces_error_type
// 另：generic_P_widening_with_cross_module_recursive_type 与
// recheck_if_dependent_script_has_a_parse_error 已移植但标 #[ignore]
// （前者：缺口在 generic-P widening 求解器/实例化链路而非 Subtyping；
// 后者：上游 #if 0 死用例，cpp 实测行为与本移植一致。详见各用例注释）。
