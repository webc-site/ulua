use strum::IntoEnumIterator;
use ulua_ast::{
  enums::mode::Mode,
  records::{hot_comment::HotComment, location::Location, parse_options::ParseOptions},
};
use ulua_bytecode::records::bytecode_encoder::NoopEncoder;
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

/// parse_config 测试公共样板：默认 `ConfigOptions::default()` + 断言无错误
/// （对齐 cpp `REQUIRE(!err)`；失败即 panic 带出错误全文）。
fn parse_ok(source: &str, config: &mut Config) {
  if let Err(err) = parse_config(source, config, &ConfigOptions::default()) {
    panic!("解析 {source} 期望无错误，实际 {err}");
  }
}

/// [`parse_ok`] 的失败面：断言错误消息全等。
fn parse_err_eq(source: &str, config: &mut Config, expected: &str) {
  let err = parse_config(source, config, &ConfigOptions::default());
  assert_eq!(err.unwrap_err().to_string(), expected);
}

/// extract_luau_config 测试失败样板：全新默认 config、无 alias、无中断回调，
/// 返回错误消息（unwrap 失败即 panic，两处循环/用例共用）。
fn extract_err_message(source: &str) -> String {
  let mut config = Config::default();
  let err = extract_luau_config(
    &String::from(source),
    &mut config,
    None,
    InterruptCallbacks::default(),
  );
  err.unwrap_err().to_string()
}

/// 两条 extract_luau_config 链路（source / bytecode）共用的 alias 选项
fn luau_alias_options() -> AliasOptions {
  AliasOptions {
    config_location: Some(String::from("/some/path")),
    overwrite_aliases: true,
  }
}

/// `test_config_source()` 应套用的完整期望：mode/lint/lintErrors/typeErrors/
/// globals/aliases 逐项断言，source 路与 bytecode 路共用同一份判定。
fn assert_luau_config_applied(config: &Config) {
  assert_eq!(config.mode, Mode::Strict);
  for code in Code::iter() {
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
fn test_language_mode() {
  let mut config = Config::default();
  parse_ok(r#"{"languageMode":"strict"}"#, &mut config);
  assert_eq!(config.mode, Mode::Strict);
}

#[test]
fn test_disable_a_lint_rule() {
  let mut config = Config::default();
  parse_ok(
    r#"{
      "lint": {
        "UnknownGlobal": false
      }
    }"#,
    &mut config,
  );
  assert!(!config.enabled_lint.is_enabled(Code::UnknownGlobal));
  assert!(config.enabled_lint.is_enabled(Code::DeprecatedGlobal));
}

#[test]
fn test_report_a_syntax_error() {
  let mut config = Config::default();
  parse_err_eq(
    r#"{
      "lint": {
        "UnknownGlobal": "oops"
      }
    }"#,
    &mut config,
    "In key UnknownGlobal: Bad setting 'oops'.  Valid options are true and false",
  );
}

#[test]
fn test_noinfer_is_still_allowed() {
  let mut config = Config::default();
  let opts = ConfigOptions {
    compat: true,
    alias_options: None,
  };
  let src = r#"{"language": {"mode": "noinfer"}}"#;
  if let Err(err) = parse_config(src, &mut config, &opts) {
    panic!("解析 {src} 期望无错误，实际 {err}");
  }
  assert_eq!(config.mode, Mode::NoCheck);
}

#[test]
fn test_lint_warnings_are_ordered() {
  let mut root = Config::default();
  parse_ok(r#"{"lint": {"*": true, "LocalShadow": false}}"#, &mut root);

  let mut foo = root.clone();
  parse_ok(r#"{"lint": {"LocalShadow": true, "*": false}}"#, &mut foo);

  assert!(!root.enabled_lint.is_enabled(Code::LocalShadow));
  assert!(root.enabled_lint.is_enabled(Code::LocalUnused));

  assert!(!foo.enabled_lint.is_enabled(Code::LocalShadow));
}

#[test]
fn test_comments() {
  let mut config = Config::default();
  parse_ok(
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
  );
  assert!(!config.enabled_lint.is_enabled(Code::LocalShadow));
  assert!(config.enabled_lint.is_enabled(Code::ImportUnused));
}

#[test]
fn test_issue_severity() {
  let mut config = Config::default();
  assert!(!config.lint_errors);
  assert!(config.type_errors);

  parse_ok(
    r#"{
      "lintErrors": true,
      "typeErrors": false
    }"#,
    &mut config,
  );
  assert!(config.lint_errors);
  assert!(!config.type_errors);
}

#[test]
fn test_extra_globals() {
  let mut config = Config::default();
  parse_ok(
    r#"{
      "globals": ["it", "__DEV__"]
    }"#,
    &mut config,
  );
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
  if let Err(err) = parse_config(
    r#"{
      "lint": {
        "SameLineStatement": "enabled",
        "FunctionUnused": "disabled",
        "ImportUnused": "fatal"
      }
    }"#,
    &mut config,
    &opts,
  ) {
    panic!("compat 解析期望无错误，实际 {err}");
  }
  assert!(config.enabled_lint.is_enabled(Code::SameLineStatement));
  assert!(!config.enabled_lint.is_enabled(Code::FunctionUnused));
  assert!(config.enabled_lint.is_enabled(Code::ImportUnused));
  assert!(config.fatal_lint.is_enabled(Code::ImportUnused));
}

