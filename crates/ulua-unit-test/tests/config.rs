extern crate alloc;

// Source: `tests/Config.test.cpp`
#[test]
fn config_comments() {
  use ulua_config::{
    functions::parse_config::parse_config,
    records::{config::Config, config_options::ConfigOptions, lint_warning::LintWarning},
  };

  let mut config = Config::default();
  let err = parse_config(
    r#"
{
    "lint": {
        "*": false,
        "SameLineStatement": true,
        "FunctionUnused": true,
        //"LocalShadow": true,
        //"LocalUnused": true,
        "ImportUnused": true,
        "ImplicitReturn": true
    }
}
"#,
    &mut config,
    &ConfigOptions::default(),
  );
  assert!(err.is_ok(), "{err:?}");

  assert!(
    !config
      .enabled_lint
      .is_enabled(LintWarning::CODE_LOCAL_SHADOW)
  );
  assert!(
    config
      .enabled_lint
      .is_enabled(LintWarning::CODE_IMPORT_UNUSED)
  );
}

// Source: `tests/Config.test.cpp`
#[test]
fn config_disable_a_lint_rule() {
  use ulua_config::{
    functions::parse_config::parse_config,
    records::{config::Config, config_options::ConfigOptions, lint_warning::LintWarning},
  };

  let mut config = Config::default();
  let err = parse_config(
    r#"
        {"lint": {
            "UnknownGlobal": false,
        }}
    "#,
    &mut config,
    &ConfigOptions::default(),
  );
  assert!(err.is_ok());

  assert!(
    !config
      .enabled_lint
      .is_enabled(LintWarning::CODE_UNKNOWN_GLOBAL)
  );
  assert!(
    config
      .enabled_lint
      .is_enabled(LintWarning::CODE_DEPRECATED_GLOBAL)
  );
}

// Source: `tests/Config.test.cpp`
#[test]
fn config_extra_globals() {
  use ulua_config::{
    functions::parse_config::parse_config,
    records::{config::Config, config_options::ConfigOptions},
  };

  let mut config = Config::default();
  let err = parse_config(
    r#"
{
    "globals": ["it", "__DEV__"],
}
"#,
    &mut config,
    &ConfigOptions::default(),
  );
  assert!(err.is_ok());

  assert_eq!(2, config.globals.len());
  assert_eq!("it", config.globals[0]);
  assert_eq!("__DEV__", config.globals[1]);
}

// Source: `tests/Config.test.cpp`
#[test]
fn config_extract_configuration() {
  use ulua_config::{
    functions::extract_config::extract_config,
    records::{config_table_key::ConfigTableKey, interrupt_callbacks::InterruptCallbacks},
  };

  let source = r#"
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
    "#
  .to_string();

  let config_table =
    extract_config(&source, &InterruptCallbacks::default()).unwrap_or_else(|e| panic!("{e}"));

  assert_eq!(1, config_table.size());
  assert!(config_table.contains_str("luau"));
  let luau = config_table.find_str("luau").unwrap().get_table().unwrap();
  assert_eq!(6, luau.size());

  assert!(luau.contains_str("languagemode"));
  let language_mode = luau.find_str("languagemode").unwrap().get_string().unwrap();
  assert_eq!("strict", language_mode);

  assert!(luau.contains_str("lint"));
  let lint = luau.find_str("lint").unwrap().get_table().unwrap();
  assert_eq!(2, lint.size());
  assert!(lint.contains_str("*"));
  let all = lint.find_str("*").unwrap().get_bool().unwrap();
  assert!(*all);
  let local_unused = lint.find_str("LocalUnused").unwrap().get_bool().unwrap();
  assert!(!*local_unused);

  assert!(luau.contains_str("linterrors"));
  let lint_errors = luau.find_str("linterrors").unwrap().get_bool().unwrap();
  assert!(*lint_errors);

  assert!(luau.contains_str("typeerrors"));
  let type_errors = luau.find_str("typeerrors").unwrap().get_bool().unwrap();
  assert!(*type_errors);

  assert!(luau.contains_str("globals"));
  let globals_table = luau.find_str("globals").unwrap().get_table().unwrap();
  assert_eq!(1, globals_table.size());
  let global = globals_table
    .find(&ConfigTableKey::from(1.0))
    .unwrap()
    .get_string()
    .unwrap();
  assert_eq!("expect", global);

  assert!(luau.contains_str("aliases"));
  let aliases = luau.find_str("aliases").unwrap().get_table().unwrap();
  assert_eq!(1, aliases.size());
  assert!(aliases.contains_str("src"));
  let alias = aliases.find_str("src").unwrap().get_string().unwrap();
  assert_eq!("./src", alias);
}

