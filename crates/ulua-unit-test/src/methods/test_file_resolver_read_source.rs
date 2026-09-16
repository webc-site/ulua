use ulua_analysis::records::source_code::SourceCode;

use crate::records::test_file_resolver::TestFileResolver;

impl TestFileResolver {
  pub fn read_source(&mut self, name: &str) -> Option<SourceCode> {
    let source = self.source.get(name)?.clone();
    let source_type = self
      .source_types
      .get(name)
      .copied()
      .unwrap_or(SourceCode::MODULE);

    Some(SourceCode {
      source,
      r#type: source_type,
    })
  }
}
