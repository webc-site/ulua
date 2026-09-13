use core::{ffi::c_int, ptr::null};

use ulua_common::FFlag::{
  DebugLuauUserDefinedClasses, DebugLuauUserDefinedClassesRuntime, LuauExportValueSyntax,
};
extern crate alloc;
// Port of `cpp/tests/RequireByString.test.cpp`.
// RequireByString tests.

#[cfg(test)]
#[test]
fn require_by_string_alias_has_illegal_format() {
  use alloc::string::String;

  use ulua_cli_test::{
    methods::repl_with_path_fixture_run_protected_require::repl_with_path_fixture_run_protected_require,
    records::repl_with_path_fixture::ReplWithPathFixture,
  };

  let mut fixture = ReplWithPathFixture::new();
  let illegal_character = String::from("@@");
  repl_with_path_fixture_run_protected_require(&fixture, &illegal_character);
  fixture.assert_output_contains_all(&[
    String::from("false"),
    String::from("@@ is not a valid alias"),
  ]);
  let path_alias1 = String::from("@.");
  repl_with_path_fixture_run_protected_require(&fixture, &path_alias1);
  fixture.assert_output_contains_all(&[
    String::from("false"),
    String::from(". is not a valid alias"),
  ]);
  let path_alias2 = String::from("@..");
  repl_with_path_fixture_run_protected_require(&fixture, &path_alias2);
  fixture.assert_output_contains_all(&[
    String::from("false"),
    String::from(".. is not a valid alias"),
  ]);
  let empty_alias = String::from("@");
  repl_with_path_fixture_run_protected_require(&fixture, &empty_alias);
  fixture
    .assert_output_contains_all(&[String::from("false"), String::from(" is not a valid alias")]);
}

#[cfg(test)]
#[test]
fn require_by_string_alias_not_parsed_if_configs_ambiguous() {
  use alloc::string::String;

  use ulua_cli_test::{
    enums::path_type::PathType,
    methods::{
      repl_with_path_fixture_get_luau_directory::repl_with_path_fixture_get_luau_directory,
      repl_with_path_fixture_run_protected_require::repl_with_path_fixture_run_protected_require,
    },
    records::repl_with_path_fixture::ReplWithPathFixture,
  };

  let mut fixture = ReplWithPathFixture::new();
  let path = repl_with_path_fixture_get_luau_directory(&fixture, PathType::Relative)
    + "/tests/require/config_tests/config_ambiguity/requirer";
  repl_with_path_fixture_run_protected_require(&fixture, &path);
  fixture.assert_output_contains_all(&[
    String::from("false"),
    String::from("could not resolve alias \"dep\" (ambiguous configuration file)"),
  ]);
}

#[cfg(test)]
#[test]
fn require_by_string_cannot_require_config_luau() {
  use alloc::string::String;

  use ulua_cli_test::{
    enums::path_type::PathType,
    methods::{
      repl_with_path_fixture_get_luau_directory::repl_with_path_fixture_get_luau_directory,
      repl_with_path_fixture_run_protected_require::repl_with_path_fixture_run_protected_require,
    },
    records::repl_with_path_fixture::ReplWithPathFixture,
  };

  let mut fixture = ReplWithPathFixture::new();
  let path = repl_with_path_fixture_get_luau_directory(&fixture, PathType::Relative)
    + "/tests/require/config_tests/config_cannot_be_required/requirer";
  repl_with_path_fixture_run_protected_require(&fixture, &path);
  fixture.assert_output_contains_all(&[
    String::from("false"),
    String::from("could not resolve child component \".config\""),
  ]);
}

#[cfg(test)]
#[test]
fn require_by_string_cannot_require_init_luau_directly() {
  use alloc::string::String;

  use ulua_cli_test::{
    enums::path_type::PathType,
    methods::{
      repl_with_path_fixture_get_luau_directory::repl_with_path_fixture_get_luau_directory,
      repl_with_path_fixture_run_protected_require::repl_with_path_fixture_run_protected_require,
    },
    records::repl_with_path_fixture::ReplWithPathFixture,
  };

  let mut fixture = ReplWithPathFixture::new();
  let path = repl_with_path_fixture_get_luau_directory(&fixture, PathType::Relative)
    + "/tests/require/without_config/nested/init";
  repl_with_path_fixture_run_protected_require(&fixture, &path);
  fixture.assert_output_contains_all(&[
    String::from("false"),
    String::from("could not resolve child component \"init\""),
  ]);
}

#[cfg(test)]
#[test]
fn require_by_string_check_cache_after_require_init_lua() {
  use alloc::{ffi::CString, string::String};

  use ulua_cli_test::{
    enums::path_type::PathType,
    methods::{
      repl_with_path_fixture_get_luau_directory::repl_with_path_fixture_get_luau_directory,
      repl_with_path_fixture_run_protected_require::repl_with_path_fixture_run_protected_require,
    },
    records::repl_with_path_fixture::ReplWithPathFixture,
  };
  use ulua_vm::{
    enums::lua_type::LuaType,
    functions::{lua_getfield::lua_getfield, lua_l_findtable::lua_l_findtable, lua_type::lua_type},
    macros::lua_registryindex::LUA_REGISTRYINDEX,
    records::lua_state::lua_State,
  };

  let mut fixture = ReplWithPathFixture::new();
  let l = fixture.l as *mut lua_State;
  let relative_path = repl_with_path_fixture_get_luau_directory(&fixture, PathType::Relative)
    + "/tests/require/without_config/lua";
  let absolute_path = repl_with_path_fixture_get_luau_directory(&fixture, PathType::Absolute)
    + "/tests/require/without_config/lua";

  let cache_key = CString::new(format!("{}/init.lua", absolute_path)).unwrap();

  unsafe {
    lua_l_findtable(l, LUA_REGISTRYINDEX, c"_MODULES".as_ptr(), 1);
    lua_getfield(l, -1, cache_key.as_ptr());
    assert!(
      lua_type(l, -1) == LuaType::Nil as c_int,
      "Cache already contained module result"
    );
  }

  repl_with_path_fixture_run_protected_require(&fixture, &relative_path);

  fixture.assert_output_contains_all(&[String::from("true"), String::from("result from init.lua")]);

  unsafe {
    lua_l_findtable(l, LUA_REGISTRYINDEX, c"_MODULES".as_ptr(), 1);
    lua_getfield(l, -1, cache_key.as_ptr());
    assert!(
      lua_type(l, -1) != LuaType::Nil as c_int,
      "Cache did not contain module result"
    );
  }
}

#[cfg(test)]
#[test]
fn require_by_string_check_cache_after_require_init_luau() {
  use alloc::{ffi::CString, string::String};

  use ulua_cli_test::{
    enums::path_type::PathType,
    methods::{
      repl_with_path_fixture_get_luau_directory::repl_with_path_fixture_get_luau_directory,
      repl_with_path_fixture_run_protected_require::repl_with_path_fixture_run_protected_require,
    },
    records::repl_with_path_fixture::ReplWithPathFixture,
  };
  use ulua_vm::{
    enums::lua_type::LuaType,
    functions::{lua_getfield::lua_getfield, lua_l_findtable::lua_l_findtable, lua_type::lua_type},
    macros::lua_registryindex::LUA_REGISTRYINDEX,
    records::lua_state::lua_State,
  };

  let mut fixture = ReplWithPathFixture::new();
  let l = fixture.l as *mut lua_State;
  let relative_path = repl_with_path_fixture_get_luau_directory(&fixture, PathType::Relative)
    + "/tests/require/without_config/luau";
  let absolute_path = repl_with_path_fixture_get_luau_directory(&fixture, PathType::Absolute)
    + "/tests/require/without_config/luau";

  let cache_key = CString::new(format!("{}/init.luau", absolute_path)).unwrap();

  unsafe {
    lua_l_findtable(l, LUA_REGISTRYINDEX, c"_MODULES".as_ptr(), 1);
    lua_getfield(l, -1, cache_key.as_ptr());
    assert!(
      lua_type(l, -1) == LuaType::Nil as c_int,
      "Cache already contained module result"
    );
  }

  repl_with_path_fixture_run_protected_require(&fixture, &relative_path);

  fixture
    .assert_output_contains_all(&[String::from("true"), String::from("result from init.luau")]);

  unsafe {
    lua_l_findtable(l, LUA_REGISTRYINDEX, c"_MODULES".as_ptr(), 1);
    lua_getfield(l, -1, cache_key.as_ptr());
    assert!(
      lua_type(l, -1) != LuaType::Nil as c_int,
      "Cache did not contain module result"
    );
  }
}

#[cfg(test)]
#[test]
fn require_by_string_check_cache_after_require_lua() {
  use alloc::{ffi::CString, string::String};

  use ulua_cli_test::{
    enums::path_type::PathType,
    methods::{
      repl_with_path_fixture_get_luau_directory::repl_with_path_fixture_get_luau_directory,
      repl_with_path_fixture_run_protected_require::repl_with_path_fixture_run_protected_require,
    },
    records::repl_with_path_fixture::ReplWithPathFixture,
  };
  use ulua_vm::{
    enums::lua_type::LuaType,
    functions::{lua_getfield::lua_getfield, lua_l_findtable::lua_l_findtable, lua_type::lua_type},
    macros::lua_registryindex::LUA_REGISTRYINDEX,
    records::lua_state::lua_State,
  };

  let mut fixture = ReplWithPathFixture::new();
  let l = fixture.l as *mut lua_State;
  let relative_path = repl_with_path_fixture_get_luau_directory(&fixture, PathType::Relative)
    + "/tests/require/without_config/lua_dependency";
  let absolute_path = repl_with_path_fixture_get_luau_directory(&fixture, PathType::Absolute)
    + "/tests/require/without_config/lua_dependency";

  unsafe {
    lua_l_findtable(l, LUA_REGISTRYINDEX, c"_MODULES".as_ptr(), 1);
    let key = CString::new(format!("{}.luau", absolute_path)).unwrap();
    lua_getfield(l, -1, key.as_ptr());
    assert!(
      lua_type(l, -1) == LuaType::Nil as c_int,
      "Cache already contained module result"
    );
  }

  repl_with_path_fixture_run_protected_require(&fixture, &relative_path);

  fixture.assert_output_contains_all(&[
    String::from("true"),
    String::from("result from lua_dependency"),
  ]);

  unsafe {
    lua_l_findtable(l, LUA_REGISTRYINDEX, c"_MODULES".as_ptr(), 1);
    let key = CString::new(format!("{}.lua", absolute_path)).unwrap();
    lua_getfield(l, -1, key.as_ptr());
    assert!(
      lua_type(l, -1) != LuaType::Nil as c_int,
      "Cache did not contain module result"
    );
  }
}

#[cfg(test)]
#[test]
fn require_by_string_check_cache_after_require_luau() {
  use alloc::{ffi::CString, string::String};

  use ulua_cli_test::{
    enums::path_type::PathType,
    methods::{
      repl_with_path_fixture_get_luau_directory::repl_with_path_fixture_get_luau_directory,
      repl_with_path_fixture_run_protected_require::repl_with_path_fixture_run_protected_require,
    },
    records::repl_with_path_fixture::ReplWithPathFixture,
  };
  use ulua_vm::{
    enums::lua_type::LuaType,
    functions::{lua_getfield::lua_getfield, lua_l_findtable::lua_l_findtable, lua_type::lua_type},
    macros::lua_registryindex::LUA_REGISTRYINDEX,
    records::lua_state::lua_State,
  };

  let mut fixture = ReplWithPathFixture::new();
  let l = fixture.l as *mut lua_State;
  let relative_path = repl_with_path_fixture_get_luau_directory(&fixture, PathType::Relative)
    + "/tests/require/without_config/module";
  let absolute_path = repl_with_path_fixture_get_luau_directory(&fixture, PathType::Absolute)
    + "/tests/require/without_config/module";

  let cache_key = CString::new(format!("{}.luau", absolute_path)).unwrap();

  unsafe {
    lua_l_findtable(l, LUA_REGISTRYINDEX, c"_MODULES".as_ptr(), 1);
    lua_getfield(l, -1, cache_key.as_ptr());
    assert!(
      lua_type(l, -1) == LuaType::Nil as c_int,
      "Cache already contained module result"
    );
  }

  repl_with_path_fixture_run_protected_require(&fixture, &relative_path);

  fixture.assert_output_contains_all(&[
    String::from("true"),
    String::from("result from dependency"),
    String::from("required into module"),
  ]);

  unsafe {
    lua_l_findtable(l, LUA_REGISTRYINDEX, c"_MODULES".as_ptr(), 1);
    lua_getfield(l, -1, cache_key.as_ptr());
    assert!(
      lua_type(l, -1) != LuaType::Nil as c_int,
      "Cache did not contain module result"
    );
  }
}

