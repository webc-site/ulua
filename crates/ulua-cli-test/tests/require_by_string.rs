use core::ffi::c_char;
extern crate alloc;

use alloc::{string::String, vec};
use core::{ffi::c_int, ptr::null};

use ulua_cli_lib::functions::{
  get_parent_path::get_parent_path, join_paths_file_utils::join_paths,
  normalize_path::normalize_path,
};
use ulua_cli_test::{
  enums::path_type::PathType, records::repl_with_path_fixture::ReplWithPathFixture,
};
use ulua_common::{
  dfflag::LuauSelfIsSelfAndAlwaysSelf,
  fflag::{
    DebugLuauUserDefinedClasses, DebugLuauUserDefinedClassesRuntime, LuauCyclicRequireShortCircuit,
    LuauExportValueSyntax,
  },
  functions::c_str::with_c_str,
  records::f_value::FValue,
};
use ulua_config::{
  functions::parse_config::parse_config,
  records::{
    alias_info::AliasInfo, alias_options::AliasOptions, config::Config,
    config_options::ConfigOptions,
  },
};
use ulua_repl_cli::functions::{
  create_cli_require_context::create_cli_require_context, repl_main::repl_main,
  require_config_init::require_config_init, run_code::run_code,
};
use ulua_require::functions::{
  luarequire_clearcache::luarequire_clearcache,
  luarequire_clearcacheentry::luarequire_clearcacheentry,
  luarequire_pushproxyrequire::luarequire_pushproxyrequire,
  luarequire_registermodule::luarequire_registermodule,
};
use ulua_vm::{
  enums::lua_type::LuaType,
  functions::{
    lua_call::lua_call, lua_getfield::lua_getfield, lua_l_findtable::lua_l_findtable,
    lua_pushcclosurek::lua_pushcclosurek, lua_pushstring::lua_pushstring,
    lua_settable::lua_settable, lua_type::lua_type,
  },
  macros::{
    lua_newtable::lua_newtable, lua_registryindex::LUA_REGISTRYINDEX, lua_setglobal::lua_setglobal,
  },
  records::lua_state::LuaState,
};

// `cpp/tests/RequireByString.test.cpp` 的移植：require-by-string 解析与导出测试。
//
// cpp 里绝大多数 TEST_CASE 是同一骨架（置 ScopedFastFlag → runProtectedRequire
// → assertOutputContainsAll），此处收口为三张表驱动的 `require_case`/
// `require_pair` 循环；断言片段与路径逐条对照 cpp 原文，仅压写法不减覆盖。

/// 断言 `_MODULES` 缓存表中 `key` 对应模块的存在性符合 `present`
///
/// 对齐 cpp 中重复出现的缓存检查样板（`luaL_findtable` + `lua_getfield` + CHECK）
/// NUL 结尾字节串（`b"name\0"`）→ `*const c_char` 的唯一收口点（review.md §10：
/// C 字符串字面量（c 前缀）豁免于 C ABI 实现层，测试代码不在豁免内）。
fn cstr(bytes: &'static [u8]) -> *const c_char {
  bytes.as_ptr().cast()
}

fn assert_module_cache(l: *mut LuaState, key: &str, present: bool, context: &str) {
  // Safety: `l` 指向 fixture 初始化完成的主线程；findtable/getfield 的栈操作配平。
  // `key` 经 `with_c_str` 门面即时补 NUL（review.md §10：不散落 `CString`），
  // `lua_getfield` 走 `lua_s_new` 当场入 intern 表、不保存该指针。
  unsafe {
    lua_l_findtable(l, LUA_REGISTRYINDEX, cstr(b"_MODULES\0"), 1);
    with_c_str(key.as_bytes(), |key| lua_getfield(l, -1, key));
    let cached = lua_type(l, -1) != LuaType::Nil as c_int;
    assert!(cached == present, "{context}");
  }
}

/// 标准两态文案的便捷包装：`key` 已在 `_MODULES` 缓存中（cpp "did not contain" 断言）。
fn assert_cached(l: *mut LuaState, key: &str) {
  assert_module_cache(l, key, true, CACHE_HIT_MSG);
}

/// 标准两态文案的便捷包装：`key` 尚不在 `_MODULES` 缓存中（cpp "already contained" 断言）。
fn assert_not_cached(l: *mut LuaState, key: &str) {
  assert_module_cache(l, key, false, CACHE_MISS_MSG);
}