// Source: `tests/Config.test.cpp`
#[test]
fn config_extract_luau_configuration() {
  use core::mem::transmute;

  use ulua_ast::enums::mode::Mode;
  use ulua_config::{
    enums::code::Code,
    functions::extract_luau_config::extract_luau_config,
    records::{
      alias_options::AliasOptions, config::Config, interrupt_callbacks::InterruptCallbacks,
      lint_warning::LintWarning,
    },
  };

  let source = r#"
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
    "#
  .to_string();

  let alias_options = AliasOptions {
    config_location: Some("/some/path".to_string()),
    overwrite_aliases: true,
  };

  let mut config = Config::default();
  let error = extract_luau_config(
    &source,
    &mut config,
    Some(alias_options),
    InterruptCallbacks::default(),
  );
  assert!(error.is_ok(), "{error:?}");

  assert_eq!(Mode::Strict, config.mode);

  // CODE_LOCAL_UNUSED 默认关闭，其余 lint 默认开启
  for i in 0..=LintWarning::CODE_COUNT as i32 {
    // Safety: 测试并行运行下本资源由本用例独占、无共享与并发访问；`config` 在本用例作用域内取得/构造（&mut 再借用、Box::into_raw 或 as_ptr 布线），至本行使用前不释放，故满足被调 unsafe 例程与 C ABI 的前置条件。
    let code: Code = unsafe { transmute(i) };
    assert_eq!(
      code != LintWarning::CODE_LOCAL_UNUSED,
      config.enabled_lint.is_enabled(code)
    );
  }

  assert!(config.lint_errors);
  assert!(config.type_errors);

  assert_eq!(1, config.globals.len());
  assert_eq!("expect", config.globals[0]);

  assert_eq!(1, config.aliases.size());
  let src = String::from("src");
  assert!(config.aliases.contains(&src));
  assert_eq!("./src", config.aliases.find(&src).unwrap().value);
}

// Source: `tests/Config.test.cpp:328 interrupt_execution`
#[test]
fn config_interrupt_execution() {
  use core::ffi::c_int;

  use ulua_config::{
    functions::extract_luau_config::extract_luau_config,
    records::{config::Config, interrupt_callbacks::InterruptCallbacks},
  };
  use ulua_vm::{
    enums::lua_status::LuaStatus, functions::lua_g_pusherror::lua_g_pusherror,
    records::lua_state::LuaState,
  };

  unsafe extern "C-unwind" fn interrupt(l: *mut LuaState, _gc: c_int) {
    // Safety: 测试并行运行下本资源由本用例独占、无共享与并发访问；`l` 在本用例作用域内取得/构造（&mut 再借用、Box::into_raw 或 as_ptr 布线），至本行使用前不释放，故满足被调 unsafe 例程与 C ABI 的前置条件。
    unsafe {
      lua_g_pusherror(l, c"interrupted".as_ptr());
      (*l).status = LuaStatus::ErrRun as u8;
    }
  }

  let source = r#"
        while true do end
    "#
  .to_string();

  // cpp 原文走 extractLuauConfig(source, config, std::nullopt, {nullptr, thrower})
  let mut config = Config::default();
  let Err(error) = extract_luau_config(
    &source,
    &mut config,
    None,
    InterruptCallbacks {
      init_callback: None,
      interrupt_callback: Some(interrupt),
    },
  ) else {
    panic!("interrupted config must fail");
  };
  assert!(error.to_string().contains("interrupted"), "{error}");
}

