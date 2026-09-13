use crate::records::{
  file_resolver::{
    FileResolver, FileResolverVtable, file_resolver_get_environment_for_module_default,
    file_resolver_get_human_readable_module_name_default, file_resolver_resolve_module_default,
  },
  source_code::SourceCode,
};

impl FileResolver {
  pub fn new() -> Self {
    Self {
      vtable: FileResolverVtable {
        read_source: |_, _| -> Option<SourceCode> { panic!("read_source is pure virtual") },
        resolve_module: file_resolver_resolve_module_default,
        get_human_readable_module_name: file_resolver_get_human_readable_module_name_default,
        get_environment_for_module: file_resolver_get_environment_for_module_default,
      },
      require_suggester: None,
    }
  }
}

impl Default for FileResolver {
  fn default() -> Self {
    Self::new()
  }
}
