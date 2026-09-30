use ulua_analysis::records::source_code::SourceCode;

use crate::records::test_file_resolver::TestFileResolver;

impl TestFileResolver {
  pub fn read_source(&mut self, name: &str) -> Option<SourceCode> {
    // `SourceTable::get` 已给出拥有值（表内部存的是 `String`），无需再 clone。
    let source = self.source.get(name)?;
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
