use ulua_config::records::config::Config;

use crate::{
  enums::path_type::PathType,
  functions::{
    extract_alias::extract_alias, get_path_type::get_path_type, utf8_boundary::utf8_owned,
  },
  records::{
    alias_cycle_tracker::AliasCycleTracker,
    error_handler::ErrorHandler,
    navigation_context::NavigationContextTrait,
    navigator::{Error, Navigator},
  },
};

/// cpp 固定文案：路径前缀非法。
const UNSUPPORTED_PREFIX: &[u8] = b"require path must start with a valid prefix: ./, ../, or @";
/// cpp `alias == "self"` 的字节比较基准。
const SELF_ALIAS: &[u8] = b"self";

impl<C: NavigationContextTrait, E: ErrorHandler> Navigator<'_, C, E> {
  pub(crate) fn navigate_impl(&mut self, path: &[u8]) -> Error {
    let path_type = get_path_type(path);

    if path_type == PathType::Unsupported {
      return Some(UNSUPPORTED_PREFIX.to_vec());
    }

    if path_type != PathType::Aliased {
      // 相对路径：回到 requirer 的父级后逐组件遍历
      if path_type == PathType::RelativeToCurrent || path_type == PathType::RelativeToParent {
        if let Some(error) = self.reset_to_requirer() {
          return Some(error);
        }
        if let Some(error) = self.navigate_to_parent(None) {
          return Some(error);
        }
        if let Some(error) = self.navigate_through_path(path) {
          return Some(error);
        }
      }

      return None;
    }

    // cpp 主线（AliasOverrideOrderFix 已永久合入）：先回到 requirer 再尝试别名覆盖
    // 别名按 ASCII 字节转小写，与 cpp 逐字节 `('A'..='Z') -> +32` 等价
    let alias = extract_alias(path).to_ascii_lowercase();

    if let Some(error) = self.reset_to_requirer() {
      return Some(error);
    }

    let (error, was_overridden) = self.navigate_to_alias_override(&alias);
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

    // "@self"：配置中没有该别名时，退回 requirer 上下文直接遍历
    if alias == SELF_ALIAS {
      if let Some(error) = self.reset_to_requirer() {
        return Some(error);
      }

      return self.navigate_through_path(path);
    }

    if let Some(error) = self.navigate_to_alias_fallback(&alias) {
      return Some(error);
    }

    self.navigate_through_path(path)
  }
}
