use ulua_ast::{
  enums::mode::Mode,
  records::{hot_comment::HotComment, location::Location, parse_options::ParseOptions},
};
use ulua_bytecode::records::bytecode_encoder::BytecodeEncoder;
use ulua_compiler::{functions::compile::compile, records::compile_options::CompileOptions};
use ulua_config::{
  enums::code::Code,
  functions::{
    extract_config::extract_config, extract_luau_config::extract_luau_config,
    extract_luau_config_from_bytecode::extract_luau_config_from_bytecode,
    parse_config::parse_config,
  },
  records::{
    alias_options::AliasOptions, config::Config, config_options::ConfigOptions,
    config_table_key::ConfigTableKey, interrupt_callbacks::InterruptCallbacks,
    lint_warning::LintWarning,
  },
};

#[test]
fn test_language_mode() {
  let mut config = Config::default();
  let err = parse_config(
    r#"{"languageMode":"strict"}"#,
    &mut config,
    &ConfigOptions::default(),
  );
  assert!(err.is_none());
  assert_eq!(config.mode, Mode::Strict);
}

#[test]
fn test_disable_a_lint_rule() {
  let mut config = Config::default();
  let err = parse_config(
    r#"{
      "lint": {
        "UnknownGlobal": false
      }
    }"#,
    &mut config,
    &ConfigOptions::default(),
  );
  assert!(err.is_none());
  assert!(!config.enabled_lint.is_enabled(Code::UnknownGlobal));
  assert!(config.enabled_lint.is_enabled(Code::DeprecatedGlobal));
}

#[test]
fn test_report_a_syntax_error() {
  let mut config = Config::default();
  let err = parse_config(
    r#"{
      "lint": {
        "UnknownGlobal": "oops"
      }
    }"#,
    &mut config,
    &ConfigOptions::default(),
  );
  assert!(err.is_some());
  assert_eq!(
    err.as_deref(),
    Some("In key UnknownGlobal: Bad setting 'oops'.  Valid options are true and false")
  );
}

#[test]
fn test_noinfer_is_still_allowed() {
  let mut config = Config::default();
  let opts = ConfigOptions {
    compat: true,
    alias_options: None,
  };
  let err = parse_config(r#"{"language": {"mode": "noinfer"}}"#, &mut config, &opts);
  assert!(err.is_none());
  assert_eq!(config.mode, Mode::NoCheck);
}

#[test]
fn test_lint_warnings_are_ordered() {
  let mut root = Config::default();
  let err = parse_config(
    r#"{"lint": {"*": true, "LocalShadow": false}}"#,
    &mut root,
    &ConfigOptions::default(),
  );
  assert!(err.is_none());

  let mut foo = root.clone();
  let err2 = parse_config(
    r#"{"lint": {"LocalShadow": true, "*": false}}"#,
    &mut foo,
    &ConfigOptions::default(),
  );
  assert!(err2.is_none());

  assert!(!root.enabled_lint.is_enabled(Code::LocalShadow));
  assert!(root.enabled_lint.is_enabled(Code::LocalUnused));

  assert!(!foo.enabled_lint.is_enabled(Code::LocalShadow));
}

#[test]
fn test_comments() {
  let mut config = Config::default();
  let err = parse_config(
    r#"{
      "lint": {
        "*": false,
        "SameLineStatement": true,
        "FunctionUnused": true,
        //"LocalShadow": true,
        //"LocalUnused": true,
        "ImportUnused": true,
        "ImplicitReturn": true
      }
    }"#,
    &mut config,
    &ConfigOptions::default(),
  );
  assert!(err.is_none());
  assert!(!config.enabled_lint.is_enabled(Code::LocalShadow));
  assert!(config.enabled_lint.is_enabled(Code::ImportUnused));
}

#[test]
fn test_issue_severity() {
  let mut config = Config::default();
  assert!(!config.lint_errors);
  assert!(config.type_errors);

  let err = parse_config(
    r#"{
      "lintErrors": true,
      "typeErrors": false
    }"#,
    &mut config,
    &ConfigOptions::default(),
  );
  assert!(err.is_none());
  assert!(config.lint_errors);
  assert!(!config.type_errors);
}

