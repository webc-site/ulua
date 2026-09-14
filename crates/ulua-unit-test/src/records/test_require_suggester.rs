//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Record:Luau.UnitTest:tests/Fixture.h:69:test_require_suggester`
//! Source: `tests/Fixture.h`
//! Graph edges:
//! - declared_by: source_file tests/Fixture.h
//! - incoming:
//!   - type_ref <- method TestRequireSuggester::getNode (tests/Fixture.cpp)
//!   - type_ref <- method TestRequireSuggester::TestRequireSuggester (tests/Fixture.h)
//! - outgoing:
//!   - type_ref -> record RequireNode (Analysis/include/Luau/FileResolver.h)
//!   - translates_to -> rust_item TestRequireSuggester

use core::{cell::Cell, ptr::null};
use std::{boxed::Box, collections::HashMap};

use ulua_analysis::{
  records::{require_node::RequireNode, require_suggester::RequireSuggester},
  type_aliases::module_name_type::ModuleName,
};

use crate::records::test_require_node::TestRequireNode;

// C++ 里 `TestRequireSuggester` 经 thread-local 指针读取 `TestFileResolver` 的
// source 表；布线点为 `TestFileResolver::enable_require_suggester`。
thread_local! {
  static REQUIRE_SUGGESTER_SOURCES: Cell<*const HashMap<ModuleName, String>> =
    const { Cell::new(null()) };
}

/// 把 source 表地址布线给 `get_node`（`enable_require_suggester` 调用）。
pub(crate) fn wire_require_suggester_sources(sources: *const HashMap<ModuleName, String>) {
  REQUIRE_SUGGESTER_SOURCES.with(|cell| cell.set(sources));
}

#[derive(Debug, Clone, Default)]
pub struct TestRequireSuggester {
  _private: (),
}

impl RequireSuggester for TestRequireSuggester {
  /// C++ `TestRequireSuggester::getNode`：给出持有全量 source 表的测试节点。
  fn get_node(&self, name: &ModuleName) -> Option<Box<dyn RequireNode>> {
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
}