#[test]
fn test_extract_configuration() {
  let source = test_config_source();

  let config_table =
    extract_config(&source, &InterruptCallbacks::default()).unwrap_or_else(|e| panic!("{e}"));
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

  let mut config = Config::default();
  if let Err(err) = extract_luau_config(
    &source,
    &mut config,
    Some(luau_alias_options()),
    InterruptCallbacks::default(),
  ) {
    panic!("unexpected error: {err}");
  }

  // 默认全开（含哨兵位），仅 LocalUnused 被 "*" 后的覆盖关闭
  assert!((config.enabled_lint.warning_mask >> Code::Count as i32) & 1 == 1);
  assert_luau_config_applied(&config);
}

#[test]
fn test_extract_luau_config_from_bytecode() {
  let source = test_config_source();

  let bytecode = compile(
    &source,
    &CompileOptions::default(),
    &ParseOptions::default(),
    NoopEncoder,
  );

  let mut config = Config::default();
  if let Err(err) = extract_luau_config_from_bytecode(
    &bytecode,
    &mut config,
    Some(luau_alias_options()),
    InterruptCallbacks::default(),
  ) {
    panic!("unexpected error: {err}");
  }

  assert_luau_config_applied(&config);
}

#[test]
fn test_globals_numeric_indices() {
  for (table, expected) in [
    ("{}", vec![]),
    ("{[2] = 'second', [1] = 'first'}", vec!["first", "second"]),
    ("{[1.5] = 'first'}", vec!["first"]),
  ] {
    let source = format!("return {{luau = {{globals = {table}}}}}");
    let mut config = Config::default();
    config.globals.push("old".into());
    if let Err(err) = extract_luau_config(&source, &mut config, None, InterruptCallbacks::default())
    {
      panic!("{table}: unexpected error {err}");
    }
    assert_eq!(config.globals, expected, "{table}");
  }
}

#[test]
fn test_invalid_globals_preserve_previous_value() {
  for (table, expected) in [
    (
      "{[0] = 'bad'}",
      "configuration array \"globals\" contains invalid numeric key",
    ),
    (
      "{[-1] = 'bad'}",
      "configuration array \"globals\" contains invalid numeric key",
    ),
    (
      "{[2] = 'bad'}",
      "configuration array \"globals\" contains invalid numeric key",
    ),
    (
      "{[math.huge] = 'bad'}",
      "configuration array \"globals\" contains invalid numeric key",
    ),
    (
      "{name = 'bad'}",
      "configuration array \"globals\" must only have numeric keys",
    ),
    (
      "{false}",
      "configuration value in \"globals\" table must be a string",
    ),
  ] {
    let source = format!("return {{luau = {{globals = {table}}}}}");
    let mut config = Config::default();
    config.globals.push("old".into());
    let err = extract_luau_config(&source, &mut config, None, InterruptCallbacks::default());
    assert_eq!(err.unwrap_err().to_string(), expected, "{table}");
    assert_eq!(config.globals, ["old"], "{table}");
  }
}

#[test]
fn test_yielded_configuration() {
  let msg = extract_err_message("coroutine.yield()");
  assert_eq!(msg, "configuration execution cannot yield");
}

#[test]
fn test_interrupt_execution() {
  use core::ffi::c_int;

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
  assert!(err.unwrap_err().to_string().contains("interrupted"));
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
    assert_eq!(
      extract_err_message(source),
      expected_error,
      "expected error for '{source}'"
    );
  }
}

