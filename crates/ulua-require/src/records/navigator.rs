use alloc::vec::Vec;

use ulua_common::dfflag::LuauSelfIsSelfAndAlwaysSelf;
use ulua_config::{
  functions::{extract_luau_config::extract_luau_config, parse_config::parse_config},
  records::{
    alias_options::AliasOptions, config::Config, config_options::ConfigOptions,
    interrupt_callbacks::InterruptCallbacks,
  },
};

use crate::{
  enums::{
    config_behavior::ConfigBehavior, config_status::ConfigStatus, navigate_result::NavigateResult,
    path_type::PathType, status_require_navigator::Status,
  },
  functions::{
    extract_alias::extract_alias,
    get_path_type::get_path_type,
    navigate_error::{AMBIGUOUS_SUFFIX, ambiguous, invalid_alias, quoted, suffixed},
    path_bytes::{ALIAS_PREFIX, PATH_SEPARATOR, PATH_SEPARATOR_ALT},
    utf8_boundary::{utf8_owned, utf8_view},
  },
  records::{
    alias_cycle_tracker::AliasCycleTracker, error_handler::ErrorHandler,
    navigation_context::NavigationContextTrait,
  },
};

/// 路径导航器，持有导航上下文与错误报告器的可变引用。
/// 用泛型参数替代 `dyn`，静态分派消除虚调用开销。
pub struct Navigator<'ctx, C: NavigationContextTrait, E: ErrorHandler> {
  pub(crate) navigation_context: &'ctx mut C,
  pub(crate) error_handler: &'ctx mut E,
}

/// 导航过程中的错误（对应 C++ `Navigator::Error = std::optional<std::string>`）：
/// 消息内嵌路径/别名字节，故用 `Vec<u8>` 保持与 cpp 一致的字节拼接语义。
pub(crate) type Error = Option<Vec<u8>>;

/// cpp 固定文案：路径前缀非法。
const UNSUPPORTED_PREFIX: &[u8] = b"require path must start with a valid prefix: ./, ../, or @";
/// cpp `alias == "self"` 的字节比较基准。
const SELF_ALIAS: &[u8] = b"self";
/// cpp 原文案前缀：`"could not jump to alias \"" + path + "\""`。
const JUMP_PREFIX: &[u8] = b"could not jump to alias ";
/// 带组件名时的消息前缀（cpp `"could not get parent of component \"" + c + "\""`）。
const PARENT_OF_PREFIX: &[u8] = b"could not get parent of component ";
/// 无前置组件时的固定消息。
const PARENT_OF_REQUIRER: &[u8] = b"could not get parent of requiring context";
/// cpp 原文案前缀：`"could not resolve child component \"" + component + "\""`。
const CHILD_PREFIX: &[u8] = b"could not resolve child component ";
/// cpp 固定文案。
const RESET_FAILED: &[u8] = b"could not reset to requiring context";
/// 向上遍历祖先链失败的消息前缀。
const ANCESTRY_PREFIX: &[u8] = b"could not navigate up the ancestry chain during search for alias ";
/// 别名无法解析 / 配置文件含糊不清的消息前缀与后缀。
const RESOLVE_ALIAS_PREFIX: &[u8] = b"could not resolve alias ";
const AMBIGUOUS_CONFIG_SUFFIX: &[u8] = b" (ambiguous configuration file)";
/// 取不到配置文件内容的消息前缀。
const MISSING_CONTENTS_PREFIX: &[u8] =
  b"could not get configuration file contents to resolve alias ";

/// 进入嵌入方别名槽的两种时机（对应 cpp `Navigator::toAliasOverride` /
/// `toAliasFallback` 这一对薄壳，合并为单实现 + 槽位参数）：
/// - `Override`：配置解析前尝试，未命中不算错误（交回调用方继续走配置）；
/// - `Fallback`：配置解析不出别名时的最后兜底，未命中即报「非法别名」。
pub(crate) enum AliasSlot {
  Override,
  Fallback,
}

