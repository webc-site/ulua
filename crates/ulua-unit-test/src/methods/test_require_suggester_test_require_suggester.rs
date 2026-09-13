use crate::records::test_require_suggester::TestRequireSuggester;
impl TestRequireSuggester {
  pub fn test_require_suggester_test_require_suggester(&mut self) {
    // Dead duplicate skeleton ctor: `TestRequireSuggester` is an empty unused
    // record; the test require-suggester is built directly as a
    // `RequireSuggester` vtable in `TestFileResolver::enable_require_suggester`
    // (`crates/luau-unit-test/src/records/test_file_resolver.rs`). No call site.
    unreachable!(
      "test require-suggester is built as a RequireSuggester vtable in records/test_file_resolver.rs; this skeleton node is unused"
    );
  }
}
