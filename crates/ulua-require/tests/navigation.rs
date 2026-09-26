//! Require 模块导航纯逻辑测试，对照 C++ `Require/src`（RequireNavigator.cpp、
//! Navigation.cpp、AliasCycleTracker.cpp、PathUtilities.cpp）的行为与错误消息。
//!
//! 路径/别名一律按字节处理（cpp 为 `std::string`），故断言也用字节串，
//! 并专门覆盖非 UTF-8 字节不被替换为 U+FFFD 的语义。

use std::collections::VecDeque;

use ulua_require::{
  enums::{
    config_behavior::ConfigBehavior, config_status::ConfigStatus, navigate_result::NavigateResult,
    path_type::PathType, status_require_impl::Status as ResolvedStatus,
    status_require_navigator::Status,
  },
  functions::{extract_alias::extract_alias, get_path_type::get_path_type},
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
  /// 优先于 parent/parent_successes 消费：逐次返回预设结果，用于
  /// 「先成功后失败」的场景（cpp navigateToParent(Some(component)) 分支）。
  parent_script: VecDeque<NavigateResult>,
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
      parent_script: VecDeque::new(),
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
    // 脚本化 parent 序列优先：逐次返回预设结果（cpp `could not get parent of
    // component "X"` 分支需要「先成功后失败」，见 navigate_gap_branch_cases）。
    if let Some(scripted) = self.parent_script.pop_front() {
      return scripted;
    }
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

// 以下 24 例同构：预置 MockContext → navigate(path) → 断言 (错误字节, 操作序列)，
// 部分用例附带 last_component / last_alias 的字节级断言。收口为一张表驱动循环，
// 断言与逐个 #[test] 版本逐例等价，仅压写法。预置/附加断言用不捕获的闭包
// （可协变为 fn 指针），让每行的夹具改动直接可见。

/// 同构 navigate 用例的统一骨架
///
/// `error` 为期望的报告错误字节（`&[]` 表示期望无错误）；`expected_ops` 为期望
/// 操作序列（`None` 表示原用例未断言操作序列，保持不断言）；`extra` 在主断言
/// 后追加执行（默认 `|_| {}`）。
fn nav_case(
  path: &[u8],
  setup: fn(&mut MockContext),
  error: &[u8],
  expected_ops: Option<&[&str]>,
  extra: fn(&MockContext),
) {
  let mut ctx = MockContext::new();
  setup(&mut ctx);
  let (recorded, err) = ctx.navigate(path);
  let want_err = (!error.is_empty()).then(|| error.to_vec());
  assert_eq!(err, want_err, "用例 {path:?}");
  if let Some(expected) = expected_ops {
    assert_eq!(recorded, ops(expected), "用例 {path:?}");
  }
  extra(&ctx);
}

/// navigate 用例表行的类型：(入参, 夹具预置, 期望错误, 期望操作序列, 附加断言)
type NavRow = (
  &'static [u8],
  fn(&mut MockContext),
  &'static [u8],
  Option<&'static [&'static str]>,
  fn(&MockContext),
);