#[cfg(test)]
#[test]
fn require_by_string_check_cached_result() {
  use alloc::string::String;

  use ulua_cli_test::{
    enums::path_type::PathType,
    methods::{
      repl_with_path_fixture_get_luau_directory::repl_with_path_fixture_get_luau_directory,
      repl_with_path_fixture_run_protected_require::repl_with_path_fixture_run_protected_require,
    },
    records::repl_with_path_fixture::ReplWithPathFixture,
  };

  let mut fixture = ReplWithPathFixture::new();
  let path = repl_with_path_fixture_get_luau_directory(&fixture, PathType::Relative)
    + "/tests/require/without_config/validate_cache";
  repl_with_path_fixture_run_protected_require(&fixture, &path);
  fixture.assert_output_contains_all(&[String::from("true")]);
}

#[cfg(test)]
#[test]
fn require_by_string_check_clear_cache() {
  use alloc::{ffi::CString, string::String};

  use ulua_cli_test::{
    enums::path_type::PathType,
    methods::{
      repl_with_path_fixture_get_luau_directory::repl_with_path_fixture_get_luau_directory,
      repl_with_path_fixture_run_protected_require::repl_with_path_fixture_run_protected_require,
    },
    records::repl_with_path_fixture::ReplWithPathFixture,
  };
  use ulua_require::functions::luarequire_clearcache::luarequire_clearcache;
  use ulua_vm::{
    enums::lua_type::LuaType,
    functions::{
      lua_call::lua_call, lua_getfield::lua_getfield, lua_l_findtable::lua_l_findtable,
      lua_pushcclosurek::lua_pushcclosurek, lua_type::lua_type,
    },
    macros::lua_registryindex::LUA_REGISTRYINDEX,
    records::lua_state::lua_State,
  };

  let mut fixture = ReplWithPathFixture::new();
  let l = fixture.l as *mut lua_State;
  let relative_path = repl_with_path_fixture_get_luau_directory(&fixture, PathType::Relative)
    + "/tests/require/without_config/module";
  let absolute_path = repl_with_path_fixture_get_luau_directory(&fixture, PathType::Absolute)
    + "/tests/require/without_config/module";
  let cache_key = CString::new(format!("{}.luau", absolute_path)).unwrap();

  unsafe {
    lua_l_findtable(l, LUA_REGISTRYINDEX, c"_MODULES".as_ptr(), 1);
    lua_getfield(l, -1, cache_key.as_ptr());
    assert!(
      lua_type(l, -1) == LuaType::Nil as c_int,
      "Cache already contained module result"
    );
  }

  repl_with_path_fixture_run_protected_require(&fixture, &relative_path);

  fixture.assert_output_contains_all(&[
    String::from("true"),
    String::from("result from dependency"),
    String::from("required into module"),
  ]);

  unsafe {
    lua_l_findtable(l, LUA_REGISTRYINDEX, c"_MODULES".as_ptr(), 1);
    lua_getfield(l, -1, cache_key.as_ptr());
    assert!(
      lua_type(l, -1) != LuaType::Nil as c_int,
      "Cache did not contain module result"
    );

    lua_pushcclosurek(l, Some(luarequire_clearcache), null(), 0, None);
    lua_call(l, 0, 0);

    lua_l_findtable(l, LUA_REGISTRYINDEX, c"_MODULES".as_ptr(), 1);
    lua_getfield(l, -1, cache_key.as_ptr());
    assert!(
      lua_type(l, -1) == LuaType::Nil as c_int,
      "Cache was not cleared"
    );
  }
}

#[cfg(test)]
#[test]
fn require_by_string_check_clear_cache_entry() {
  use alloc::{ffi::CString, string::String};

  use ulua_cli_test::{
    enums::path_type::PathType,
    methods::{
      repl_with_path_fixture_get_luau_directory::repl_with_path_fixture_get_luau_directory,
      repl_with_path_fixture_run_protected_require::repl_with_path_fixture_run_protected_require,
    },
    records::repl_with_path_fixture::ReplWithPathFixture,
  };
  use ulua_require::functions::luarequire_clearcacheentry::luarequire_clearcacheentry;
  use ulua_vm::{
    enums::lua_type::LuaType,
    functions::{
      lua_call::lua_call, lua_getfield::lua_getfield, lua_l_findtable::lua_l_findtable,
      lua_pushcclosurek::lua_pushcclosurek, lua_pushstring::lua_pushstring, lua_type::lua_type,
    },
    macros::lua_registryindex::LUA_REGISTRYINDEX,
    records::lua_state::lua_State,
  };

  let mut fixture = ReplWithPathFixture::new();
  let l = fixture.l as *mut lua_State;
  let relative_path = repl_with_path_fixture_get_luau_directory(&fixture, PathType::Relative)
    + "/tests/require/without_config/module";
  let absolute_path = repl_with_path_fixture_get_luau_directory(&fixture, PathType::Absolute)
    + "/tests/require/without_config/module";
  let cache_key_str = format!("{}.luau", absolute_path);
  let cache_key = CString::new(cache_key_str).unwrap();

  unsafe {
    lua_l_findtable(l, LUA_REGISTRYINDEX, c"_MODULES".as_ptr(), 1);
    lua_getfield(l, -1, cache_key.as_ptr());
    assert!(
      lua_type(l, -1) == LuaType::Nil as c_int,
      "Cache already contained module result"
    );
  }

  repl_with_path_fixture_run_protected_require(&fixture, &relative_path);

  fixture.assert_output_contains_all(&[
    String::from("true"),
    String::from("result from dependency"),
    String::from("required into module"),
  ]);

  unsafe {
    lua_l_findtable(l, LUA_REGISTRYINDEX, c"_MODULES".as_ptr(), 1);
    lua_getfield(l, -1, cache_key.as_ptr());
    assert!(
      lua_type(l, -1) != LuaType::Nil as c_int,
      "Cache did not contain module result"
    );

    lua_pushcclosurek(l, Some(luarequire_clearcacheentry), null(), 0, None);
    lua_pushstring(l, cache_key.as_ptr());
    lua_call(l, 1, 0);

    lua_l_findtable(l, LUA_REGISTRYINDEX, c"_MODULES".as_ptr(), 1);
    lua_getfield(l, -1, cache_key.as_ptr());
    assert!(
      lua_type(l, -1) == LuaType::Nil as c_int,
      "Cache was not cleared"
    );
  }
}

#[cfg(test)]
#[test]
fn require_by_string_export_as_function() {
  use alloc::string::String;

  use ulua_cli_test::{
    enums::path_type::PathType,
    methods::{
      repl_with_path_fixture_get_luau_directory::repl_with_path_fixture_get_luau_directory,
      repl_with_path_fixture_run_protected_require::repl_with_path_fixture_run_protected_require,
    },
    records::repl_with_path_fixture::ReplWithPathFixture,
  };

  // C++: ScopedFastFlag sffs[] = {{FFlag::LuauExportValueSyntax, true}, };
  LuauExportValueSyntax.push_test_override(true);
  struct Sff;
  impl Drop for Sff {
    fn drop(&mut self) {
      LuauExportValueSyntax.pop_test_override();
    }
  }
  let _sff = Sff;

  let mut fixture = ReplWithPathFixture::new();
  let path = repl_with_path_fixture_get_luau_directory(&fixture, PathType::Relative)
    + "/tests/require/without_config/export_keyword/export_as_function";
  repl_with_path_fixture_run_protected_require(&fixture, &path);
  fixture.assert_output_contains_all(&[String::from("true")]);
}

#[cfg(test)]
#[test]
fn require_by_string_export_counter() {
  use alloc::string::String;

  use ulua_cli_test::{
    enums::path_type::PathType,
    methods::{
      repl_with_path_fixture_get_luau_directory::repl_with_path_fixture_get_luau_directory,
      repl_with_path_fixture_run_protected_require::repl_with_path_fixture_run_protected_require,
    },
    records::repl_with_path_fixture::ReplWithPathFixture,
  };

  // C++: ScopedFastFlag sffs[] = {{FFlag::LuauExportValueSyntax, true}, };
  LuauExportValueSyntax.push_test_override(true);
  struct Sff;
  impl Drop for Sff {
    fn drop(&mut self) {
      LuauExportValueSyntax.pop_test_override();
    }
  }
  let _sff = Sff;

  let mut fixture = ReplWithPathFixture::new();
  let path = repl_with_path_fixture_get_luau_directory(&fixture, PathType::Relative)
    + "/tests/require/without_config/export_keyword/require_export_counter_module";
  repl_with_path_fixture_run_protected_require(&fixture, &path);
  fixture.assert_output_contains_all(&[String::from("true")]);
}

#[cfg(test)]
#[test]
fn require_by_string_export_post_return_mutation_error() {
  use alloc::string::String;

  use ulua_cli_test::{
    enums::path_type::PathType,
    methods::{
      repl_with_path_fixture_get_luau_directory::repl_with_path_fixture_get_luau_directory,
      repl_with_path_fixture_run_protected_require::repl_with_path_fixture_run_protected_require,
    },
    records::repl_with_path_fixture::ReplWithPathFixture,
  };

  // C++: ScopedFastFlag sffs[] = {{FFlag::LuauExportValueSyntax, true}, };
  LuauExportValueSyntax.push_test_override(true);
  struct Sff;
  impl Drop for Sff {
    fn drop(&mut self) {
      LuauExportValueSyntax.pop_test_override();
    }
  }
  let _sff = Sff;

  let mut fixture = ReplWithPathFixture::new();
  let path = repl_with_path_fixture_get_luau_directory(&fixture, PathType::Relative)
    + "/tests/require/without_config/export_keyword/require_export_post_return_mutation_error";
  repl_with_path_fixture_run_protected_require(&fixture, &path);
  fixture.assert_output_contains_all(&[String::from("true")]);
}

#[cfg(test)]
#[test]
fn require_by_string_load_string_relative() {
  use alloc::string::String;

  use ulua_cli_test::{
    functions::run_code::run_code, records::repl_with_path_fixture::ReplWithPathFixture,
  };
  use ulua_vm::records::lua_state::lua_State;

  let mut fixture = ReplWithPathFixture::new();
  let l = fixture.l as *mut lua_State;
  unsafe {
    run_code(
      l,
      &String::from(
        "return pcall(function() return loadstring(\"require('a/relative/path')\")() end)",
      ),
    );
  }
  fixture.assert_output_contains_all(&[
    String::from("false"),
    String::from("require is not supported in this context"),
  ]);
}