/// 缓存存在性断言的标准 cpp 文案（`assert_module_cache` 的 `context` 参数）。
const CACHE_HIT_MSG: &str = "Cache did not contain module result";
const CACHE_MISS_MSG: &str = "Cache already contained module result";

/// 在主线程上执行 Luau 源码（对齐 cpp 的 `runCode(L, src)`）
fn run_code_str(l: *mut LuaState, src: &str) {
  // Safety: `l` 指向 fixture 初始化完成的主线程，`src` 为合法 UTF-8 源码。
  let _ = unsafe { run_code(l, src) };
}

/// 注册运行时模块 `@test/helloworld`（hello = "world"）供 require 读取
fn register_test_module(l: *mut LuaState) {
  // Safety: `l` 指向 fixture 初始化完成的主线程；pushcclosurek/pushstring/newtable/settable/call 的栈序列配平。
  unsafe {
    lua_pushcclosurek(l, Some(luarequire_registermodule), null(), 0, None);
    lua_pushstring(l, cstr(b"@test/helloworld\0"));
    lua_newtable(l);
    lua_pushstring(l, cstr(b"hello\0"));
    lua_pushstring(l, cstr(b"world\0"));
    lua_settable(l, -3);
    lua_call(l, 2, 0);
  }
}

/// cpp `ScopedFastFlag{...}` 一族的通用 RAII guard：构造期按序 push 覆盖，
/// Drop 期逆序 pop（对齐 cpp 栈式作用域）。本文件全部 flag 组合共用此单点。
struct ScopedFlags(&'static [&'static FValue<bool>]);

impl ScopedFlags {
  fn push(flags: &'static [&'static FValue<bool>]) -> ScopedFlags {
    for flag in flags {
      flag.push_test_override(true);
    }
    ScopedFlags(flags)
  }
}

impl Drop for ScopedFlags {
  fn drop(&mut self) {
    for flag in self.0.iter().rev() {
      flag.pop_test_override();
    }
  }
}

/// 空 flag 组合：无需覆盖的用例统一走 `ScopedFlags` 通道，表列类型保持一致
fn sff_none() -> ScopedFlags {
  ScopedFlags::push(&[])
}

/// `ScopedFastFlag{LuauExportValueSyntax, true}` 的 RAII guard，Drop 时弹出覆盖
fn sff_export_value() -> ScopedFlags {
  // 字面量数组在此处是 const 初值，rvalue static promotion 使其升为 'static。
  const FLAGS: &[&FValue<bool>] = &[&LuauExportValueSyntax];
  ScopedFlags::push(FLAGS)
}

/// cpp cyclic require 用例的双 flag 组合 `{LuauCyclicRequireShortCircuit,
/// LuauExportValueSyntax}` 的 RAII guard
fn sff_cyclic_short_circuit() -> ScopedFlags {
  const FLAGS: &[&FValue<bool>] = &[&LuauCyclicRequireShortCircuit, &LuauExportValueSyntax];
  ScopedFlags::push(FLAGS)
}

/// cpp `ScopedFastFlag{DFFlag::LuauSelfIsSelfAndAlwaysSelf, true}` 的 RAII guard
fn sff_self_is_self() -> ScopedFlags {
  const FLAGS: &[&FValue<bool>] = &[&LuauSelfIsSelfAndAlwaysSelf];
  ScopedFlags::push(FLAGS)
}

/// 用户自定义类三 flag 组合的 RAII guard（cpp 的 ScopedFastFlag 数组移植）
fn sff_user_defined_classes() -> ScopedFlags {
  const FLAGS: &[&FValue<bool>] = &[
    &LuauExportValueSyntax,
    &DebugLuauUserDefinedClasses,
    &DebugLuauUserDefinedClassesRuntime,
  ];
  ScopedFlags::push(FLAGS)
}

/// `get_luau_directory(Relative) + "/tests/require/..."` 两行样板的文件内单点收口
fn req_rel(sub: &str) -> String {
  ReplWithPathFixture::get_luau_directory(PathType::Relative) + sub
}