#[test]
fn navigate_cases() {
  let cases: &[NavRow] = &[
    // 无前缀路径：直接报错，未触碰上下文
    (
      b"no/prefix",
      |_| {},
      b"require path must start with a valid prefix: ./, ../, or @",
      Some(&[]),
      |_| {},
    ),
    // "./a/b"：reset -> parent -> child(a) -> child(b)
    (
      b"./a/b",
      |_| {},
      &[],
      Some(&["reset", "parent", "child:a", "child:b"]),
      |_| {},
    ),
    // 反斜杠按 cpp `std::replace` 归一为字节级 '/'，非 UTF-8 字节不受影响；
    // ops 里是展示用的 lossy 文本（\xff → U+FFFD），判定链上仍是原字节
    (
      b".\\a\\\xff",
      |_| {},
      &[],
      Some(&["reset", "parent", "child:a", "child:\u{FFFD}"]),
      |c| assert_eq!(c.last_component, Some(vec![0xff])),
    ),
    // "./.."：reset -> parent -> parent（首个 previous 为空）
    (
      b"./..",
      |_| {},
      &[],
      Some(&["reset", "parent", "parent"]),
      |_| {},
    ),
    // 额外的 '.' 与空组件被跳过
    (
      b"././a//b/.//",
      |_| {},
      &[],
      Some(&["reset", "parent", "child:a", "child:b"]),
      |_| {},
    ),
    // 0xC3 0x28 不是合法 UTF-8：展示视图退化为 U+FFFD，但组件字节保持原样
    (
      b"./\xc3\x28",
      |_| {},
      &[],
      Some(&["reset", "parent", "child:\u{FFFD}("]),
      |c| assert_eq!(c.last_component, Some(vec![0xc3, 0x28])),
    ),
    (
      b"./a/..",
      |c| c.parent = NavigateResult::Ambiguous,
      b"could not get parent of requiring context (ambiguous)",
      Some(&["reset", "parent"]),
      |_| {},
    ),
    (
      b"./a",
      |c| c.child = NavigateResult::Ambiguous,
      b"could not resolve child component \"a\" (ambiguous)",
      Some(&["reset", "parent", "child:a"]),
      |_| {},
    ),
    // 错误消息内嵌组件字节：cpp 拼接 std::string，非法 UTF-8 不得变成 U+FFFD
    (
      b"./\xff\xfe",
      |c| c.child = NavigateResult::NotFound,
      b"could not resolve child component \"\xff\xfe\"",
      None,
      |_| {},
    ),
    // "@self/x"：reset -> override(self, NotFound) -> 向上到根 -> reset -> child(x)
    (
      b"@self/x",
      |_| {},
      &[],
      Some(&[
        "reset",
        "override:self",
        "parent",
        "parent",
        "parent",
        "reset",
        "child:x",
      ]),
      |_| {},
    ),
    // 别名先转小写，@SELF 等价 @self
    (
      b"@SELF/x",
      |_| {},
      &[],
      Some(&[
        "reset",
        "override:self",
        "parent",
        "parent",
        "parent",
        "reset",
        "child:x",
      ]),
      |_| {},
    ),
    // 覆盖成功后直接从覆盖位置遍历路径，不再查配置
    (
      b"@bar/x",
      |c| c.alias_override = NavigateResult::Success,
      &[],
      Some(&["reset", "override:bar", "child:x"]),
      |_| {},
    ),
    // 别名按 ASCII 小写后原样传给上下文：非 UTF-8 字节保持不变
    (
      b"@\xffAB/x",
      |c| c.alias_override = NavigateResult::Success,
      &[],
      None,
      |c| assert_eq!(c.last_alias, Some(vec![0xff, b'a', b'b'])),
    ),
    (
      b"@foo/x",
      |c| c.alias_fallback = NavigateResult::Ambiguous,
      b"@foo is not a valid alias (ambiguous)",
      Some(&[
        "reset",
        "override:foo",
        "parent",
        "parent",
        "parent",
        "fallback:foo",
      ]),
      |_| {},
    ),
    (
      b"@\xff/x",
      |c| c.alias_fallback = NavigateResult::NotFound,
      b"@\xff is not a valid alias",
      None,
      |_| {},
    ),
    (
      b"@foo/x",
      |c| c.alias_override = NavigateResult::Ambiguous,
      b"@foo is not a valid alias (ambiguous)",
      Some(&["reset", "override:foo"]),
      |_| {},
    ),
    // cpp 主线：reset 先于 override 执行
    (
      b"@foo/x",
      |c| c.reset = NavigateResult::Ambiguous,
      b"could not reset to requiring context (ambiguous)",
      Some(&["reset"]),
      |_| {},
    ),
    // 配置给出绝对路径别名值 -> jumpToAlias -> 再遍历原路径子组件
    (
      b"@foo/x",
      |c| {
        c.status = ConfigStatus::PresentJson;
        c.alias_value = Some(Vec::from(&b"/abs/path"[..]));
      },
      &[],
      Some(&["reset", "override:foo", "parent", "jump", "child:x"]),
      |c| assert_eq!(c.last_alias, Some(b"/abs/path".to_vec())),
    ),
    // 配置给出相对路径别名值 -> 直接遍历
    (
      b"@foo",
      |c| {
        c.status = ConfigStatus::PresentJson;
        c.alias_value = Some(Vec::from(&b"./dep"[..]));
      },
      &[],
      Some(&["reset", "override:foo", "parent", "child:dep"]),
      |_| {},
    ),
    // getAlias 返回 None：cpp 主线直接报错（null 检查已永久合入）
    (
      b"@foo",
      |c| c.status = ConfigStatus::PresentJson,
      b"could not resolve alias \"foo\"",
      Some(&["reset", "override:foo", "parent"]),
      |_| {},
    ),
    (
      b"@foo",
      |c| c.status = ConfigStatus::Ambiguous,
      b"could not resolve alias \"foo\" (ambiguous configuration file)",
      Some(&["reset", "override:foo", "parent"]),
      |_| {},
    ),
    (
      b"@foo",
      |c| c.parent = NavigateResult::Ambiguous,
      b"could not navigate up the ancestry chain during search for alias \"foo\" (ambiguous)",
      Some(&["reset", "override:foo", "parent"]),
      |_| {},
    ),
    (
      b"@foo",
      |c| {
        c.status = ConfigStatus::PresentJson;
        c.behavior = ConfigBehavior::GetConfig;
      },
      b"could not get configuration file contents to resolve alias \"foo\"",
      Some(&["reset", "override:foo", "parent"]),
      |_| {},
    ),
    // 配置缺失 -> 向上遍历到根 -> fallback（NotFound 也报错）
    (
      b"@foo",
      |_| {},
      b"@foo is not a valid alias",
      Some(&[
        "reset",
        "override:foo",
        "parent",
        "parent",
        "parent",
        "fallback:foo",
      ]),
      |_| {},
    ),
  ];
  for (path, setup, error, expected_ops, extra) in cases {
    nav_case(path, *setup, error, *expected_ops, *extra);
  }
}

