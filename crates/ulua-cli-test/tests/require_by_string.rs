extern crate alloc;

use core::{
  ffi::{CStr, c_int},
  ptr::null,
};

use ulua_cli_test::{
  enums::path_type::PathType, records::repl_with_path_fixture::ReplWithPathFixture,
};
use ulua_common::fflag::{
  DebugLuauUserDefinedClasses, DebugLuauUserDefinedClassesRuntime, LuauExportValueSyntax,
};
use ulua_vm::{
  enums::lua_type::LuaType,
  functions::{lua_getfield::lua_getfield, lua_l_findtable::lua_l_findtable, lua_type::lua_type},
  macros::lua_registryindex::LUA_REGISTRYINDEX,
  records::lua_state::lua_State,
};

// `cpp/tests/RequireByString.test.cpp` 的移植：require-by-string 解析与导出测试。

/// 断言 `_MODULES` 缓存表中 `key` 对应模块的存在性符合 `present`
///
/// 对齐 cpp 中重复出现的缓存检查样板（`luaL_findtable` + `lua_getfield` + CHECK）
fn assert_module_cache(l: *mut lua_State, key: &CStr, present: bool, context: &str) {
  // SAFETY: `l` 指向 fixture 初始化完成的主线程；`key` 为合法 C 字符串；findtable/getfield 的栈操作配平。
  unsafe {
    lua_l_findtable(l, LUA_REGISTRYINDEX, c"_MODULES".as_ptr(), 1);
    lua_getfield(l, -1, key.as_ptr());
    let cached = lua_type(l, -1) != LuaType::Nil as c_int;
    assert!(cached == present, "{context}");
  }
}

/// 在主线程上执行 Luau 源码（对齐 cpp 的 `runCode(L, src)`）
fn run_code_str(l: *mut lua_State, src: &str) {
  use ulua_repl_cli::functions::run_code::run_code;

  // SAFETY: `l` 指向 fixture 初始化完成的主线程，`src` 为合法 UTF-8 源码。
  let _ = unsafe { run_code(l, src) };
}

/// 注册运行时模块 `@test/helloworld`（hello = "world"）供 require 读取
fn register_test_module(l: *mut lua_State) {
  use ulua_require::functions::luarequire_registermodule::luarequire_registermodule;
  use ulua_vm::{
    functions::{
      lua_call::lua_call, lua_pushcclosurek::lua_pushcclosurek, lua_pushstring::lua_pushstring,
      lua_settable::lua_settable,
    },
    macros::lua_newtable::lua_newtable,
  };

  // SAFETY: `l` 指向 fixture 初始化完成的主线程；pushcclosurek/pushstring/newtable/settable/call 的栈序列配平。
  unsafe {
    lua_pushcclosurek(l, Some(luarequire_registermodule), null(), 0, None);
    lua_pushstring(l, c"@test/helloworld".as_ptr());
    lua_newtable(l);
    lua_pushstring(l, c"hello".as_ptr());
    lua_pushstring(l, c"world".as_ptr());
    lua_settable(l, -3);
    lua_call(l, 2, 0);
  }
}

/// `ScopedFastFlag{LuauExportValueSyntax, true}` 的 RAII guard，Drop 时弹出覆盖
fn sff_export_value() -> impl Drop {
  LuauExportValueSyntax.push_test_override(true);
  struct Sff;
  impl Drop for Sff {
    fn drop(&mut self) {
      LuauExportValueSyntax.pop_test_override();
    }
  }
  Sff
}

/// 用户自定义类三 flag 组合的 RAII guard（cpp 的 ScopedFastFlag 数组移植）
fn sff_user_defined_classes() -> impl Drop {
  LuauExportValueSyntax.push_test_override(true);
  DebugLuauUserDefinedClasses.push_test_override(true);
  DebugLuauUserDefinedClassesRuntime.push_test_override(true);
  struct Sff;
  impl Drop for Sff {
    fn drop(&mut self) {
      DebugLuauUserDefinedClassesRuntime.pop_test_override();
      DebugLuauUserDefinedClasses.pop_test_override();
      LuauExportValueSyntax.pop_test_override();
    }
  }
  Sff
}

/// 夹具目录前缀:`tests/require` 下无配置与带配置两组夹具
const WITHOUT_CONFIG_ROOT: &str = "/tests/require/without_config/";
const CONFIG_TESTS_ROOT: &str = "/tests/require/config_tests/";

/// `get_luau_directory(pt) + 前缀 + rest` 样板收敛(对照 cpp `getLuauDirectory`)
fn without_config_dir(fixture: &ReplWithPathFixture, pt: PathType, rest: &str) -> String {
  fixture.get_luau_directory(pt) + WITHOUT_CONFIG_ROOT + rest
}

/// 同上,`config_tests` 组
fn config_tests_dir(fixture: &ReplWithPathFixture, pt: PathType, rest: &str) -> String {
  fixture.get_luau_directory(pt) + CONFIG_TESTS_ROOT + rest
}

#[test]
fn require_by_string_alias_has_illegal_format() {
  use ulua_cli_test::{
    methods::repl_with_path_fixture_run_protected_require::repl_with_path_fixture_run_protected_require,
    records::repl_with_path_fixture::ReplWithPathFixture,
  };

  let mut fixture = ReplWithPathFixture::new();
  let illegal_character = "@@";
  repl_with_path_fixture_run_protected_require(&fixture, illegal_character);
  fixture.assert_output_contains_all(&["false", "@@ is not a valid alias"]);
  let path_alias1 = "@.";
  repl_with_path_fixture_run_protected_require(&fixture, path_alias1);
  fixture.assert_output_contains_all(&["false", ". is not a valid alias"]);
  let path_alias2 = "@..";
  repl_with_path_fixture_run_protected_require(&fixture, path_alias2);
  fixture.assert_output_contains_all(&["false", ".. is not a valid alias"]);
  let empty_alias = "@";
  repl_with_path_fixture_run_protected_require(&fixture, empty_alias);
  fixture.assert_output_contains_all(&["false", " is not a valid alias"]);
}

#[test]
fn require_by_string_alias_not_parsed_if_configs_ambiguous() {
  use ulua_cli_test::{
    enums::path_type::PathType,
    methods::repl_with_path_fixture_run_protected_require::repl_with_path_fixture_run_protected_require,
    records::repl_with_path_fixture::ReplWithPathFixture,
  };

  let mut fixture = ReplWithPathFixture::new();
  let path = config_tests_dir(&fixture, PathType::Relative, "config_ambiguity/requirer");
  repl_with_path_fixture_run_protected_require(&fixture, &path);
  fixture.assert_output_contains_all(&[
    "false",
    "could not resolve alias \"dep\" (ambiguous configuration file)",
  ]);
}

#[test]
fn require_by_string_cannot_require_config_luau() {
  use ulua_cli_test::{
    enums::path_type::PathType,
    methods::repl_with_path_fixture_run_protected_require::repl_with_path_fixture_run_protected_require,
    records::repl_with_path_fixture::ReplWithPathFixture,
  };

  let mut fixture = ReplWithPathFixture::new();
  let path = config_tests_dir(
    &fixture,
    PathType::Relative,
    "config_cannot_be_required/requirer",
  );
  repl_with_path_fixture_run_protected_require(&fixture, &path);
  fixture.assert_output_contains_all(&["false", "could not resolve child component \".config\""]);
}

#[test]
fn require_by_string_cannot_require_init_luau_directly() {
  use ulua_cli_test::{
    enums::path_type::PathType,
    methods::repl_with_path_fixture_run_protected_require::repl_with_path_fixture_run_protected_require,
    records::repl_with_path_fixture::ReplWithPathFixture,
  };

  let mut fixture = ReplWithPathFixture::new();
  let path = without_config_dir(&fixture, PathType::Relative, "nested/init");
  repl_with_path_fixture_run_protected_require(&fixture, &path);
  fixture.assert_output_contains_all(&["false", "could not resolve child component \"init\""]);
}

#[test]
fn require_by_string_check_cache_after_require_init_lua() {
  use alloc::ffi::CString;

  use ulua_cli_test::{
    enums::path_type::PathType,
    methods::repl_with_path_fixture_run_protected_require::repl_with_path_fixture_run_protected_require,
    records::repl_with_path_fixture::ReplWithPathFixture,
  };

  let mut fixture = ReplWithPathFixture::new();
  let l = fixture.l;
  let relative_path = without_config_dir(&fixture, PathType::Relative, "lua");
  let absolute_path = without_config_dir(&fixture, PathType::Absolute, "lua");

  let cache_key = CString::new(format!("{}/init.lua", absolute_path)).unwrap();

  assert_module_cache(
    l,
    &cache_key,
    false,
    "Cache already contained module result",
  );

  repl_with_path_fixture_run_protected_require(&fixture, &relative_path);

  fixture.assert_output_contains_all(&["true", "result from init.lua"]);

  assert_module_cache(l, &cache_key, true, "Cache did not contain module result");
}

#[test]
fn require_by_string_check_cache_after_require_init_luau() {
  use alloc::ffi::CString;

  use ulua_cli_test::{
    enums::path_type::PathType,
    methods::repl_with_path_fixture_run_protected_require::repl_with_path_fixture_run_protected_require,
    records::repl_with_path_fixture::ReplWithPathFixture,
  };

  let mut fixture = ReplWithPathFixture::new();
  let l = fixture.l;
  let relative_path = without_config_dir(&fixture, PathType::Relative, "luau");
  let absolute_path = without_config_dir(&fixture, PathType::Absolute, "luau");

  let cache_key = CString::new(format!("{}/init.luau", absolute_path)).unwrap();

  assert_module_cache(
    l,
    &cache_key,
    false,
    "Cache already contained module result",
  );

  repl_with_path_fixture_run_protected_require(&fixture, &relative_path);

  fixture.assert_output_contains_all(&["true", "result from init.luau"]);

  assert_module_cache(l, &cache_key, true, "Cache did not contain module result");
}

#[test]
fn require_by_string_check_cache_after_require_lua() {
  use alloc::ffi::CString;

  use ulua_cli_test::{
    enums::path_type::PathType,
    methods::repl_with_path_fixture_run_protected_require::repl_with_path_fixture_run_protected_require,
    records::repl_with_path_fixture::ReplWithPathFixture,
  };

  let mut fixture = ReplWithPathFixture::new();
  let l = fixture.l;
  let relative_path = without_config_dir(&fixture, PathType::Relative, "lua_dependency");
  let absolute_path = without_config_dir(&fixture, PathType::Absolute, "lua_dependency");

  let key = CString::new(format!("{}.luau", absolute_path)).unwrap();
  assert_module_cache(l, &key, false, "Cache already contained module result");

  repl_with_path_fixture_run_protected_require(&fixture, &relative_path);

  fixture.assert_output_contains_all(&["true", "result from lua_dependency"]);

  let key = CString::new(format!("{}.lua", absolute_path)).unwrap();
  assert_module_cache(l, &key, true, "Cache did not contain module result");
}

