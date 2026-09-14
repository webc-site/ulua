use alloc::string::String;

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
  records::{
    error_handler::ErrorHandler,
    navigation_context::NavigationContextTrait,
    navigator::{Error, Navigator},
  },
};

impl<C: NavigationContextTrait, E: ErrorHandler> Navigator<'_, C, E> {
  pub fn navigate_to_and_populate_config(
    &mut self,
    desired_alias: &str,
    config: &mut Config,
  ) -> Error {
    let desired_alias = String::from(desired_alias);

    while !config.aliases.contains(&desired_alias) {
      *config = Config::default(); // 清空已有配置数据

      match self.navigation_context.to_parent() {
        NavigateResult::Ambiguous => {
          return Some(format!(
            "could not navigate up the ancestry chain during search for alias \"{desired_alias}\" (ambiguous)"
          ));
        }
        // NotFound 不视为错误：解释为到达根节点
        NavigateResult::NotFound => break,
        NavigateResult::Success => {}
      }

      let status = self.navigation_context.get_config_status();
      match status {
        ConfigStatus::Absent => continue,
        ConfigStatus::Ambiguous => {
          return Some(format!(
            "could not resolve alias \"{desired_alias}\" (ambiguous configuration file)"
          ));
        }
        ConfigStatus::PresentJson | ConfigStatus::PresentLuau => {}
      }

      // cpp 主线（null 检查已永久合入）：get_alias 缺失即报错，而非写入空别名
      if self.navigation_context.get_config_behavior() == ConfigBehavior::GetAlias {
        let Some(alias_path) = self.navigation_context.get_alias(&desired_alias) else {
          return Some(format!("could not resolve alias \"{desired_alias}\""));
        };
        config.set_alias(desired_alias, alias_path);
        break;
      }

      let Some(config_contents) = self.navigation_context.get_config() else {
        return Some(format!(
          "could not get configuration file contents to resolve alias \"{desired_alias}\""
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
      } else {
        // PresentLuau
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

    None
  }
}
