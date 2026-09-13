//! Require 模块导航纯逻辑测试，对照 C++ `Require/src`（RequireNavigator.cpp、
//! Navigation.cpp、AliasCycleTracker.cpp、PathUtilities.cpp）的行为与错误消息。

use alloc::{string::String, vec::Vec};

use crate::{
  enums::{
    config_behavior::ConfigBehavior, config_status::ConfigStatus, navigate_result::NavigateResult,
    path_type::PathType,
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
  alias_value: Option<String>,
  config_value: Option<String>,
  ops: Vec<String>,
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
    }
  }

  /// 执行一次 require 导航，返回 (操作序列, 报告的错误)。
  fn navigate(&mut self, path: &str) -> (Vec<String>, Option<String>) {
    let mut handler = TestErrorHandler::default();
    let mut navigator = Navigator::new(self, &mut handler);
    navigator.navigate(String::from(path));
    (self.ops.clone(), handler.message)
  }
}

impl NavigationContextTrait for MockContext {
  fn reset_to_requirer(&mut self) -> NavigateResult {
    self.ops.push(String::from("reset"));
    self.reset
  }

  fn jump_to_alias(&mut self, _path: &str) -> NavigateResult {
    self.ops.push(String::from("jump"));
    self.jump
  }

  fn to_alias_override(&mut self, alias_unprefixed: &str) -> NavigateResult {
    self.ops.push(format!("override:{alias_unprefixed}"));
    self.alias_override
  }

  fn to_alias_fallback(&mut self, alias_unprefixed: &str) -> NavigateResult {
    self.ops.push(format!("fallback:{alias_unprefixed}"));
    self.alias_fallback
  }