impl<'ctx, C: NavigationContextTrait, E: ErrorHandler> Navigator<'ctx, C, E> {
  pub fn new(navigation_context: &'ctx mut C, error_handler: &'ctx mut E) -> Navigator<'ctx, C, E> {
    Navigator {
      navigation_context,
      error_handler,
    }
  }

  /// `impl AsRef<[u8]>` 同时接受 `&[u8]`、`Vec<u8>`、`&str` 与 `String`
  /// （Lua 路径是字节串，ulua-analyze-cli 传 `String`），内部零拷贝借用。
  pub fn navigate(&mut self, path: impl AsRef<[u8]>) -> Status {
    let path = path.as_ref();

    // 对应 cpp `std::replace(path.begin(), path.end(), '\\', '/')`：等宽字节替换。
    // 无反斜杠时零拷贝借用原字节串，避免重新分配；有则单次遍历一次性生成新字节串。
    let normalized;
    let path = if path.contains(&PATH_SEPARATOR_ALT) {
      normalized = path
        .iter()
        .map(|&b| {
          if b == PATH_SEPARATOR_ALT {
            PATH_SEPARATOR
          } else {
            b
          }
        })
        .collect::<Vec<u8>>();
      &normalized
    } else {
      path
    };
    let error = self.navigate_impl(path);

    if let Some(error) = error {
      self.error_handler.report_error(error);
      return Status::ErrorReported;
    }

    Status::Success
  }

  pub(crate) fn navigate_impl(&mut self, path: &[u8]) -> Error {
    let path_type = get_path_type(path);

    if path_type == PathType::Unsupported {
      return Some(UNSUPPORTED_PREFIX.to_vec());
    }

    if matches!(
      path_type,
      PathType::RelativeToCurrent | PathType::RelativeToParent
    ) {
      // 相对路径：回到 requirer 的父级后逐组件遍历
      if let Some(error) = self.reset_to_requirer() {
        return Some(error);
      }
      if let Some(error) = self.navigate_to_parent(None) {
        return Some(error);
      }
      if let Some(error) = self.navigate_through_path(path) {
        return Some(error);
      }

      return None;
    }

    // cpp 主线（AliasOverrideOrderFix 已永久合入）：先回到 requirer 再尝试别名覆盖
    // 别名按 ASCII 字节转小写，与 cpp 逐字节 `('A'..='Z') -> +32` 等价
    let alias = extract_alias(path).to_ascii_lowercase();

    if let Some(error) = self.reset_to_requirer() {
      return Some(error);
    }

    // DFFlag `LuauSelfIsSelfAndAlwaysSelf`：开启时 `@self` 不经任何
    // embedder/用户别名覆盖，直接在 requirer 上下文向下遍历
    // （cpp RequireNavigator.cpp:80-92）
    if LuauSelfIsSelfAndAlwaysSelf.get() && alias == SELF_ALIAS {
      return self.navigate_through_path(path);
    }

    let (error, was_overridden) = self.try_alias(&alias, AliasSlot::Override);
    if error.is_some() {
      return error;
    }
    if was_overridden {
      return self.navigate_through_path(path);
    }

    let mut config = Config::default();
    if let Some(error) = self.navigate_to_and_populate_config(&alias, &mut config) {
      return Some(error);
    }

    if config.aliases.contains(&utf8_owned(&alias)) {
      if let Some(error) = self.navigate_to_alias(&alias, &config, AliasCycleTracker::new()) {
        return Some(error);
      }

      return self.navigate_through_path(path);
    }

    // DFFlag 开启时 `@self` 已在覆盖尝试前返回，绝不应走到这里（cpp 的 LUAU_ASSERT）；
    // 关闭时保留旧行为："@self" 在配置中没有该别名时，退回 requirer 上下文直接遍历
    if LuauSelfIsSelfAndAlwaysSelf.get() {
      ulua_common::LUAU_ASSERT!(alias != SELF_ALIAS);
    } else if alias == SELF_ALIAS {
      if let Some(error) = self.reset_to_requirer() {
        return Some(error);
      }

      return self.navigate_through_path(path);
    }

    if let Some(error) = self.try_alias(&alias, AliasSlot::Fallback).0 {
      return Some(error);
    }

    self.navigate_through_path(path)
  }

  pub(crate) fn navigate_through_path(&mut self, path: &[u8]) -> Error {
    let mut components = path.split(|&byte| byte == PATH_SEPARATOR);
    // 别名路径跳过首段别名：到别名的导航由调用方负责
    if path.first() == Some(&ALIAS_PREFIX) {
      let _ = components.next();
    }

    // 组件按字节零拷贝借用原路径，非 UTF-8 字节原样传给导航上下文；
    // 末尾空段与 '.' / 空组件同样跳过，与原逐段切分的行为一致
    let mut previous_component: Option<&[u8]> = None;
    for component in components {
      match component {
        // '.' 与空组件直接跳过
        b"." | b"" => continue,
        b".." => {
          if let Some(error) = self.navigate_to_parent(previous_component) {
            return Some(error);
          }
        }
        _ => {
          if let Some(error) = self.navigate_to_child(component) {
            return Some(error);
          }
        }
      }
      previous_component = Some(component);
    }

    None
  }

  pub(crate) fn reset_to_requirer(&mut self) -> Error {
    let result = self.navigation_context.reset_to_requirer();
    if result == NavigateResult::Success {
      return None;
    }

    Some(suffixed(RESET_FAILED, ambiguous(result)))
  }

  pub(crate) fn jump_to_alias(&mut self, alias_path: &[u8]) -> Error {
    let result = self.navigation_context.jump_to_alias(alias_path);
    if result == NavigateResult::Success {
      return None;
    }

    Some(quoted(JUMP_PREFIX, alias_path, ambiguous(result)))
  }

  pub(crate) fn navigate_to_parent(&mut self, previous_component: Option<&[u8]>) -> Error {
    let result = self.navigation_context.to_parent();
    if result == NavigateResult::Success {
      return None;
    }

    let suffix = ambiguous(result);
    Some(match previous_component {
      Some(component) => quoted(PARENT_OF_PREFIX, component, suffix),
      None => suffixed(PARENT_OF_REQUIRER, suffix),
    })
  }

  pub(crate) fn navigate_to_child(&mut self, component: &[u8]) -> Error {
    let result = self.navigation_context.to_child(component);
    if result == NavigateResult::Success {
      return None;
    }

    Some(quoted(CHILD_PREFIX, component, ambiguous(result)))
  }

  /// 进入嵌入方别名槽（cpp `toAliasOverride` / `toAliasFallback` 的合并单实现）：
  /// 返回（错误, 是否已跳转）。两槽仅在「NotFound 是否算错」与调用的上下文钩子
  /// 上不同，其余结果映射逐分支等价（Ambiguous 两槽都报 ambiguous 文案，
  /// `Fallback` 的成功/失败语义与原实现一致，调用方只取 `.0` 的 bool 无差别）。
  pub(crate) fn try_alias(&mut self, alias: &[u8], slot: AliasSlot) -> (Error, bool) {
    let result = match slot {
      AliasSlot::Override => self.navigation_context.to_alias_override(alias),
      AliasSlot::Fallback => self.navigation_context.to_alias_fallback(alias),
    };

    match result {
      NavigateResult::Success => (None, true),
      NavigateResult::NotFound => (
        // Override 未命中不是错误（继续走配置）；Fallback 未命中即非法别名
        matches!(slot, AliasSlot::Fallback).then(|| invalid_alias(alias, false)),
        false,
      ),
      NavigateResult::Ambiguous => (Some(invalid_alias(alias, true)), false),
    }
  }

  /// 沿配置别名链导航（对应 cpp `Navigator::navigateToAlias`）：命中相对路径直接
  /// 遍历；命中别名递归解析（环检测）；否则按绝对路径跳转。
  pub(crate) fn navigate_to_alias(
    &mut self,
    alias: &[u8],
    config: &Config,
    mut cycle_tracker: AliasCycleTracker,
  ) -> Error {
    // 别名表键为 `String`：见 `utf8_boundary` 的边界说明
    let key = utf8_owned(alias);

    // 单次查找：调用方（navigate_impl 与本函数的递归分支）都以 `contains` 前置，
    // 未命中即为不变量破坏——expect 不可达，保留作纵深防御。
    let value = config
      .aliases
      .find(&key)
      .expect("alias must exist")
      .value
      .as_bytes();
    let path_type = get_path_type(value);

    if matches!(
      path_type,
      PathType::RelativeToCurrent | PathType::RelativeToParent
    ) {
      return self.navigate_through_path(value);
    }

    if path_type == PathType::Aliased {
      if let Some(error) = cycle_tracker.add(alias.to_vec()) {
        return Some(error);
      }

      let next_alias = extract_alias(value);
      let next_key = utf8_owned(next_alias);

      let (error, was_overridden) = self.try_alias(next_alias, AliasSlot::Override);
      if error.is_some() {
        return error;
      }
      if was_overridden {
        return self.navigate_through_path(value);
      }

      if config.aliases.contains(&next_key) {
        if let Some(error) = self.navigate_to_alias(next_alias, config, cycle_tracker) {
          return Some(error);
        }
      } else {
        let mut parent_config = Config::default();
        if let Some(error) = self.navigate_to_and_populate_config(next_alias, &mut parent_config) {
          return Some(error);
        }

        if parent_config.aliases.contains(&next_key) {
          if let Some(error) =
            self.navigate_to_alias(next_alias, &parent_config, AliasCycleTracker::new())
          {
            return Some(error);
          }
        } else if let Some(error) = self.try_alias(next_alias, AliasSlot::Fallback).0 {
          return Some(error);
        }
      }

      return self.navigate_through_path(value);
    }

    // 其余视为绝对路径：直接跳转
    self.jump_to_alias(value)
  }

  pub(crate) fn navigate_to_and_populate_config(
    &mut self,
    desired_alias: &[u8],
    config: &mut Config,
  ) -> Error {
    // 别名表键为 `String`：循环内多次查表，故只转一次
    let desired_key = utf8_owned(desired_alias);

    while !config.aliases.contains(&desired_key) {
      *config = Config::default(); // 清空已有配置数据

      match self.navigation_context.to_parent() {
        NavigateResult::Ambiguous => {
          return Some(quoted(ANCESTRY_PREFIX, desired_alias, AMBIGUOUS_SUFFIX));
        }
        // NotFound 不视为错误：解释为到达根节点
        NavigateResult::NotFound => break,
        NavigateResult::Success => {}
      }

      let status = self.navigation_context.get_config_status();
      match status {
        ConfigStatus::Absent => continue,
        ConfigStatus::Ambiguous => {
          return Some(quoted(
            RESOLVE_ALIAS_PREFIX,
            desired_alias,
            AMBIGUOUS_CONFIG_SUFFIX,
          ));
        }
        ConfigStatus::PresentJson | ConfigStatus::PresentLuau => {}
      }

      // cpp 主线（null 检查已永久合入）：get_alias 缺失即报错，而非写入空别名
      if self.navigation_context.get_config_behavior() == ConfigBehavior::GetAlias {
        let Some(alias_path) = self.navigation_context.get_alias(desired_alias) else {
          return Some(quoted(RESOLVE_ALIAS_PREFIX, desired_alias, &[]));
        };
        // Config 的别名值为 `String`：非 UTF-8 别名路径字节在此退化（见 utf8_boundary）
        config.set_alias(desired_key, utf8_owned(&alias_path));
        break;
      }

      let Some(config_contents) = self.navigation_context.get_config() else {
        return Some(quoted(MISSING_CONTENTS_PREFIX, desired_alias, &[]));
      };

      let mut opts = ConfigOptions {
        alias_options: Some(AliasOptions {
          overwrite_aliases: false,
          ..Default::default()
        }),
        ..Default::default()
      };

      // 配置文件文本走 UTF-8 边界：合法 UTF-8 时零拷贝且与 cpp 解析结果一致
      let contents = utf8_view(&config_contents);

      if status == ConfigStatus::PresentJson {
        if let Err(error) = parse_config(&contents, config, &opts) {
          return Some(error.to_string().into_bytes());
        }
      } else {
        // PresentLuau
        let callbacks = InterruptCallbacks {
          init_callback: self.navigation_context.luau_config_init(),
          interrupt_callback: self.navigation_context.luau_config_interrupt(),
        };

        if let Err(error) =
          extract_luau_config(&contents, config, opts.alias_options.take(), callbacks)
        {
          return Some(error.to_string().into_bytes());
        }
      }
    }

    None
  }
}
