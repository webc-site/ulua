//! `DemoConfigResolver::DemoConfigResolver()` (`CLI/src/Web.cpp:51-54`).
//!
//! ```cpp
//! DemoConfigResolver()
//! {
//!     defaultConfig.mode = Luau::Mode::Strict;
//! }
//! ```
//!
//! Wires the `ConfigResolver` vtable slot (`getConfig`) to the demo thunk.
//!
//! DELIBERATE DEVIATION from `Web.cpp`, which hard-sets `Mode::Strict`: the
//! playground defaults to `Nonstrict` so each script's own `--!strict` /
//! `--!nonstrict` mode comment governs (matching the `ulua-analyze` CLI). This
//! avoids type-checking unannotated example scripts under strict and reporting
//! findings that read, to a casual visitor, as the checker flagging clean code.

use ulua_analysis::records::config_resolver::ConfigResolver;
use ulua_ast::enums::mode::Mode;
use ulua_config::records::config::Config;

use crate::records::demo_config_resolver::{
  DemoConfigResolver, demo_config_resolver_get_config_thunk,
};

impl DemoConfigResolver {
  pub fn new() -> Self {
    Self::default()
  }
}

impl Default for DemoConfigResolver {
  fn default() -> Self {
    let default_config = Config {
      mode: Mode::Nonstrict,
      ..Default::default()
    };

    DemoConfigResolver {
      base: ConfigResolver {
        get_config: Some(demo_config_resolver_get_config_thunk),
      },
      default_config,
    }
  }
}
