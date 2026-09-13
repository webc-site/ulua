use ulua_config::records::config::Config;

use crate::{
  enums::path_type::PathType,
  functions::{extract_alias::extract_alias, get_path_type::get_path_type},
  records::{
    alias_cycle_tracker::AliasCycleTracker,
    navigator::{Error, Navigator},
  },
};

impl Navigator<'_> {
  pub fn navigate_to_alias(
    &mut self,
    alias: &str,
    config: &Config,
    mut cycle_tracker: AliasCycleTracker,
  ) -> Error {
    let alias_string = alias.to_string();
    debug_assert!(config.aliases.contains(&alias_string));

    let value = config
      .aliases
      .find(&alias_string)
      .expect("alias must exist")
      .value
      .clone();
    let path_type = get_path_type(&value);

    if path_type == PathType::RelativeToCurrent || path_type == PathType::RelativeToParent {
      if let Some(error) = self.navigate_through_path(&value) {
        return Some(error);
      }
    } else if path_type == PathType::Aliased {
      if let Some(error) = cycle_tracker.add(alias_string) {
        return Some(error);
      }

      let next_alias = extract_alias(&value);

      let (error, was_overridden) = self.to_alias_override(next_alias);
      if error.is_some() {
        return error;
      } else if was_overridden {
        if let Some(error) = self.navigate_through_path(&value) {
          return Some(error);
        }

        return None;
      }

      let next_alias_str = next_alias.to_string();
      if config.aliases.contains(&next_alias_str) {
        if let Some(error) = self.navigate_to_alias(next_alias, config, cycle_tracker) {
          return Some(error);
        }
      } else {
        let mut parent_config = Config::default();
        if let Some(error) = self.navigate_to_and_populate_config(next_alias, &mut parent_config) {
          return Some(error);
        }

        if parent_config.aliases.contains(&next_alias_str) {
          if let Some(error) =
            self.navigate_to_alias(next_alias, &parent_config, AliasCycleTracker::new())
          {
            return Some(error);
          }
        } else if let Some(error) = self.to_alias_fallback(next_alias) {
          return Some(error);
        }
      }

      if let Some(error) = self.navigate_through_path(&value) {
        return Some(error);
      }
    } else if let Some(error) = self.jump_to_alias(&value) {
      return Some(error);
    }

    None
  }
}