#[test]
fn test_extra_globals() {
  let mut config = Config::default();
  let err = parse_config(
    r#"{
      "globals": ["it", "__DEV__"]
    }"#,
    &mut config,
    &ConfigOptions::default(),
  );
  assert!(err.is_none());
  assert_eq!(config.globals.len(), 2);
  assert_eq!(config.globals[0], "it");
  assert_eq!(config.globals[1], "__DEV__");
}

#[test]
fn test_lint_rules_compat() {
  let mut config = Config::default();
  let opts = ConfigOptions {
    compat: true,
    alias_options: None,
  };
  let err = parse_config(
    r#"{
      "lint": {
        "SameLineStatement": "enabled",
        "FunctionUnused": "disabled",
        "ImportUnused": "fatal"
      }
    }"#,
    &mut config,
    &opts,
  );
  assert!(err.is_none());
  assert!(config.enabled_lint.is_enabled(Code::SameLineStatement));
  assert!(!config.enabled_lint.is_enabled(Code::FunctionUnused));
  assert!(config.enabled_lint.is_enabled(Code::ImportUnused));
  assert!(config.fatal_lint.is_enabled(Code::ImportUnused));
}

#[test]
fn test_extract_configuration() {
  let source = test_config_source();

  let mut error = String::new();
  let config_table = extract_config(&source, &InterruptCallbacks::default(), &mut error);
  assert!(config_table.is_some(), "error: {error}");
  let config_table = config_table.unwrap();
  assert_eq!(config_table.size(), 1);
  assert!(config_table.contains_str("luau"));

  let luau = config_table.find_str("luau").unwrap().get_table().unwrap();
  assert_eq!(luau.size(), 6);
  assert_eq!(
    luau.find_str("languagemode").unwrap().get_string().unwrap(),
    "strict"
  );

  let lint = luau.find_str("lint").unwrap().get_table().unwrap();
  assert_eq!(lint.size(), 2);
  assert!(lint.find_str("*").unwrap().get_bool().unwrap());
  assert!(!lint.find_str("LocalUnused").unwrap().get_bool().unwrap());

  assert!(luau.find_str("linterrors").unwrap().get_bool().unwrap());
  assert!(luau.find_str("typeerrors").unwrap().get_bool().unwrap());

  let globals = luau.find_str("globals").unwrap().get_table().unwrap();
  assert_eq!(globals.size(), 1);
  assert_eq!(
    globals
      .find(&ConfigTableKey::from(1.0))
      .unwrap()
      .get_string()
      .unwrap(),
    "expect"
  );

  let aliases = luau.find_str("aliases").unwrap().get_table().unwrap();
  assert_eq!(aliases.size(), 1);
  assert_eq!(
    aliases.find_str("src").unwrap().get_string().unwrap(),
    "./src"
  );
}

#[test]
fn test_extract_luau_configuration() {
  let source = test_config_source();

  let alias_options = AliasOptions {
    config_location: Some(String::from("/some/path")),
    overwrite_aliases: true,
  };

  let mut config = Config::default();
  let err = extract_luau_config(
    &source,
    &mut config,
    Some(alias_options),
    InterruptCallbacks::default(),
  );
  assert!(err.is_none(), "unexpected error: {:?}", err);

  assert_eq!(config.mode, Mode::Strict);
  // 默认全开（含哨兵位），仅 LocalUnused 被 "*" 后的覆盖关闭
  assert!((config.enabled_lint.warning_mask >> Code::Count as i32) & 1 == 1);
  for code in Code::ALL {
    if code == Code::LocalUnused {
      assert!(!config.enabled_lint.is_enabled(code));
    } else {
      assert!(
        config.enabled_lint.is_enabled(code),
        "not enabled: {code:?}"
      );
    }
  }
  assert!(config.lint_errors);
  assert!(config.type_errors);
  assert_eq!(config.globals, vec!["expect"]);
  assert_eq!(config.aliases.size(), 1);
  assert!(config.aliases.contains_key(&String::from("src")));
  assert_eq!(
    config.aliases.find(&String::from("src")).unwrap().value,
    "./src"
  );
}

