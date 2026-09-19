//! Require 模块导航纯逻辑测试，对照 C++ `Require/src`（RequireNavigator.cpp、
//! Navigation.cpp、AliasCycleTracker.cpp、PathUtilities.cpp）的行为与错误消息。
//!
//! 路径/别名一律按字节处理（cpp 为 `std::string`），故断言也用字节串，
//! 并专门覆盖非 UTF-8 字节不被替换为 U+FFFD 的语义。

use alloc::{
  string::{String, ToString},
  vec::Vec,
};

use crate::{
  enums::{
    config_behavior::ConfigBehavior, config_status::ConfigStatus, navigate_result::NavigateResult,
    path_type::PathType, status_require_navigator::Status,
  },
  functions::{extract_alias::extract_alias, get_path_type::get_path_type, split_path::split_path},
  records::{
    alias_cycle_tracker::AliasCycleTracker, error_handler::ErrorHandler,
    navigation_context::NavigationContextTrait, navigator::Navigator,
    resolved_require::ResolvedRequire, runtime_error_handler::RuntimeErrorHandler,
  },
};

/// 可编程导航上下文 mock：记录操作序列、返回预设结果。
struct MockContext {
  reset: NavigateResult,
  jump: NavigateResult,
  parent: NavigateResult,
  // parent 为 Success 时最多返回 Success 的次数，之后 NotFound（模拟到达根）
  parent_successes: usize,
  parent_calls: usize,
  child: NavigateResult,
  alias_override: NavigateResult,
  alias_fallback: NavigateResult,
  status: ConfigStatus,
  behavior: ConfigBehavior,
  alias_value: Option<Vec<u8>>,
  config_value: Option<Vec<u8>>,
  ops: Vec<String>,
  /// 传给上下文的原始字节，用于断言字节语义（组件 / 别名 / 跳转路径）
  last_component: Option<Vec<u8>>,
  last_alias: Option<Vec<u8>>,
}

impl MockContext {
  fn new() -> Self {
    Self {
      reset: NavigateResult::Success,
      jump: NavigateResult::Success,
      parent: NavigateResult::Success,
      // 默认 2 次：相对路径 "./.." 需要两次成功 parent
      parent_successes: 2,
      parent_calls: 0,
      child: NavigateResult::Success,
      alias_override: NavigateResult::NotFound,
      alias_fallback: NavigateResult::NotFound,
      status: ConfigStatus::Absent,
      behavior: ConfigBehavior::GetAlias,
      alias_value: None,
      config_value: None,
      ops: Vec::new(),
      last_component: None,
      last_alias: None,
    }
  }

  /// 执行一次 require 导航，返回 (操作序列, 报告的错误字节)。
  fn navigate(&mut self, path: &[u8]) -> (Vec<String>, Option<Vec<u8>>) {
    let mut handler = TestErrorHandler::default();
    let mut navigator = Navigator::new(self, &mut handler);
    navigator.navigate(path);
    (self.ops.clone(), handler.message)
  }
}

impl NavigationContextTrait for MockContext {
  fn reset_to_requirer(&mut self) -> NavigateResult {
    self.ops.push("reset".to_string());
    self.reset
  }

  fn jump_to_alias(&mut self, path: &[u8]) -> NavigateResult {
    self.ops.push("jump".to_string());
    self.last_alias = Some(path.to_vec());
    self.jump
  }

  fn to_alias_override(&mut self, alias_unprefixed: &[u8]) -> NavigateResult {
    self.ops.push(format!(
      "override:{}",
      String::from_utf8_lossy(alias_unprefixed)
    ));
    self.last_alias = Some(alias_unprefixed.to_vec());
    self.alias_override
  }

  fn to_alias_fallback(&mut self, alias_unprefixed: &[u8]) -> NavigateResult {
    self.ops.push(format!(
      "fallback:{}",
      String::from_utf8_lossy(alias_unprefixed)
    ));
    self.last_alias = Some(alias_unprefixed.to_vec());
    self.alias_fallback
  }

