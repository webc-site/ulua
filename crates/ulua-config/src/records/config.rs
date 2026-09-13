use alloc::{string::String, vec::Vec};

use ulua_ast::{enums::mode::Mode, records::parse_options::ParseOptions};
use ulua_common::records::{dense_hash_map::DenseHashMap, dense_hash_table::DenseDefault};

use crate::records::{alias_info::AliasInfo, lint_options::LintOptions};

#[derive(Debug)]
pub struct Config {
  pub mode: Mode,
  pub parse_options: ParseOptions,
  pub enabled_lint: LintOptions,
  pub fatal_lint: LintOptions,
  pub lint_errors: bool,
  pub type_errors: bool,
  pub globals: Vec<String>,
  pub aliases: DenseHashMap<String, AliasInfo>,
}

impl Default for Config {
  fn default() -> Self {
    let mut result = Self {
      mode: Mode::Nonstrict,
      parse_options: ParseOptions::default(),
      enabled_lint: LintOptions::default(),
      fatal_lint: LintOptions::default(),
      lint_errors: false,
      type_errors: true,
      globals: Vec::new(),
      aliases: DenseHashMap::new(String::new()),
    };
    // 默认启用全部 lint 警告
    result.enabled_lint.set_defaults();
    result
  }
}

impl Clone for Config {
  fn clone(&self) -> Self {
    let mut result = Config::default();
    result.copy_from(self);
    result
  }

  fn clone_from(&mut self, source: &Self) {
    self.copy_from(source);
  }
}

impl Config {
  /// 对应 C++ 拷贝构造：别名经 `set_alias` 重建，保证小写键与 configLocation 一致。
  pub(crate) fn copy_from(&mut self, other: &Config) {
    self.mode = other.mode;
    self.parse_options = other.parse_options.clone();
    self.enabled_lint = other.enabled_lint;
    self.fatal_lint = other.fatal_lint;
    self.lint_errors = other.lint_errors;
    self.type_errors = other.type_errors;
    self.globals = other.globals.clone();
    self.aliases = DenseHashMap::new(String::new());

    for (_, alias_info) in other.aliases.iter() {
      if alias_info.config_location.is_empty() {
        self.set_alias(alias_info.original_case.clone(), alias_info.value.clone());
      } else {
        self.set_alias_with_location(
          alias_info.original_case.clone(),
          alias_info.value.clone(),
          &alias_info.config_location,
        );
      }
    }
  }

  /// 对应 C++ `operator=(const Config&)`：拷贝构造 + swap。
  pub fn config_assign(&mut self, other: &Config) -> &mut Self {
    self.copy_from(other);
    self
  }

  /// 语义同 `set_alias`，下游兼容入口。
  pub fn set_alias_simple(&mut self, alias: String, value: String) {
    self.set_alias(alias, value);
  }
}

impl DenseDefault for AliasInfo {
  fn dense_default() -> Self {
    Self::default()
  }
}
