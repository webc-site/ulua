//! @interface-stub
use alloc::boxed::Box;

use ulua_analysis::records::require_node::RequireNode;

use crate::records::test_require_suggester::TestRequireSuggester;

impl TestRequireSuggester {
  pub fn get_node(&self, _name: &str) -> Box<dyn RequireNode> {
    // Dead duplicate skeleton node: `TestRequireSuggester` is an empty unused
    // record. The real getNode (building a `TestRequireNode`) lives in the
    // `RequireSuggester` vtable handler `test_require_suggester_get_node` in
    // `crates/luau-unit-test/src/records/test_file_resolver.rs`, wired by
    // `TestFileResolver::enable_require_suggester`. No call site here.
    unreachable!(
      "canonical getNode lives in records/test_file_resolver.rs (RequireSuggester vtable); this skeleton node is unused"
    );
  }
}
