//! Node: `cxx:Record:Luau.Analysis:Analysis/include/Luau/ModuleResolver.h:51:null_module_resolver`
//! Source: `Analysis/include/Luau/ModuleResolver.h`
//!
//! C++ `struct NullModuleResolver : ModuleResolver` (ModuleResolver.h:51-69):
//! a stateless resolver that answers "module unknown" to every query. It has
//! no data members; the `ModuleResolver` interface overrides live as
//! `NullModuleResolver` methods (see `methods/null_module_resolver_*`).

#[derive(Debug, Clone, Copy, Default)]
pub struct NullModuleResolver;

impl NullModuleResolver {
  /// Construct the (stateless) null resolver.
  pub fn new() -> Self {
    NullModuleResolver
  }
}