  fn to_parent(&mut self) -> NavigateResult {
    self.ops.push("parent".to_string());
    if self.parent != NavigateResult::Success {
      return self.parent;
    }
    // 有限次 Success 后返回 NotFound，模拟到达根（避免无限向上）
    if self.parent_calls < self.parent_successes {
      self.parent_calls += 1;
      NavigateResult::Success
    } else {
      NavigateResult::NotFound
    }
  }

  fn to_child(&mut self, component: &[u8]) -> NavigateResult {
    self
      .ops
      .push(format!("child:{}", String::from_utf8_lossy(component)));
    self.last_component = Some(component.to_vec());
    self.child
  }

  fn get_config_status(&self) -> ConfigStatus {
    self.status
  }

  fn get_config_behavior(&self) -> ConfigBehavior {
    self.behavior
  }

  fn get_alias(&self, _alias: &[u8]) -> Option<Vec<u8>> {
    self.alias_value.clone()
  }

  fn get_config(&self) -> Option<Vec<u8>> {
    self.config_value.clone()
  }
}

/// 记录报告错误的 ErrorHandler。
#[derive(Default)]
struct TestErrorHandler {
  message: Option<Vec<u8>>,
}

impl ErrorHandler for TestErrorHandler {
  fn report_error(&mut self, message: Vec<u8>) {
    self.message = Some(message);
  }
}

fn ops(values: &[&str]) -> Vec<String> {
  values.iter().map(|s| String::from(*s)).collect()
}

/// 字节消息断言辅助：把期望的 ASCII 文案转成字节。
fn msg(message: &[u8]) -> Option<Vec<u8>> {
  Some(message.to_vec())
}

// ---------- PathUtilities.cpp 对应 ----------

#[test]
fn path_type_prefixes() {
  assert_eq!(get_path_type(b"./a"), PathType::RelativeToCurrent);
  assert_eq!(get_path_type(b"../a"), PathType::RelativeToParent);
  assert_eq!(get_path_type(b"@a"), PathType::Aliased);
  assert_eq!(get_path_type(b"a"), PathType::Unsupported);
  assert_eq!(get_path_type(b""), PathType::Unsupported);
  assert_eq!(get_path_type(b"."), PathType::Unsupported);
  assert_eq!(get_path_type(b".."), PathType::Unsupported);
  assert_eq!(get_path_type(b"@"), PathType::Aliased);
}

#[test]
fn split_path_components() {
  assert_eq!(split_path(b"a/b"), (&b"a"[..], &b"b"[..]));
  assert_eq!(split_path(b"ab"), (&b"ab"[..], &[] as &[u8]));
  assert_eq!(split_path(b"a/b/c"), (&b"a"[..], &b"b/c"[..]));
  assert_eq!(split_path(b"/x"), (&b""[..], &b"x"[..]));
  assert_eq!(split_path(b""), (&b""[..], &[] as &[u8]));
  // 非 UTF-8 字节不影响切分位置
  assert_eq!(split_path(b"a/\xff\xfe"), (&b"a"[..], &b"\xff\xfe"[..]));
}

#[test]
fn extract_alias_values() {
  // '@' 前缀被忽略；有分隔符时取别名到分隔符，否则取整段
  assert_eq!(extract_alias(b"@foo/bar"), &b"foo"[..]);
  assert_eq!(extract_alias(b"@foo"), &b"foo"[..]);
  assert_eq!(extract_alias(b"@/"), &b""[..]);
  assert_eq!(extract_alias(b"@a//b"), &b"a"[..]);
  assert_eq!(extract_alias(b"@Foo/BAR"), &b"Foo"[..]);
  // 别名字节原样返回：非法 UTF-8 不被替换
  assert_eq!(extract_alias(b"@\xff\xfe/x"), &b"\xff\xfe"[..]);
}

// ---------- AliasCycleTracker.cpp 对应 ----------

