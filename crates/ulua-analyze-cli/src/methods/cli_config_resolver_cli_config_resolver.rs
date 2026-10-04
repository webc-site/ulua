use alloc::{
  rc::Rc,
  vec::Vec,
};
use core::cell::{RefCell, UnsafeCell};

use ulua_analysis::type_aliases::collections::HashMap;
use ulua_ast::enums::mode::Mode;
use ulua_config::records::config::Config;

use crate::records::cli_config_resolver::CliConfigResolver;
impl CliConfigResolver {
  /// C++ `CliConfigResolver(Luau::Mode mode) { defaultConfig.mode = mode; }`
  /// (`CLI/src/Analyze.cpp:238-241`).
  pub fn new(mode: Mode) -> Self {
    let default_config = Config {
      mode,
      ..Default::default()
    };

    CliConfigResolver {
      default_config,
      config_cache: UnsafeCell::new(HashMap::new()),
      config_errors: Rc::new(RefCell::new(Vec::new())),
    }
  }
}