#[test]
fn require_by_string_check_cache_after_require_luau() {
  use alloc::ffi::CString;

  use ulua_cli_test::{
    enums::path_type::PathType,
    methods::repl_with_path_fixture_run_protected_require::repl_with_path_fixture_run_protected_require,
    records::repl_with_path_fixture::ReplWithPathFixture,
  };

  let mut fixture = ReplWithPathFixture::new();
  let l = fixture.l;
  let relative_path = without_config_dir(&fixture, PathType::Relative, "module");
  let absolute_path = without_config_dir(&fixture, PathType::Absolute, "module");

  let cache_key = CString::new(format!("{}.luau", absolute_path)).unwrap();

  assert_module_cache(
    l,
    &cache_key,
    false,
    "Cache already contained module result",
  );

  repl_with_path_fixture_run_protected_require(&fixture, &relative_path);

  fixture.assert_output_contains_all(&["true", "result from dependency", "required into module"]);

  assert_module_cache(l, &cache_key, true, "Cache did not contain module result");
}

#[test]
fn require_by_string_check_cached_result() {
  use ulua_cli_test::{
    enums::path_type::PathType,
    methods::repl_with_path_fixture_run_protected_require::repl_with_path_fixture_run_protected_require,
    records::repl_with_path_fixture::ReplWithPathFixture,
  };

  let mut fixture = ReplWithPathFixture::new();
  let path = without_config_dir(&fixture, PathType::Relative, "validate_cache");
  repl_with_path_fixture_run_protected_require(&fixture, &path);
  fixture.assert_output_contains_all(&["true"]);
}

#[test]
fn require_by_string_check_clear_cache() {
  use alloc::ffi::CString;

  use ulua_cli_test::{
    enums::path_type::PathType,
    methods::repl_with_path_fixture_run_protected_require::repl_with_path_fixture_run_protected_require,
    records::repl_with_path_fixture::ReplWithPathFixture,
  };
  use ulua_require::functions::luarequire_clearcache::luarequire_clearcache;
  use ulua_vm::functions::{lua_call::lua_call, lua_pushcclosurek::lua_pushcclosurek};

  let mut fixture = ReplWithPathFixture::new();
  let l = fixture.l;
  let relative_path = without_config_dir(&fixture, PathType::Relative, "module");
  let absolute_path = without_config_dir(&fixture, PathType::Absolute, "module");
  let cache_key = CString::new(format!("{}.luau", absolute_path)).unwrap();

  assert_module_cache(
    l,
    &cache_key,
    false,
    "Cache already contained module result",
  );

  repl_with_path_fixture_run_protected_require(&fixture, &relative_path);

  fixture.assert_output_contains_all(&["true", "result from dependency", "required into module"]);

  assert_module_cache(l, &cache_key, true, "Cache did not contain module result");

  unsafe {
    lua_pushcclosurek(l, Some(luarequire_clearcache), null(), 0, None);
    lua_call(l, 0, 0);
  }

  assert_module_cache(l, &cache_key, false, "Cache was not cleared");
}

#[test]
fn require_by_string_check_clear_cache_entry() {
  use alloc::ffi::CString;

  use ulua_cli_test::{
    enums::path_type::PathType,
    methods::repl_with_path_fixture_run_protected_require::repl_with_path_fixture_run_protected_require,
    records::repl_with_path_fixture::ReplWithPathFixture,
  };
  use ulua_require::functions::luarequire_clearcacheentry::luarequire_clearcacheentry;
  use ulua_vm::functions::{
    lua_call::lua_call, lua_pushcclosurek::lua_pushcclosurek, lua_pushstring::lua_pushstring,
  };

  let mut fixture = ReplWithPathFixture::new();
  let l = fixture.l;
  let relative_path = without_config_dir(&fixture, PathType::Relative, "module");
  let absolute_path = without_config_dir(&fixture, PathType::Absolute, "module");
  let cache_key_str = format!("{}.luau", absolute_path);
  let cache_key = CString::new(cache_key_str).unwrap();

  assert_module_cache(
    l,
    &cache_key,
    false,
    "Cache already contained module result",
  );

  repl_with_path_fixture_run_protected_require(&fixture, &relative_path);

  fixture.assert_output_contains_all(&["true", "result from dependency", "required into module"]);

  assert_module_cache(l, &cache_key, true, "Cache did not contain module result");

  unsafe {
    lua_pushcclosurek(l, Some(luarequire_clearcacheentry), null(), 0, None);
    lua_pushstring(l, cache_key.as_ptr());
    lua_call(l, 1, 0);
  }

  assert_module_cache(l, &cache_key, false, "Cache was not cleared");
}

#[test]
fn require_by_string_export_as_function() {
  use ulua_cli_test::{
    enums::path_type::PathType,
    methods::repl_with_path_fixture_run_protected_require::repl_with_path_fixture_run_protected_require,
    records::repl_with_path_fixture::ReplWithPathFixture,
  };

  // 对齐 cpp 的 ScopedFastFlag{LuauExportValueSyntax, true}
  let _sff = sff_export_value();

  let mut fixture = ReplWithPathFixture::new();
  let path = without_config_dir(
    &fixture,
    PathType::Relative,
    "export_keyword/export_as_function",
  );
  repl_with_path_fixture_run_protected_require(&fixture, &path);
  fixture.assert_output_contains_all(&["true"]);
}

#[test]
fn require_by_string_export_counter() {
  use ulua_cli_test::{
    enums::path_type::PathType,
    methods::repl_with_path_fixture_run_protected_require::repl_with_path_fixture_run_protected_require,
    records::repl_with_path_fixture::ReplWithPathFixture,
  };

  // 对齐 cpp 的 ScopedFastFlag{LuauExportValueSyntax, true}
  let _sff = sff_export_value();

  let mut fixture = ReplWithPathFixture::new();
  let path = without_config_dir(
    &fixture,
    PathType::Relative,
    "export_keyword/require_export_counter_module",
  );
  repl_with_path_fixture_run_protected_require(&fixture, &path);
  fixture.assert_output_contains_all(&["true"]);
}

#[test]
fn require_by_string_export_post_return_mutation_error() {
  use ulua_cli_test::{
    enums::path_type::PathType,
    methods::repl_with_path_fixture_run_protected_require::repl_with_path_fixture_run_protected_require,
    records::repl_with_path_fixture::ReplWithPathFixture,
  };

  // 对齐 cpp 的 ScopedFastFlag{LuauExportValueSyntax, true}
  let _sff = sff_export_value();

  let mut fixture = ReplWithPathFixture::new();
  let path = without_config_dir(
    &fixture,
    PathType::Relative,
    "export_keyword/require_export_post_return_mutation_error",
  );
  repl_with_path_fixture_run_protected_require(&fixture, &path);
  fixture.assert_output_contains_all(&["true"]);
}

#[test]
fn require_by_string_load_string_relative() {
  use ulua_cli_test::records::repl_with_path_fixture::ReplWithPathFixture;

  let mut fixture = ReplWithPathFixture::new();
  let l = fixture.l;
  run_code_str(
    l,
    "return pcall(function() return loadstring(\"require('a/relative/path')\")() end)",
  );
  fixture.assert_output_contains_all(&["false", "require is not supported in this context"]);
}

#[test]
fn require_by_string_parse_aliases() {
  use ulua_config::{
    functions::parse_config::parse_config,
    records::{
      alias_info::AliasInfo, alias_options::AliasOptions, config::Config,
      config_options::ConfigOptions,
    },
  };

  fn check_contents(config: &Config) {
    assert_eq!(config.aliases.size(), 1);

    let key = "myalias".to_string();
    assert!(config.aliases.contains(&key));

    let alias_info: &AliasInfo = config.aliases.find(&key).unwrap();
    assert_eq!(alias_info.value, "/my/alias/path");
    assert_eq!(alias_info.original_case, "MyAlias");
  }

  let config_json = r#"{
    "aliases": {
        "MyAlias": "/my/alias/path",
    }
}"#;

  let mut config = Config::default();

  let alias_options = AliasOptions {
    config_location: Some("/default/location".to_string()),
    overwrite_aliases: true,
  };

  let options = ConfigOptions {
    compat: false,
    alias_options: Some(alias_options),
  };

  let error = parse_config(config_json, &mut config, &options);
  assert!(error.is_none(), "{error:?}");

  check_contents(&config);

  let copy_constructed_config = config.clone();
  check_contents(&copy_constructed_config);

  let mut copy_assigned_config = Config::default();
  copy_assigned_config.clone_from(&config);
  check_contents(&copy_assigned_config);
}

#[test]
fn require_by_string_path_normalization() {
  use ulua_cli_lib::functions::normalize_path::normalize_path;

  #[cfg(windows)]
  let prefix = "C:/";
  #[cfg(not(windows))]
  let prefix = "/";

  let tests: Vec<(String, String)> = vec![
    ("".into(), "./".into()),
    (".".into(), "./".into()),
    ("..".into(), "../".into()),
    ("a/relative/path".into(), "./a/relative/path".into()),
    (
      "./remove/extraneous/symbols/".into(),
      "./remove/extraneous/symbols".into(),
    ),
    (
      "./remove/extraneous//symbols".into(),
      "./remove/extraneous/symbols".into(),
    ),
    (
      "./remove/extraneous/symbols/.".into(),
      "./remove/extraneous/symbols".into(),
    ),
    (
      "./remove/extraneous/./symbols".into(),
      "./remove/extraneous/symbols".into(),
    ),
    (
      "../remove/extraneous/symbols/".into(),
      "../remove/extraneous/symbols".into(),
    ),
    (
      "../remove/extraneous//symbols".into(),
      "../remove/extraneous/symbols".into(),
    ),
    (
      "../remove/extraneous/symbols/.".into(),
      "../remove/extraneous/symbols".into(),
    ),
    (
      "../remove/extraneous/./symbols".into(),
      "../remove/extraneous/symbols".into(),
    ),
    (
      format!("{prefix}remove/extraneous/symbols/"),
      format!("{prefix}remove/extraneous/symbols"),
    ),
    (
      format!("{prefix}remove/extraneous//symbols"),
      format!("{prefix}remove/extraneous/symbols"),
    ),
    (
      format!("{prefix}remove/extraneous/symbols/."),
      format!("{prefix}remove/extraneous/symbols"),
    ),
    (
      format!("{prefix}remove/extraneous/./symbols"),
      format!("{prefix}remove/extraneous/symbols"),
    ),
    ("./remove/me/..".into(), "./remove".into()),
    ("./remove/me/../".into(), "./remove".into()),
    ("../remove/me/..".into(), "../remove".into()),
    ("../remove/me/../".into(), "../remove".into()),
    (format!("{prefix}remove/me/.."), format!("{prefix}remove")),
    (format!("{prefix}remove/me/../"), format!("{prefix}remove")),
    ("./..".into(), "../".into()),
    ("./../".into(), "../".into()),
    ("../..".into(), "../../".into()),
    ("../../".into(), "../../".into()),
    (format!("{prefix}.."), prefix.to_string()),
  ];

  for (input, expected) in tests {
    assert_eq!(normalize_path(&input), expected);
  }
}

