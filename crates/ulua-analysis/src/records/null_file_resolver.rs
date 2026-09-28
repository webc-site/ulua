use crate::{
  records::{file_resolver::FileResolver, source_code::SourceCode},
  type_aliases::module_name_type::ModuleName,
};

/// C++ `struct NullFileResolver : FileResolver`
/// （`Analysis/include/Luau/FileResolver.h`）：不解析任何文件的空 resolver。
#[derive(Debug, Default)]
pub struct NullFileResolver;

impl NullFileResolver {
  pub fn new() -> Self {
    Self
  }
}

impl FileResolver for NullFileResolver {
  /// `readSource` 覆写：恒返回 `nullopt`。
  fn read_source(&mut self, _name: &ModuleName) -> Option<SourceCode> {
    None
  }
}