  fn to_parent(&mut self) -> NavigateResult {
    self.ops.push(String::from("parent"));
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

  fn to_child(&mut self, component: &str) -> NavigateResult {
    self.ops.push(format!("child:{component}"));
    self.child
  }

  fn get_config_status(&self) -> ConfigStatus {
    self.status
  }

  fn get_config_behavior(&self) -> ConfigBehavior {
    self.behavior
  }

  fn get_alias(&self, _alias: &str) -> Option<String> {
    self.alias_value.clone()
  }

  fn get_config(&self) -> Option<String> {
    self.config_value.clone()
  }
}

/// 记录报告错误的 ErrorHandler。
#[derive(Default)]
struct TestErrorHandler {
  message: Option<String>,
}

impl ErrorHandler for TestErrorHandler {
  fn report_error(&mut self, message: String) {
    self.message = Some(message);
  }
}

fn ops(values: &[&str]) -> Vec<String> {
  values.iter().map(|s| String::from(*s)).collect()
}

// ---------- PathUtilities.cpp 对应 ----------

#[test]
fn path_type_prefixes() {
  assert_eq!(get_path_type("./a"), PathType::RelativeToCurrent);
  assert_eq!(get_path_type("../a"), PathType::RelativeToParent);
  assert_eq!(get_path_type("@a"), PathType::Aliased);
  assert_eq!(get_path_type("a"), PathType::Unsupported);
  assert_eq!(get_path_type(""), PathType::Unsupported);
  assert_eq!(get_path_type("."), PathType::Unsupported);
  assert_eq!(get_path_type(".."), PathType::Unsupported);
  assert_eq!(get_path_type("@"), PathType::Aliased);
}

#[test]
fn split_path_components() {
  assert_eq!(split_path("a/b"), ("a", "b"));
  assert_eq!(split_path("ab"), ("ab", ""));
  assert_eq!(split_path("a/b/c"), ("a", "b/c"));
  assert_eq!(split_path("/x"), ("", "x"));
  assert_eq!(split_path(""), ("", ""));
}

#[test]
fn extract_alias_values() {
  // '@' 前缀被忽略；有分隔符时取别名到分隔符，否则取整段
  assert_eq!(extract_alias("@foo/bar"), "foo");
  assert_eq!(extract_alias("@foo"), "foo");
  assert_eq!(extract_alias("@/"), "");
  assert_eq!(extract_alias("@a//b"), "a");
  assert_eq!(extract_alias("@Foo/BAR"), "Foo");
}

// ---------- AliasCycleTracker.cpp 对应 ----------

#[test]
fn alias_cycle_tracker_detects_cycle() {
  let mut tracker = AliasCycleTracker::new();
  assert_eq!(tracker.add(String::from("a")), None);
  assert_eq!(
    tracker.add(String::from("a")),
    Some(String::from("detected alias cycle (@a -> @a)"))
  );
}

#[test]
fn alias_cycle_tracker_multi_level() {
  let mut tracker = AliasCycleTracker::new();
  assert_eq!(tracker.add(String::from("a")), None);
  assert_eq!(tracker.add(String::from("b")), None);
  assert_eq!(
    tracker.add(String::from("a")),
    Some(String::from("detected alias cycle (@a -> @b -> @a)"))
  );
}

// ---------- Navigation.cpp 对应 ----------

#[test]
fn runtime_error_handler_prefixes_message() {
  let mut handler = RuntimeErrorHandler::new(String::from("foo/bar"));
  handler.report_error(String::from("boom"));
  assert_eq!(
    handler.get_reported_error(),
    "error requiring module \"foo/bar\": boom"
  );
}

// ---------- RequireNavigator.cpp 对应 ----------

#[test]
fn navigate_unsupported_prefix_reports_error() {
  let mut ctx = MockContext::new();
  let (recorded, error) = ctx.navigate("no/prefix");
  assert_eq!(
    error,
    Some(String::from(
      "require path must start with a valid prefix: ./, ../, or @"
    ))
  );
  assert_eq!(recorded, Vec::<String>::new());
}

#[test]
fn navigate_relative_path_walks_requirer_then_components() {
  // "./a/b"：reset -> parent -> child(a) -> child(b)
  let mut ctx = MockContext::new();
  let (recorded, error) = ctx.navigate("./a/b");
  assert_eq!(error, None);
  assert_eq!(recorded, ops(&["reset", "parent", "child:a", "child:b"]));
}

#[test]
fn navigate_relative_dot_dot_navigates_to_parent() {
  // "./.."：reset -> parent -> parent（首个 previous 为空）
  let mut ctx = MockContext::new();
  let (recorded, error) = ctx.navigate("./..");
  assert_eq!(error, None);
  assert_eq!(recorded, ops(&["reset", "parent", "parent"]));
}

#[test]
fn navigate_skips_dot_and_empty_components() {
  // 额外的 '.' 与空组件被跳过
  let mut ctx = MockContext::new();
  let (recorded, error) = ctx.navigate("././a//b/.//");
  assert_eq!(error, None);
  assert_eq!(recorded, ops(&["reset", "parent", "child:a", "child:b"]));
}

#[test]
fn navigate_relative_parent_of_component_error() {
  let mut ctx = MockContext::new();
  ctx.parent = NavigateResult::Ambiguous;
  let (recorded, error) = ctx.navigate("./a/..");
  assert_eq!(
    error,
    Some(String::from(
      "could not get parent of requiring context (ambiguous)"
    ))
  );
  assert_eq!(recorded, ops(&["reset", "parent"]));
}

#[test]
fn navigate_child_error_message() {
  let mut ctx = MockContext::new();
  ctx.child = NavigateResult::Ambiguous;
  let (recorded, error) = ctx.navigate("./a");
  assert_eq!(
    error,
    Some(String::from(
      "could not resolve child component \"a\" (ambiguous)"
    ))
  );
  assert_eq!(recorded, ops(&["reset", "parent", "child:a"]));
}

#[test]
fn navigate_self_alias_falls_back_to_requirer() {
  // "@self/x"（DFFlag 关）：override -> reset -> 向上到根 -> reset -> child(x)
  let mut ctx = MockContext::new();
  let (recorded, error) = ctx.navigate("@self/x");
  assert_eq!(error, None);
  assert_eq!(
    recorded,
    ops(&[
      "override:self",
      "reset",
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
  let (recorded, error) = ctx.navigate("@SELF/x");
  assert_eq!(error, None);
  assert_eq!(
    recorded,
    ops(&[
      "override:self",
      "reset",
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
  let (recorded, error) = ctx.navigate("@bar/x");
  assert_eq!(error, None);
  assert_eq!(recorded, ops(&["override:bar", "child:x"]));
}

#[test]
fn navigate_alias_fallback_ambiguous_message() {
  let mut ctx = MockContext::new();
  ctx.alias_fallback = NavigateResult::Ambiguous;
  let (recorded, error) = ctx.navigate("@foo/x");
  assert_eq!(
    error,
    Some(String::from("@foo is not a valid alias (ambiguous)"))
  );
  assert_eq!(
    recorded,
    ops(&[
      "override:foo",
      "reset",
      "parent",
      "parent",
      "parent",
      "fallback:foo"
    ])
  );
}

#[test]
fn navigate_alias_override_ambiguous_message() {
  let mut ctx = MockContext::new();
  ctx.alias_override = NavigateResult::Ambiguous;
  let (recorded, error) = ctx.navigate("@foo/x");
  assert_eq!(
    error,
    Some(String::from("@foo is not a valid alias (ambiguous)"))
  );
  assert_eq!(recorded, ops(&["override:foo"]));
}

#[test]
fn navigate_reset_ambiguous_message() {
  let mut ctx = MockContext::new();
  ctx.reset = NavigateResult::Ambiguous;
  let (recorded, error) = ctx.navigate("@foo/x");
  assert_eq!(
    error,
    Some(String::from(
      "could not reset to requiring context (ambiguous)"
    ))
  );
  // DFFlag 关：reset 在 override 之后
  assert_eq!(recorded, ops(&["override:foo", "reset"]));
}

#[test]
fn navigate_alias_from_config_uses_jump_for_absolute_value() {
  // 配置给出绝对路径别名值 -> jumpToAlias -> 再遍历原路径子组件
  let mut ctx = MockContext::new();
  ctx.status = ConfigStatus::PresentJson;
  ctx.alias_value = Some(String::from("/abs/path"));
  let (recorded, error) = ctx.navigate("@foo/x");
  assert_eq!(error, None);
  assert_eq!(
    recorded,
    ops(&["override:foo", "reset", "parent", "jump", "child:x"])
  );
}

#[test]
fn navigate_alias_from_config_walks_relative_value() {
  // 配置给出相对路径别名值 -> 直接遍历
  let mut ctx = MockContext::new();
  ctx.status = ConfigStatus::PresentJson;
  ctx.alias_value = Some(String::from("./dep"));
  let (recorded, error) = ctx.navigate("@foo");
  assert_eq!(error, None);
  assert_eq!(
    recorded,
    ops(&["override:foo", "reset", "parent", "child:dep"])
  );
}

#[test]
fn navigate_alias_null_value_uses_jump_for_empty_value() {
  // getAlias 返回 None 且 DFFlag 关：写入空值别名 -> jump
  let mut ctx = MockContext::new();
  ctx.status = ConfigStatus::PresentJson;
  let (recorded, error) = ctx.navigate("@foo");
  assert_eq!(error, None);
  assert_eq!(recorded, ops(&["override:foo", "reset", "parent", "jump"]));
}

#[test]
fn navigate_config_get_alias_none_with_flag_reports_error() {
  // DFFlag::LuauRequireResolveAliasNullCheck 开：getAlias 为 None 报错
  use ulua_common::FFlag;
  FFlag::LuauRequireResolveAliasNullCheck.set(true);
  let mut ctx = MockContext::new();
  ctx.parent = NavigateResult::Success;
  ctx.status = ConfigStatus::PresentJson;
  let (_, error) = ctx.navigate("@foo");
  assert_eq!(error, Some(String::from("could not resolve alias \"foo\"")));
  FFlag::LuauRequireResolveAliasNullCheck.set(false);
}

#[test]
fn navigate_ambiguous_config_file_message() {
  let mut ctx = MockContext::new();
  ctx.parent = NavigateResult::Success;
  ctx.status = ConfigStatus::Ambiguous;
  let (recorded, error) = ctx.navigate("@foo");
  assert_eq!(
    error,
    Some(String::from(
      "could not resolve alias \"foo\" (ambiguous configuration file)"
    ))
  );
  assert_eq!(recorded, ops(&["override:foo", "reset", "parent"]));
}

#[test]
fn navigate_ambiguous_ancestry_message() {
  let mut ctx = MockContext::new();
  ctx.parent = NavigateResult::Ambiguous;
  let (recorded, error) = ctx.navigate("@foo");
  assert_eq!(
    error,
    Some(String::from(
      "could not navigate up the ancestry chain during search for alias \"foo\" (ambiguous)"
    ))
  );
  assert_eq!(recorded, ops(&["override:foo", "reset", "parent"]));
}

#[test]
fn navigate_missing_config_contents_message() {
  let mut ctx = MockContext::new();
  ctx.parent = NavigateResult::Success;
  ctx.status = ConfigStatus::PresentJson;
  ctx.behavior = ConfigBehavior::GetConfig;
  ctx.config_value = None;
  let (recorded, error) = ctx.navigate("@foo");
  assert_eq!(
    error,
    Some(String::from(
      "could not get configuration file contents to resolve alias \"foo\""
    ))
  );
  assert_eq!(recorded, ops(&["override:foo", "reset", "parent"]));
}

#[test]
fn navigate_absent_config_falls_back_to_alias_fallback() {
  // 配置缺失 -> 向上遍历到根 -> fallback（NotFound 也报错）
  let mut ctx = MockContext::new();
  let (recorded, error) = ctx.navigate("@foo");
  assert_eq!(error, Some(String::from("@foo is not a valid alias")));
  assert_eq!(
    recorded,
    ops(&[
      "override:foo",
      "reset",
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
  let resolved = ResolvedRequire::from_error_message("boom");
  let debug = format!("{resolved:?}");
  assert!(debug.contains("ErrorReported"), "{debug}");
  assert!(debug.contains("boom"), "{debug}");
}

#[test]
fn resolved_require_from_error_handler_carries_reported_error() {
  let mut handler = RuntimeErrorHandler::new(String::from("m"));
  handler.report_error(String::from("x"));
  let resolved = ResolvedRequire::from_error_handler(&handler);
  assert_eq!(resolved.error, "error requiring module \"m\": x");
}