/// `LintWarning::parseMask` 分支表对账：oracle `cpp/Config/src/LinterConfig.cpp:31-61`
/// （`compare(0,6,"nolint")` 前缀判定 / `find_first_not_of(" \t", 6)` 空白跳过 /
/// npos 全关 / 紧贴无效 / parseName Unknown 丢弃）。原用例用 `assert_ne!(mask, 0)`
/// 与逐位 `assert_ne!` 的弱断言，此处锁定完整掩码值。
#[test]
fn hotcomment_parse_mask_branch_table() {
  let shadow = Code::LocalShadow.mask_bit();
  let unused = Code::LocalUnused.mask_bit();

  // (header, content, 期望掩码)；!0 即 cpp 的 `return ~0ull` 全关哨兵
  let singles: &[(&str, bool, &str, u64)] = &[
    // cpp:39-44：裸 `nolint`（含其后仅空白）关闭全部警告
    ("bare", true, "nolint", !0),
    ("trailing_ws", true, "nolint  \t", !0),
    // cpp:50-54：具名单码
    ("named", true, "nolint LocalShadow", shadow),
    // cpp:47-48：`nolint` 后紧贴非空白 → 无效指令，不关任何东西
    ("glued", true, "nolintLocalShadow", 0),
    // cpp:39：不足 6 字节的前缀 compare 即不等，跳过
    ("short", true, "nolin", 0),
    // cpp:47-48：前缀命中但第 6 字节紧贴非空白（'X'），同「紧贴无效」丢弃
    ("glued_x", true, "nolintX", 0),
    // cpp compare 区分大小写
    ("cased", true, "NOLINT LocalShadow", 0),
    // cpp:50-54：parseName 取整段尾部，未知名（含多段拼接名）折为 Unknown 丢弃
    ("unknown", true, "nolint NoSuchLint", 0),
    ("two_words", true, "nolint LocalShadow LocalUnused", 0),
    // cpp:35-36：非 header 热注释不参与
    ("body", false, "nolint", 0),
  ];
  for (name, header, content, want) in singles {
    let hotcomments = vec![HotComment {
      header: *header,
      location: Location::default(),
      content: String::from(*content),
    }];
    assert_eq!(LintWarning::parse_mask(&hotcomments), *want, "case {name}");
  }

  // cpp 循环累加：多条具名注释按位或，全关哨兵短路优先
  let merged = vec![
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
  assert_eq!(LintWarning::parse_mask(&merged), shadow | unused);
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

// ---------- lint 规则取值表对账（cpp Config.cpp:110-176 parseLintRuleString） ----------

/// 拒绝面：未知规则名 / 非法取值 / 通配符 "*" 的包装文案，逐字对照 cpp。
#[test]
fn lint_rule_reject_table() {
  // (规则名, 设置值, compat?, 期望错误全文)
  let rejects = [
    // cpp:169-171：先解析规则名，未知即报（compat 与否同文案）
    ("NoSuchLint", "true", false, "Unknown lint NoSuchLint"),
    ("NoSuchLint", "true", true, "Unknown lint NoSuchLint"),
    // cpp:141-148 + :174：compat 下非法第三值 → enabled/disabled/fatal 文案
    (
      "SameLineStatement",
      "oops",
      true,
      "In key SameLineStatement: Bad setting 'oops'.  Valid options are enabled, disabled, and fatal",
    ),
    // cpp:152-154 + :174：非 compat 只认 true/false，compat 专值即非法值
    (
      "SameLineStatement",
      "enabled",
      false,
      "In key SameLineStatement: Bad setting 'enabled'.  Valid options are true and false",
    ),
    // cpp:156-164："*" 逐码套用，首个失败以 `In key *` 包装返回
    (
      "*",
      "oops",
      false,
      "In key *: Bad setting 'oops'.  Valid options are true and false",
    ),
  ];
  for (name, value, compat, want) in rejects {
    let mut config = Config::default();
    let opts = ConfigOptions {
      compat,
      alias_options: None,
    };
    let src = format!(r#"{{"lint": {{"{name}": "{value}"}}}}"#);
    let err = parse_config(&src, &mut config, &opts);
    assert_eq!(err.unwrap_err().to_string(), want, "src={src}");
  }
}

/// 接受面：五种取值 → (enabled_lint, fatal_lint) 位图对账（cpp:116-142）。
#[test]
fn lint_rule_value_bits_table() {
  // (设置值, 期望 enabled, 期望 fatal)；fatal_lint 默认全关（cpp 默认构造）
  let rows = [
    ("true", true, false),      // cpp:116-118
    ("false", false, false),    // cpp:119-122
    ("enabled", true, false),   // cpp:126-131：开 enabled 关 fatal
    ("disabled", false, false), // cpp:132-136：双关
    ("fatal", true, true),      // cpp:137-141：双开
  ];
  for (value, want_enabled, want_fatal) in rows {
    let mut config = Config::default();
    let opts = ConfigOptions {
      compat: true,
      alias_options: None,
    };
    let src = format!(r#"{{"lint": {{"SameLineStatement": "{value}"}}}}"#);
    parse_ok_src(&src, &mut config, &opts);
    assert_eq!(
      config.enabled_lint.is_enabled(Code::SameLineStatement),
      want_enabled,
      "value={value}"
    );
    assert_eq!(
      config.fatal_lint.is_enabled(Code::SameLineStatement),
      want_fatal,
      "value={value}"
    );
  }

  // cpp:156-162："*" 从 Code_Unknown 扫到 Code__Count——compat "fatal" 时
  // 全部码（含 Unknown 哨兵位）双开；逐码对账整张位图。
  let mut config = Config::default();
  let opts = ConfigOptions {
    compat: true,
    alias_options: None,
  };
  parse_ok_src(r#"{"lint": {"*": "fatal"}}"#, &mut config, &opts);
  for code in Code::iter() {
    assert!(
      config.enabled_lint.is_enabled(code) && config.fatal_lint.is_enabled(code),
      "cpp 的 for (code = Code_Unknown; code < Code__Count) 全量生效，{code:?} 例外"
    );
  }
}

/// `parse_ok` 的 ConfigOptions 版：断言 ok（用于 compat 行）。
fn parse_ok_src(source: &str, config: &mut Config, opts: &ConfigOptions) {
  if let Err(err) = parse_config(source, config, opts) {
    panic!("解析 {source} 期望无错误，实际 {err}");
  }
}