/// 原 `ulua_cli_lib::functions::resolve_path`（生产代码零消费者，已死删）的测试侧
/// 内联：get_parent_path + join_paths + normalize_path 三步组合，行为逐行等同。
fn resolve_path(path: &str, base_file_path: &str) -> Option<String> {
  let base_file_path_parent = get_parent_path(base_file_path)?;
  let joined = join_paths(&base_file_path_parent, path, false);
  Some(normalize_path(&joined))
}

/// 同构用例的统一骨架（cpp 各 TEST_CASE 的 `runProtectedRequire` +
/// `assertOutputContainsAll` 两步）：置 flag 组合 → 新夹具 → require 相对路径
/// （`/tests/require` 下的 `sub` 子路径）→ 逐片段断言输出。
///
/// 断言与 `assert_output_contains_all` 语义相同（每个片段都 `contains`），
/// 失败文案额外带上 `sub` 以定位表行。
fn require_case(flags: fn() -> ScopedFlags, sub: &str, expected: &[&str]) {
  let _sff = flags();
  let mut fixture = ReplWithPathFixture::new();
  let path = req_rel(&format!("/tests/require{sub}"));
  fixture.run_protected_require(&path);
  let out = fixture.get_captured_output();
  for frag in expected {
    assert!(
      out.contains(frag),
      "用例 {sub} 期望片段 {frag:?}，实际输出: {out}"
    );
  }
}

/// 同构「双配置根」用例的统一骨架：cpp 里同一 TEST_CASE 先后对
/// `with_config`（`.luaurc`）与 `with_config_luau`（`.config.luau`）两套夹具
/// 跑同一 require，且共用同一夹具状态（模块缓存跨两次 require），此处逐例保持。
fn require_pair(flags: fn() -> ScopedFlags, suffix: &str, expected: &[&str]) {
  let _sff = flags();
  let mut fixture = ReplWithPathFixture::new();
  for root in [
    "/config_tests/with_config",
    "/config_tests/with_config_luau",
  ] {
    let path = req_rel(&format!("/tests/require{root}{suffix}"));
    fixture.run_protected_require(&path);
    let out = fixture.get_captured_output();
    for frag in expected {
      assert!(
        out.contains(frag),
        "用例 {root}{suffix} 期望片段 {frag:?}，实际输出: {out}"
      );
    }
  }
}

// ---------- 以下为 cpp 各 TEST_CASE 组的表驱动收口 ----------

/// export 关键字用例组（`ScopedFastFlag{LuauExportValueSyntax, true}`）：
/// 路径均在 `/tests/require/without_config/export_keyword/` 下。
/// 成功用例只断言输出含 `"true"`；报错用例断言 (模块名, 报错文案)。
#[test]
fn require_by_string_export_keyword() {
  const ROOT: &str = "/without_config/export_keyword";
  let ok = [
    "export_as_function",
    "require_export_alias",
    "require_export_alias2",
    "require_export_counter_module",
    // Source: tests/RequireByString.test.cpp:1042（RequireExportCompound，复合赋值的 export）
    "require_export_compound",
    "require_export_post_return_mutation_error",
    "require_export_edge_cases",
    "require_export_forward_rebind",
    "require_export_freeze_local_nil_error",
    "require_export_freeze_shadowing",
    "require_export_frozen",
    "require_export_frozen_mutate",
    "require_export_function",
    "require_export_function_rebind",
    "require_export_internal_call",
    "require_export_mixed",
    "require_export_multi_assign",
    "require_export_multi_swap",
    "require_export_multi_var",
    "require_export_mutual_recursion",
    "require_export_nested_table",
    "require_export_shadowing",
    "require_export_trap",
    "require_export_type_with_return",
    "require_export_upvalue",
    "require_export_value",
  ];
  for name in ok {
    require_case(sff_export_value, &format!("{ROOT}/{name}"), &["true"]);
  }

  // `export` 只允许出现在顶层语句：7 个位置各一例，文案相同
  let top_level_only = [
    "export_in_do_block_error",
    "export_in_elseif_error",
    "export_in_for_error",
    "export_in_function_error",
    "export_in_if_error",
    "export_in_repeat_error",
    "export_in_while_error",
  ];
  for name in top_level_only {
    require_case(
      sff_export_value,
      &format!("{ROOT}/{name}"),
      &["'export' may only be applied to top-level statements"],
    );
  }

  let errs = [
    (
      "export_const_error",
      "Variable 'foo' is constant and may not be reassigned",
    ),
    (
      "export_with_return_error",
      "Exporting values is not compatible with top-level return",
    ),
  ];
  for (name, msg) in errs {
    require_case(sff_export_value, &format!("{ROOT}/{name}"), &[msg]);
  }
}