#[test]
fn require_by_string_path_resolution() {
  use ulua_cli_lib::functions::resolve_path::resolve_path;

  #[cfg(windows)]
  let prefix = "C:/";
  #[cfg(not(windows))]
  let prefix = "/";

  let tests: Vec<(String, String, String)> = vec![
    (
      "./dep".into(),
      "./src/modules/module.luau".into(),
      "./src/modules/dep".into(),
    ),
    (
      "../dep".into(),
      "./src/modules/module.luau".into(),
      "./src/dep".into(),
    ),
    (
      "../../dep".into(),
      "./src/modules/module.luau".into(),
      "./dep".into(),
    ),
    (
      "../../".into(),
      "./src/modules/module.luau".into(),
      "./".into(),
    ),
    (
      "./dep".into(),
      "../src/modules/module.luau".into(),
      "../src/modules/dep".into(),
    ),
    (
      "../dep".into(),
      "../src/modules/module.luau".into(),
      "../src/dep".into(),
    ),
    (
      "../../dep".into(),
      "../src/modules/module.luau".into(),
      "../dep".into(),
    ),
    (
      "../../".into(),
      "../src/modules/module.luau".into(),
      "../".into(),
    ),
    (
      "./dep".into(),
      format!("{prefix}src/modules/module.luau"),
      format!("{prefix}src/modules/dep"),
    ),
    (
      "../dep".into(),
      format!("{prefix}src/modules/module.luau"),
      format!("{prefix}src/dep"),
    ),
    (
      "../../dep".into(),
      format!("{prefix}src/modules/module.luau"),
      format!("{prefix}dep"),
    ),
    (
      "../../".into(),
      format!("{prefix}src/modules/module.luau"),
      prefix.to_string(),
    ),
    (
      "../../../".into(),
      "./src/modules/module.luau".into(),
      "../".into(),
    ),
    (
      "../../../".into(),
      "../src/modules/module.luau".into(),
      "../../".into(),
    ),
    (
      "../../../".into(),
      format!("{prefix}src/modules/module.luau"),
      prefix.to_string(),
    ),
  ];

  for (input_path, input_base_file_path, expected) in tests {
    let resolved = resolve_path(&input_path, &input_base_file_path);
    assert_eq!(resolved.as_deref(), Some(expected.as_str()));
  }
}

#[test]
fn require_by_string_proxy_require() {
  use ulua_cli_test::{
    enums::path_type::PathType,
    methods::repl_with_path_fixture_run_protected_require::repl_with_path_fixture_run_protected_require,
    records::repl_with_path_fixture::ReplWithPathFixture,
  };
  use ulua_repl_cli::functions::{
    create_cli_require_context::create_cli_require_context,
    require_config_init::require_config_init,
  };
  use ulua_require::functions::luarequire_pushproxyrequire::luarequire_pushproxyrequire;
  use ulua_vm::macros::lua_setglobal::lua_setglobal;

  let mut fixture = ReplWithPathFixture::new();
  let l = fixture.l;

  unsafe {
    let ctx = create_cli_require_context(l);
    luarequire_pushproxyrequire(l, Some(require_config_init), ctx);
    lua_setglobal(l, c"proxyrequire".as_ptr());
  }

  let path = without_config_dir(&fixture, PathType::Relative, "proxy_requirer");
  repl_with_path_fixture_run_protected_require(&fixture, &path);
  fixture.assert_output_contains_all(&[
    "true",
    "result from dependency",
    "required into proxy_requirer",
  ]);
}

#[test]
fn require_by_string_register_runtime_module() {
  use ulua_cli_test::records::repl_with_path_fixture::ReplWithPathFixture;

  let mut fixture = ReplWithPathFixture::new();
  let l = fixture.l;

  register_test_module(l);

  run_code_str(l, "return require('@test/helloworld').hello == 'world'");
  fixture.assert_output_contains_all(&["true"]);
}

#[test]
fn require_by_string_register_runtime_module_case_insensitive() {
  use ulua_cli_test::records::repl_with_path_fixture::ReplWithPathFixture;

  let mut fixture = ReplWithPathFixture::new();
  let l = fixture.l;

  register_test_module(l);

  run_code_str(l, "return require('@TeSt/heLLoWoRld').hello == 'world'");
  fixture.assert_output_contains_all(&["true"]);
}

#[test]
fn require_by_string_require_absolute_path() {
  use ulua_cli_test::{
    methods::repl_with_path_fixture_run_protected_require::repl_with_path_fixture_run_protected_require,
    records::repl_with_path_fixture::ReplWithPathFixture,
  };

  let mut fixture = ReplWithPathFixture::new();
  let absolute_path = "/an/absolute/path";
  repl_with_path_fixture_run_protected_require(&fixture, absolute_path);
  fixture.assert_output_contains_all(&[
    "false",
    "require path must start with a valid prefix: ./, ../, or @",
  ]);
}

#[test]
fn require_by_string_require_alias_that_does_not_exist() {
  use ulua_cli_test::{
    methods::repl_with_path_fixture_run_protected_require::repl_with_path_fixture_run_protected_require,
    records::repl_with_path_fixture::ReplWithPathFixture,
  };

  let mut fixture = ReplWithPathFixture::new();
  let non_existent_alias = "@this.alias.does.not.exist";
  repl_with_path_fixture_run_protected_require(&fixture, non_existent_alias);
  fixture.assert_output_contains_all(&["false", "@this.alias.does.not.exist is not a valid alias"]);
}

#[test]
fn require_by_string_require_boolean() {
  use ulua_cli_test::{
    enums::path_type::PathType,
    methods::repl_with_path_fixture_run_protected_require::repl_with_path_fixture_run_protected_require,
    records::repl_with_path_fixture::ReplWithPathFixture,
  };

  let mut fixture = ReplWithPathFixture::new();
  let path = without_config_dir(&fixture, PathType::Relative, "types/boolean");
  repl_with_path_fixture_run_protected_require(&fixture, &path);
  fixture.assert_output_contains_all(&["true", "false"]);
}

#[test]
fn require_by_string_require_buffer() {
  use ulua_cli_test::{
    enums::path_type::PathType,
    methods::repl_with_path_fixture_run_protected_require::repl_with_path_fixture_run_protected_require,
    records::repl_with_path_fixture::ReplWithPathFixture,
  };

  let mut fixture = ReplWithPathFixture::new();
  let path = without_config_dir(&fixture, PathType::Relative, "types/buffer");
  repl_with_path_fixture_run_protected_require(&fixture, &path);
  fixture.assert_output_contains_all(&["true", "buffer"]);
}

#[test]
fn require_by_string_require_chained_aliases_failure_cyclic() {
  use ulua_cli_test::{
    enums::path_type::PathType,
    methods::repl_with_path_fixture_run_protected_require::repl_with_path_fixture_run_protected_require,
    records::repl_with_path_fixture::ReplWithPathFixture,
  };

  let mut fixture = ReplWithPathFixture::new();
  {
    let path = config_tests_dir(
      &fixture,
      PathType::Relative,
      "with_config/chained_aliases/subdirectory/failing_requirer_cyclic",
    );
    repl_with_path_fixture_run_protected_require(&fixture, &path);
    fixture.assert_output_contains_all(&[
            "false",
            "error requiring module \"@cyclicentry\": detected alias cycle (@cyclic1 -> @cyclic2 -> @cyclic3 -> @cyclic1)",
        ]);
  }
  {
    let path = config_tests_dir(
      &fixture,
      PathType::Relative,
      "with_config_luau/chained_aliases/subdirectory/failing_requirer_cyclic",
    );
    repl_with_path_fixture_run_protected_require(&fixture, &path);
    fixture.assert_output_contains_all(&[
            "false",
            "error requiring module \"@cyclicentry\": detected alias cycle (@cyclic1 -> @cyclic2 -> @cyclic3 -> @cyclic1)",
        ]);
  }
}

#[test]
fn require_by_string_require_chained_aliases_failure_depend_on_inner_alias() {
  use ulua_cli_test::{
    enums::path_type::PathType,
    methods::repl_with_path_fixture_run_protected_require::repl_with_path_fixture_run_protected_require,
    records::repl_with_path_fixture::ReplWithPathFixture,
  };

  let mut fixture = ReplWithPathFixture::new();
  {
    let path = config_tests_dir(
      &fixture,
      PathType::Relative,
      "with_config/chained_aliases/subdirectory/failing_requirer_inner_dependency",
    );
    repl_with_path_fixture_run_protected_require(&fixture, &path);
    fixture.assert_output_contains_all(&[
      "false",
      "error requiring module \"@dependoninner\": @passthroughinner is not a valid alias",
    ]);
  }
  {
    let path = config_tests_dir(
      &fixture,
      PathType::Relative,
      "with_config_luau/chained_aliases/subdirectory/failing_requirer_inner_dependency",
    );
    repl_with_path_fixture_run_protected_require(&fixture, &path);
    fixture.assert_output_contains_all(&[
      "false",
      "error requiring module \"@dependoninner\": @passthroughinner is not a valid alias",
    ]);
  }
}

