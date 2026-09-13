use alloc::string::String;

use ulua_common::FFlag::LuauRequireResolveAliasNullCheck;
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
  },
  records::navigator::{Error, Navigator},
};

impl Navigator<'_> {
  pub fn navigate_to_and_populate_config(
    &mut self,
    desired_alias: &str,
    config: &mut Config,
  ) -> Error {
    let desired_alias = String::from(desired_alias);

    while !config.aliases.contains(&desired_alias) {
      *config = Config::default();

      let result = self.navigation_context.to_parent();
      if result == NavigateResult::Ambiguous {
        return Some(format!(
          "could not navigate up the ancestry chain during search for alias \"{}\" (ambiguous)",
          desired_alias
        ));
      }
      if result == NavigateResult::NotFound {
        break;
      }

      let status = self.navigation_context.get_config_status();
      if status == ConfigStatus::Absent {
        continue;
      } else if status == ConfigStatus::Ambiguous {
        return Some(format!(
          "could not resolve alias \"{}\" (ambiguous configuration file)",
          desired_alias
        ));
      } else {
        if self.navigation_context.get_config_behavior() == ConfigBehavior::GetAlias {
          let alias_path = self.navigation_context.get_alias(&desired_alias);
          if LuauRequireResolveAliasNullCheck.get() && alias_path.is_none() {
            return Some(format!("could not resolve alias \"{}\"", desired_alias));
          }

          config.set_alias_simple(desired_alias.clone(), alias_path.unwrap_or_default());
          break;
        }

        let Some(config_contents) = self.navigation_context.get_config() else {
          return Some(format!(
            "could not get configuration file contents to resolve alias \"{}\"",
            desired_alias
          ));
        };

        let mut opts = ConfigOptions {
          alias_options: Some(AliasOptions {
            overwrite_aliases: false,
            ..Default::default()
          }),
          ..Default::default()
        };

        if status == ConfigStatus::PresentJson {
          if let Some(error) = parse_config(&config_contents, config, &opts) {
            return Some(error);
          }
        } else if status == ConfigStatus::PresentLuau {
          let callbacks = InterruptCallbacks {
            init_callback: self.navigation_context.luau_config_init(),
            interrupt_callback: self.navigation_context.luau_config_interrupt(),
          };

          if let Some(error) = extract_luau_config(
            &config_contents,
            config,
            opts.alias_options.take(),
            callbacks,
          ) {
            return Some(error);
          }
        }
      }
    }

    None
  }
}
