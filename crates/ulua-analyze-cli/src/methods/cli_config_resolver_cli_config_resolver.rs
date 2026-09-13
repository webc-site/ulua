use alloc::vec::Vec;
use std::collections::HashMap;

use ulua_analysis::records::config_resolver::ConfigResolver;
use ulua_ast::enums::mode::Mode;
use ulua_config::records::config::Config;

use crate::{
  methods::cli_config_resolver_get_config::cli_config_resolver_get_config_thunk,
  records::cli_config_resolver::CliConfigResolver,
};
impl CliConfigResolver {
  /// C++ `CliConfigResolver(Luau::Mode mode) { defaultConfig.mode = mode; }`
  /// (`CLI/src/Analyze.cpp:238-241`).
  pub fn new(mode: Mode) -> Self {
    let default_config = Config {
      mode,
      ..Default::default()
    };

    CliConfigResolver {
      base: ConfigResolver {
        get_config: Some(cli_config_resolver_get_config_thunk),
      },
      default_config,
      config_cache: HashMap::new(),
      config_errors: Vec::new(),
    }
  }
}