#[test]
fn require_by_string_require_chained_aliases_failure_missing() {
  use ulua_cli_test::{
    enums::path_type::PathType,
    methods::repl_with_path_fixture_run_protected_require::repl_with_path_fixture_run_protected_require,
    records::repl_with_path_fixture::ReplWithPathFixture,
  };

  let mut fixture = ReplWithPathFixture::new();
  {
    let path = config_tests_dir(
      &fixture,
      PathType::Relative,
      "with_config/chained_aliases/subdirectory/failing_requirer_missing",
    );
    repl_with_path_fixture_run_protected_require(&fixture, &path);
    fixture.assert_output_contains_all(&[
      "false",
      "error requiring module \"@brokenchain\": @missing is not a valid alias",
    ]);
  }
  {
    let path = config_tests_dir(
      &fixture,
      PathType::Relative,
      "with_config_luau/chained_aliases/subdirectory/failing_requirer_missing",
    );
    repl_with_path_fixture_run_protected_require(&fixture, &path);
    fixture.assert_output_contains_all(&[
      "false",
      "error requiring module \"@brokenchain\": @missing is not a valid alias",
    ]);
  }
}

#[test]
fn require_by_string_require_chained_aliases_success() {
  use ulua_cli_test::{
    enums::path_type::PathType,
    methods::repl_with_path_fixture_run_protected_require::repl_with_path_fixture_run_protected_require,
    records::repl_with_path_fixture::ReplWithPathFixture,
  };

  let mut fixture = ReplWithPathFixture::new();
  {
    let path = config_tests_dir(
      &fixture,
      PathType::Relative,
      "with_config/chained_aliases/subdirectory/successful_requirer",
    );
    repl_with_path_fixture_run_protected_require(&fixture, &path);
    fixture.assert_output_contains_all(&[
      "true",
      "result from inner_dependency",
      "result from outer_dependency",
    ]);
  }
  {
    let path = config_tests_dir(
      &fixture,
      PathType::Relative,
      "with_config_luau/chained_aliases/subdirectory/successful_requirer",
    );
    repl_with_path_fixture_run_protected_require(&fixture, &path);
    fixture.assert_output_contains_all(&[
      "true",
      "result from inner_dependency",
      "result from outer_dependency",
    ]);
  }
}

#[test]
fn require_by_string_require_class_extends_non_open_parent() {
  use ulua_cli_test::{
    enums::path_type::PathType,
    methods::repl_with_path_fixture_run_protected_require::repl_with_path_fixture_run_protected_require,
    records::repl_with_path_fixture::ReplWithPathFixture,
  };

  // 对齐 cpp 的 ScopedFastFlag 组合（feedback/cost-model 相关 flag 在 Rust 端不生效，略去）
  let _sff = sff_user_defined_classes();

  let mut fixture = ReplWithPathFixture::new();
  let path = without_config_dir(
    &fixture,
    PathType::Relative,
    "class_extends_non_open_parent",
  );
  repl_with_path_fixture_run_protected_require(&fixture, &path);
  fixture.assert_output_contains_all(&["Non-open class 'Parent' cannot be extended"]);
}

#[test]
fn require_by_string_require_class_override_instance_member_error() {
  use ulua_cli_test::{
    enums::path_type::PathType,
    methods::repl_with_path_fixture_run_protected_require::repl_with_path_fixture_run_protected_require,
    records::repl_with_path_fixture::ReplWithPathFixture,
  };

  // 对齐 cpp 的 ScopedFastFlag 组合（feedback/cost-model 相关 flag 在 Rust 端不生效，略去）
  let _sff = sff_user_defined_classes();

  let mut fixture = ReplWithPathFixture::new();
  let path = without_config_dir(
    &fixture,
    PathType::Relative,
    "class_override_instance_member_error",
  );
  repl_with_path_fixture_run_protected_require(&fixture, &path);
  fixture.assert_output_contains_all(&[
    "Cannot override instance member 'x' of parent class 'Parent' in child class 'Child'",
  ]);
}

#[test]
fn require_by_string_require_export_alias() {
  use ulua_cli_test::{
    enums::path_type::PathType,
    methods::repl_with_path_fixture_run_protected_require::repl_with_path_fixture_run_protected_require,
    records::repl_with_path_fixture::ReplWithPathFixture,
  };

  // 对齐 cpp 的 ScopedFastFlag{LuauExportValueSyntax, true}
  let _sff = sff_export_value();

  let mut fixture = ReplWithPathFixture::new();
  let path = without_config_dir(
    &fixture,
    PathType::Relative,
    "export_keyword/require_export_alias",
  );
  repl_with_path_fixture_run_protected_require(&fixture, &path);
  fixture.assert_output_contains_all(&["true"]);
}

#[test]
fn require_by_string_require_export_alias_2() {
  use ulua_cli_test::{
    enums::path_type::PathType,
    methods::repl_with_path_fixture_run_protected_require::repl_with_path_fixture_run_protected_require,
    records::repl_with_path_fixture::ReplWithPathFixture,
  };

  // 对齐 cpp 的 ScopedFastFlag{LuauExportValueSyntax, true}
  let _sff = sff_export_value();

  let mut fixture = ReplWithPathFixture::new();
  let path = without_config_dir(
    &fixture,
    PathType::Relative,
    "export_keyword/require_export_alias2",
  );
  repl_with_path_fixture_run_protected_require(&fixture, &path);
  fixture.assert_output_contains_all(&["true"]);
}

#[test]
fn require_by_string_require_export_class() {
  use ulua_cli_test::{
    enums::path_type::PathType,
    methods::repl_with_path_fixture_run_protected_require::repl_with_path_fixture_run_protected_require,
    records::repl_with_path_fixture::ReplWithPathFixture,
  };

  // 对齐 cpp 的 ScopedFastFlag 三 flag 组合
  let _sff = sff_user_defined_classes();

  // we create a new fixture so the new lua_State has the class library
  let mut fixture = ReplWithPathFixture::new();
  let path = without_config_dir(
    &fixture,
    PathType::Relative,
    "export_keyword/require_export_class",
  );
  repl_with_path_fixture_run_protected_require(&fixture, &path);
  fixture.assert_output_contains_all(&["true"]);
}

#[test]
fn require_by_string_require_export_class_child_without_parent() {
  use ulua_cli_test::{
    enums::path_type::PathType,
    methods::repl_with_path_fixture_run_protected_require::repl_with_path_fixture_run_protected_require,
    records::repl_with_path_fixture::ReplWithPathFixture,
  };

  // 对齐 cpp 的 ScopedFastFlag 三 flag 组合
  let _sff = sff_user_defined_classes();

  let mut fixture = ReplWithPathFixture::new();
  let path = without_config_dir(
    &fixture,
    PathType::Relative,
    "export_keyword/require_export_class_child_without_parent",
  );
  repl_with_path_fixture_run_protected_require(&fixture, &path);
  fixture.assert_output_contains_all(&["true"]);
}

#[test]
fn require_by_string_require_export_class_both_exported() {
  use ulua_cli_test::{
    enums::path_type::PathType,
    methods::repl_with_path_fixture_run_protected_require::repl_with_path_fixture_run_protected_require,
    records::repl_with_path_fixture::ReplWithPathFixture,
  };

  // 对齐 cpp 的 ScopedFastFlag 三 flag 组合
  let _sff = sff_user_defined_classes();

  let mut fixture = ReplWithPathFixture::new();
  let path = without_config_dir(
    &fixture,
    PathType::Relative,
    "export_keyword/require_export_class_both_exported",
  );
  repl_with_path_fixture_run_protected_require(&fixture, &path);
  fixture.assert_output_contains_all(&["true"]);
}

#[test]
fn require_by_string_require_export_class_multi_level() {
  use ulua_cli_test::{
    enums::path_type::PathType,
    methods::repl_with_path_fixture_run_protected_require::repl_with_path_fixture_run_protected_require,
    records::repl_with_path_fixture::ReplWithPathFixture,
  };

  // 对齐 cpp 的 ScopedFastFlag 三 flag 组合
  let _sff = sff_user_defined_classes();

  let mut fixture = ReplWithPathFixture::new();
  let path = without_config_dir(
    &fixture,
    PathType::Relative,
    "export_keyword/require_export_class_multi_level",
  );
  repl_with_path_fixture_run_protected_require(&fixture, &path);
  fixture.assert_output_contains_all(&["true"]);
}

#[test]
fn require_by_string_require_export_compound() {
  use ulua_cli_test::{
    enums::path_type::PathType,
    methods::repl_with_path_fixture_run_protected_require::repl_with_path_fixture_run_protected_require,
    records::repl_with_path_fixture::ReplWithPathFixture,
  };

  // 对齐 cpp 的 ScopedFastFlag{LuauExportValueSyntax, true}
  let _sff = sff_export_value();

  let mut fixture = ReplWithPathFixture::new();
  let path = without_config_dir(
    &fixture,
    PathType::Relative,
    "export_keyword/require_export_compound",
  );
  repl_with_path_fixture_run_protected_require(&fixture, &path);
  fixture.assert_output_contains_all(&["true"]);
}

#[test]
fn require_by_string_require_export_const_error() {
  use ulua_cli_test::{
    enums::path_type::PathType,
    methods::repl_with_path_fixture_run_protected_require::repl_with_path_fixture_run_protected_require,
    records::repl_with_path_fixture::ReplWithPathFixture,
  };

  // 对齐 cpp 的 ScopedFastFlag{LuauExportValueSyntax, true}
  let _sff = sff_export_value();

  let mut fixture = ReplWithPathFixture::new();
  let path = without_config_dir(
    &fixture,
    PathType::Relative,
    "export_keyword/export_const_error",
  );
  repl_with_path_fixture_run_protected_require(&fixture, &path);
  fixture.assert_output_contains_all(&["Variable 'foo' is constant and may not be reassigned"]);
}

#[test]
fn require_by_string_require_export_edge_cases() {
  use ulua_cli_test::{
    enums::path_type::PathType,
    methods::repl_with_path_fixture_run_protected_require::repl_with_path_fixture_run_protected_require,
    records::repl_with_path_fixture::ReplWithPathFixture,
  };

  // 对齐 cpp 的 ScopedFastFlag{LuauExportValueSyntax, true}
  let _sff = sff_export_value();

  let mut fixture = ReplWithPathFixture::new();
  let path = without_config_dir(
    &fixture,
    PathType::Relative,
    "export_keyword/require_export_edge_cases",
  );
  repl_with_path_fixture_run_protected_require(&fixture, &path);
  fixture.assert_output_contains_all(&["true"]);
}

