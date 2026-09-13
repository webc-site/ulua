use alloc::string::String;

use ulua_common::DFFlag;
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
  pub fn navigate_impl(&mut self, path: &str) -> Error {
    let path_type = get_path_type(path);

    if path_type == PathType::Unsupported {
      return Some(String::from(
        "require path must start with a valid prefix: ./, ../, or @",
      ));
    }

    if path_type == PathType::Aliased {
      let alias = extract_alias(path).to_ascii_lowercase();

      if DFFlag::LuauRequireAliasOverrideOrderFix.get()
        && let Some(error) = self.reset_to_requirer()
      {
        return Some(error);
      }

      let (error, was_overridden) = self.to_alias_override(&alias);
      if error.is_some() {
        return error;
      } else if was_overridden {
        if let Some(error) = self.navigate_through_path(path) {
          return Some(error);
        }

        return None;
      }

      if !DFFlag::LuauRequireAliasOverrideOrderFix.get()
        && let Some(error) = self.reset_to_requirer()
      {
        return Some(error);
      }

      let mut config = Config::default();
      if let Some(error) = self.navigate_to_and_populate_config(&alias, &mut config) {
        return Some(error);
      }

      if config.aliases.contains(&alias) {
        if let Some(error) = self.navigate_to_alias(&alias, &config, AliasCycleTracker::new()) {
          return Some(error);
        }
        if let Some(error) = self.navigate_through_path(path) {
          return Some(error);
        }

        return None;
      }

      if alias == "self" {
        if let Some(error) = self.reset_to_requirer() {
          return Some(error);
        }
        if let Some(error) = self.navigate_through_path(path) {
          return Some(error);
        }

        return None;
      }

      if let Some(error) = self.to_alias_fallback(&alias) {
        return Some(error);
      }
      if let Some(error) = self.navigate_through_path(path) {
        return Some(error);
      }

      return None;
    }

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

    None
  }
}
