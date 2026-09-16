//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Method:Luau.UnitTest:tests/Fixture.cpp:254:test_config_resolver_get_config`
//! Source: `tests/Fixture.cpp`
//! Graph edges:
//! - declared_by: source_file tests/Fixture.cpp
//! - source_includes:
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file Analysis/include/Luau/AstQuery.h
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file Common/include/Luau/Common.h
//!   - includes -> source_file Analysis/include/Luau/Constraint.h
//!   - includes -> source_file Analysis/include/Luau/FileResolver.h
//!   - includes -> source_file Analysis/include/Luau/ModuleResolver.h
//!   - includes -> source_file Ast/include/Luau/Parser.h
//!   - includes -> source_file Ast/include/Luau/PrettyPrinter.h
//!   - includes -> source_file Analysis/include/Luau/Subtyping.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file Analysis/include/Luau/TypeAttach.h
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//! - incoming:
//!   - declares <- source_file tests/Fixture.cpp
//! - outgoing:
//!   - type_ref -> record Config (Config/include/Luau/Config.h)
//!   - type_ref -> record TypeCheckLimits (Analysis/include/Luau/TypeCheckLimits.h)
//!   - type_ref -> record TestConfigResolver (tests/Fixture.h)
//!   - translates_to -> rust_item TestConfigResolver::getConfig

use ulua_config::{records::config::Config, type_aliases::module_name::ModuleName};

use crate::records::test_config_resolver::TestConfigResolver;

impl TestConfigResolver {
  pub fn get_config(&self, name: &ModuleName) -> &Config {
    self.config_files.get(name).unwrap_or(&self.default_config)
  }
}