#[test]
fn alias_cycle_tracker_detects_cycle() {
  let mut tracker = AliasCycleTracker::new();
  assert_eq!(tracker.add(b"a".to_vec()), None);
  assert_eq!(
    tracker.add(b"a".to_vec()),
    msg(b"detected alias cycle (@a -> @a)")
  );
}

#[test]
fn alias_cycle_tracker_excludes_prefix_and_preserves_order() {
  let mut tracker = AliasCycleTracker::new();
  let repeated = "模块".as_bytes().to_vec();
  for alias in [b"prefix".to_vec(), repeated.clone(), b"tail".to_vec()] {
    assert_eq!(tracker.add(alias), None);
  }
  // 环从重复出现的那个别名开始拼接，前缀 "prefix" 被跳过；每个别名前都带 '@'
  let expected =
    msg(b"detected alias cycle (@\xe6\xa8\xa1\xe5\x9d\x97 -> @tail -> @\xe6\xa8\xa1\xe5\x9d\x97)");
  for _ in 0..2 {
    assert_eq!(tracker.add(repeated.clone()), expected);
  }
  assert_eq!(tracker.add(b"next".to_vec()), None);
  assert_eq!(
    tracker.add(b"tail".to_vec()),
    msg(b"detected alias cycle (@tail -> @next -> @tail)")
  );
}

#[test]
fn alias_cycle_tracker_multi_level() {
  let mut tracker = AliasCycleTracker::new();
  assert_eq!(tracker.add(b"a".to_vec()), None);
  assert_eq!(tracker.add(b"b".to_vec()), None);
  assert_eq!(
    tracker.add(b"a".to_vec()),
    msg(b"detected alias cycle (@a -> @b -> @a)")
  );
}

#[test]
fn alias_cycle_tracker_keeps_non_utf8_alias_bytes() {
  // 非 UTF-8 别名字节参与环检测与消息拼装时必须原样保留
  let mut tracker = AliasCycleTracker::new();
  assert_eq!(tracker.add(b"\xff".to_vec()), None);
  assert_eq!(
    tracker.add(b"\xff".to_vec()),
    msg(b"detected alias cycle (@\xff -> @\xff)")
  );
}

// ---------- Navigation.cpp 对应 ----------

#[test]
fn runtime_error_handler_prefixes_message() {
  let mut handler = RuntimeErrorHandler::new(b"foo/bar");
  handler.report_error(b"boom");
  assert_eq!(
    handler.get_reported_error(),
    &b"error requiring module \"foo/bar\": boom"[..]
  );
}

#[test]
fn runtime_error_handler_keeps_non_utf8_path_bytes() {
  // 路径含非 UTF-8 字节时前缀必须逐字节保留（cpp 为 std::string 拼接）
  let mut handler = RuntimeErrorHandler::new(b"\xffpath");
  handler.report_error(b"bad \xfe");
  assert_eq!(
    handler.get_reported_error(),
    &b"error requiring module \"\xffpath\": bad \xfe"[..]
  );
}

// ---------- RequireNavigator.cpp 对应 ----------

#[test]
fn navigate_unsupported_prefix_reports_error() {
  let mut ctx = MockContext::new();
  let (recorded, error) = ctx.navigate(b"no/prefix");
  assert_eq!(
    error,
    msg(b"require path must start with a valid prefix: ./, ../, or @")
  );
  assert!(recorded.is_empty());
}

#[test]
fn navigate_relative_path_walks_requirer_then_components() {
  // "./a/b"：reset -> parent -> child(a) -> child(b)
  let mut ctx = MockContext::new();
  let (recorded, error) = ctx.navigate(b"./a/b");
  assert_eq!(error, None);
  assert_eq!(recorded, ops(&["reset", "parent", "child:a", "child:b"]));
}

