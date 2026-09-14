//! @interface-stub
use ulua_analysis::records::{frontend::Frontend, module_resolver::ModuleResolver};

// Dead FragmentAutocomplete test-fixture scaffolding (C++
// `FragmentAutocomplete.test.cpp:49 getModuleResolver(Frontend&)`): it would
// return `frontend.module_resolver` or `module_resolver_for_autocomplete` keyed
// on `FFlag::DebugLuauForceOldSolver`. Those fields are `FrontendModuleResolver`,
// not the `ModuleResolver` base this generated signature returns, and no
// translated test calls this node (the live `get_module_resolver` is the
// unrelated luau-analysis function). Unused skeleton node.
pub fn get_module_resolver(_frontend: &mut Frontend) -> &mut ModuleResolver {
  unreachable!(
    "dead FragmentAutocomplete test fixture: no call site; signature does not match Frontend's FrontendModuleResolver fields"
  )
}
