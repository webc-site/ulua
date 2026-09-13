use crate::records::require_suggester::RequireSuggester;

impl Drop for RequireSuggester {
  fn drop(&mut self) {
    // virtual ~RequireSuggester() {}
  }
}