/// 用户自定义类用例组（cpp `ScopedFastFlag` 三 flag 组合；ClassRuntimeErrors
/// 两例移自 `cpp/tests/ClassRuntimeErrors.test.cpp`，feedback/cost-model 相关
/// flag 在 Rust 端不生效，略去）。成功用例在 export_keyword 下，报错用例在
/// without_config 根下。
#[test]
fn require_by_string_user_defined_classes() {
  let ok = [
    "require_export_class",
    "require_export_class_child_without_parent",
    "require_export_class_both_exported",
    "require_export_class_multi_level",
  ];
  for name in ok {
    require_case(
      sff_user_defined_classes,
      &format!("/without_config/export_keyword/{name}"),
      &["true"],
    );
  }

  let errs = [
    (
      "class_extends_non_open_parent",
      "Non-open class 'Parent' cannot be extended",
    ),
    (
      "class_override_instance_member_error",
      "Cannot override instance member 'x' of parent class 'Parent' in child class 'Child'",
    ),
  ];
  for (name, msg) in errs {
    require_case(
      sff_user_defined_classes,
      &format!("/without_config/{name}"),
      &[msg],
    );
  }
}

/// cyclic require 用例组（`{LuauCyclicRequireShortCircuit, LuauExportValueSyntax}`），
/// 对齐 `RequireByString.test.cpp:654,962-995`：cyclic 链路（VM `lua_usesexport`、
/// Require `createPlaceholder`）已移植。`Some(msg)` 表示期望 `false` + 报错文案。
#[test]
fn require_by_string_cyclic() {
  let cases: &[(&str, Option<&str>)] = &[
    // 两个模块都用 export 关键字：编译器把运行时提供的占位表当导出表，环自动解掉
    ("cyclic_requirer", None),
    (
      "cyclic_access_a",
      Some("Cannot access the exported field 'Tree'"),
    ),
    (
      "cyclic_mutation_b",
      Some("Cannot set the exported field 'foo'"),
    ),
    (
      "cyclic_access_nonstringkey_a",
      Some("Cannot access the exported field 'unknown'"),
    ),
  ];
  for (name, err) in cases {
    let expected = match err {
      // 成功：只断言 pcall 未报错
      None => vec!["true"],
      Some(msg) => vec!["false", msg],
    };
    require_case(
      sff_cyclic_short_circuit,
      &format!("/without_config/{name}"),
      &expected,
    );
  }
}

/// 无 flag 的 without_config / config_tests 常规用例组（路径为
/// `/tests/require` 下的子路径，含类型导出、init 解析与各类报错）
#[test]
fn require_by_string_without_config() {
  let cases: &[(&str, &[&str])] = &[
    (
      "/config_tests/config_ambiguity/requirer",
      &[
        "false",
        "could not resolve alias \"dep\" (ambiguous configuration file)",
      ],
    ),
    (
      "/config_tests/config_cannot_be_required/requirer",
      &["false", "could not resolve child component \".config\""],
    ),
    (
      "/without_config/ambiguous_directory_requirer",
      &[
        "false",
        "error requiring module \"./ambiguous/directory/dependency\": could not resolve child component \"dependency\" (ambiguous)",
      ],
    ),
    (
      "/without_config/ambiguous_file_requirer",
      &[
        "false",
        "error requiring module \"./ambiguous/file/dependency\": could not resolve child component \"dependency\" (ambiguous)",
      ],
    ),
    (
      "/without_config/dependency",
      &["true", "result from dependency"],
    ),
    ("/without_config/lua", &["true", "result from init.lua"]),
    (
      "/without_config/lua_dependency",
      &["true", "result from lua_dependency"],
    ),
    ("/without_config/luau", &["true", "result from init.luau"]),
    (
      "/without_config/module",
      &["true", "result from dependency", "required into module"],
    ),
    ("/without_config/nested", &["true", "result from submodule"]),
    // `init` 组件不可直接 require（cpp CannotRequireInitLuauDirectly）
    (
      "/without_config/nested/init",
      &["false", "could not resolve child component \"init\""],
    ),
    (
      "/without_config/nested_inits_requirer",
      &[
        "true",
        "result from nested_inits/init",
        "required into module",
      ],
    ),
    (
      "/without_config/nested_module_requirer",
      &["true", "result from submodule"],
    ),
    ("/without_config/types/boolean", &["true", "false"]),
    ("/without_config/types/buffer", &["true", "buffer"]),
    ("/without_config/types/function", &["true", "function"]),
    ("/without_config/types/nil", &["true", "nil"]),
    ("/without_config/types/number", &["true", "12345"]),
    ("/without_config/types/string", &["true", "\"foo\""]),
    (
      "/without_config/types/table",
      &["true", "{\"foo\", \"bar\"}"],
    ),
    ("/without_config/types/thread", &["true", "thread"]),
    ("/without_config/types/userdata", &["true", "userdata"]),
    ("/without_config/types/vector", &["true", "1, 2, 3"]),
  ];
  for (sub, expected) in cases {
    require_case(sff_none, sub, expected);
  }
}