#[test]
fn test_extract_luau_config_from_bytecode() {
  let source = test_config_source();

  struct NoopEncoder;

  impl BytecodeEncoder for NoopEncoder {
    fn encode(&mut self, _data: &mut [u32]) {}
  }

  let mut encoder = NoopEncoder;
  let bytecode = compile(
    &source,
    &CompileOptions::default(),
    &ParseOptions::default(),
    &mut encoder as *mut dyn BytecodeEncoder,
  );

  let alias_options = AliasOptions {
    config_location: Some(String::from("/some/path")),
    overwrite_aliases: true,
  };

  let mut config = Config::default();
  let err = extract_luau_config_from_bytecode(
    bytecode.as_bytes(),
    &mut config,
    Some(alias_options),
    InterruptCallbacks::default(),
  );
  assert!(err.is_none(), "unexpected error: {:?}", err);

  assert_eq!(config.mode, Mode::Strict);
  for code in Code::ALL {
    if code == Code::LocalUnused {
      assert!(!config.enabled_lint.is_enabled(code));
    } else {
      assert!(
        config.enabled_lint.is_enabled(code),
        "not enabled: {code:?}"
      );
    }
  }
  assert!(config.lint_errors);
  assert!(config.type_errors);
  assert_eq!(config.globals, vec!["expect"]);
  assert_eq!(config.aliases.size(), 1);
  assert_eq!(
    config.aliases.find(&String::from("src")).unwrap().value,
    "./src"
  );
}

#[test]
fn test_yielded_configuration() {
  let source = String::from("coroutine.yield()");
  let mut config = Config::default();
  let err = extract_luau_config(&source, &mut config, None, InterruptCallbacks::default());
  assert!(err.is_some());
  assert_eq!(err.as_deref(), Some("configuration execution cannot yield"));
}

#[test]
fn test_interrupt_execution() {
  use core::ffi::c_int;

  use ulua_vm::{
    enums::lua_status::LuaStatus, functions::lua_g_pusherror::lua_g_pusherror,
    type_aliases::lua_state::lua_State,
  };

  unsafe extern "C-unwind" fn interrupt(l: *mut lua_State, _gc: c_int) {
    unsafe {
      lua_g_pusherror(l, c"interrupted".as_ptr());
      (*l).status = LuaStatus::ErrRun as u8;
    }
  }

  let source = String::from("while true do end");
  let mut config = Config::default();
  let err = extract_luau_config(
    &source,
    &mut config,
    None,
    InterruptCallbacks {
      init_callback: None,
      interrupt_callback: Some(interrupt),
    },
  );
  assert!(err.is_some());
  assert!(err.as_deref().unwrap().contains("interrupted"));
}

#[test]
fn test_validate_return_value() {
  let test_cases = [
    ("", "configuration must return exactly one value"),
    (
      "return {}, {}",
      "configuration must return exactly one value",
    ),
    ("return 'a string'", "configuration did not return a table"),
  ];

  for (source, expected_error) in test_cases {
    let mut config = Config::default();
    let err = extract_luau_config(
      &String::from(source),
      &mut config,
      None,
      InterruptCallbacks::default(),
    );
    assert!(err.is_some(), "expected error for '{source}'");
    assert_eq!(err.as_deref(), Some(expected_error));
  }
}

#[test]
fn test_hotcomment_parse_mask() {
  let hotcomments = vec![HotComment {
    header: true,
    location: Location::default(),
    content: String::from("nolint"),
  }];
  assert_eq!(LintWarning::parse_mask(&hotcomments), !0u64);

  let specific_comments = vec![
    HotComment {
      header: true,
      location: Location::default(),
      content: String::from("nolint LocalShadow"),
    },
    HotComment {
      header: true,
      location: Location::default(),
      content: String::from("nolint LocalUnused"),
    },
  ];
  let mask = LintWarning::parse_mask(&specific_comments);
  assert_ne!(mask, 0);
  assert_ne!(mask, !0u64);
  assert_ne!(mask & (1u64 << (Code::LocalShadow as i32)), 0);
  assert_ne!(mask & (1u64 << (Code::LocalUnused as i32)), 0);
  assert_eq!(mask & (1u64 << (Code::UnknownGlobal as i32)), 0);
}

/// 对应 C++ 测试共用的配置源码
fn test_config_source() -> String {
  String::from(
    r#"
      local config = {}
      config.luau = {}

      config.luau.languagemode = "strict"
      config.luau.lint = {
          ["*"] = true,
          LocalUnused = false
      }
      config.luau.linterrors = true
      config.luau.typeerrors = true
      config.luau.globals = {"expect"}
      config.luau.aliases = {
          src = "./src"
      }

      return config
    "#,
  )
}