/// 入口是 `impl AsRef<[u8]>`：内部传 `&[u8]`，注入方（ulua-analyze-cli）传 `&str`/`String`
#[test]
fn navigate_accepts_str_and_string_paths() {
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

// ---------- ResolvedRequire 对应 ----------

/// `ResolvedRequire::fromErrorMessage` 的精确构造契约
/// oracle `cpp/Require/src/RequireImpl.cpp:37`：`{ErrorReported, "", "", "", message}`
/// ——仅 status 与 error 有值，chunkname/loadname/cache_key 恒空。原用例只
/// `debug.contains(...)`（弱断言、Debug 文本无 cpp 出处），此处改为逐字段锁定。
#[test]
fn resolved_require_from_error_message_carries_error() {
  // cpp RequireImpl.cpp:37-40 `fromErrorMessage`：仅 status=ErrorReported 与
  // error=message 有值，chunkname/loadname/cacheKey 恒为空串。
  let resolved = ResolvedRequire::from_error_message(b"boom");
  assert_eq!(resolved.status, ResolvedStatus::ErrorReported);
  assert_eq!(resolved.error, &b"boom"[..]);
  assert_eq!(resolved.chunkname, &b""[..]);
  assert_eq!(resolved.loadname, &b""[..]);
  assert_eq!(resolved.cache_key, &b""[..]);
}

/// `ResolvedRequire::fromErrorHandler` 的精确构造契约
/// oracle `cpp/Require/src/RequireImpl.cpp:32`：`{ErrorReported, "", "", "",
/// errorHandler.getReportedError()}`，error 取错误处理器已报告的前缀全文。
#[test]
fn resolved_require_from_error_handler_carries_reported_error() {
  // cpp RequireImpl.cpp:31-35 `fromErrorHandler`：同上骨架，error 取 handler
  // 汇总消息（Navigation.cpp `RuntimeErrorHandler::reportError` 拼
  // `"error requiring module "" + path + "": " + message`）。
  let mut handler = RuntimeErrorHandler::new(b"m");
  handler.report_error(b"x");
  let resolved = ResolvedRequire::from_error_handler(&handler);
  assert_eq!(resolved.status, ResolvedStatus::ErrorReported);
  assert_eq!(resolved.error, &b"error requiring module \"m\": x"[..]);
  assert_eq!(resolved.chunkname, &b""[..]);
  assert_eq!(resolved.loadname, &b""[..]);
  assert_eq!(resolved.cache_key, &b""[..]);
}

// ---------- 边界补漏：jumpToAlias 失败 / 带组件 parent / 空前缀 ----------
// 以下为 cpp RequireNavigator.cpp 中既有文案、此前无用例锁定的分支。

#[test]
fn navigate_gap_branch_cases() {
  use std::collections::VecDeque;

  use NavigateResult::{Ambiguous, NotFound, Success};

  let cases: &[NavRow] = &[
    // cpp RequireNavigator.cpp:335-345 `jumpToAlias`：别名值为绝对路径
    // （PathType::Unsupported）时跳转失败 → could not jump to alias "<path>"
    (
      b"@foo/x",
      |c| {
        c.status = ConfigStatus::PresentJson;
        c.alias_value = Some(Vec::from(&b"/abs/path"[..]));
        c.jump = NotFound;
      },
      b"could not jump to alias \"/abs/path\"",
      Some(&["reset", "override:foo", "parent", "jump"]),
      |_| {},
    ),
    // 同上 Ambiguous：cpp:343 追加 " (ambiguous)"
    (
      b"@foo/x",
      |c| {
        c.status = ConfigStatus::PresentJson;
        c.alias_value = Some(Vec::from(&b"/abs/path"[..]));
        c.jump = Ambiguous;
      },
      b"could not jump to alias \"/abs/path\" (ambiguous)",
      Some(&["reset", "override:foo", "parent", "jump"]),
      |c| assert_eq!(c.last_alias, Some(b"/abs/path".to_vec())),
    ),
    // cpp RequireNavigator.cpp:183-188 + 347-355：`.`/`..` 归一后对具名组件
    // 取父失败 → could not get parent of component "a"（首个 parent 在
    // navigateImpl:351 以 nullopt 调用，成功后才轮到具名组件）
    (
      b"./a/..",
      |c| {
        c.parent_script = VecDeque::from([Success, NotFound]);
      },
      b"could not get parent of component \"a\"",
      Some(&["reset", "parent", "child:a", "parent"]),
      |_| {},
    ),
    // 同上 Ambiguous：cpp:359 追加 " (ambiguous)"
    (
      b"./a/..",
      |c| {
        c.parent_script = VecDeque::from([Success, Ambiguous]);
      },
      b"could not get parent of component \"a\" (ambiguous)",
      Some(&["reset", "parent", "child:a", "parent"]),
      |_| {},
    ),
    // cpp PathUtilities.cpp:11-19 getPathType + RequireNavigator.cpp:59-62：
    // 空串与纯 "." / ".." 均无合法前缀，且完全不动上下文
    (
      b"",
      |_| {},
      b"require path must start with a valid prefix: ./, ../, or @",
      Some(&[]),
      |_| {},
    ),
    (
      b".",
      |_| {},
      b"require path must start with a valid prefix: ./, ../, or @",
      Some(&[]),
      |_| {},
    ),
    (
      b"..",
      |_| {},
      b"require path must start with a valid prefix: ./, ../, or @",
      Some(&[]),
      |_| {},
    ),
  ];
  for (path, setup, error, expected_ops, extra) in cases {
    nav_case(path, *setup, error, *expected_ops, *extra);
  }
}