#[test]
fn navigate_accepts_str_and_string_paths() {
  // 入口是 `impl AsRef<[u8]>`：内部传 `&[u8]`，注入方（ulua-analyze-cli）传 `&str`/`String`
  let mut ctx = MockContext::new();
  let mut handler = TestErrorHandler::default();
  {
    let mut navigator = Navigator::new(&mut ctx, &mut handler);
    assert_eq!(navigator.navigate("./a"), Status::Success);
    assert_eq!(navigator.navigate(String::from("./b")), Status::Success);
  }
  assert_eq!(handler.message, None);
  assert_eq!(
    ctx.ops,
    ops(&["reset", "parent", "child:a", "reset", "parent", "child:b"])
  );
}

#[test]
fn navigate_backslash_normalized_to_separator() {
  // 反斜杠按 cpp `std::replace` 归一为字节级 '/'，非 UTF-8 字节不受影响
  let mut ctx = MockContext::new();
  let (recorded, error) = ctx.navigate(b".\\a\\\xff");
  assert_eq!(error, None);
  // ops 里是展示用的 lossy 文本（\xff → U+FFFD），判定链上仍是原字节
  assert_eq!(
    recorded,
    ops(&["reset", "parent", "child:a", "child:\u{FFFD}"])
  );
  assert_eq!(ctx.last_component, Some(vec![0xff]));
}

#[test]
fn navigate_relative_dot_dot_navigates_to_parent() {
  // "./.."：reset -> parent -> parent（首个 previous 为空）
  let mut ctx = MockContext::new();
  let (recorded, error) = ctx.navigate(b"./..");
  assert_eq!(error, None);
  assert_eq!(recorded, ops(&["reset", "parent", "parent"]));
}

#[test]
fn navigate_skips_dot_and_empty_components() {
  // 额外的 '.' 与空组件被跳过
  let mut ctx = MockContext::new();
  let (recorded, error) = ctx.navigate(b"././a//b/.//");
  assert_eq!(error, None);
  assert_eq!(recorded, ops(&["reset", "parent", "child:a", "child:b"]));
}

#[test]
fn navigate_non_utf8_component_is_byte_exact() {
  // 组件字节必须原样交给导航上下文（判定用途不做 UTF-8 转换）
  let mut ctx = MockContext::new();
  let (recorded, error) = ctx.navigate(b"./\xc3\x28");
  assert_eq!(error, None);
  // 0xC3 0x28 不是合法 UTF-8：展示视图退化为 U+FFFD，但组件字节保持原样
  assert_eq!(recorded, ops(&["reset", "parent", "child:\u{FFFD}("]));
  assert_eq!(ctx.last_component, Some(vec![0xc3, 0x28]));
}

#[test]
fn navigate_relative_parent_of_component_error() {
  let mut ctx = MockContext::new();
  ctx.parent = NavigateResult::Ambiguous;
  let (recorded, error) = ctx.navigate(b"./a/..");
  assert_eq!(
    error,
    msg(b"could not get parent of requiring context (ambiguous)")
  );
  assert_eq!(recorded, ops(&["reset", "parent"]));
}

#[test]
fn navigate_child_error_message() {
  let mut ctx = MockContext::new();
  ctx.child = NavigateResult::Ambiguous;
  let (recorded, error) = ctx.navigate(b"./a");
  assert_eq!(
    error,
    msg(b"could not resolve child component \"a\" (ambiguous)")
  );
  assert_eq!(recorded, ops(&["reset", "parent", "child:a"]));
}

#[test]
fn navigate_child_error_message_keeps_non_utf8_component() {
  // 错误消息内嵌组件字节：cpp 拼接 std::string，非法 UTF-8 不得变成 U+FFFD
  let mut ctx = MockContext::new();
  ctx.child = NavigateResult::NotFound;
  let (_, error) = ctx.navigate(b"./\xff\xfe");
  assert_eq!(
    error,
    msg(b"could not resolve child component \"\xff\xfe\"")
  );
}

