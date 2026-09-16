//! Source: `Analysis/include/Luau/ConfigResolver.h:12` (hand-ported)
//! C++ abstract interface — modeled as a struct with a fn-pointer vtable slot
//! (the project convention for pure-virtual classes).

use ulua_config::{records::config::Config, type_aliases::module_name::ModuleName};

use crate::records::type_check_limits::TypeCheckLimits;

#[derive(Debug)]
pub struct ConfigResolver {
  /// virtual const Config& getConfig(const ModuleName&, const TypeCheckLimits&) const
  pub get_config: Option<
    unsafe fn(
      this: *const ConfigResolver,
      name: *const ModuleName,
      limits: *const TypeCheckLimits,
    ) -> *const Config,
  >,
}