#[test]
fn require_by_string_require_export_forward_rebind() {
  use ulua_cli_test::{
    enums::path_type::PathType,
    methods::repl_with_path_fixture_run_protected_require::repl_with_path_fixture_run_protected_require,
    records::repl_with_path_fixture::ReplWithPathFixture,
  };

  // 对齐 cpp 的 ScopedFastFlag{LuauExportValueSyntax, true}
  let _sff = sff_export_value();

  let mut fixture = ReplWithPathFixture::new();
  let path = without_config_dir(
    &fixture,
    PathType::Relative,
    "export_keyword/require_export_forward_rebind",
  );
  repl_with_path_fixture_run_protected_require(&fixture, &path);
  fixture.assert_output_contains_all(&["true"]);
}

#[test]
fn require_by_string_require_export_freeze_local_nil_ignored() {
  use ulua_cli_test::{
    enums::path_type::PathType,
    methods::repl_with_path_fixture_run_protected_require::repl_with_path_fixture_run_protected_require,
    records::repl_with_path_fixture::ReplWithPathFixture,
  };

  // 对齐 cpp 的 ScopedFastFlag{LuauExportValueSyntax, true}
  let _sff = sff_export_value();

  let mut fixture = ReplWithPathFixture::new();
  let path = without_config_dir(
    &fixture,
    PathType::Relative,
    "export_keyword/require_export_freeze_local_nil_error",
  );
  repl_with_path_fixture_run_protected_require(&fixture, &path);
  fixture.assert_output_contains_all(&["true"]);
}

#[test]
fn require_by_string_require_export_freeze_shadowing_ignored() {
  use ulua_cli_test::{
    enums::path_type::PathType,
    methods::repl_with_path_fixture_run_protected_require::repl_with_path_fixture_run_protected_require,
    records::repl_with_path_fixture::ReplWithPathFixture,
  };

  // 对齐 cpp 的 ScopedFastFlag{LuauExportValueSyntax, true}
  let _sff = sff_export_value();

  let mut fixture = ReplWithPathFixture::new();
  let path = without_config_dir(
    &fixture,
    PathType::Relative,
    "export_keyword/require_export_freeze_shadowing",
  );
  repl_with_path_fixture_run_protected_require(&fixture, &path);
  fixture.assert_output_contains_all(&["true"]);
}

#[test]
fn require_by_string_require_export_frozen() {
  use ulua_cli_test::{
    enums::path_type::PathType,
    methods::repl_with_path_fixture_run_protected_require::repl_with_path_fixture_run_protected_require,
    records::repl_with_path_fixture::ReplWithPathFixture,
  };

  // 对齐 cpp 的 ScopedFastFlag{LuauExportValueSyntax, true}
  let _sff = sff_export_value();

  let mut fixture = ReplWithPathFixture::new();
  let path = without_config_dir(
    &fixture,
    PathType::Relative,
    "export_keyword/require_export_frozen",
  );
  repl_with_path_fixture_run_protected_require(&fixture, &path);
  fixture.assert_output_contains_all(&["true"]);
}

#[test]
fn require_by_string_require_export_frozen_mutate() {
  use ulua_cli_test::{
    enums::path_type::PathType,
    methods::repl_with_path_fixture_run_protected_require::repl_with_path_fixture_run_protected_require,
    records::repl_with_path_fixture::ReplWithPathFixture,
  };

  // 对齐 cpp 的 ScopedFastFlag{LuauExportValueSyntax, true}
  let _sff = sff_export_value();

  let mut fixture = ReplWithPathFixture::new();
  let path = without_config_dir(
    &fixture,
    PathType::Relative,
    "export_keyword/require_export_frozen_mutate",
  );
  repl_with_path_fixture_run_protected_require(&fixture, &path);
  fixture.assert_output_contains_all(&["true"]);
}

#[test]
fn require_by_string_require_export_function() {
  use ulua_cli_test::{
    enums::path_type::PathType,
    methods::repl_with_path_fixture_run_protected_require::repl_with_path_fixture_run_protected_require,
    records::repl_with_path_fixture::ReplWithPathFixture,
  };

  // 对齐 cpp 的 ScopedFastFlag{LuauExportValueSyntax, true}
  let _sff = sff_export_value();

  let mut fixture = ReplWithPathFixture::new();
  let path = without_config_dir(
    &fixture,
    PathType::Relative,
    "export_keyword/require_export_function",
  );
  repl_with_path_fixture_run_protected_require(&fixture, &path);
  fixture.assert_output_contains_all(&["true"]);
}

#[test]
fn require_by_string_require_export_function_rebind() {
  use ulua_cli_test::{
    enums::path_type::PathType,
    methods::repl_with_path_fixture_run_protected_require::repl_with_path_fixture_run_protected_require,
    records::repl_with_path_fixture::ReplWithPathFixture,
  };

  // 对齐 cpp 的 ScopedFastFlag{LuauExportValueSyntax, true}
  let _sff = sff_export_value();

  let mut fixture = ReplWithPathFixture::new();
  let path = without_config_dir(
    &fixture,
    PathType::Relative,
    "export_keyword/require_export_function_rebind",
  );
  repl_with_path_fixture_run_protected_require(&fixture, &path);
  fixture.assert_output_contains_all(&["true"]);
}

#[test]
fn require_by_string_require_export_in_do_block_error() {
  use ulua_cli_test::{
    enums::path_type::PathType,
    methods::repl_with_path_fixture_run_protected_require::repl_with_path_fixture_run_protected_require,
    records::repl_with_path_fixture::ReplWithPathFixture,
  };

  // 对齐 cpp 的 ScopedFastFlag{LuauExportValueSyntax, true}
  let _sff = sff_export_value();

  let mut fixture = ReplWithPathFixture::new();
  let path = without_config_dir(
    &fixture,
    PathType::Relative,
    "export_keyword/export_in_do_block_error",
  );
  repl_with_path_fixture_run_protected_require(&fixture, &path);
  fixture.assert_output_contains_all(&["'export' may only be applied to top-level statements"]);
}

#[test]
fn require_by_string_require_export_in_else_if_error() {
  use ulua_cli_test::{
    enums::path_type::PathType,
    methods::repl_with_path_fixture_run_protected_require::repl_with_path_fixture_run_protected_require,
    records::repl_with_path_fixture::ReplWithPathFixture,
  };

  // 对齐 cpp 的 ScopedFastFlag{LuauExportValueSyntax, true}
  let _sff = sff_export_value();

  let mut fixture = ReplWithPathFixture::new();
  let path = without_config_dir(
    &fixture,
    PathType::Relative,
    "export_keyword/export_in_elseif_error",
  );
  repl_with_path_fixture_run_protected_require(&fixture, &path);
  fixture.assert_output_contains_all(&["'export' may only be applied to top-level statements"]);
}

#[test]
fn require_by_string_require_export_in_for_error() {
  use ulua_cli_test::{
    enums::path_type::PathType,
    methods::repl_with_path_fixture_run_protected_require::repl_with_path_fixture_run_protected_require,
    records::repl_with_path_fixture::ReplWithPathFixture,
  };

  // 对齐 cpp 的 ScopedFastFlag{LuauExportValueSyntax, true}
  let _sff = sff_export_value();

  let mut fixture = ReplWithPathFixture::new();
  let path = without_config_dir(
    &fixture,
    PathType::Relative,
    "export_keyword/export_in_for_error",
  );
  repl_with_path_fixture_run_protected_require(&fixture, &path);
  fixture.assert_output_contains_all(&["'export' may only be applied to top-level statements"]);
}

#[test]
fn require_by_string_require_export_in_function_error() {
  use ulua_cli_test::{
    enums::path_type::PathType,
    methods::repl_with_path_fixture_run_protected_require::repl_with_path_fixture_run_protected_require,
    records::repl_with_path_fixture::ReplWithPathFixture,
  };

  // 对齐 cpp 的 ScopedFastFlag{LuauExportValueSyntax, true}
  let _sff = sff_export_value();

  let mut fixture = ReplWithPathFixture::new();
  let path = without_config_dir(
    &fixture,
    PathType::Relative,
    "export_keyword/export_in_function_error",
  );
  repl_with_path_fixture_run_protected_require(&fixture, &path);
  fixture.assert_output_contains_all(&["'export' may only be applied to top-level statements"]);
}

#[test]
fn require_by_string_require_export_in_if_error() {
  use ulua_cli_test::{
    enums::path_type::PathType,
    methods::repl_with_path_fixture_run_protected_require::repl_with_path_fixture_run_protected_require,
    records::repl_with_path_fixture::ReplWithPathFixture,
  };

  // 对齐 cpp 的 ScopedFastFlag{LuauExportValueSyntax, true}
  let _sff = sff_export_value();

  let mut fixture = ReplWithPathFixture::new();
  let path = without_config_dir(
    &fixture,
    PathType::Relative,
    "export_keyword/export_in_if_error",
  );
  repl_with_path_fixture_run_protected_require(&fixture, &path);
  fixture.assert_output_contains_all(&["'export' may only be applied to top-level statements"]);
}

#[test]
fn require_by_string_require_export_in_repeat_error() {
  use ulua_cli_test::{
    enums::path_type::PathType,
    methods::repl_with_path_fixture_run_protected_require::repl_with_path_fixture_run_protected_require,
    records::repl_with_path_fixture::ReplWithPathFixture,
  };

  // 对齐 cpp 的 ScopedFastFlag{LuauExportValueSyntax, true}
  let _sff = sff_export_value();

  let mut fixture = ReplWithPathFixture::new();
  let path = without_config_dir(
    &fixture,
    PathType::Relative,
    "export_keyword/export_in_repeat_error",
  );
  repl_with_path_fixture_run_protected_require(&fixture, &path);
  fixture.assert_output_contains_all(&["'export' may only be applied to top-level statements"]);
}

#[test]
fn require_by_string_require_export_in_while_error() {
  use ulua_cli_test::{
    enums::path_type::PathType,
    methods::repl_with_path_fixture_run_protected_require::repl_with_path_fixture_run_protected_require,
    records::repl_with_path_fixture::ReplWithPathFixture,
  };

  // 对齐 cpp 的 ScopedFastFlag{LuauExportValueSyntax, true}
  let _sff = sff_export_value();

  let mut fixture = ReplWithPathFixture::new();
  let path = without_config_dir(
    &fixture,
    PathType::Relative,
    "export_keyword/export_in_while_error",
  );
  repl_with_path_fixture_run_protected_require(&fixture, &path);
  fixture.assert_output_contains_all(&["'export' may only be applied to top-level statements"]);
}