/// 字面量非法入参用例组（不走 `req_rel`，require 入参即表中的原文）
#[test]
fn require_by_string_invalid_inputs() {
  // (入参, 期望片段)：前四条是 cpp AliasHasIllegalFormat 的四个子检查
  let cases = [
    ("@@", &["false", "@@ is not a valid alias"][..]),
    ("@.", &["false", ". is not a valid alias"]),
    ("@..", &["false", ".. is not a valid alias"]),
    ("@", &["false", " is not a valid alias"]),
    // 不存在的别名（cpp RequireAliasThatDoesNotExist）
    (
      "@this.alias.does.not.exist",
      &["false", "@this.alias.does.not.exist is not a valid alias"],
    ),
    // 绝对 / 无前缀路径共用文案（cpp RequireAbsolutePath / RequireUnprefixedPath）
    (
      "/an/absolute/path",
      &[
        "false",
        "require path must start with a valid prefix: ./, ../, or @",
      ],
    ),
    (
      "an/unprefixed/path",
      &[
        "false",
        "require path must start with a valid prefix: ./, ../, or @",
      ],
    ),
  ];
  for (input, expected) in cases {
    let _sff = sff_none();
    let mut fixture = ReplWithPathFixture::new();
    fixture.run_protected_require(input);
    let out = fixture.get_captured_output();
    for frag in expected {
      assert!(
        out.contains(frag),
        "入参 {input} 期望片段 {frag:?}，实际输出: {out}"
      );
    }
  }
}