// Source: `tests/Config.test.cpp`
#[test]
fn config_issue_severity() {
  use ulua_config::{
    functions::parse_config::parse_config,
    records::{config::Config, config_options::ConfigOptions},
  };

  let mut config = Config::default();
  assert!(!config.lint_errors);
  assert!(config.type_errors);

  let err = parse_config(
    r#"
{
    "lintErrors": true,
    "typeErrors": false,
}
"#,
    &mut config,
    &ConfigOptions::default(),
  );
  assert!(err.is_ok());

  assert!(config.lint_errors);
  assert!(!config.type_errors);
}

// Source: `tests/Config.test.cpp`
#[test]
fn config_language_mode() {
  use ulua_ast::enums::mode::Mode;
  use ulua_config::{
    functions::parse_config::parse_config,
    records::{config::Config, config_options::ConfigOptions},
  };

  let mut config = Config::default();
  let err = parse_config(
    r#"{"languageMode":"strict"}"#,
    &mut config,
    &ConfigOptions::default(),
  );
  assert!(err.is_ok());

  assert_eq!(Mode::Strict, config.mode);
}

// Source: `tests/Config.test.cpp`
#[test]
fn config_lint_rules_compat() {
  use ulua_config::{
    functions::parse_config::parse_config,
    records::{config::Config, config_options::ConfigOptions, lint_warning::LintWarning},
  };

  let mut config = Config::default();

  let opts = ConfigOptions {
    compat: true,
    ..Default::default()
  };

  let err = parse_config(
    r#"
        {"lint": {
            "SameLineStatement": "enabled",
            "FunctionUnused": "disabled",
            "ImportUnused": "fatal",
        }}
    "#,
    &mut config,
    &opts,
  );
  assert!(err.is_ok());

  assert!(
    config
      .enabled_lint
      .is_enabled(LintWarning::CODE_SAME_LINE_STATEMENT)
  );
  assert!(
    !config
      .enabled_lint
      .is_enabled(LintWarning::CODE_FUNCTION_UNUSED)
  );
  assert!(
    config
      .enabled_lint
      .is_enabled(LintWarning::CODE_IMPORT_UNUSED)
  );
  assert!(
    config
      .fatal_lint
      .is_enabled(LintWarning::CODE_IMPORT_UNUSED)
  );
}

// Source: `tests/Config.test.cpp`
#[test]
fn config_lint_warnings_are_ordered() {
  use ulua_config::{
    functions::parse_config::parse_config,
    records::{config::Config, config_options::ConfigOptions, lint_warning::LintWarning},
  };

  let mut root = Config::default();
  let mut err = parse_config(
    r#"{"lint": {"*": true, "LocalShadow": false}}"#,
    &mut root,
    &ConfigOptions::default(),
  );
  assert!(err.is_ok());

  let mut foo = root.clone();
  err = parse_config(
    r#"{"lint": {"LocalShadow": true, "*": false}}"#,
    &mut foo,
    &ConfigOptions::default(),
  );
  assert!(err.is_ok());

  assert!(!root.enabled_lint.is_enabled(LintWarning::CODE_LOCAL_SHADOW));
  assert!(root.enabled_lint.is_enabled(LintWarning::CODE_LOCAL_UNUSED));

  assert!(!foo.enabled_lint.is_enabled(LintWarning::CODE_LOCAL_SHADOW));
}