#[test]
fn require_by_string_require_export_internal_call() {
  use ulua_cli_test::{
    enums::path_type::PathType,
    methods::repl_with_path_fixture_run_protected_require::repl_with_path_fixture_run_protected_require,
    records::repl_with_path_fixture::ReplWithPathFixture,
  };

  // 对齐 cpp 的 ScopedFastFlag{LuauExportValueSyntax, true}
  let _sff = sff_export_value();

  let mut fixture = ReplWithPathFixture::new();
  let path = without_config_dir(
    &fixture,
    PathType::Relative,
    "export_keyword/require_export_internal_call",
  );
  repl_with_path_fixture_run_protected_require(&fixture, &path);
  fixture.assert_output_contains_all(&["true"]);
}

#[test]
fn require_by_string_require_export_mixed() {
  use ulua_cli_test::{
    enums::path_type::PathType,
    methods::repl_with_path_fixture_run_protected_require::repl_with_path_fixture_run_protected_require,
    records::repl_with_path_fixture::ReplWithPathFixture,
  };

  // 对齐 cpp 的 ScopedFastFlag{LuauExportValueSyntax, true}
  let _sff = sff_export_value();

  let mut fixture = ReplWithPathFixture::new();
  let path = without_config_dir(
    &fixture,
    PathType::Relative,
    "export_keyword/require_export_mixed",
  );
  repl_with_path_fixture_run_protected_require(&fixture, &path);
  fixture.assert_output_contains_all(&["true"]);
}

#[test]
fn require_by_string_require_export_multi_assign() {
  use ulua_cli_test::{
    enums::path_type::PathType,
    methods::repl_with_path_fixture_run_protected_require::repl_with_path_fixture_run_protected_require,
    records::repl_with_path_fixture::ReplWithPathFixture,
  };

  // 对齐 cpp 的 ScopedFastFlag{LuauExportValueSyntax, true}
  let _sff = sff_export_value();

  let mut fixture = ReplWithPathFixture::new();
  let path = without_config_dir(
    &fixture,
    PathType::Relative,
    "export_keyword/require_export_multi_assign",
  );
  repl_with_path_fixture_run_protected_require(&fixture, &path);
  fixture.assert_output_contains_all(&["true"]);
}

#[test]
fn require_by_string_require_export_multi_swap() {
  use ulua_cli_test::{
    enums::path_type::PathType,
    methods::repl_with_path_fixture_run_protected_require::repl_with_path_fixture_run_protected_require,
    records::repl_with_path_fixture::ReplWithPathFixture,
  };

  // 对齐 cpp 的 ScopedFastFlag{LuauExportValueSyntax, true}
  let _sff = sff_export_value();

  let mut fixture = ReplWithPathFixture::new();
  let path = without_config_dir(
    &fixture,
    PathType::Relative,
    "export_keyword/require_export_multi_swap",
  );
  repl_with_path_fixture_run_protected_require(&fixture, &path);
  fixture.assert_output_contains_all(&["true"]);
}

#[test]
fn require_by_string_require_export_multi_var() {
  use ulua_cli_test::{
    enums::path_type::PathType,
    methods::repl_with_path_fixture_run_protected_require::repl_with_path_fixture_run_protected_require,
    records::repl_with_path_fixture::ReplWithPathFixture,
  };

  // 对齐 cpp 的 ScopedFastFlag{LuauExportValueSyntax, true}
  let _sff = sff_export_value();

  let mut fixture = ReplWithPathFixture::new();
  let path = without_config_dir(
    &fixture,
    PathType::Relative,
    "export_keyword/require_export_multi_var",
  );
  repl_with_path_fixture_run_protected_require(&fixture, &path);
  fixture.assert_output_contains_all(&["true"]);
}

#[test]
fn require_by_string_require_export_mutual_recursion() {
  use ulua_cli_test::{
    enums::path_type::PathType,
    methods::repl_with_path_fixture_run_protected_require::repl_with_path_fixture_run_protected_require,
    records::repl_with_path_fixture::ReplWithPathFixture,
  };

  // 对齐 cpp 的 ScopedFastFlag{LuauExportValueSyntax, true}
  let _sff = sff_export_value();

  let mut fixture = ReplWithPathFixture::new();
  let path = without_config_dir(
    &fixture,
    PathType::Relative,
    "export_keyword/require_export_mutual_recursion",
  );
  repl_with_path_fixture_run_protected_require(&fixture, &path);
  fixture.assert_output_contains_all(&["true"]);
}

#[test]
fn require_by_string_require_export_nested_table() {
  use ulua_cli_test::{
    enums::path_type::PathType,
    methods::repl_with_path_fixture_run_protected_require::repl_with_path_fixture_run_protected_require,
    records::repl_with_path_fixture::ReplWithPathFixture,
  };

  // 对齐 cpp 的 ScopedFastFlag{LuauExportValueSyntax, true}
  let _sff = sff_export_value();

  let mut fixture = ReplWithPathFixture::new();
  let path = without_config_dir(
    &fixture,
    PathType::Relative,
    "export_keyword/require_export_nested_table",
  );
  repl_with_path_fixture_run_protected_require(&fixture, &path);
  fixture.assert_output_contains_all(&["true"]);
}

#[test]
fn require_by_string_require_export_shadowing() {
  use ulua_cli_test::{
    enums::path_type::PathType,
    methods::repl_with_path_fixture_run_protected_require::repl_with_path_fixture_run_protected_require,
    records::repl_with_path_fixture::ReplWithPathFixture,
  };

  // 对齐 cpp 的 ScopedFastFlag{LuauExportValueSyntax, true}
  let _sff = sff_export_value();

  let mut fixture = ReplWithPathFixture::new();
  let path = without_config_dir(
    &fixture,
    PathType::Relative,
    "export_keyword/require_export_shadowing",
  );
  repl_with_path_fixture_run_protected_require(&fixture, &path);
  fixture.assert_output_contains_all(&["true"]);
}

#[test]
fn require_by_string_require_export_trap() {
  use ulua_cli_test::{
    enums::path_type::PathType,
    methods::repl_with_path_fixture_run_protected_require::repl_with_path_fixture_run_protected_require,
    records::repl_with_path_fixture::ReplWithPathFixture,
  };

  // 对齐 cpp 的 ScopedFastFlag{LuauExportValueSyntax, true}
  let _sff = sff_export_value();

  let mut fixture = ReplWithPathFixture::new();
  let path = without_config_dir(
    &fixture,
    PathType::Relative,
    "export_keyword/require_export_trap",
  );
  repl_with_path_fixture_run_protected_require(&fixture, &path);
  fixture.assert_output_contains_all(&["true"]);
}

#[test]
fn require_by_string_require_export_type_with_return() {
  use ulua_cli_test::{
    enums::path_type::PathType,
    methods::repl_with_path_fixture_run_protected_require::repl_with_path_fixture_run_protected_require,
    records::repl_with_path_fixture::ReplWithPathFixture,
  };

  // 对齐 cpp 的 ScopedFastFlag{LuauExportValueSyntax, true}
  let _sff = sff_export_value();

  let mut fixture = ReplWithPathFixture::new();
  let path = without_config_dir(
    &fixture,
    PathType::Relative,
    "export_keyword/require_export_type_with_return",
  );
  repl_with_path_fixture_run_protected_require(&fixture, &path);
  fixture.assert_output_contains_all(&["true"]);
}

#[test]
fn require_by_string_require_export_upvalue() {
  use ulua_cli_test::{
    enums::path_type::PathType,
    methods::repl_with_path_fixture_run_protected_require::repl_with_path_fixture_run_protected_require,
    records::repl_with_path_fixture::ReplWithPathFixture,
  };

  // 对齐 cpp 的 ScopedFastFlag{LuauExportValueSyntax, true}
  let _sff = sff_export_value();

  let mut fixture = ReplWithPathFixture::new();
  let path = without_config_dir(
    &fixture,
    PathType::Relative,
    "export_keyword/require_export_upvalue",
  );
  repl_with_path_fixture_run_protected_require(&fixture, &path);
  fixture.assert_output_contains_all(&["true"]);
}

#[test]
fn require_by_string_require_export_value() {
  use ulua_cli_test::{
    enums::path_type::PathType,
    methods::repl_with_path_fixture_run_protected_require::repl_with_path_fixture_run_protected_require,
    records::repl_with_path_fixture::ReplWithPathFixture,
  };

  // 对齐 cpp 的 ScopedFastFlag{LuauExportValueSyntax, true}
  let _sff = sff_export_value();

  let mut fixture = ReplWithPathFixture::new();
  let path = without_config_dir(
    &fixture,
    PathType::Relative,
    "export_keyword/require_export_value",
  );
  repl_with_path_fixture_run_protected_require(&fixture, &path);
  fixture.assert_output_contains_all(&["true"]);
}

#[test]
fn require_by_string_require_export_with_return_error() {
  use ulua_cli_test::{
    enums::path_type::PathType,
    methods::repl_with_path_fixture_run_protected_require::repl_with_path_fixture_run_protected_require,
    records::repl_with_path_fixture::ReplWithPathFixture,
  };

  // 对齐 cpp 的 ScopedFastFlag{LuauExportValueSyntax, true}
  let _sff = sff_export_value();

  let mut fixture = ReplWithPathFixture::new();
  let path = without_config_dir(
    &fixture,
    PathType::Relative,
    "export_keyword/export_with_return_error",
  );
  repl_with_path_fixture_run_protected_require(&fixture, &path);
  fixture.assert_output_contains_all(&["Exporting values is not compatible with top-level return"]);
}

#[test]
fn require_by_string_require_from_luau_binary() {
  use alloc::{string::String, vec};

  use ulua_cli_test::{
    enums::path_type::PathType, records::repl_with_path_fixture::ReplWithPathFixture,
  };
  use ulua_repl_cli::functions::repl_main::repl_main;

  let fixture = ReplWithPathFixture::new();

  let rel = PathType::Relative;
  let abs = PathType::Absolute;

  let paths: vec::Vec<String> = vec![
    without_config_dir(&fixture, rel, "dependency.luau"),
    without_config_dir(&fixture, abs, "dependency.luau"),
    without_config_dir(&fixture, rel, "module.luau"),
    without_config_dir(&fixture, abs, "module.luau"),
    without_config_dir(&fixture, rel, "nested/init.luau"),
    without_config_dir(&fixture, abs, "nested/init.luau"),
    config_tests_dir(&fixture, rel, "with_config/src/submodule/init.luau"),
    config_tests_dir(&fixture, abs, "with_config/src/submodule/init.luau"),
    config_tests_dir(&fixture, rel, "with_config_luau/src/submodule/init.luau"),
    config_tests_dir(&fixture, abs, "with_config_luau/src/submodule/init.luau"),
  ];

  for path in &paths {
    assert_eq!(
      repl_main(&["luau", path.as_str()]),
      0,
      "replMain failed for {}",
      path
    );
  }
}