#[test]
fn navigate_self_alias_falls_back_to_requirer() {
  // "@self/x"：reset -> override(self, NotFound) -> 向上到根 -> reset -> child(x)
  let mut ctx = MockContext::new();
  let (recorded, error) = ctx.navigate(b"@self/x");
  assert_eq!(error, None);
  assert_eq!(
    recorded,
    ops(&[
      "reset",
      "override:self",
      "parent",
      "parent",
      "parent",
      "reset",
      "child:x"
    ])
  );
}

#[test]
fn navigate_alias_lowercase_matches_self() {
  // 别名先转小写，@SELF 等价 @self
  let mut ctx = MockContext::new();
  let (recorded, error) = ctx.navigate(b"@SELF/x");
  assert_eq!(error, None);
  assert_eq!(
    recorded,
    ops(&[
      "reset",
      "override:self",
      "parent",
      "parent",
      "parent",
      "reset",
      "child:x"
    ])
  );
}

#[test]
fn navigate_alias_override_success_walks_path() {
  // 覆盖成功后直接从覆盖位置遍历路径，不再查配置
  let mut ctx = MockContext::new();
  ctx.alias_override = NavigateResult::Success;
  let (recorded, error) = ctx.navigate(b"@bar/x");
  assert_eq!(error, None);
  assert_eq!(recorded, ops(&["reset", "override:bar", "child:x"]));
}

#[test]
fn navigate_alias_bytes_reach_context_byte_exact() {
  // 别名按 ASCII 小写后原样传给上下文：非 UTF-8 字节保持不变
  let mut ctx = MockContext::new();
  ctx.alias_override = NavigateResult::Success;
  let (_, error) = ctx.navigate(b"@\xffAB/x");
  assert_eq!(error, None);
  assert_eq!(ctx.last_alias, Some(vec![0xff, b'a', b'b']));
}

#[test]
fn navigate_alias_fallback_ambiguous_message() {
  let mut ctx = MockContext::new();
  ctx.alias_fallback = NavigateResult::Ambiguous;
  let (recorded, error) = ctx.navigate(b"@foo/x");
  assert_eq!(error, msg(b"@foo is not a valid alias (ambiguous)"));
  assert_eq!(
    recorded,
    ops(&[
      "reset",
      "override:foo",
      "parent",
      "parent",
      "parent",
      "fallback:foo"
    ])
  );
}

#[test]
fn navigate_alias_fallback_message_keeps_non_utf8_alias() {
  let mut ctx = MockContext::new();
  ctx.alias_fallback = NavigateResult::NotFound;
  let (_, error) = ctx.navigate(b"@\xff/x");
  assert_eq!(error, msg(b"@\xff is not a valid alias"));
}

#[test]
fn navigate_alias_override_ambiguous_message() {
  let mut ctx = MockContext::new();
  ctx.alias_override = NavigateResult::Ambiguous;
  let (recorded, error) = ctx.navigate(b"@foo/x");
  assert_eq!(error, msg(b"@foo is not a valid alias (ambiguous)"));
  assert_eq!(recorded, ops(&["reset", "override:foo"]));
}

#[test]
fn navigate_reset_ambiguous_message() {
  let mut ctx = MockContext::new();
  ctx.reset = NavigateResult::Ambiguous;
  let (recorded, error) = ctx.navigate(b"@foo/x");
  assert_eq!(
    error,
    msg(b"could not reset to requiring context (ambiguous)")
  );
  // cpp 主线：reset 先于 override 执行
  assert_eq!(recorded, ops(&["reset"]));
}

#[test]
fn navigate_alias_from_config_uses_jump_for_absolute_value() {
  // 配置给出绝对路径别名值 -> jumpToAlias -> 再遍历原路径子组件
  let mut ctx = MockContext::new();
  ctx.status = ConfigStatus::PresentJson;
  ctx.alias_value = Some(Vec::from(&b"/abs/path"[..]));
  let (recorded, error) = ctx.navigate(b"@foo/x");
  assert_eq!(error, None);
  assert_eq!(
    recorded,
    ops(&["reset", "override:foo", "parent", "jump", "child:x"])
  );
  assert_eq!(ctx.last_alias, Some(b"/abs/path".to_vec()));
}

