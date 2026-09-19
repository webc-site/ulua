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
  functions::{
    navigate_error::{AMBIGUOUS_SUFFIX, quoted},
    utf8_boundary::{utf8_owned, utf8_view},
  },
  records::{
    error_handler::ErrorHandler,
    navigation_context::NavigationContextTrait,
    navigator::{Error, Navigator},
  },
};

/// 向上遍历祖先链失败的消息前缀。
const ANCESTRY_PREFIX: &[u8] = b"could not navigate up the ancestry chain during search for alias ";
/// 别名无法解析 / 配置文件含糊不清的消息前缀与后缀。
const RESOLVE_ALIAS_PREFIX: &[u8] = b"could not resolve alias ";
const AMBIGUOUS_CONFIG_SUFFIX: &[u8] = b" (ambiguous configuration file)";
/// 取不到配置文件内容的消息前缀。
const MISSING_CONTENTS_PREFIX: &[u8] =
  b"could not get configuration file contents to resolve alias ";

impl<C: NavigationContextTrait, E: ErrorHandler> Navigator<'_, C, E> {
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
        if let Some(error) = parse_config(&contents, config, &opts) {
          return Some(error.into_bytes());
        }
      } else {
        // PresentLuau
        let callbacks = InterruptCallbacks {
          init_callback: self.navigation_context.luau_config_init(),
          interrupt_callback: self.navigation_context.luau_config_interrupt(),
        };

        if let Some(error) =
          extract_luau_config(&contents, config, opts.alias_options.take(), callbacks)
        {
          return Some(error.into_bytes());
        }
      }
    }

    None
  }
}