#[test]
fn require_by_string_require_function() {
  use ulua_cli_test::{
    enums::path_type::PathType,
    methods::repl_with_path_fixture_run_protected_require::repl_with_path_fixture_run_protected_require,
    records::repl_with_path_fixture::ReplWithPathFixture,
  };

  let mut fixture = ReplWithPathFixture::new();
  let path = without_config_dir(&fixture, PathType::Relative, "types/function");
  repl_with_path_fixture_run_protected_require(&fixture, &path);
  fixture.assert_output_contains_all(&["true", "function"]);
}

#[test]
fn require_by_string_require_init_lua() {
  use ulua_cli_test::{
    enums::path_type::PathType,
    methods::repl_with_path_fixture_run_protected_require::repl_with_path_fixture_run_protected_require,
    records::repl_with_path_fixture::ReplWithPathFixture,
  };

  let mut fixture = ReplWithPathFixture::new();
  let path = without_config_dir(&fixture, PathType::Relative, "lua");
  repl_with_path_fixture_run_protected_require(&fixture, &path);
  fixture.assert_output_contains_all(&["true", "result from init.lua"]);
}

#[test]
fn require_by_string_require_init_luau() {
  use ulua_cli_test::{
    enums::path_type::PathType,
    methods::repl_with_path_fixture_run_protected_require::repl_with_path_fixture_run_protected_require,
    records::repl_with_path_fixture::ReplWithPathFixture,
  };

  let mut fixture = ReplWithPathFixture::new();
  let path = without_config_dir(&fixture, PathType::Relative, "luau");
  repl_with_path_fixture_run_protected_require(&fixture, &path);
  fixture.assert_output_contains_all(&["true", "result from init.luau"]);
}

#[test]
fn require_by_string_require_lua() {
  use ulua_cli_test::{
    enums::path_type::PathType,
    methods::repl_with_path_fixture_run_protected_require::repl_with_path_fixture_run_protected_require,
    records::repl_with_path_fixture::ReplWithPathFixture,
  };

  let mut fixture = ReplWithPathFixture::new();
  let path = without_config_dir(&fixture, PathType::Relative, "lua_dependency");
  repl_with_path_fixture_run_protected_require(&fixture, &path);
  fixture.assert_output_contains_all(&["true", "result from lua_dependency"]);
}

#[test]
fn require_by_string_require_nested_inits() {
  use ulua_cli_test::{
    enums::path_type::PathType,
    methods::repl_with_path_fixture_run_protected_require::repl_with_path_fixture_run_protected_require,
    records::repl_with_path_fixture::ReplWithPathFixture,
  };

  let mut fixture = ReplWithPathFixture::new();
  let path = without_config_dir(&fixture, PathType::Relative, "nested_inits_requirer");
  repl_with_path_fixture_run_protected_require(&fixture, &path);
  fixture.assert_output_contains_all(&[
    "true",
    "result from nested_inits/init",
    "required into module",
  ]);
}

#[test]
fn require_by_string_require_nil() {
  use ulua_cli_test::{
    enums::path_type::PathType,
    methods::repl_with_path_fixture_run_protected_require::repl_with_path_fixture_run_protected_require,
    records::repl_with_path_fixture::ReplWithPathFixture,
  };

  let mut fixture = ReplWithPathFixture::new();
  let path = without_config_dir(&fixture, PathType::Relative, "types/nil");
  repl_with_path_fixture_run_protected_require(&fixture, &path);
  fixture.assert_output_contains_all(&["true", "nil"]);
}

#[test]
fn require_by_string_require_number() {
  use ulua_cli_test::{
    enums::path_type::PathType,
    methods::repl_with_path_fixture_run_protected_require::repl_with_path_fixture_run_protected_require,
    records::repl_with_path_fixture::ReplWithPathFixture,
  };

  let mut fixture = ReplWithPathFixture::new();
  let path = without_config_dir(&fixture, PathType::Relative, "types/number");
  repl_with_path_fixture_run_protected_require(&fixture, &path);
  fixture.assert_output_contains_all(&["true", "12345"]);
}

#[test]
fn require_by_string_require_path_with_alias() {
  use ulua_cli_test::{
    enums::path_type::PathType,
    methods::repl_with_path_fixture_run_protected_require::repl_with_path_fixture_run_protected_require,
    records::repl_with_path_fixture::ReplWithPathFixture,
  };

  let mut fixture = ReplWithPathFixture::new();
  {
    let path = config_tests_dir(
      &fixture,
      PathType::Relative,
      "with_config/src/alias_requirer",
    );
    repl_with_path_fixture_run_protected_require(&fixture, &path);
    fixture.assert_output_contains_all(&["true", "result from dependency"]);
  }
  {
    let path = config_tests_dir(
      &fixture,
      PathType::Relative,
      "with_config_luau/src/alias_requirer",
    );
    repl_with_path_fixture_run_protected_require(&fixture, &path);
    fixture.assert_output_contains_all(&["true", "result from dependency"]);
  }
}

#[test]
fn require_by_string_require_path_with_alias_pointing_to_directory() {
  use ulua_cli_test::{
    enums::path_type::PathType,
    methods::repl_with_path_fixture_run_protected_require::repl_with_path_fixture_run_protected_require,
    records::repl_with_path_fixture::ReplWithPathFixture,
  };

  let mut fixture = ReplWithPathFixture::new();
  {
    let path = config_tests_dir(
      &fixture,
      PathType::Relative,
      "with_config/src/directory_alias_requirer",
    );
    repl_with_path_fixture_run_protected_require(&fixture, &path);
    fixture.assert_output_contains_all(&["true", "result from subdirectory_dependency"]);
  }
  {
    let path = config_tests_dir(
      &fixture,
      PathType::Relative,
      "with_config_luau/src/directory_alias_requirer",
    );
    repl_with_path_fixture_run_protected_require(&fixture, &path);
    fixture.assert_output_contains_all(&["true", "result from subdirectory_dependency"]);
  }
}

#[test]
fn require_by_string_require_path_with_parent_alias() {
  use ulua_cli_test::{
    enums::path_type::PathType,
    methods::repl_with_path_fixture_run_protected_require::repl_with_path_fixture_run_protected_require,
    records::repl_with_path_fixture::ReplWithPathFixture,
  };

  let mut fixture = ReplWithPathFixture::new();
  {
    let path = config_tests_dir(
      &fixture,
      PathType::Relative,
      "with_config/src/parent_alias_requirer",
    );
    repl_with_path_fixture_run_protected_require(&fixture, &path);
    fixture.assert_output_contains_all(&["true", "result from other_dependency"]);
  }
  {
    let path = config_tests_dir(
      &fixture,
      PathType::Relative,
      "with_config_luau/src/parent_alias_requirer",
    );
    repl_with_path_fixture_run_protected_require(&fixture, &path);
    fixture.assert_output_contains_all(&["true", "result from other_dependency"]);
  }
}

#[test]
fn require_by_string_require_relative_to_requiring_file() {
  use ulua_cli_test::{
    enums::path_type::PathType,
    methods::repl_with_path_fixture_run_protected_require::repl_with_path_fixture_run_protected_require,
    records::repl_with_path_fixture::ReplWithPathFixture,
  };

  let mut fixture = ReplWithPathFixture::new();
  let path = without_config_dir(&fixture, PathType::Relative, "module");
  repl_with_path_fixture_run_protected_require(&fixture, &path);
  fixture.assert_output_contains_all(&["true", "result from dependency", "required into module"]);
}

#[test]
fn require_by_string_require_simple_relative_path() {
  use ulua_cli_test::{
    enums::path_type::PathType,
    methods::repl_with_path_fixture_run_protected_require::repl_with_path_fixture_run_protected_require,
    records::repl_with_path_fixture::ReplWithPathFixture,
  };

  let mut fixture = ReplWithPathFixture::new();
  let path = without_config_dir(&fixture, PathType::Relative, "dependency");
  repl_with_path_fixture_run_protected_require(&fixture, &path);
  fixture.assert_output_contains_all(&["true", "result from dependency"]);
}

#[test]
fn require_by_string_require_simple_relative_path_within_pcall() {
  use ulua_cli_test::{
    enums::path_type::PathType, records::repl_with_path_fixture::ReplWithPathFixture,
  };

  let mut fixture = ReplWithPathFixture::new();
  let path = without_config_dir(&fixture, PathType::Relative, "dependency");
  let code: String = format!("return pcall(require, \"{}\")", path);
  run_code_str(fixture.l, &code);
  fixture.assert_output_contains_all(&["true", "result from dependency"]);
}

#[test]
fn require_by_string_require_string() {
  use ulua_cli_test::{
    enums::path_type::PathType,
    methods::repl_with_path_fixture_run_protected_require::repl_with_path_fixture_run_protected_require,
    records::repl_with_path_fixture::ReplWithPathFixture,
  };

  let mut fixture = ReplWithPathFixture::new();
  let path = without_config_dir(&fixture, PathType::Relative, "types/string");
  repl_with_path_fixture_run_protected_require(&fixture, &path);
  fixture.assert_output_contains_all(&["true", "\"foo\""]);
}

#[test]
fn require_by_string_require_submodule_using_self_directly() {
  use ulua_cli_test::{
    enums::path_type::PathType,
    methods::repl_with_path_fixture_run_protected_require::repl_with_path_fixture_run_protected_require,
    records::repl_with_path_fixture::ReplWithPathFixture,
  };

  let mut fixture = ReplWithPathFixture::new();
  let path = without_config_dir(&fixture, PathType::Relative, "nested");
  repl_with_path_fixture_run_protected_require(&fixture, &path);
  fixture.assert_output_contains_all(&["true", "result from submodule"]);
}

#[test]
fn require_by_string_require_submodule_using_self_indirectly() {
  use ulua_cli_test::{
    enums::path_type::PathType,
    methods::repl_with_path_fixture_run_protected_require::repl_with_path_fixture_run_protected_require,
    records::repl_with_path_fixture::ReplWithPathFixture,
  };

  let mut fixture = ReplWithPathFixture::new();
  let path = without_config_dir(&fixture, PathType::Relative, "nested_module_requirer");
  repl_with_path_fixture_run_protected_require(&fixture, &path);
  fixture.assert_output_contains_all(&["true", "result from submodule"]);
}

