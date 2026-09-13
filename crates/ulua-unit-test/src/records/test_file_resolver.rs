//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Record:Luau.UnitTest:tests/Fixture.h:80:test_file_resolver`
//! Source: `tests/Fixture.h`
//! Graph edges:
//! - declared_by: source_file tests/Fixture.h
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/BuiltinTypeFunctions.h
//!   - includes -> source_file Config/include/Luau/Config.h
//!   - includes -> source_file Analysis/include/Luau/Error.h
//!   - includes -> source_file Analysis/include/Luau/FileResolver.h
//!   - includes -> source_file Analysis/include/Luau/Frontend.h
//!   - includes -> source_file Analysis/include/Luau/IostreamHelpers.h
//!   - includes -> source_file Analysis/include/Luau/Linter.h
//!   - includes -> source_file Ast/include/Luau/Location.h
//!   - includes -> source_file Analysis/include/Luau/ModuleResolver.h
//!   - includes -> source_file Analysis/include/Luau/Scope.h
//!   - includes -> source_file Analysis/include/Luau/ToString.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file Analysis/include/Luau/TypeFunction.h
//!   - includes -> source_file tests/IostreamOptional.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/Fixture.h
//!   - type_ref <- method TestRequireSuggester::TestRequireSuggester (tests/Fixture.h)
//!   - type_ref <- record TestRequireSuggester (tests/Fixture.h)
//!   - type_ref <- record Fixture (tests/Fixture.h)
//!   - type_ref <- test frontend_check_without_builtin_next (tests/Frontend.test.cpp)
//!   - type_ref <- record RequireTracerFixture (tests/RequireTracer.test.cpp)
//!   - type_ref <- method TestFileResolver::resolveModuleInfo (tests/Fixture.cpp)
//!   - type_ref <- method TestFileResolver::getModule (tests/Fixture.cpp)
//!   - type_ref <- method TestFileResolver::moduleExists (tests/Fixture.cpp)
//!   - type_ref <- method TestFileResolver::readSource (tests/Fixture.cpp)
//!   - type_ref <- method TestFileResolver::resolveModule (tests/Fixture.cpp)
//!   - type_ref <- method TestFileResolver::getHumanReadableModuleName (tests/Fixture.cpp)
//!   - type_ref <- method TestFileResolver::getEnvironmentForModule (tests/Fixture.cpp)
//!   - type_ref <- method TestFileResolver::TestFileResolver (tests/Fixture.h)
//! - outgoing:
//!   - type_ref -> method TestFileResolver::TestFileResolver (tests/Fixture.h)
//!   - type_ref -> record FileResolver (Analysis/include/Luau/FileResolver.h)
//!   - type_ref -> record ModuleResolver (Analysis/include/Luau/ModuleResolver.h)
//!   - type_ref -> record ModuleInfo (Analysis/include/Luau/FileResolver.h)
//!   - type_ref -> record AstExpr (Ast/include/Luau/Ast.h)
//!   - type_ref -> record SourceCode (Analysis/include/Luau/FileResolver.h)
//!   - type_ref -> record TypeCheckLimits (Analysis/include/Luau/TypeCheckLimits.h)
//!   - translates_to -> rust_item TestFileResolver

use core::ptr::null;
use std::{boxed::Box, cell::Cell, collections::HashMap, sync::Arc};

use ulua_analysis::{
  enums::type_file_resolver::Type as SourceCodeType,
  records::{
    file_resolver::{FileResolver, FileResolverVtable},
    module_info::ModuleInfo,
    require_node::RequireNode,
    require_suggester::{RequireSuggester, RequireSuggesterVtable},
    source_code::SourceCode,
    type_check_limits::TypeCheckLimits,
  },
  type_aliases::module_name_type::ModuleName,
};
use ulua_ast::records::ast_expr::AstExpr;

use crate::records::test_require_node::TestRequireNode;
std::thread_local! {
    static REQUIRE_SUGGESTER_SOURCES: Cell<*const HashMap<ModuleName, String>> =
        const { Cell::new(null()) };
}

#[derive(Debug)]
#[repr(C)]
pub struct TestFileResolver {
  pub base: FileResolver,
  pub source: HashMap<ModuleName, String>,
  pub source_types: HashMap<ModuleName, SourceCodeType>,
  pub environments: HashMap<ModuleName, String>,
}

impl Default for TestFileResolver {
  fn default() -> Self {
    Self {
      base: FileResolver {
        vtable: FileResolverVtable {
          read_source: test_file_resolver_read_source,
          resolve_module: test_file_resolver_resolve_module,
          get_human_readable_module_name: test_file_resolver_get_human_readable_module_name,
          get_environment_for_module: test_file_resolver_get_environment_for_module,
        },
        require_suggester: None,
      },
      source: HashMap::new(),
      source_types: HashMap::new(),
      environments: HashMap::new(),
    }
  }
}

impl TestFileResolver {
  pub fn enable_require_suggester(&mut self) {
    REQUIRE_SUGGESTER_SOURCES.with(|sources| {
      sources.set(&self.source as *const HashMap<ModuleName, String>);
    });

    if self.base.require_suggester.is_none() {
      self.base.require_suggester = Some(Arc::new(RequireSuggester {
        vtable: RequireSuggesterVtable {
          get_node: test_require_suggester_get_node,
        },
      }));
    }
  }
}

unsafe fn test_require_suggester_get_node(
  _this: *const RequireSuggester,
  name: &ModuleName,
) -> Option<Box<dyn RequireNode>> {
  REQUIRE_SUGGESTER_SOURCES.with(|sources| {
    let all_sources = sources.get();
    if all_sources.is_null() {
      None
    } else {
      Some(Box::new(TestRequireNode {
        module_name: name.clone(),
        all_sources,
      }) as Box<dyn RequireNode>)
    }
  })
}

unsafe fn test_file_resolver_read_source(
  this: *mut FileResolver,
  name: &ModuleName,
) -> Option<SourceCode> {
  let resolver = this as *mut TestFileResolver;
  unsafe { (*resolver).read_source(name) }
}

unsafe fn test_file_resolver_resolve_module(
  this: *mut FileResolver,
  context: *const ModuleInfo,
  expr: *mut AstExpr,
  limits: &TypeCheckLimits,
) -> Option<ModuleInfo> {
  if expr.is_null() {
    return None;
  }

  let resolver = this as *const TestFileResolver;
  let context = if context.is_null() {
    None
  } else {
    Some(unsafe { &*context })
  };

  unsafe { (*resolver).resolve_module(context, &*expr, limits) }
}

unsafe fn test_file_resolver_get_human_readable_module_name(
  this: *const FileResolver,
  name: &ModuleName,
) -> String {
  let resolver = this as *const TestFileResolver;
  unsafe { (*resolver).get_human_readable_module_name(name) }
}

unsafe fn test_file_resolver_get_environment_for_module(
  this: *const FileResolver,
  name: &ModuleName,
) -> Option<String> {
  let resolver = this as *const TestFileResolver;
  unsafe { (*resolver).get_environment_for_module(name) }
}