#[test]
fn navigate_alias_from_config_walks_relative_value() {
  // 配置给出相对路径别名值 -> 直接遍历
  let mut ctx = MockContext::new();
  ctx.status = ConfigStatus::PresentJson;
  ctx.alias_value = Some(Vec::from(&b"./dep"[..]));
  let (recorded, error) = ctx.navigate(b"@foo");
  assert_eq!(error, None);
  assert_eq!(
    recorded,
    ops(&["reset", "override:foo", "parent", "child:dep"])
  );
}

#[test]
fn navigate_alias_null_value_reports_error() {
  // getAlias 返回 None：cpp 主线直接报错（null 检查已永久合入）
  let mut ctx = MockContext::new();
  ctx.status = ConfigStatus::PresentJson;
  let (recorded, error) = ctx.navigate(b"@foo");
  assert_eq!(error, msg(b"could not resolve alias \"foo\""));
  assert_eq!(recorded, ops(&["reset", "override:foo", "parent"]));
}

#[test]
fn navigate_ambiguous_config_file_message() {
  let mut ctx = MockContext::new();
  ctx.parent = NavigateResult::Success;
  ctx.status = ConfigStatus::Ambiguous;
  let (recorded, error) = ctx.navigate(b"@foo");
  assert_eq!(
    error,
    msg(b"could not resolve alias \"foo\" (ambiguous configuration file)")
  );
  assert_eq!(recorded, ops(&["reset", "override:foo", "parent"]));
}

#[test]
fn navigate_ambiguous_ancestry_message() {
  let mut ctx = MockContext::new();
  ctx.parent = NavigateResult::Ambiguous;
  let (recorded, error) = ctx.navigate(b"@foo");
  assert_eq!(
    error,
    msg(b"could not navigate up the ancestry chain during search for alias \"foo\" (ambiguous)")
  );
  assert_eq!(recorded, ops(&["reset", "override:foo", "parent"]));
}

#[test]
fn navigate_missing_config_contents_message() {
  let mut ctx = MockContext::new();
  ctx.parent = NavigateResult::Success;
  ctx.status = ConfigStatus::PresentJson;
  ctx.behavior = ConfigBehavior::GetConfig;
  ctx.config_value = None;
  let (recorded, error) = ctx.navigate(b"@foo");
  assert_eq!(
    error,
    msg(b"could not get configuration file contents to resolve alias \"foo\"")
  );
  assert_eq!(recorded, ops(&["reset", "override:foo", "parent"]));
}

#[test]
fn navigate_absent_config_falls_back_to_alias_fallback() {
  // 配置缺失 -> 向上遍历到根 -> fallback（NotFound 也报错）
  let mut ctx = MockContext::new();
  let (recorded, error) = ctx.navigate(b"@foo");
  assert_eq!(error, msg(b"@foo is not a valid alias"));
  assert_eq!(
    recorded,
    ops(&[
      "reset",
      "override:foo",
      "parent",
      "parent",
      "parent",
      "fallback:foo"
    ])
  );
}

// ---------- ResolvedRequire 对应 ----------

#[test]
fn resolved_require_from_error_message_carries_error() {
  let resolved = ResolvedRequire::from_error_message(b"boom");
  let debug = format!("{resolved:?}");
  assert!(debug.contains("ErrorReported"), "{debug}");
  assert!(debug.contains("boom"), "{debug}");
}

#[test]
fn resolved_require_from_error_handler_carries_reported_error() {
  let mut handler = RuntimeErrorHandler::new(b"m");
  handler.report_error(b"x");
  let resolved = ResolvedRequire::from_error_handler(&handler);
  assert_eq!(resolved.error, &b"error requiring module \"m\": x"[..]);
}
