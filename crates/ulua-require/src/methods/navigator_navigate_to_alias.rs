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

impl<C: NavigationContextTrait, E: ErrorHandler> Navigator<'_, C, E> {
  pub(crate) fn navigate_to_alias(
    &mut self,
    alias: &[u8],
    config: &Config,
    mut cycle_tracker: AliasCycleTracker,
  ) -> Error {
    // 别名表键为 `String`：见 `utf8_boundary` 的边界说明
    let key = utf8_owned(alias);

    // 单次查找：调用方（navigate_impl 与本函数的递归分支）都以 `contains` 前置，
    // 命中不了即为不变量破坏
    let value = config
      .aliases
      .find(&key)
      .expect("alias must exist")
      .value
      .as_bytes();
    let path_type = get_path_type(value);

    if path_type == PathType::RelativeToCurrent || path_type == PathType::RelativeToParent {
      return self.navigate_through_path(value);
    }

    if path_type == PathType::Aliased {
      if let Some(error) = cycle_tracker.add(alias.to_vec()) {
        return Some(error);
      }

      let next_alias = extract_alias(value);
      let next_key = utf8_owned(next_alias);

      let (error, was_overridden) = self.navigate_to_alias_override(next_alias);
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
        } else if let Some(error) = self.navigate_to_alias_fallback(next_alias) {
          return Some(error);
        }
      }

      return self.navigate_through_path(value);
    }

    // 其余视为绝对路径：直接跳转
    self.jump_to_alias(value)
  }
}