/// 双配置根用例表行的类型：(flag 组合, 去掉配置根后的模块路径, 期望输出片段)
type PairRow = (fn() -> ScopedFlags, &'static str, &'static [&'static str]);

/// 双配置根用例组：`with_config`（JSON）与 `with_config_luau`（Luau）各跑一遍，
/// 同一夹具内先后 require（对齐 cpp 每个 TEST_CASE 的两个块）。
#[test]
fn require_by_string_config_pairs() {
  let cases: &[PairRow] = &[
    (
      sff_none,
      "/chained_aliases/subdirectory/failing_requirer_cyclic",
      &[
        "false",
        "error requiring module \"@cyclicentry\": detected alias cycle (@cyclic1 -> @cyclic2 -> @cyclic3 -> @cyclic1)",
      ],
    ),
    (
      sff_none,
      "/chained_aliases/subdirectory/failing_requirer_inner_dependency",
      &[
        "false",
        "error requiring module \"@dependoninner\": @passthroughinner is not a valid alias",
      ],
    ),
    (
      sff_none,
      "/chained_aliases/subdirectory/failing_requirer_missing",
      &[
        "false",
        "error requiring module \"@brokenchain\": @missing is not a valid alias",
      ],
    ),
    (
      sff_none,
      "/chained_aliases/subdirectory/successful_requirer",
      &[
        "true",
        "result from inner_dependency",
        "result from outer_dependency",
      ],
    ),
    (
      sff_none,
      "/src/alias_requirer",
      &["true", "result from dependency"],
    ),
    (
      sff_none,
      "/src/directory_alias_requirer",
      &["true", "result from subdirectory_dependency"],
    ),
    (
      sff_none,
      "/src/parent_alias_requirer",
      &["true", "result from other_dependency"],
    ),
    // cpp RequireSubmoduleUsingSelfWithOverrideAttempt：DFFlag LuauSelfIsSelfAndAlwaysSelf
    (
      sff_self_is_self,
      "/nested_override",
      &["true", "result from submodule"],
    ),
  ];
  for (flags, suffix, expected) in cases {
    require_pair(*flags, suffix, expected);
  }
}
// ---------- 缓存行为（结构各异，独立成测） ----------

/// require 前后 `_MODULES` 缓存键的命中变化：(模块子路径, 缓存键后缀, 输出标记)
#[test]
fn require_by_string_check_cache_after_require() {
  let cases: &[(&str, &str, &[&str])] = &[
    (
      "/without_config/lua",
      "/init.lua",
      &["true", "result from init.lua"],
    ),
    (
      "/without_config/luau",
      "/init.luau",
      &["true", "result from init.luau"],
    ),
    (
      "/without_config/module",
      ".luau",
      &["true", "result from dependency", "required into module"],
    ),
  ];
  for (sub, key_suffix, markers) in cases {
    let mut fixture = ReplWithPathFixture::new();
    let l = fixture.l();
    let cache_key = ReplWithPathFixture::get_luau_directory(PathType::Absolute)
      + "/tests/require"
      + sub
      + key_suffix;

    assert_not_cached(l, &cache_key);
    fixture.run_protected_require(&req_rel(&format!("/tests/require{sub}")));
    let out = fixture.get_captured_output();
    for frag in *markers {
      assert!(
        out.contains(frag),
        "用例 {sub} 期望片段 {frag:?}，实际输出: {out}"
      );
    }
    assert_cached(l, &cache_key);
  }
}

/// `.luau` 未命中前置断言 + require `.lua` 后命中的交叉形态（cpp CheckCacheAfterRequireLua）
#[test]
fn require_by_string_check_cache_after_require_lua() {
  let mut fixture = ReplWithPathFixture::new();
  let l = fixture.l();
  let abs = ReplWithPathFixture::get_luau_directory(PathType::Absolute)
    + "/tests/require/without_config/lua_dependency";

  let key = format!("{abs}.luau");
  assert_not_cached(l, &key);

  fixture.run_protected_require(&req_rel("/tests/require/without_config/lua_dependency"));
  fixture.assert_output_contains_all(&["true", "result from lua_dependency"]);

  let key = format!("{abs}.lua");
  assert_cached(l, &key);
}

#[test]
fn require_by_string_check_cached_result() {
  let mut fixture = ReplWithPathFixture::new();
  let path = req_rel("/tests/require/without_config/validate_cache");
  fixture.run_protected_require(&path);
  fixture.assert_output_contains_all(&["true"]);
}

#[test]
fn require_by_string_check_clear_cache() {
  let mut fixture = ReplWithPathFixture::new();
  let l = fixture.l();
  let cache_key = ReplWithPathFixture::get_luau_directory(PathType::Absolute)
    + "/tests/require/without_config/module.luau";

  assert_not_cached(l, &cache_key);

  fixture.run_protected_require(&req_rel("/tests/require/without_config/module"));
  fixture.assert_output_contains_all(&["true", "result from dependency", "required into module"]);

  assert_cached(l, &cache_key);

  // Safety: `l` 为 fixture 主线程；pushcclosurek + call 栈序列配平。
  unsafe {
    lua_pushcclosurek(l, Some(luarequire_clearcache), null(), 0, None);
    lua_call(l, 0, 0);
  }

  assert_module_cache(l, &cache_key, false, "Cache was not cleared");
}

#[test]
fn require_by_string_check_clear_cache_entry() {
  let mut fixture = ReplWithPathFixture::new();
  let l = fixture.l();
  let cache_key = ReplWithPathFixture::get_luau_directory(PathType::Absolute)
    + "/tests/require/without_config/module.luau";

  assert_not_cached(l, &cache_key);

  fixture.run_protected_require(&req_rel("/tests/require/without_config/module"));
  fixture.assert_output_contains_all(&["true", "result from dependency", "required into module"]);

  assert_cached(l, &cache_key);

  // Safety: `l` 为 fixture 主线程；pushcclosurek/pushstring/call 栈序列配平。
  unsafe {
    lua_pushcclosurek(l, Some(luarequire_clearcacheentry), null(), 0, None);
  }
  // `cache_key` 经 `with_c_str` 门面即时补 NUL（review.md §10）：`lua_pushstring`
  // 走 `lua_s_new` 当场入 intern 表、不保存该指针。
  with_c_str(cache_key.as_bytes(), |key| unsafe {
    lua_pushstring(l, key);
  });
  // Safety: 同上。
  unsafe {
    lua_call(l, 1, 0);
  }

  assert_module_cache(l, &cache_key, false, "Cache was not cleared");
}

// ---------- 其余结构各异的独立用例 ----------

#[test]
fn require_by_string_load_string_relative() {
  let mut fixture = ReplWithPathFixture::new();
  let l = fixture.l();
  run_code_str(
    l,
    "return pcall(function() return loadstring(\"require('a/relative/path')\")() end)",
  );
  fixture.assert_output_contains_all(&["false", "require is not supported in this context"]);
}

#[test]
fn require_by_string_parse_aliases() {
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
  assert!(error.is_ok(), "{error:?}");

  check_contents(&config);

  let copy_constructed_config = config.clone();
  check_contents(&copy_constructed_config);

  let mut copy_assigned_config = Config::default();
  copy_assigned_config.clone_from(&config);
  check_contents(&copy_assigned_config);
}

#[test]
fn require_by_string_path_normalization() {
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
  let mut fixture = ReplWithPathFixture::new();
  let l = fixture.l();

  // Safety: `l` 为 fixture 主线程；create/pushproxyrequire/setglobal 栈序列配平。
  unsafe {
    let ctx = create_cli_require_context(l);
    luarequire_pushproxyrequire(l, Some(require_config_init), ctx);
    lua_setglobal(l, cstr(b"proxyrequire\0"));
  }

  let path = req_rel("/tests/require/without_config/proxy_requirer");
  fixture.run_protected_require(&path);
  fixture.assert_output_contains_all(&[
    "true",
    "result from dependency",
    "required into proxy_requirer",
  ]);
}

/// 运行时注册模块后 require 读取：原大小写与大小写不敏感两种拼法（cpp
/// RegisterRuntimeModule / RegisterRuntimeModuleCaseInsensitive）
#[test]
fn require_by_string_register_runtime_module() {
  let cases = [
    "@test/helloworld",
    // 别名解析先转小写：@TeSt/heLLoWoRld 等价 @test/helloworld
    "@TeSt/heLLoWoRld",
  ];
  for module in cases {
    let mut fixture = ReplWithPathFixture::new();
    let l = fixture.l();

    register_test_module(l);

    run_code_str(l, &format!("return require('{module}').hello == 'world'"));
    fixture.assert_output_contains_all(&["true"]);
  }
}

#[test]
fn require_by_string_require_from_luau_binary() {
  // cpp 的 TEST_CASE_FIXTURE 会构造 ReplWithPathFixture；本例只走 replMain
  // （它自建状态），构造仅为对齐上游前置条件。
  let _fixture = ReplWithPathFixture::new();

  let dir_rel = || ReplWithPathFixture::get_luau_directory(PathType::Relative);
  let dir_abs = || ReplWithPathFixture::get_luau_directory(PathType::Absolute);

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

#[test]
fn require_by_string_require_simple_relative_path_within_pcall() {
  let mut fixture = ReplWithPathFixture::new();
  let path = req_rel("/tests/require/without_config/dependency");
  let code: String = format!("return pcall(require, \"{}\")", path);
  run_code_str(fixture.l(), &code);
  fixture.assert_output_contains_all(&["true", "result from dependency"]);
}

#[test]
fn require_by_string_require_with_ambiguity_in_alias_discovery() {
  // 同 RequireFromLuauBinary：只用 replMain，构造夹具仅为对齐 cpp 的
  // TEST_CASE_FIXTURE 前置。
  let _fixture = ReplWithPathFixture::new();

  let paths: vec::Vec<String> = vec![
    req_rel("/tests/require/config_tests/with_config/parent_ambiguity/folder/requirer.luau"),
    ReplWithPathFixture::get_luau_directory(PathType::Absolute)
      + "/tests/require/config_tests/with_config/parent_ambiguity/folder/requirer.luau",
    req_rel("/tests/require/config_tests/with_config_luau/parent_ambiguity/folder/requirer.luau"),
    ReplWithPathFixture::get_luau_directory(PathType::Absolute)
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