#[test]
fn require_by_string_require_table() {
  use ulua_cli_test::{
    enums::path_type::PathType,
    methods::repl_with_path_fixture_run_protected_require::repl_with_path_fixture_run_protected_require,
    records::repl_with_path_fixture::ReplWithPathFixture,
  };

  let mut fixture = ReplWithPathFixture::new();
  let path = without_config_dir(&fixture, PathType::Relative, "types/table");
  repl_with_path_fixture_run_protected_require(&fixture, &path);
  fixture.assert_output_contains_all(&["true", "{\"foo\", \"bar\"}"]);
}

#[test]
fn require_by_string_require_thread() {
  use ulua_cli_test::{
    enums::path_type::PathType,
    methods::repl_with_path_fixture_run_protected_require::repl_with_path_fixture_run_protected_require,
    records::repl_with_path_fixture::ReplWithPathFixture,
  };

  let mut fixture = ReplWithPathFixture::new();
  let path = without_config_dir(&fixture, PathType::Relative, "types/thread");
  repl_with_path_fixture_run_protected_require(&fixture, &path);
  fixture.assert_output_contains_all(&["true", "thread"]);
}

#[test]
fn require_by_string_require_unprefixed_path() {
  use ulua_cli_test::{
    methods::repl_with_path_fixture_run_protected_require::repl_with_path_fixture_run_protected_require,
    records::repl_with_path_fixture::ReplWithPathFixture,
  };

  let mut fixture = ReplWithPathFixture::new();
  repl_with_path_fixture_run_protected_require(&fixture, "an/unprefixed/path");
  fixture.assert_output_contains_all(&[
    "false",
    "require path must start with a valid prefix: ./, ../, or @",
  ]);
}

#[test]
fn require_by_string_require_userdata() {
  use ulua_cli_test::{
    enums::path_type::PathType,
    methods::repl_with_path_fixture_run_protected_require::repl_with_path_fixture_run_protected_require,
    records::repl_with_path_fixture::ReplWithPathFixture,
  };

  let mut fixture = ReplWithPathFixture::new();
  let path = without_config_dir(&fixture, PathType::Relative, "types/userdata");
  repl_with_path_fixture_run_protected_require(&fixture, &path);
  fixture.assert_output_contains_all(&["true", "userdata"]);
}

#[test]
fn require_by_string_require_vector() {
  use ulua_cli_test::{
    enums::path_type::PathType,
    methods::repl_with_path_fixture_run_protected_require::repl_with_path_fixture_run_protected_require,
    records::repl_with_path_fixture::ReplWithPathFixture,
  };

  let mut fixture = ReplWithPathFixture::new();
  let path = without_config_dir(&fixture, PathType::Relative, "types/vector");
  repl_with_path_fixture_run_protected_require(&fixture, &path);
  fixture.assert_output_contains_all(&["true", "1, 2, 3"]);
}

#[test]
fn require_by_string_require_with_ambiguity_in_alias_discovery() {
  use alloc::{string::String, vec};

  use ulua_cli_test::{
    enums::path_type::PathType, records::repl_with_path_fixture::ReplWithPathFixture,
  };
  use ulua_repl_cli::functions::repl_main::repl_main;

  let fixture = ReplWithPathFixture::new();

  let paths: vec::Vec<String> = vec![
    config_tests_dir(
      &fixture,
      PathType::Relative,
      "with_config/parent_ambiguity/folder/requirer.luau",
    ),
    config_tests_dir(
      &fixture,
      PathType::Absolute,
      "with_config/parent_ambiguity/folder/requirer.luau",
    ),
    config_tests_dir(
      &fixture,
      PathType::Relative,
      "with_config_luau/parent_ambiguity/folder/requirer.luau",
    ),
    config_tests_dir(
      &fixture,
      PathType::Absolute,
      "with_config_luau/parent_ambiguity/folder/requirer.luau",
    ),
  ];

  for path in &paths {
    assert_eq!(
      repl_main(&["luau", path.as_str()]),
      0,
      "replMain failed for {}",
      path
    );
  }
}

#[test]
fn require_by_string_require_with_directory_ambiguity() {
  use ulua_cli_test::{
    enums::path_type::PathType,
    methods::repl_with_path_fixture_run_protected_require::repl_with_path_fixture_run_protected_require,
    records::repl_with_path_fixture::ReplWithPathFixture,
  };

  let mut fixture = ReplWithPathFixture::new();
  let path = without_config_dir(&fixture, PathType::Relative, "ambiguous_directory_requirer");
  repl_with_path_fixture_run_protected_require(&fixture, &path);
  fixture.assert_output_contains_all(&[
        "false",
        "error requiring module \"./ambiguous/directory/dependency\": could not resolve child component \"dependency\" (ambiguous)",
    ]);
}

#[test]
fn require_by_string_require_with_file_ambiguity() {
  use ulua_cli_test::{
    enums::path_type::PathType,
    methods::repl_with_path_fixture_run_protected_require::repl_with_path_fixture_run_protected_require,
    records::repl_with_path_fixture::ReplWithPathFixture,
  };

  let mut fixture = ReplWithPathFixture::new();
  let path = without_config_dir(&fixture, PathType::Relative, "ambiguous_file_requirer");
  repl_with_path_fixture_run_protected_require(&fixture, &path);
  fixture.assert_output_contains_all(&[
        "false",
        "error requiring module \"./ambiguous/file/dependency\": could not resolve child component \"dependency\" (ambiguous)",
    ]);
}

// KNOWN-GAP：以下 5 例对齐 cpp `RequireByString.test.cpp:654,962-995`，夹具
// （fixtures/tests/require/without_config/cyclic_*、config_tests/*/nested_override）
// 已在库中。阻塞部件尚未移植：FFlag `LuauCyclicRequireShortCircuit`（cyclic 系列）、
// DFFlag `LuauSelfIsSelfAndAlwaysSelf`（override 例）、VM `lua_usesexport` 与
// Require `createPlaceholder`（见 repl-cli load.rs 的同名缺口注释）。
// 补齐链路后去掉 #[ignore] 即可启用。

#[test]
#[ignore = "KNOWN-GAP: LuauCyclicRequireShortCircuit + lua_usesexport + createPlaceholder not ported"]
fn require_by_string_require_cyclic_path() {
  use ulua_cli_test::{
    enums::path_type::PathType,
    methods::repl_with_path_fixture_run_protected_require::repl_with_path_fixture_run_protected_require,
    records::repl_with_path_fixture::ReplWithPathFixture,
  };

  let _sff = sff_export_value();
  let mut fixture = ReplWithPathFixture::new();
  // cpp 同时置 LuauCyclicRequireShortCircuit=true；该 flag 未移植。
  // Both modules use the export keyword. The compiler uses the runtime-provided
  // placeholder as the export table, so the cycle resolves automatically.
  let path = without_config_dir(&fixture, PathType::Relative, "cyclic_requirer");
  repl_with_path_fixture_run_protected_require(&fixture, &path);
  fixture.assert_output_contains_all(&["true"]);
}

#[test]
#[ignore = "KNOWN-GAP: LuauCyclicRequireShortCircuit + lua_usesexport + createPlaceholder not ported"]
fn require_by_string_require_cyclic_dependency_error_on_access() {
  use ulua_cli_test::{
    enums::path_type::PathType,
    methods::repl_with_path_fixture_run_protected_require::repl_with_path_fixture_run_protected_require,
    records::repl_with_path_fixture::ReplWithPathFixture,
  };

  let _sff = sff_export_value();
  let mut fixture = ReplWithPathFixture::new();
  let path = without_config_dir(&fixture, PathType::Relative, "cyclic_access_a");
  repl_with_path_fixture_run_protected_require(&fixture, &path);
  fixture.assert_output_contains_all(&["false", "Cannot access the exported field 'Tree'"]);
}

#[test]
#[ignore = "KNOWN-GAP: LuauCyclicRequireShortCircuit + lua_usesexport + createPlaceholder not ported"]
fn require_by_string_require_cyclic_dependency_error_on_mutation() {
  use ulua_cli_test::{
    enums::path_type::PathType,
    methods::repl_with_path_fixture_run_protected_require::repl_with_path_fixture_run_protected_require,
    records::repl_with_path_fixture::ReplWithPathFixture,
  };

  let _sff = sff_export_value();
  let mut fixture = ReplWithPathFixture::new();
  let path = without_config_dir(&fixture, PathType::Relative, "cyclic_mutation_b");
  repl_with_path_fixture_run_protected_require(&fixture, &path);
  fixture.assert_output_contains_all(&["false", "Cannot set the exported field 'foo'"]);
}

#[test]
#[ignore = "KNOWN-GAP: LuauCyclicRequireShortCircuit + lua_usesexport + createPlaceholder not ported"]
fn require_by_string_require_cyclic_dependency_error_on_non_string_key() {
  use ulua_cli_test::{
    enums::path_type::PathType,
    methods::repl_with_path_fixture_run_protected_require::repl_with_path_fixture_run_protected_require,
    records::repl_with_path_fixture::ReplWithPathFixture,
  };

  let _sff = sff_export_value();
  let mut fixture = ReplWithPathFixture::new();
  let path = without_config_dir(&fixture, PathType::Relative, "cyclic_access_nonstringkey_a");
  repl_with_path_fixture_run_protected_require(&fixture, &path);
  fixture.assert_output_contains_all(&["false", "Cannot access the exported field 'unknown'"]);
}

#[test]
#[ignore = "KNOWN-GAP: DFFlag LuauSelfIsSelfAndAlwaysSelf not ported"]
fn require_by_string_require_submodule_using_self_with_override_attempt() {
  use ulua_cli_test::{
    enums::path_type::PathType,
    methods::repl_with_path_fixture_run_protected_require::repl_with_path_fixture_run_protected_require,
    records::repl_with_path_fixture::ReplWithPathFixture,
  };

  let mut fixture = ReplWithPathFixture::new();
  {
    let path = config_tests_dir(&fixture, PathType::Relative, "with_config/nested_override");
    repl_with_path_fixture_run_protected_require(&fixture, &path);
    fixture.assert_output_contains_all(&["true", "result from submodule"]);
  }
  {
    let path = config_tests_dir(
      &fixture,
      PathType::Relative,
      "with_config_luau/nested_override",
    );
    repl_with_path_fixture_run_protected_require(&fixture, &path);
    fixture.assert_output_contains_all(&["true", "result from submodule"]);
  }
}
