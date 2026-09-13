//! Port of `DemoConfigResolver : Luau::ConfigResolver` (`CLI/src/Web.cpp:49-62`).
//!
//! The luau.org/demo config resolver. Defaults to `Mode::Nonstrict` so each
//! script's own mode comment governs (a deliberate playground deviation from
//! `Web.cpp`'s hard `Strict`; see `methods/demo_config_resolver_demo_config_resolver`).
//! Like the
//! analysis `ConfigResolver` (a struct with a fn-pointer vtable slot for the
//! single pure virtual `getConfig`), this concrete subclass is `#[repr(C)]` with
//! `base: ConfigResolver` first so a `*const ConfigResolver` (the vtable
//! receiver) can be cast back to `*const DemoConfigResolver` to reach
//! `default_config`.
//!
//! C++ member: `Luau::Config defaultConfig;`.

use ulua_analysis::records::{config_resolver::ConfigResolver, type_check_limits::TypeCheckLimits};
use ulua_config::{records::config::Config, type_aliases::module_name::ModuleName};

#[repr(C)]
#[derive(Debug)]
pub struct DemoConfigResolver {
  pub base: ConfigResolver,
  pub default_config: Config,
}

/// `const Config& getConfig(const ModuleName&, const TypeCheckLimits&) const` thunk.
///
/// # Safety
/// `this` must point at the `base` subobject of a live `DemoConfigResolver`.
pub(crate) unsafe fn demo_config_resolver_get_config_thunk(
  this: *const ConfigResolver,
  name: *const ModuleName,
  limits: *const TypeCheckLimits,
) -> *const Config {
  unsafe {
    let this = this as *const DemoConfigResolver;
    (*this).get_config(&*name, &*limits) as *const Config
  }
}