#[cfg(test)]
#[test]
fn require_by_string_parse_aliases() {
  use ulua_config::{
    functions::parse_config::parse_config,
    records::{
      alias_info::AliasInfo,
      config::Config,
      config_options::{AliasOptions, ConfigOptions},
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
  copy_assigned_config.config_assign(&config);
  check_contents(&copy_assigned_config);
}

#[cfg(test)]
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

#[cfg(test)]
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

#[cfg(test)]
#[test]
fn require_by_string_proxy_require() {
  use alloc::string::String;

  use ulua_cli_test::{
    enums::path_type::PathType,
    functions::{
      create_cli_require_context::create_cli_require_context,
      require_config_init::require_config_init,
    },
    methods::{
      repl_with_path_fixture_get_luau_directory::repl_with_path_fixture_get_luau_directory,
      repl_with_path_fixture_run_protected_require::repl_with_path_fixture_run_protected_require,
    },
    records::repl_with_path_fixture::ReplWithPathFixture,
  };
  use ulua_require::functions::luarequire_pushproxyrequire::luarequire_pushproxyrequire;
  use ulua_vm::{macros::lua_setglobal::lua_setglobal, records::lua_state::lua_State};

  let mut fixture = ReplWithPathFixture::new();
  let l = fixture.l as *mut lua_State;

  unsafe {
    let ctx = create_cli_require_context(l);
    luarequire_pushproxyrequire(l, Some(require_config_init), ctx);
    lua_setglobal(l, c"proxyrequire".as_ptr());
  }

  let path = repl_with_path_fixture_get_luau_directory(&fixture, PathType::Relative)
    + "/tests/require/without_config/proxy_requirer";
  repl_with_path_fixture_run_protected_require(&fixture, &path);
  fixture.assert_output_contains_all(&[
    String::from("true"),
    String::from("result from dependency"),
    String::from("required into proxy_requirer"),
  ]);
}

#[cfg(test)]
#[test]
fn require_by_string_register_runtime_module() {
  use alloc::string::String;

  use ulua_cli_test::{
    functions::run_code::run_code, records::repl_with_path_fixture::ReplWithPathFixture,
  };
  use ulua_require::functions::luarequire_registermodule::luarequire_registermodule;
  use ulua_vm::{
    functions::{
      lua_call::lua_call, lua_pushcclosurek::lua_pushcclosurek, lua_pushstring::lua_pushstring,
      lua_settable::lua_settable,
    },
    macros::lua_newtable::lua_newtable,
    records::lua_state::lua_State,
  };

  let mut fixture = ReplWithPathFixture::new();
  let l = fixture.l as *mut lua_State;

  unsafe {
    lua_pushcclosurek(l, Some(luarequire_registermodule), null(), 0, None);
    lua_pushstring(l, c"@test/helloworld".as_ptr());
    lua_newtable(l);
    lua_pushstring(l, c"hello".as_ptr());
    lua_pushstring(l, c"world".as_ptr());
    lua_settable(l, -3);
    lua_call(l, 2, 0);
  }

  unsafe {
    run_code(
      l,
      &String::from("return require('@test/helloworld').hello == 'world'"),
    );
  }
  fixture.assert_output_contains_all(&[String::from("true")]);
}

#[cfg(test)]
#[test]
fn require_by_string_register_runtime_module_case_insensitive() {
  use alloc::string::String;

  use ulua_cli_test::{
    functions::run_code::run_code, records::repl_with_path_fixture::ReplWithPathFixture,
  };
  use ulua_require::functions::luarequire_registermodule::luarequire_registermodule;
  use ulua_vm::{
    functions::{
      lua_call::lua_call, lua_pushcclosurek::lua_pushcclosurek, lua_pushstring::lua_pushstring,
      lua_settable::lua_settable,
    },
    macros::lua_newtable::lua_newtable,
    records::lua_state::lua_State,
  };

  let mut fixture = ReplWithPathFixture::new();
  let l = fixture.l as *mut lua_State;

  unsafe {
    lua_pushcclosurek(l, Some(luarequire_registermodule), null(), 0, None);
    lua_pushstring(l, c"@test/helloworld".as_ptr());
    lua_newtable(l);
    lua_pushstring(l, c"hello".as_ptr());
    lua_pushstring(l, c"world".as_ptr());
    lua_settable(l, -3);
    lua_call(l, 2, 0);
  }

  unsafe {
    run_code(
      l,
      &String::from("return require('@TeSt/heLLoWoRld').hello == 'world'"),
    );
  }
  fixture.assert_output_contains_all(&[String::from("true")]);
}

#[cfg(test)]
#[test]
fn require_by_string_require_absolute_path() {
  use alloc::string::String;

  use ulua_cli_test::{
    methods::repl_with_path_fixture_run_protected_require::repl_with_path_fixture_run_protected_require,
    records::repl_with_path_fixture::ReplWithPathFixture,
  };

  let mut fixture = ReplWithPathFixture::new();
  let absolute_path = String::from("/an/absolute/path");
  repl_with_path_fixture_run_protected_require(&fixture, &absolute_path);
  fixture.assert_output_contains_all(&[
    String::from("false"),
    String::from("require path must start with a valid prefix: ./, ../, or @"),
  ]);
}

#[cfg(test)]
#[test]
fn require_by_string_require_alias_that_does_not_exist() {
  use alloc::string::String;

  use ulua_cli_test::{
    methods::repl_with_path_fixture_run_protected_require::repl_with_path_fixture_run_protected_require,
    records::repl_with_path_fixture::ReplWithPathFixture,
  };

  let mut fixture = ReplWithPathFixture::new();
  let non_existent_alias = String::from("@this.alias.does.not.exist");
  repl_with_path_fixture_run_protected_require(&fixture, &non_existent_alias);
  fixture.assert_output_contains_all(&[
    String::from("false"),
    String::from("@this.alias.does.not.exist is not a valid alias"),
  ]);
}

#[cfg(test)]
#[test]
fn require_by_string_require_boolean() {
  use alloc::string::String;

  use ulua_cli_test::{
    enums::path_type::PathType,
    methods::{
      repl_with_path_fixture_get_luau_directory::repl_with_path_fixture_get_luau_directory,
      repl_with_path_fixture_run_protected_require::repl_with_path_fixture_run_protected_require,
    },
    records::repl_with_path_fixture::ReplWithPathFixture,
  };

  let mut fixture = ReplWithPathFixture::new();
  let path = repl_with_path_fixture_get_luau_directory(&fixture, PathType::Relative)
    + "/tests/require/without_config/types/boolean";
  repl_with_path_fixture_run_protected_require(&fixture, &path);
  fixture.assert_output_contains_all(&[String::from("true"), String::from("false")]);
}

#[cfg(test)]
#[test]
fn require_by_string_require_buffer() {
  use alloc::string::String;

  use ulua_cli_test::{
    enums::path_type::PathType,
    methods::{
      repl_with_path_fixture_get_luau_directory::repl_with_path_fixture_get_luau_directory,
      repl_with_path_fixture_run_protected_require::repl_with_path_fixture_run_protected_require,
    },
    records::repl_with_path_fixture::ReplWithPathFixture,
  };

  let mut fixture = ReplWithPathFixture::new();
  let path = repl_with_path_fixture_get_luau_directory(&fixture, PathType::Relative)
    + "/tests/require/without_config/types/buffer";
  repl_with_path_fixture_run_protected_require(&fixture, &path);
  fixture.assert_output_contains_all(&[String::from("true"), String::from("buffer")]);
}

#[cfg(test)]
#[test]
fn require_by_string_require_chained_aliases_failure_cyclic() {
  use alloc::string::String;

  use ulua_cli_test::{
    enums::path_type::PathType,
    methods::{
      repl_with_path_fixture_get_luau_directory::repl_with_path_fixture_get_luau_directory,
      repl_with_path_fixture_run_protected_require::repl_with_path_fixture_run_protected_require,
    },
    records::repl_with_path_fixture::ReplWithPathFixture,
  };

  let mut fixture = ReplWithPathFixture::new();
  {
    let path = repl_with_path_fixture_get_luau_directory(&fixture, PathType::Relative)
      + "/tests/require/config_tests/with_config/chained_aliases/subdirectory/failing_requirer_cyclic";
    repl_with_path_fixture_run_protected_require(&fixture, &path);
    fixture.assert_output_contains_all(&[
            String::from("false"),
            String::from("error requiring module \"@cyclicentry\": detected alias cycle (@cyclic1 -> @cyclic2 -> @cyclic3 -> @cyclic1)"),
        ]);
  }
  {
    let path = repl_with_path_fixture_get_luau_directory(&fixture, PathType::Relative)
      + "/tests/require/config_tests/with_config_luau/chained_aliases/subdirectory/failing_requirer_cyclic";
    repl_with_path_fixture_run_protected_require(&fixture, &path);
    fixture.assert_output_contains_all(&[
            String::from("false"),
            String::from("error requiring module \"@cyclicentry\": detected alias cycle (@cyclic1 -> @cyclic2 -> @cyclic3 -> @cyclic1)"),
        ]);
  }
}

#[cfg(test)]
#[test]
fn require_by_string_require_chained_aliases_failure_depend_on_inner_alias() {
  use alloc::string::String;

  use ulua_cli_test::{
    enums::path_type::PathType,
    methods::{
      repl_with_path_fixture_get_luau_directory::repl_with_path_fixture_get_luau_directory,
      repl_with_path_fixture_run_protected_require::repl_with_path_fixture_run_protected_require,
    },
    records::repl_with_path_fixture::ReplWithPathFixture,
  };

  let mut fixture = ReplWithPathFixture::new();
  {
    let path = repl_with_path_fixture_get_luau_directory(&fixture, PathType::Relative)
      + "/tests/require/config_tests/with_config/chained_aliases/subdirectory/failing_requirer_inner_dependency";
    repl_with_path_fixture_run_protected_require(&fixture, &path);
    fixture.assert_output_contains_all(&[
      String::from("false"),
      String::from(
        "error requiring module \"@dependoninner\": @passthroughinner is not a valid alias",
      ),
    ]);
  }
  {
    let path = repl_with_path_fixture_get_luau_directory(&fixture, PathType::Relative)
      + "/tests/require/config_tests/with_config_luau/chained_aliases/subdirectory/failing_requirer_inner_dependency";
    repl_with_path_fixture_run_protected_require(&fixture, &path);
    fixture.assert_output_contains_all(&[
      String::from("false"),
      String::from(
        "error requiring module \"@dependoninner\": @passthroughinner is not a valid alias",
      ),
    ]);
  }
}

#[cfg(test)]
#[test]
fn require_by_string_require_chained_aliases_failure_missing() {
  use alloc::string::String;

  use ulua_cli_test::{
    enums::path_type::PathType,
    methods::{
      repl_with_path_fixture_get_luau_directory::repl_with_path_fixture_get_luau_directory,
      repl_with_path_fixture_run_protected_require::repl_with_path_fixture_run_protected_require,
    },
    records::repl_with_path_fixture::ReplWithPathFixture,
  };

  let mut fixture = ReplWithPathFixture::new();
  {
    let path = repl_with_path_fixture_get_luau_directory(&fixture, PathType::Relative)
      + "/tests/require/config_tests/with_config/chained_aliases/subdirectory/failing_requirer_missing";
    repl_with_path_fixture_run_protected_require(&fixture, &path);
    fixture.assert_output_contains_all(&[
      String::from("false"),
      String::from("error requiring module \"@brokenchain\": @missing is not a valid alias"),
    ]);
  }
  {
    let path = repl_with_path_fixture_get_luau_directory(&fixture, PathType::Relative)
      + "/tests/require/config_tests/with_config_luau/chained_aliases/subdirectory/failing_requirer_missing";
    repl_with_path_fixture_run_protected_require(&fixture, &path);
    fixture.assert_output_contains_all(&[
      String::from("false"),
      String::from("error requiring module \"@brokenchain\": @missing is not a valid alias"),
    ]);
  }
}

#[cfg(test)]
#[test]
fn require_by_string_require_chained_aliases_success() {
  use alloc::string::String;

  use ulua_cli_test::{
    enums::path_type::PathType,
    methods::{
      repl_with_path_fixture_get_luau_directory::repl_with_path_fixture_get_luau_directory,
      repl_with_path_fixture_run_protected_require::repl_with_path_fixture_run_protected_require,
    },
    records::repl_with_path_fixture::ReplWithPathFixture,
  };

  let mut fixture = ReplWithPathFixture::new();
  {
    let path = repl_with_path_fixture_get_luau_directory(&fixture, PathType::Relative)
      + "/tests/require/config_tests/with_config/chained_aliases/subdirectory/successful_requirer";
    repl_with_path_fixture_run_protected_require(&fixture, &path);
    fixture.assert_output_contains_all(&[
      String::from("true"),
      String::from("result from inner_dependency"),
      String::from("result from outer_dependency"),
    ]);
  }
  {
    let path = repl_with_path_fixture_get_luau_directory(&fixture, PathType::Relative)
      + "/tests/require/config_tests/with_config_luau/chained_aliases/subdirectory/successful_requirer";
    repl_with_path_fixture_run_protected_require(&fixture, &path);
    fixture.assert_output_contains_all(&[
      String::from("true"),
      String::from("result from inner_dependency"),
      String::from("result from outer_dependency"),
    ]);
  }
}

#[cfg(test)]
#[test]
fn require_by_string_require_export_alias() {
  use alloc::string::String;

  use ulua_cli_test::{
    enums::path_type::PathType,
    methods::{
      repl_with_path_fixture_get_luau_directory::repl_with_path_fixture_get_luau_directory,
      repl_with_path_fixture_run_protected_require::repl_with_path_fixture_run_protected_require,
    },
    records::repl_with_path_fixture::ReplWithPathFixture,
  };

  // C++: ScopedFastFlag sffs[] = {{FFlag::LuauExportValueSyntax, true}, };
  LuauExportValueSyntax.push_test_override(true);
  struct Sff;
  impl Drop for Sff {
    fn drop(&mut self) {
      LuauExportValueSyntax.pop_test_override();
    }
  }
  let _sff = Sff;

  let mut fixture = ReplWithPathFixture::new();
  let path = repl_with_path_fixture_get_luau_directory(&fixture, PathType::Relative)
    + "/tests/require/without_config/export_keyword/require_export_alias";
  repl_with_path_fixture_run_protected_require(&fixture, &path);
  fixture.assert_output_contains_all(&[String::from("true")]);
}

#[cfg(test)]
#[test]
fn require_by_string_require_export_alias_2() {
  use alloc::string::String;

  use ulua_cli_test::{
    enums::path_type::PathType,
    methods::{
      repl_with_path_fixture_get_luau_directory::repl_with_path_fixture_get_luau_directory,
      repl_with_path_fixture_run_protected_require::repl_with_path_fixture_run_protected_require,
    },
    records::repl_with_path_fixture::ReplWithPathFixture,
  };

  // C++: ScopedFastFlag sffs[] = {{FFlag::LuauExportValueSyntax, true}, };
  LuauExportValueSyntax.push_test_override(true);
  struct Sff;
  impl Drop for Sff {
    fn drop(&mut self) {
      LuauExportValueSyntax.pop_test_override();
    }
  }
  let _sff = Sff;

  let mut fixture = ReplWithPathFixture::new();
  let path = repl_with_path_fixture_get_luau_directory(&fixture, PathType::Relative)
    + "/tests/require/without_config/export_keyword/require_export_alias2";
  repl_with_path_fixture_run_protected_require(&fixture, &path);
  fixture.assert_output_contains_all(&[String::from("true")]);
}

#[cfg(test)]
#[test]
fn require_by_string_require_export_class() {
  use alloc::string::String;

  use ulua_cli_test::{
    enums::path_type::PathType,
    methods::{
      repl_with_path_fixture_get_luau_directory::repl_with_path_fixture_get_luau_directory,
      repl_with_path_fixture_run_protected_require::repl_with_path_fixture_run_protected_require,
    },
    records::repl_with_path_fixture::ReplWithPathFixture,
  };

  // C++: ScopedFastFlag sffs[] = {{LuauExportValueSyntax, true},
  //   {DebugLuauUserDefinedClasses, true}, {DebugLuauUserDefinedClassesRuntime, true}};
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
  let _sff = Sff;

  // we create a new fixture so the new lua_State has the class library
  let mut fixture = ReplWithPathFixture::new();
  let path = repl_with_path_fixture_get_luau_directory(&fixture, PathType::Relative)
    + "/tests/require/without_config/export_keyword/require_export_class";
  repl_with_path_fixture_run_protected_require(&fixture, &path);
  fixture.assert_output_contains_all(&[String::from("true")]);
}

#[cfg(test)]
#[test]
fn require_by_string_require_export_compound() {
  use alloc::string::String;

  use ulua_cli_test::{
    enums::path_type::PathType,
    methods::{
      repl_with_path_fixture_get_luau_directory::repl_with_path_fixture_get_luau_directory,
      repl_with_path_fixture_run_protected_require::repl_with_path_fixture_run_protected_require,
    },
    records::repl_with_path_fixture::ReplWithPathFixture,
  };

  // C++: ScopedFastFlag sffs[] = {{FFlag::LuauExportValueSyntax, true}, };
  LuauExportValueSyntax.push_test_override(true);
  struct Sff;
  impl Drop for Sff {
    fn drop(&mut self) {
      LuauExportValueSyntax.pop_test_override();
    }
  }
  let _sff = Sff;

  let mut fixture = ReplWithPathFixture::new();
  let path = repl_with_path_fixture_get_luau_directory(&fixture, PathType::Relative)
    + "/tests/require/without_config/export_keyword/require_export_compound";
  repl_with_path_fixture_run_protected_require(&fixture, &path);
  fixture.assert_output_contains_all(&[String::from("true")]);
}

#[cfg(test)]
#[test]
fn require_by_string_require_export_const_error() {
  use alloc::string::String;

  use ulua_cli_test::{
    enums::path_type::PathType,
    methods::{
      repl_with_path_fixture_get_luau_directory::repl_with_path_fixture_get_luau_directory,
      repl_with_path_fixture_run_protected_require::repl_with_path_fixture_run_protected_require,
    },
    records::repl_with_path_fixture::ReplWithPathFixture,
  };

  // C++: ScopedFastFlag sffs[] = {{FFlag::LuauExportValueSyntax, true}, };
  LuauExportValueSyntax.push_test_override(true);
  struct Sff;
  impl Drop for Sff {
    fn drop(&mut self) {
      LuauExportValueSyntax.pop_test_override();
    }
  }
  let _sff = Sff;

  let mut fixture = ReplWithPathFixture::new();
  let path = repl_with_path_fixture_get_luau_directory(&fixture, PathType::Relative)
    + "/tests/require/without_config/export_keyword/export_const_error";
  repl_with_path_fixture_run_protected_require(&fixture, &path);
  fixture.assert_output_contains_all(&[String::from(
    "Variable 'foo' is constant and may not be reassigned",
  )]);
}

#[cfg(test)]
#[test]
fn require_by_string_require_export_edge_cases() {
  use alloc::string::String;

  use ulua_cli_test::{
    enums::path_type::PathType,
    methods::{
      repl_with_path_fixture_get_luau_directory::repl_with_path_fixture_get_luau_directory,
      repl_with_path_fixture_run_protected_require::repl_with_path_fixture_run_protected_require,
    },
    records::repl_with_path_fixture::ReplWithPathFixture,
  };

  // C++: ScopedFastFlag sffs[] = {{FFlag::LuauExportValueSyntax, true}, };
  LuauExportValueSyntax.push_test_override(true);
  struct Sff;
  impl Drop for Sff {
    fn drop(&mut self) {
      LuauExportValueSyntax.pop_test_override();
    }
  }
  let _sff = Sff;

  let mut fixture = ReplWithPathFixture::new();
  let path = repl_with_path_fixture_get_luau_directory(&fixture, PathType::Relative)
    + "/tests/require/without_config/export_keyword/require_export_edge_cases";
  repl_with_path_fixture_run_protected_require(&fixture, &path);
  fixture.assert_output_contains_all(&[String::from("true")]);
}

#[cfg(test)]
#[test]
fn require_by_string_require_export_forward_rebind() {
  use alloc::string::String;

  use ulua_cli_test::{
    enums::path_type::PathType,
    methods::{
      repl_with_path_fixture_get_luau_directory::repl_with_path_fixture_get_luau_directory,
      repl_with_path_fixture_run_protected_require::repl_with_path_fixture_run_protected_require,
    },
    records::repl_with_path_fixture::ReplWithPathFixture,
  };

  // C++: ScopedFastFlag sffs[] = {{FFlag::LuauExportValueSyntax, true}, };
  LuauExportValueSyntax.push_test_override(true);
  struct Sff;
  impl Drop for Sff {
    fn drop(&mut self) {
      LuauExportValueSyntax.pop_test_override();
    }
  }
  let _sff = Sff;

  let mut fixture = ReplWithPathFixture::new();
  let path = repl_with_path_fixture_get_luau_directory(&fixture, PathType::Relative)
    + "/tests/require/without_config/export_keyword/require_export_forward_rebind";
  repl_with_path_fixture_run_protected_require(&fixture, &path);
  fixture.assert_output_contains_all(&[String::from("true")]);
}

#[cfg(test)]
#[test]
fn require_by_string_require_export_freeze_local_nil_ignored() {
  use alloc::string::String;

  use ulua_cli_test::{
    enums::path_type::PathType,
    methods::{
      repl_with_path_fixture_get_luau_directory::repl_with_path_fixture_get_luau_directory,
      repl_with_path_fixture_run_protected_require::repl_with_path_fixture_run_protected_require,
    },
    records::repl_with_path_fixture::ReplWithPathFixture,
  };

  // C++: ScopedFastFlag sffs[] = {{FFlag::LuauExportValueSyntax, true}, };
  LuauExportValueSyntax.push_test_override(true);
  struct Sff;
  impl Drop for Sff {
    fn drop(&mut self) {
      LuauExportValueSyntax.pop_test_override();
    }
  }
  let _sff = Sff;

  let mut fixture = ReplWithPathFixture::new();
  let path = repl_with_path_fixture_get_luau_directory(&fixture, PathType::Relative)
    + "/tests/require/without_config/export_keyword/require_export_freeze_local_nil_error";
  repl_with_path_fixture_run_protected_require(&fixture, &path);
  fixture.assert_output_contains_all(&[String::from("true")]);
}

#[cfg(test)]
#[test]
fn require_by_string_require_export_freeze_shadowing_ignored() {
  use alloc::string::String;

  use ulua_cli_test::{
    enums::path_type::PathType,
    methods::{
      repl_with_path_fixture_get_luau_directory::repl_with_path_fixture_get_luau_directory,
      repl_with_path_fixture_run_protected_require::repl_with_path_fixture_run_protected_require,
    },
    records::repl_with_path_fixture::ReplWithPathFixture,
  };

  // C++: ScopedFastFlag sffs[] = {{FFlag::LuauExportValueSyntax, true}, };
  LuauExportValueSyntax.push_test_override(true);
  struct Sff;
  impl Drop for Sff {
    fn drop(&mut self) {
      LuauExportValueSyntax.pop_test_override();
    }
  }
  let _sff = Sff;

  let mut fixture = ReplWithPathFixture::new();
  let path = repl_with_path_fixture_get_luau_directory(&fixture, PathType::Relative)
    + "/tests/require/without_config/export_keyword/require_export_freeze_shadowing";
  repl_with_path_fixture_run_protected_require(&fixture, &path);
  fixture.assert_output_contains_all(&[String::from("true")]);
}

#[cfg(test)]
#[test]
fn require_by_string_require_export_frozen() {
  use alloc::string::String;

  use ulua_cli_test::{
    enums::path_type::PathType,
    methods::{
      repl_with_path_fixture_get_luau_directory::repl_with_path_fixture_get_luau_directory,
      repl_with_path_fixture_run_protected_require::repl_with_path_fixture_run_protected_require,
    },
    records::repl_with_path_fixture::ReplWithPathFixture,
  };

  // C++: ScopedFastFlag sffs[] = {{FFlag::LuauExportValueSyntax, true}, };
  LuauExportValueSyntax.push_test_override(true);
  struct Sff;
  impl Drop for Sff {
    fn drop(&mut self) {
      LuauExportValueSyntax.pop_test_override();
    }
  }
  let _sff = Sff;

  let mut fixture = ReplWithPathFixture::new();
  let path = repl_with_path_fixture_get_luau_directory(&fixture, PathType::Relative)
    + "/tests/require/without_config/export_keyword/require_export_frozen";
  repl_with_path_fixture_run_protected_require(&fixture, &path);
  fixture.assert_output_contains_all(&[String::from("true")]);
}

#[cfg(test)]
#[test]
fn require_by_string_require_export_frozen_mutate() {
  use alloc::string::String;

  use ulua_cli_test::{
    enums::path_type::PathType,
    methods::{
      repl_with_path_fixture_get_luau_directory::repl_with_path_fixture_get_luau_directory,
      repl_with_path_fixture_run_protected_require::repl_with_path_fixture_run_protected_require,
    },
    records::repl_with_path_fixture::ReplWithPathFixture,
  };

  // C++: ScopedFastFlag sffs[] = {{FFlag::LuauExportValueSyntax, true}, };
  LuauExportValueSyntax.push_test_override(true);
  struct Sff;
  impl Drop for Sff {
    fn drop(&mut self) {
      LuauExportValueSyntax.pop_test_override();
    }
  }
  let _sff = Sff;

  let mut fixture = ReplWithPathFixture::new();
  let path = repl_with_path_fixture_get_luau_directory(&fixture, PathType::Relative)
    + "/tests/require/without_config/export_keyword/require_export_frozen_mutate";
  repl_with_path_fixture_run_protected_require(&fixture, &path);
  fixture.assert_output_contains_all(&[String::from("true")]);
}

#[cfg(test)]
#[test]
fn require_by_string_require_export_function() {
  use alloc::string::String;

  use ulua_cli_test::{
    enums::path_type::PathType,
    methods::{
      repl_with_path_fixture_get_luau_directory::repl_with_path_fixture_get_luau_directory,
      repl_with_path_fixture_run_protected_require::repl_with_path_fixture_run_protected_require,
    },
    records::repl_with_path_fixture::ReplWithPathFixture,
  };

  // C++: ScopedFastFlag sffs[] = {{FFlag::LuauExportValueSyntax, true}, };
  LuauExportValueSyntax.push_test_override(true);
  struct Sff;
  impl Drop for Sff {
    fn drop(&mut self) {
      LuauExportValueSyntax.pop_test_override();
    }
  }
  let _sff = Sff;

  let mut fixture = ReplWithPathFixture::new();
  let path = repl_with_path_fixture_get_luau_directory(&fixture, PathType::Relative)
    + "/tests/require/without_config/export_keyword/require_export_function";
  repl_with_path_fixture_run_protected_require(&fixture, &path);
  fixture.assert_output_contains_all(&[String::from("true")]);
}

#[cfg(test)]
#[test]
fn require_by_string_require_export_function_rebind() {
  use alloc::string::String;

  use ulua_cli_test::{
    enums::path_type::PathType,
    methods::{
      repl_with_path_fixture_get_luau_directory::repl_with_path_fixture_get_luau_directory,
      repl_with_path_fixture_run_protected_require::repl_with_path_fixture_run_protected_require,
    },
    records::repl_with_path_fixture::ReplWithPathFixture,
  };

  // C++: ScopedFastFlag sffs[] = {{FFlag::LuauExportValueSyntax, true}, };
  LuauExportValueSyntax.push_test_override(true);
  struct Sff;
  impl Drop for Sff {
    fn drop(&mut self) {
      LuauExportValueSyntax.pop_test_override();
    }
  }
  let _sff = Sff;

  let mut fixture = ReplWithPathFixture::new();
  let path = repl_with_path_fixture_get_luau_directory(&fixture, PathType::Relative)
    + "/tests/require/without_config/export_keyword/require_export_function_rebind";
  repl_with_path_fixture_run_protected_require(&fixture, &path);
  fixture.assert_output_contains_all(&[String::from("true")]);
}

#[cfg(test)]
#[test]
fn require_by_string_require_export_in_do_block_error() {
  use alloc::string::String;

  use ulua_cli_test::{
    enums::path_type::PathType,
    methods::{
      repl_with_path_fixture_get_luau_directory::repl_with_path_fixture_get_luau_directory,
      repl_with_path_fixture_run_protected_require::repl_with_path_fixture_run_protected_require,
    },
    records::repl_with_path_fixture::ReplWithPathFixture,
  };

  // C++: ScopedFastFlag sffs[] = {{FFlag::LuauExportValueSyntax, true}, };
  LuauExportValueSyntax.push_test_override(true);
  struct Sff;
  impl Drop for Sff {
    fn drop(&mut self) {
      LuauExportValueSyntax.pop_test_override();
    }
  }
  let _sff = Sff;

  let mut fixture = ReplWithPathFixture::new();
  let path = repl_with_path_fixture_get_luau_directory(&fixture, PathType::Relative)
    + "/tests/require/without_config/export_keyword/export_in_do_block_error";
  repl_with_path_fixture_run_protected_require(&fixture, &path);
  fixture.assert_output_contains_all(&[String::from(
    "'export' may only be applied to top-level statements",
  )]);
}

#[cfg(test)]
#[test]
fn require_by_string_require_export_in_else_if_error() {
  use alloc::string::String;

  use ulua_cli_test::{
    enums::path_type::PathType,
    methods::{
      repl_with_path_fixture_get_luau_directory::repl_with_path_fixture_get_luau_directory,
      repl_with_path_fixture_run_protected_require::repl_with_path_fixture_run_protected_require,
    },
    records::repl_with_path_fixture::ReplWithPathFixture,
  };

  // C++: ScopedFastFlag sffs[] = {{FFlag::LuauExportValueSyntax, true}, };
  LuauExportValueSyntax.push_test_override(true);
  struct Sff;
  impl Drop for Sff {
    fn drop(&mut self) {
      LuauExportValueSyntax.pop_test_override();
    }
  }
  let _sff = Sff;

  let mut fixture = ReplWithPathFixture::new();
  let path = repl_with_path_fixture_get_luau_directory(&fixture, PathType::Relative)
    + "/tests/require/without_config/export_keyword/export_in_elseif_error";
  repl_with_path_fixture_run_protected_require(&fixture, &path);
  fixture.assert_output_contains_all(&[String::from(
    "'export' may only be applied to top-level statements",
  )]);
}

#[cfg(test)]
#[test]
fn require_by_string_require_export_in_for_error() {
  use alloc::string::String;

  use ulua_cli_test::{
    enums::path_type::PathType,
    methods::{
      repl_with_path_fixture_get_luau_directory::repl_with_path_fixture_get_luau_directory,
      repl_with_path_fixture_run_protected_require::repl_with_path_fixture_run_protected_require,
    },
    records::repl_with_path_fixture::ReplWithPathFixture,
  };

  // C++: ScopedFastFlag sffs[] = {{FFlag::LuauExportValueSyntax, true}, };
  LuauExportValueSyntax.push_test_override(true);
  struct Sff;
  impl Drop for Sff {
    fn drop(&mut self) {
      LuauExportValueSyntax.pop_test_override();
    }
  }
  let _sff = Sff;

  let mut fixture = ReplWithPathFixture::new();
  let path = repl_with_path_fixture_get_luau_directory(&fixture, PathType::Relative)
    + "/tests/require/without_config/export_keyword/export_in_for_error";
  repl_with_path_fixture_run_protected_require(&fixture, &path);
  fixture.assert_output_contains_all(&[String::from(
    "'export' may only be applied to top-level statements",
  )]);
}

#[cfg(test)]
#[test]
fn require_by_string_require_export_in_function_error() {
  use alloc::string::String;

  use ulua_cli_test::{
    enums::path_type::PathType,
    methods::{
      repl_with_path_fixture_get_luau_directory::repl_with_path_fixture_get_luau_directory,
      repl_with_path_fixture_run_protected_require::repl_with_path_fixture_run_protected_require,
    },
    records::repl_with_path_fixture::ReplWithPathFixture,
  };

  // C++: ScopedFastFlag sffs[] = {{FFlag::LuauExportValueSyntax, true}, };
  LuauExportValueSyntax.push_test_override(true);
  struct Sff;
  impl Drop for Sff {
    fn drop(&mut self) {
      LuauExportValueSyntax.pop_test_override();
    }
  }
  let _sff = Sff;

  let mut fixture = ReplWithPathFixture::new();
  let path = repl_with_path_fixture_get_luau_directory(&fixture, PathType::Relative)
    + "/tests/require/without_config/export_keyword/export_in_function_error";
  repl_with_path_fixture_run_protected_require(&fixture, &path);
  fixture.assert_output_contains_all(&[String::from(
    "'export' may only be applied to top-level statements",
  )]);
}

#[cfg(test)]
#[test]
fn require_by_string_require_export_in_if_error() {
  use alloc::string::String;

  use ulua_cli_test::{
    enums::path_type::PathType,
    methods::{
      repl_with_path_fixture_get_luau_directory::repl_with_path_fixture_get_luau_directory,
      repl_with_path_fixture_run_protected_require::repl_with_path_fixture_run_protected_require,
    },
    records::repl_with_path_fixture::ReplWithPathFixture,
  };

  // C++: ScopedFastFlag sffs[] = {{FFlag::LuauExportValueSyntax, true}, };
  LuauExportValueSyntax.push_test_override(true);
  struct Sff;
  impl Drop for Sff {
    fn drop(&mut self) {
      LuauExportValueSyntax.pop_test_override();
    }
  }
  let _sff = Sff;

  let mut fixture = ReplWithPathFixture::new();
  let path = repl_with_path_fixture_get_luau_directory(&fixture, PathType::Relative)
    + "/tests/require/without_config/export_keyword/export_in_if_error";
  repl_with_path_fixture_run_protected_require(&fixture, &path);
  fixture.assert_output_contains_all(&[String::from(
    "'export' may only be applied to top-level statements",
  )]);
}

#[cfg(test)]
#[test]
fn require_by_string_require_export_in_repeat_error() {
  use alloc::string::String;

  use ulua_cli_test::{
    enums::path_type::PathType,
    methods::{
      repl_with_path_fixture_get_luau_directory::repl_with_path_fixture_get_luau_directory,
      repl_with_path_fixture_run_protected_require::repl_with_path_fixture_run_protected_require,
    },
    records::repl_with_path_fixture::ReplWithPathFixture,
  };

  // C++: ScopedFastFlag sffs[] = {{FFlag::LuauExportValueSyntax, true}, };
  LuauExportValueSyntax.push_test_override(true);
  struct Sff;
  impl Drop for Sff {
    fn drop(&mut self) {
      LuauExportValueSyntax.pop_test_override();
    }
  }
  let _sff = Sff;

  let mut fixture = ReplWithPathFixture::new();
  let path = repl_with_path_fixture_get_luau_directory(&fixture, PathType::Relative)
    + "/tests/require/without_config/export_keyword/export_in_repeat_error";
  repl_with_path_fixture_run_protected_require(&fixture, &path);
  fixture.assert_output_contains_all(&[String::from(
    "'export' may only be applied to top-level statements",
  )]);
}

#[cfg(test)]
#[test]
fn require_by_string_require_export_in_while_error() {
  use alloc::string::String;

  use ulua_cli_test::{
    enums::path_type::PathType,
    methods::{
      repl_with_path_fixture_get_luau_directory::repl_with_path_fixture_get_luau_directory,
      repl_with_path_fixture_run_protected_require::repl_with_path_fixture_run_protected_require,
    },
    records::repl_with_path_fixture::ReplWithPathFixture,
  };

  // C++: ScopedFastFlag sffs[] = {{FFlag::LuauExportValueSyntax, true}, };
  LuauExportValueSyntax.push_test_override(true);
  struct Sff;
  impl Drop for Sff {
    fn drop(&mut self) {
      LuauExportValueSyntax.pop_test_override();
    }
  }
  let _sff = Sff;

  let mut fixture = ReplWithPathFixture::new();
  let path = repl_with_path_fixture_get_luau_directory(&fixture, PathType::Relative)
    + "/tests/require/without_config/export_keyword/export_in_while_error";
  repl_with_path_fixture_run_protected_require(&fixture, &path);
  fixture.assert_output_contains_all(&[String::from(
    "'export' may only be applied to top-level statements",
  )]);
}

#[cfg(test)]
#[test]
fn require_by_string_require_export_internal_call() {
  use alloc::string::String;

  use ulua_cli_test::{
    enums::path_type::PathType,
    methods::{
      repl_with_path_fixture_get_luau_directory::repl_with_path_fixture_get_luau_directory,
      repl_with_path_fixture_run_protected_require::repl_with_path_fixture_run_protected_require,
    },
    records::repl_with_path_fixture::ReplWithPathFixture,
  };

  // C++: ScopedFastFlag sffs[] = {{FFlag::LuauExportValueSyntax, true}, };
  LuauExportValueSyntax.push_test_override(true);
  struct Sff;
  impl Drop for Sff {
    fn drop(&mut self) {
      LuauExportValueSyntax.pop_test_override();
    }
  }
  let _sff = Sff;

  let mut fixture = ReplWithPathFixture::new();
  let path = repl_with_path_fixture_get_luau_directory(&fixture, PathType::Relative)
    + "/tests/require/without_config/export_keyword/require_export_internal_call";
  repl_with_path_fixture_run_protected_require(&fixture, &path);
  fixture.assert_output_contains_all(&[String::from("true")]);
}

#[cfg(test)]
#[test]
fn require_by_string_require_export_mixed() {
  use alloc::string::String;

  use ulua_cli_test::{
    enums::path_type::PathType,
    methods::{
      repl_with_path_fixture_get_luau_directory::repl_with_path_fixture_get_luau_directory,
      repl_with_path_fixture_run_protected_require::repl_with_path_fixture_run_protected_require,
    },
    records::repl_with_path_fixture::ReplWithPathFixture,
  };

  // C++: ScopedFastFlag sffs[] = {{FFlag::LuauExportValueSyntax, true}, };
  LuauExportValueSyntax.push_test_override(true);
  struct Sff;
  impl Drop for Sff {
    fn drop(&mut self) {
      LuauExportValueSyntax.pop_test_override();
    }
  }
  let _sff = Sff;

  let mut fixture = ReplWithPathFixture::new();
  let path = repl_with_path_fixture_get_luau_directory(&fixture, PathType::Relative)
    + "/tests/require/without_config/export_keyword/require_export_mixed";
  repl_with_path_fixture_run_protected_require(&fixture, &path);
  fixture.assert_output_contains_all(&[String::from("true")]);
}

#[cfg(test)]
#[test]
fn require_by_string_require_export_multi_assign() {
  use alloc::string::String;

  use ulua_cli_test::{
    enums::path_type::PathType,
    methods::{
      repl_with_path_fixture_get_luau_directory::repl_with_path_fixture_get_luau_directory,
      repl_with_path_fixture_run_protected_require::repl_with_path_fixture_run_protected_require,
    },
    records::repl_with_path_fixture::ReplWithPathFixture,
  };

  // C++: ScopedFastFlag sffs[] = {{FFlag::LuauExportValueSyntax, true}, };
  LuauExportValueSyntax.push_test_override(true);
  struct Sff;
  impl Drop for Sff {
    fn drop(&mut self) {
      LuauExportValueSyntax.pop_test_override();
    }
  }
  let _sff = Sff;

  let mut fixture = ReplWithPathFixture::new();
  let path = repl_with_path_fixture_get_luau_directory(&fixture, PathType::Relative)
    + "/tests/require/without_config/export_keyword/require_export_multi_assign";
  repl_with_path_fixture_run_protected_require(&fixture, &path);
  fixture.assert_output_contains_all(&[String::from("true")]);
}

#[cfg(test)]
#[test]
fn require_by_string_require_export_multi_swap() {
  use alloc::string::String;

  use ulua_cli_test::{
    enums::path_type::PathType,
    methods::{
      repl_with_path_fixture_get_luau_directory::repl_with_path_fixture_get_luau_directory,
      repl_with_path_fixture_run_protected_require::repl_with_path_fixture_run_protected_require,
    },
    records::repl_with_path_fixture::ReplWithPathFixture,
  };

  // C++: ScopedFastFlag sffs[] = {{FFlag::LuauExportValueSyntax, true}, };
  LuauExportValueSyntax.push_test_override(true);
  struct Sff;
  impl Drop for Sff {
    fn drop(&mut self) {
      LuauExportValueSyntax.pop_test_override();
    }
  }
  let _sff = Sff;

  let mut fixture = ReplWithPathFixture::new();
  let path = repl_with_path_fixture_get_luau_directory(&fixture, PathType::Relative)
    + "/tests/require/without_config/export_keyword/require_export_multi_swap";
  repl_with_path_fixture_run_protected_require(&fixture, &path);
  fixture.assert_output_contains_all(&[String::from("true")]);
}

#[cfg(test)]
#[test]
fn require_by_string_require_export_multi_var() {
  use alloc::string::String;

  use ulua_cli_test::{
    enums::path_type::PathType,
    methods::{
      repl_with_path_fixture_get_luau_directory::repl_with_path_fixture_get_luau_directory,
      repl_with_path_fixture_run_protected_require::repl_with_path_fixture_run_protected_require,
    },
    records::repl_with_path_fixture::ReplWithPathFixture,
  };

  // C++: ScopedFastFlag sffs[] = {{FFlag::LuauExportValueSyntax, true}, };
  LuauExportValueSyntax.push_test_override(true);
  struct Sff;
  impl Drop for Sff {
    fn drop(&mut self) {
      LuauExportValueSyntax.pop_test_override();
    }
  }
  let _sff = Sff;

  let mut fixture = ReplWithPathFixture::new();
  let path = repl_with_path_fixture_get_luau_directory(&fixture, PathType::Relative)
    + "/tests/require/without_config/export_keyword/require_export_multi_var";
  repl_with_path_fixture_run_protected_require(&fixture, &path);
  fixture.assert_output_contains_all(&[String::from("true")]);
}

#[cfg(test)]
#[test]
fn require_by_string_require_export_mutual_recursion() {
  use alloc::string::String;

  use ulua_cli_test::{
    enums::path_type::PathType,
    methods::{
      repl_with_path_fixture_get_luau_directory::repl_with_path_fixture_get_luau_directory,
      repl_with_path_fixture_run_protected_require::repl_with_path_fixture_run_protected_require,
    },
    records::repl_with_path_fixture::ReplWithPathFixture,
  };

  // C++: ScopedFastFlag sffs[] = {{FFlag::LuauExportValueSyntax, true}, };
  LuauExportValueSyntax.push_test_override(true);
  struct Sff;
  impl Drop for Sff {
    fn drop(&mut self) {
      LuauExportValueSyntax.pop_test_override();
    }
  }
  let _sff = Sff;

  let mut fixture = ReplWithPathFixture::new();
  let path = repl_with_path_fixture_get_luau_directory(&fixture, PathType::Relative)
    + "/tests/require/without_config/export_keyword/require_export_mutual_recursion";
  repl_with_path_fixture_run_protected_require(&fixture, &path);
  fixture.assert_output_contains_all(&[String::from("true")]);
}

#[cfg(test)]
#[test]
fn require_by_string_require_export_nested_table() {
  use alloc::string::String;

  use ulua_cli_test::{
    enums::path_type::PathType,
    methods::{
      repl_with_path_fixture_get_luau_directory::repl_with_path_fixture_get_luau_directory,
      repl_with_path_fixture_run_protected_require::repl_with_path_fixture_run_protected_require,
    },
    records::repl_with_path_fixture::ReplWithPathFixture,
  };

  // C++: ScopedFastFlag sffs[] = {{FFlag::LuauExportValueSyntax, true}, };
  LuauExportValueSyntax.push_test_override(true);
  struct Sff;
  impl Drop for Sff {
    fn drop(&mut self) {
      LuauExportValueSyntax.pop_test_override();
    }
  }
  let _sff = Sff;

  let mut fixture = ReplWithPathFixture::new();
  let path = repl_with_path_fixture_get_luau_directory(&fixture, PathType::Relative)
    + "/tests/require/without_config/export_keyword/require_export_nested_table";
  repl_with_path_fixture_run_protected_require(&fixture, &path);
  fixture.assert_output_contains_all(&[String::from("true")]);
}

#[cfg(test)]
#[test]
fn require_by_string_require_export_shadowing() {
  use alloc::string::String;

  use ulua_cli_test::{
    enums::path_type::PathType,
    methods::{
      repl_with_path_fixture_get_luau_directory::repl_with_path_fixture_get_luau_directory,
      repl_with_path_fixture_run_protected_require::repl_with_path_fixture_run_protected_require,
    },
    records::repl_with_path_fixture::ReplWithPathFixture,
  };

  // C++: ScopedFastFlag sffs[] = {{FFlag::LuauExportValueSyntax, true}, };
  LuauExportValueSyntax.push_test_override(true);
  struct Sff;
  impl Drop for Sff {
    fn drop(&mut self) {
      LuauExportValueSyntax.pop_test_override();
    }
  }
  let _sff = Sff;

  let mut fixture = ReplWithPathFixture::new();
  let path = repl_with_path_fixture_get_luau_directory(&fixture, PathType::Relative)
    + "/tests/require/without_config/export_keyword/require_export_shadowing";
  repl_with_path_fixture_run_protected_require(&fixture, &path);
  fixture.assert_output_contains_all(&[String::from("true")]);
}

#[cfg(test)]
#[test]
fn require_by_string_require_export_trap() {
  use alloc::string::String;

  use ulua_cli_test::{
    enums::path_type::PathType,
    methods::{
      repl_with_path_fixture_get_luau_directory::repl_with_path_fixture_get_luau_directory,
      repl_with_path_fixture_run_protected_require::repl_with_path_fixture_run_protected_require,
    },
    records::repl_with_path_fixture::ReplWithPathFixture,
  };

  // C++: ScopedFastFlag sffs[] = {{FFlag::LuauExportValueSyntax, true}, };
  LuauExportValueSyntax.push_test_override(true);
  struct Sff;
  impl Drop for Sff {
    fn drop(&mut self) {
      LuauExportValueSyntax.pop_test_override();
    }
  }
  let _sff = Sff;

  let mut fixture = ReplWithPathFixture::new();
  let path = repl_with_path_fixture_get_luau_directory(&fixture, PathType::Relative)
    + "/tests/require/without_config/export_keyword/require_export_trap";
  repl_with_path_fixture_run_protected_require(&fixture, &path);
  fixture.assert_output_contains_all(&[String::from("true")]);
}

#[cfg(test)]
#[test]
fn require_by_string_require_export_type_with_return() {
  use alloc::string::String;

  use ulua_cli_test::{
    enums::path_type::PathType,
    methods::{
      repl_with_path_fixture_get_luau_directory::repl_with_path_fixture_get_luau_directory,
      repl_with_path_fixture_run_protected_require::repl_with_path_fixture_run_protected_require,
    },
    records::repl_with_path_fixture::ReplWithPathFixture,
  };

  // C++: ScopedFastFlag sffs[] = {{FFlag::LuauExportValueSyntax, true}, };
  LuauExportValueSyntax.push_test_override(true);
  struct Sff;
  impl Drop for Sff {
    fn drop(&mut self) {
      LuauExportValueSyntax.pop_test_override();
    }
  }
  let _sff = Sff;

  let mut fixture = ReplWithPathFixture::new();
  let path = repl_with_path_fixture_get_luau_directory(&fixture, PathType::Relative)
    + "/tests/require/without_config/export_keyword/require_export_type_with_return";
  repl_with_path_fixture_run_protected_require(&fixture, &path);
  fixture.assert_output_contains_all(&[String::from("true")]);
}

#[cfg(test)]
#[test]
fn require_by_string_require_export_upvalue() {
  use alloc::string::String;

  use ulua_cli_test::{
    enums::path_type::PathType,
    methods::{
      repl_with_path_fixture_get_luau_directory::repl_with_path_fixture_get_luau_directory,
      repl_with_path_fixture_run_protected_require::repl_with_path_fixture_run_protected_require,
    },
    records::repl_with_path_fixture::ReplWithPathFixture,
  };

  // C++: ScopedFastFlag sffs[] = {{FFlag::LuauExportValueSyntax, true}, };
  LuauExportValueSyntax.push_test_override(true);
  struct Sff;
  impl Drop for Sff {
    fn drop(&mut self) {
      LuauExportValueSyntax.pop_test_override();
    }
  }
  let _sff = Sff;

  let mut fixture = ReplWithPathFixture::new();
  let path = repl_with_path_fixture_get_luau_directory(&fixture, PathType::Relative)
    + "/tests/require/without_config/export_keyword/require_export_upvalue";
  repl_with_path_fixture_run_protected_require(&fixture, &path);
  fixture.assert_output_contains_all(&[String::from("true")]);
}

#[cfg(test)]
#[test]
fn require_by_string_require_export_value() {
  use alloc::string::String;

  use ulua_cli_test::{
    enums::path_type::PathType,
    methods::{
      repl_with_path_fixture_get_luau_directory::repl_with_path_fixture_get_luau_directory,
      repl_with_path_fixture_run_protected_require::repl_with_path_fixture_run_protected_require,
    },
    records::repl_with_path_fixture::ReplWithPathFixture,
  };

  // C++: ScopedFastFlag sffs[] = {{FFlag::LuauExportValueSyntax, true}, };
  LuauExportValueSyntax.push_test_override(true);
  struct Sff;
  impl Drop for Sff {
    fn drop(&mut self) {
      LuauExportValueSyntax.pop_test_override();
    }
  }
  let _sff = Sff;

  let mut fixture = ReplWithPathFixture::new();
  let path = repl_with_path_fixture_get_luau_directory(&fixture, PathType::Relative)
    + "/tests/require/without_config/export_keyword/require_export_value";
  repl_with_path_fixture_run_protected_require(&fixture, &path);
  fixture.assert_output_contains_all(&[String::from("true")]);
}

#[cfg(test)]
#[test]
fn require_by_string_require_export_with_return_error() {
  use alloc::string::String;

  use ulua_cli_test::{
    enums::path_type::PathType,
    methods::{
      repl_with_path_fixture_get_luau_directory::repl_with_path_fixture_get_luau_directory,
      repl_with_path_fixture_run_protected_require::repl_with_path_fixture_run_protected_require,
    },
    records::repl_with_path_fixture::ReplWithPathFixture,
  };

  // C++: ScopedFastFlag sffs[] = {{FFlag::LuauExportValueSyntax, true}, };
  LuauExportValueSyntax.push_test_override(true);
  struct Sff;
  impl Drop for Sff {
    fn drop(&mut self) {
      LuauExportValueSyntax.pop_test_override();
    }
  }
  let _sff = Sff;

  let mut fixture = ReplWithPathFixture::new();
  let path = repl_with_path_fixture_get_luau_directory(&fixture, PathType::Relative)
    + "/tests/require/without_config/export_keyword/export_with_return_error";
  repl_with_path_fixture_run_protected_require(&fixture, &path);
  fixture.assert_output_contains_all(&[String::from(
    "Exporting values is not compatible with top-level return",
  )]);
}

#[cfg(test)]
#[test]
fn require_by_string_require_from_luau_binary() {
  use alloc::{string::String, vec};

  use ulua_cli_test::{
    enums::path_type::PathType, functions::repl_main::repl_main,
    methods::repl_with_path_fixture_get_luau_directory::repl_with_path_fixture_get_luau_directory,
    records::repl_with_path_fixture::ReplWithPathFixture,
  };

  let fixture = ReplWithPathFixture::new();

  let dir_rel = || repl_with_path_fixture_get_luau_directory(&fixture, PathType::Relative);
  let dir_abs = || repl_with_path_fixture_get_luau_directory(&fixture, PathType::Absolute);

  let paths: vec::Vec<String> = vec![
    dir_rel() + "/tests/require/without_config/dependency.luau",
    dir_abs() + "/tests/require/without_config/dependency.luau",
    dir_rel() + "/tests/require/without_config/module.luau",
    dir_abs() + "/tests/require/without_config/module.luau",
    dir_rel() + "/tests/require/without_config/nested/init.luau",
    dir_abs() + "/tests/require/without_config/nested/init.luau",
    dir_rel() + "/tests/require/config_tests/with_config/src/submodule/init.luau",
    dir_abs() + "/tests/require/config_tests/with_config/src/submodule/init.luau",
    dir_rel() + "/tests/require/config_tests/with_config_luau/src/submodule/init.luau",
    dir_abs() + "/tests/require/config_tests/with_config_luau/src/submodule/init.luau",
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

#[cfg(test)]
#[test]
fn require_by_string_require_function() {
  use alloc::string::String;

  use ulua_cli_test::{
    enums::path_type::PathType,
    methods::{
      repl_with_path_fixture_get_luau_directory::repl_with_path_fixture_get_luau_directory,
      repl_with_path_fixture_run_protected_require::repl_with_path_fixture_run_protected_require,
    },
    records::repl_with_path_fixture::ReplWithPathFixture,
  };

  let mut fixture = ReplWithPathFixture::new();
  let path = repl_with_path_fixture_get_luau_directory(&fixture, PathType::Relative)
    + "/tests/require/without_config/types/function";
  repl_with_path_fixture_run_protected_require(&fixture, &path);
  fixture.assert_output_contains_all(&[String::from("true"), String::from("function")]);
}

#[cfg(test)]
#[test]
fn require_by_string_require_init_lua() {
  use alloc::string::String;

  use ulua_cli_test::{
    enums::path_type::PathType,
    methods::{
      repl_with_path_fixture_get_luau_directory::repl_with_path_fixture_get_luau_directory,
      repl_with_path_fixture_run_protected_require::repl_with_path_fixture_run_protected_require,
    },
    records::repl_with_path_fixture::ReplWithPathFixture,
  };

  let mut fixture = ReplWithPathFixture::new();
  let path = repl_with_path_fixture_get_luau_directory(&fixture, PathType::Relative)
    + "/tests/require/without_config/lua";
  repl_with_path_fixture_run_protected_require(&fixture, &path);
  fixture.assert_output_contains_all(&[String::from("true"), String::from("result from init.lua")]);
}

#[cfg(test)]
#[test]
fn require_by_string_require_init_luau() {
  use alloc::string::String;

  use ulua_cli_test::{
    enums::path_type::PathType,
    methods::{
      repl_with_path_fixture_get_luau_directory::repl_with_path_fixture_get_luau_directory,
      repl_with_path_fixture_run_protected_require::repl_with_path_fixture_run_protected_require,
    },
    records::repl_with_path_fixture::ReplWithPathFixture,
  };

  let mut fixture = ReplWithPathFixture::new();
  let path = repl_with_path_fixture_get_luau_directory(&fixture, PathType::Relative)
    + "/tests/require/without_config/luau";
  repl_with_path_fixture_run_protected_require(&fixture, &path);
  fixture
    .assert_output_contains_all(&[String::from("true"), String::from("result from init.luau")]);
}

#[cfg(test)]
#[test]
fn require_by_string_require_lua() {
  use alloc::string::String;

  use ulua_cli_test::{
    enums::path_type::PathType,
    methods::{
      repl_with_path_fixture_get_luau_directory::repl_with_path_fixture_get_luau_directory,
      repl_with_path_fixture_run_protected_require::repl_with_path_fixture_run_protected_require,
    },
    records::repl_with_path_fixture::ReplWithPathFixture,
  };

  let mut fixture = ReplWithPathFixture::new();
  let path = repl_with_path_fixture_get_luau_directory(&fixture, PathType::Relative)
    + "/tests/require/without_config/lua_dependency";
  repl_with_path_fixture_run_protected_require(&fixture, &path);
  fixture.assert_output_contains_all(&[
    String::from("true"),
    String::from("result from lua_dependency"),
  ]);
}

#[cfg(test)]
#[test]
fn require_by_string_require_nested_inits() {
  use alloc::string::String;

  use ulua_cli_test::{
    enums::path_type::PathType,
    methods::{
      repl_with_path_fixture_get_luau_directory::repl_with_path_fixture_get_luau_directory,
      repl_with_path_fixture_run_protected_require::repl_with_path_fixture_run_protected_require,
    },
    records::repl_with_path_fixture::ReplWithPathFixture,
  };

  let mut fixture = ReplWithPathFixture::new();
  let path = repl_with_path_fixture_get_luau_directory(&fixture, PathType::Relative)
    + "/tests/require/without_config/nested_inits_requirer";
  repl_with_path_fixture_run_protected_require(&fixture, &path);
  fixture.assert_output_contains_all(&[
    String::from("true"),
    String::from("result from nested_inits/init"),
    String::from("required into module"),
  ]);
}

#[cfg(test)]
#[test]
fn require_by_string_require_nil() {
  use alloc::string::String;

  use ulua_cli_test::{
    enums::path_type::PathType,
    methods::{
      repl_with_path_fixture_get_luau_directory::repl_with_path_fixture_get_luau_directory,
      repl_with_path_fixture_run_protected_require::repl_with_path_fixture_run_protected_require,
    },
    records::repl_with_path_fixture::ReplWithPathFixture,
  };

  let mut fixture = ReplWithPathFixture::new();
  let path = repl_with_path_fixture_get_luau_directory(&fixture, PathType::Relative)
    + "/tests/require/without_config/types/nil";
  repl_with_path_fixture_run_protected_require(&fixture, &path);
  fixture.assert_output_contains_all(&[String::from("true"), String::from("nil")]);
}

#[cfg(test)]
#[test]
fn require_by_string_require_number() {
  use alloc::string::String;

  use ulua_cli_test::{
    enums::path_type::PathType,
    methods::{
      repl_with_path_fixture_get_luau_directory::repl_with_path_fixture_get_luau_directory,
      repl_with_path_fixture_run_protected_require::repl_with_path_fixture_run_protected_require,
    },
    records::repl_with_path_fixture::ReplWithPathFixture,
  };

  let mut fixture = ReplWithPathFixture::new();
  let path = repl_with_path_fixture_get_luau_directory(&fixture, PathType::Relative)
    + "/tests/require/without_config/types/number";
  repl_with_path_fixture_run_protected_require(&fixture, &path);
  fixture.assert_output_contains_all(&[String::from("true"), String::from("12345")]);
}

#[cfg(test)]
#[test]
fn require_by_string_require_path_with_alias() {
  use alloc::string::String;

  use ulua_cli_test::{
    enums::path_type::PathType,
    methods::{
      repl_with_path_fixture_get_luau_directory::repl_with_path_fixture_get_luau_directory,
      repl_with_path_fixture_run_protected_require::repl_with_path_fixture_run_protected_require,
    },
    records::repl_with_path_fixture::ReplWithPathFixture,
  };

  let mut fixture = ReplWithPathFixture::new();
  {
    let path = repl_with_path_fixture_get_luau_directory(&fixture, PathType::Relative)
      + "/tests/require/config_tests/with_config/src/alias_requirer";
    repl_with_path_fixture_run_protected_require(&fixture, &path);
    fixture
      .assert_output_contains_all(&[String::from("true"), String::from("result from dependency")]);
  }
  {
    let path = repl_with_path_fixture_get_luau_directory(&fixture, PathType::Relative)
      + "/tests/require/config_tests/with_config_luau/src/alias_requirer";
    repl_with_path_fixture_run_protected_require(&fixture, &path);
    fixture
      .assert_output_contains_all(&[String::from("true"), String::from("result from dependency")]);
  }
}

#[cfg(test)]
#[test]
fn require_by_string_require_path_with_alias_pointing_to_directory() {
  use alloc::string::String;

  use ulua_cli_test::{
    enums::path_type::PathType,
    methods::{
      repl_with_path_fixture_get_luau_directory::repl_with_path_fixture_get_luau_directory,
      repl_with_path_fixture_run_protected_require::repl_with_path_fixture_run_protected_require,
    },
    records::repl_with_path_fixture::ReplWithPathFixture,
  };

  let mut fixture = ReplWithPathFixture::new();
  {
    let path = repl_with_path_fixture_get_luau_directory(&fixture, PathType::Relative)
      + "/tests/require/config_tests/with_config/src/directory_alias_requirer";
    repl_with_path_fixture_run_protected_require(&fixture, &path);
    fixture.assert_output_contains_all(&[
      String::from("true"),
      String::from("result from subdirectory_dependency"),
    ]);
  }
  {
    let path = repl_with_path_fixture_get_luau_directory(&fixture, PathType::Relative)
      + "/tests/require/config_tests/with_config_luau/src/directory_alias_requirer";
    repl_with_path_fixture_run_protected_require(&fixture, &path);
    fixture.assert_output_contains_all(&[
      String::from("true"),
      String::from("result from subdirectory_dependency"),
    ]);
  }
}

#[cfg(test)]
#[test]
fn require_by_string_require_path_with_parent_alias() {
  use alloc::string::String;

  use ulua_cli_test::{
    enums::path_type::PathType,
    methods::{
      repl_with_path_fixture_get_luau_directory::repl_with_path_fixture_get_luau_directory,
      repl_with_path_fixture_run_protected_require::repl_with_path_fixture_run_protected_require,
    },
    records::repl_with_path_fixture::ReplWithPathFixture,
  };

  let mut fixture = ReplWithPathFixture::new();
  {
    let path = repl_with_path_fixture_get_luau_directory(&fixture, PathType::Relative)
      + "/tests/require/config_tests/with_config/src/parent_alias_requirer";
    repl_with_path_fixture_run_protected_require(&fixture, &path);
    fixture.assert_output_contains_all(&[
      String::from("true"),
      String::from("result from other_dependency"),
    ]);
  }
  {
    let path = repl_with_path_fixture_get_luau_directory(&fixture, PathType::Relative)
      + "/tests/require/config_tests/with_config_luau/src/parent_alias_requirer";
    repl_with_path_fixture_run_protected_require(&fixture, &path);
    fixture.assert_output_contains_all(&[
      String::from("true"),
      String::from("result from other_dependency"),
    ]);
  }
}

#[cfg(test)]
#[test]
fn require_by_string_require_relative_to_requiring_file() {
  use alloc::string::String;

  use ulua_cli_test::{
    enums::path_type::PathType,
    methods::{
      repl_with_path_fixture_get_luau_directory::repl_with_path_fixture_get_luau_directory,
      repl_with_path_fixture_run_protected_require::repl_with_path_fixture_run_protected_require,
    },
    records::repl_with_path_fixture::ReplWithPathFixture,
  };

  let mut fixture = ReplWithPathFixture::new();
  let path = repl_with_path_fixture_get_luau_directory(&fixture, PathType::Relative)
    + "/tests/require/without_config/module";
  repl_with_path_fixture_run_protected_require(&fixture, &path);
  fixture.assert_output_contains_all(&[
    String::from("true"),
    String::from("result from dependency"),
    String::from("required into module"),
  ]);
}

#[cfg(test)]
#[test]
fn require_by_string_require_simple_relative_path() {
  use alloc::string::String;

  use ulua_cli_test::{
    enums::path_type::PathType,
    methods::{
      repl_with_path_fixture_get_luau_directory::repl_with_path_fixture_get_luau_directory,
      repl_with_path_fixture_run_protected_require::repl_with_path_fixture_run_protected_require,
    },
    records::repl_with_path_fixture::ReplWithPathFixture,
  };

  let mut fixture = ReplWithPathFixture::new();
  let path = repl_with_path_fixture_get_luau_directory(&fixture, PathType::Relative)
    + "/tests/require/without_config/dependency";
  repl_with_path_fixture_run_protected_require(&fixture, &path);
  fixture
    .assert_output_contains_all(&[String::from("true"), String::from("result from dependency")]);
}

#[cfg(test)]
#[test]
fn require_by_string_require_simple_relative_path_within_pcall() {
  use alloc::string::String;

  use ulua_cli_test::{
    enums::path_type::PathType, functions::run_code::run_code,
    methods::repl_with_path_fixture_get_luau_directory::repl_with_path_fixture_get_luau_directory,
    records::repl_with_path_fixture::ReplWithPathFixture,
  };
  use ulua_vm::records::lua_state::lua_State;

  let mut fixture = ReplWithPathFixture::new();
  let path = repl_with_path_fixture_get_luau_directory(&fixture, PathType::Relative)
    + "/tests/require/without_config/dependency";
  let code: String = format!("return pcall(require, \"{}\")", path);
  unsafe {
    run_code(fixture.l as *mut lua_State, &code);
  }
  fixture
    .assert_output_contains_all(&[String::from("true"), String::from("result from dependency")]);
}

#[cfg(test)]
#[test]
fn require_by_string_require_string() {
  use alloc::string::String;

  use ulua_cli_test::{
    enums::path_type::PathType,
    methods::{
      repl_with_path_fixture_get_luau_directory::repl_with_path_fixture_get_luau_directory,
      repl_with_path_fixture_run_protected_require::repl_with_path_fixture_run_protected_require,
    },
    records::repl_with_path_fixture::ReplWithPathFixture,
  };

  let mut fixture = ReplWithPathFixture::new();
  let path = repl_with_path_fixture_get_luau_directory(&fixture, PathType::Relative)
    + "/tests/require/without_config/types/string";
  repl_with_path_fixture_run_protected_require(&fixture, &path);
  fixture.assert_output_contains_all(&[String::from("true"), String::from("\"foo\"")]);
}

#[cfg(test)]
#[test]
fn require_by_string_require_submodule_using_self_directly() {
  use alloc::string::String;

  use ulua_cli_test::{
    enums::path_type::PathType,
    methods::{
      repl_with_path_fixture_get_luau_directory::repl_with_path_fixture_get_luau_directory,
      repl_with_path_fixture_run_protected_require::repl_with_path_fixture_run_protected_require,
    },
    records::repl_with_path_fixture::ReplWithPathFixture,
  };

  let mut fixture = ReplWithPathFixture::new();
  let path = repl_with_path_fixture_get_luau_directory(&fixture, PathType::Relative)
    + "/tests/require/without_config/nested";
  repl_with_path_fixture_run_protected_require(&fixture, &path);
  fixture
    .assert_output_contains_all(&[String::from("true"), String::from("result from submodule")]);
}

#[cfg(test)]
#[test]
fn require_by_string_require_submodule_using_self_indirectly() {
  use alloc::string::String;

  use ulua_cli_test::{
    enums::path_type::PathType,
    methods::{
      repl_with_path_fixture_get_luau_directory::repl_with_path_fixture_get_luau_directory,
      repl_with_path_fixture_run_protected_require::repl_with_path_fixture_run_protected_require,
    },
    records::repl_with_path_fixture::ReplWithPathFixture,
  };

  let mut fixture = ReplWithPathFixture::new();
  let path = repl_with_path_fixture_get_luau_directory(&fixture, PathType::Relative)
    + "/tests/require/without_config/nested_module_requirer";
  repl_with_path_fixture_run_protected_require(&fixture, &path);
  fixture
    .assert_output_contains_all(&[String::from("true"), String::from("result from submodule")]);
}

#[cfg(test)]
#[test]
fn require_by_string_require_table() {
  use alloc::string::String;

  use ulua_cli_test::{
    enums::path_type::PathType,
    methods::{
      repl_with_path_fixture_get_luau_directory::repl_with_path_fixture_get_luau_directory,
      repl_with_path_fixture_run_protected_require::repl_with_path_fixture_run_protected_require,
    },
    records::repl_with_path_fixture::ReplWithPathFixture,
  };

  let mut fixture = ReplWithPathFixture::new();
  let path = repl_with_path_fixture_get_luau_directory(&fixture, PathType::Relative)
    + "/tests/require/without_config/types/table";
  repl_with_path_fixture_run_protected_require(&fixture, &path);
  fixture.assert_output_contains_all(&[String::from("true"), String::from("{\"foo\", \"bar\"}")]);
}

#[cfg(test)]
#[test]
fn require_by_string_require_thread() {
  use alloc::string::String;

  use ulua_cli_test::{
    enums::path_type::PathType,
    methods::{
      repl_with_path_fixture_get_luau_directory::repl_with_path_fixture_get_luau_directory,
      repl_with_path_fixture_run_protected_require::repl_with_path_fixture_run_protected_require,
    },
    records::repl_with_path_fixture::ReplWithPathFixture,
  };

  let mut fixture = ReplWithPathFixture::new();
  let path = repl_with_path_fixture_get_luau_directory(&fixture, PathType::Relative)
    + "/tests/require/without_config/types/thread";
  repl_with_path_fixture_run_protected_require(&fixture, &path);
  fixture.assert_output_contains_all(&[String::from("true"), String::from("thread")]);
}

#[cfg(test)]
#[test]
fn require_by_string_require_unprefixed_path() {
  use alloc::string::String;

  use ulua_cli_test::{
    methods::repl_with_path_fixture_run_protected_require::repl_with_path_fixture_run_protected_require,
    records::repl_with_path_fixture::ReplWithPathFixture,
  };

  let mut fixture = ReplWithPathFixture::new();
  let path = String::from("an/unprefixed/path");
  repl_with_path_fixture_run_protected_require(&fixture, &path);
  fixture.assert_output_contains_all(&[
    String::from("false"),
    String::from("require path must start with a valid prefix: ./, ../, or @"),
  ]);
}

#[cfg(test)]
#[test]
fn require_by_string_require_userdata() {
  use alloc::string::String;

  use ulua_cli_test::{
    enums::path_type::PathType,
    methods::{
      repl_with_path_fixture_get_luau_directory::repl_with_path_fixture_get_luau_directory,
      repl_with_path_fixture_run_protected_require::repl_with_path_fixture_run_protected_require,
    },
    records::repl_with_path_fixture::ReplWithPathFixture,
  };

  let mut fixture = ReplWithPathFixture::new();
  let path = repl_with_path_fixture_get_luau_directory(&fixture, PathType::Relative)
    + "/tests/require/without_config/types/userdata";
  repl_with_path_fixture_run_protected_require(&fixture, &path);
  fixture.assert_output_contains_all(&[String::from("true"), String::from("userdata")]);
}

#[cfg(test)]
#[test]
fn require_by_string_require_vector() {
  use alloc::string::String;

  use ulua_cli_test::{
    enums::path_type::PathType,
    methods::{
      repl_with_path_fixture_get_luau_directory::repl_with_path_fixture_get_luau_directory,
      repl_with_path_fixture_run_protected_require::repl_with_path_fixture_run_protected_require,
    },
    records::repl_with_path_fixture::ReplWithPathFixture,
  };

  let mut fixture = ReplWithPathFixture::new();
  let path = repl_with_path_fixture_get_luau_directory(&fixture, PathType::Relative)
    + "/tests/require/without_config/types/vector";
  repl_with_path_fixture_run_protected_require(&fixture, &path);
  fixture.assert_output_contains_all(&[String::from("true"), String::from("1, 2, 3")]);
}

#[cfg(test)]
#[test]
fn require_by_string_require_with_ambiguity_in_alias_discovery() {
  use alloc::{string::String, vec};

  use ulua_cli_test::{
    enums::path_type::PathType, functions::repl_main::repl_main,
    methods::repl_with_path_fixture_get_luau_directory::repl_with_path_fixture_get_luau_directory,
    records::repl_with_path_fixture::ReplWithPathFixture,
  };

  let fixture = ReplWithPathFixture::new();

  let paths: vec::Vec<String> = vec![
    repl_with_path_fixture_get_luau_directory(&fixture, PathType::Relative)
      + "/tests/require/config_tests/with_config/parent_ambiguity/folder/requirer.luau",
    repl_with_path_fixture_get_luau_directory(&fixture, PathType::Absolute)
      + "/tests/require/config_tests/with_config/parent_ambiguity/folder/requirer.luau",
    repl_with_path_fixture_get_luau_directory(&fixture, PathType::Relative)
      + "/tests/require/config_tests/with_config_luau/parent_ambiguity/folder/requirer.luau",
    repl_with_path_fixture_get_luau_directory(&fixture, PathType::Absolute)
      + "/tests/require/config_tests/with_config_luau/parent_ambiguity/folder/requirer.luau",
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

#[cfg(test)]
#[test]
fn require_by_string_require_with_directory_ambiguity() {
  use alloc::string::String;

  use ulua_cli_test::{
    enums::path_type::PathType,
    methods::{
      repl_with_path_fixture_get_luau_directory::repl_with_path_fixture_get_luau_directory,
      repl_with_path_fixture_run_protected_require::repl_with_path_fixture_run_protected_require,
    },
    records::repl_with_path_fixture::ReplWithPathFixture,
  };

  let mut fixture = ReplWithPathFixture::new();
  let path = repl_with_path_fixture_get_luau_directory(&fixture, PathType::Relative)
    + "/tests/require/without_config/ambiguous_directory_requirer";
  repl_with_path_fixture_run_protected_require(&fixture, &path);
  fixture.assert_output_contains_all(&[
        String::from("false"),
        String::from("error requiring module \"./ambiguous/directory/dependency\": could not resolve child component \"dependency\" (ambiguous)"),
    ]);
}

#[cfg(test)]
#[test]
fn require_by_string_require_with_file_ambiguity() {
  use alloc::string::String;

  use ulua_cli_test::{
    enums::path_type::PathType,
    methods::{
      repl_with_path_fixture_get_luau_directory::repl_with_path_fixture_get_luau_directory,
      repl_with_path_fixture_run_protected_require::repl_with_path_fixture_run_protected_require,
    },
    records::repl_with_path_fixture::ReplWithPathFixture,
  };

  let mut fixture = ReplWithPathFixture::new();
  let path = repl_with_path_fixture_get_luau_directory(&fixture, PathType::Relative)
    + "/tests/require/without_config/ambiguous_file_requirer";
  repl_with_path_fixture_run_protected_require(&fixture, &path);
  fixture.assert_output_contains_all(&[
        String::from("false"),
        String::from("error requiring module \"./ambiguous/file/dependency\": could not resolve child component \"dependency\" (ambiguous)"),
    ]);
}