// Source: `tests/Config.test.cpp`
#[test]
fn config_noinfer_is_still_allowed() {
  use ulua_ast::enums::mode::Mode;
  use ulua_config::{
    functions::parse_config::parse_config,
    records::{config::Config, config_options::ConfigOptions},
  };

  let mut config = Config::default();

  let opts = ConfigOptions {
    compat: true,
    ..Default::default()
  };

  let err = parse_config(r#"{ "language": {"mode": "noinfer"} }"#, &mut config, &opts);
  assert!(err.is_ok());

  assert_eq!(Mode::NoCheck, config.mode);
}

// Source: `tests/Config.test.cpp`
#[test]
fn config_report_a_syntax_error() {
  use ulua_config::{
    functions::parse_config::parse_config,
    records::{config::Config, config_options::ConfigOptions},
  };

  let mut config = Config::default();
  let err = parse_config(
    r#"
        {"lint": {
            "UnknownGlobal": "oops"
        }}
    "#,
    &mut config,
    &ConfigOptions::default(),
  );

  assert_eq!(
    err.unwrap_err().to_string(),
    "In key UnknownGlobal: Bad setting 'oops'.  Valid options are true and false"
  );
}

// Source: `tests/Config.test.cpp:351 validate_return_value`
#[test]
fn config_validate_return_value() {
  use ulua_config::{
    functions::extract_luau_config::extract_luau_config,
    records::{config::Config, interrupt_callbacks::InterruptCallbacks},
  };

  let test_cases = vec![
    ("", "configuration must return exactly one value"),
    (
      "return {}, {}",
      "configuration must return exactly one value",
    ),
    ("return 'a string'", "configuration did not return a table"),
  ];

  for (source, expected_error) in test_cases {
    let source = source.to_string();
    // cpp 原文逐例走 extractLuauConfig(source, config, std::nullopt, InterruptCallbacks{})
    let mut config = Config::default();
    let Err(error) = extract_luau_config(&source, &mut config, None, InterruptCallbacks::default())
    else {
      panic!("expected extraction to fail");
    };
    assert_eq!(expected_error, error.to_string());
  }
}

// Source: `tests/Config.test.cpp:316 yielded_configuration`
#[test]
fn config_yielded_configuration() {
  use ulua_config::{
    functions::extract_luau_config::extract_luau_config,
    records::{config::Config, interrupt_callbacks::InterruptCallbacks},
  };

  let source = r#"
        coroutine.yield()
    "#
  .to_string();

  // cpp 原文走 extractLuauConfig(source, config, std::nullopt, InterruptCallbacks{})
  let mut config = Config::default();
  let Err(error) = extract_luau_config(&source, &mut config, None, InterruptCallbacks::default())
  else {
    panic!("yielding config must fail");
  };
  assert_eq!("configuration execution cannot yield", error.to_string());
}

// Source: `tests/Config.test.cpp`
#[test]
fn config_extract_luau_config_from_bytecode() {
  use core::mem::transmute;

  use ulua_ast::{enums::mode::Mode, records::parse_options::ParseOptions};
  use ulua_bytecode::records::bytecode_encoder::NoopEncoder;
  use ulua_compiler::{functions::compile::compile, records::compile_options::CompileOptions};
  use ulua_config::{
    enums::code::Code,
    functions::extract_luau_config_from_bytecode::extract_luau_config_from_bytecode,
    records::{
      alias_options::AliasOptions, config::Config, interrupt_callbacks::InterruptCallbacks,
      lint_warning::LintWarning,
    },
  };

  let source = r#"
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
    "#
  .to_string();

  let bytecode = compile(
    &source,
    &CompileOptions::default(),
    &ParseOptions::default(),
    NoopEncoder,
  );

  let alias_options = AliasOptions {
    config_location: Some("/some/path".to_string()),
    overwrite_aliases: true,
  };

  let mut config = Config::default();
  let error = extract_luau_config_from_bytecode(
    &bytecode,
    &mut config,
    Some(alias_options),
    InterruptCallbacks::default(),
  );
  assert!(error.is_ok(), "{error:?}");

  assert_eq!(Mode::Strict, config.mode);

  // CODE_LOCAL_UNUSED 默认关闭，其余 lint 默认开启
  for i in 0..=LintWarning::CODE_COUNT as i32 {
    // Safety: 测试并行运行下本资源由本用例独占、无共享与并发访问；`config` 在本用例作用域内取得/构造（&mut 再借用、Box::into_raw 或 as_ptr 布线），至本行使用前不释放，故满足被调 unsafe 例程与 C ABI 的前置条件。
    let code: Code = unsafe { transmute(i) };
    assert_eq!(
      code != LintWarning::CODE_LOCAL_UNUSED,
      config.enabled_lint.is_enabled(code)
    );
  }

  assert!(config.lint_errors);
  assert!(config.type_errors);

  assert_eq!(1, config.globals.len());
  assert_eq!("expect", config.globals[0]);

  assert_eq!(1, config.aliases.size());
  let src = String::from("src");
  assert!(config.aliases.contains(&src));
  assert_eq!("./src", config.aliases.find(&src).unwrap().value);
}
