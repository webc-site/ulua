use crate::{
  records::{
    file_resolver::{FileResolver, FileResolverVtable},
    source_code::SourceCode,
  },
  type_aliases::module_name_type::ModuleName,
};

#[derive(Debug)]
#[repr(C)]
pub struct NullFileResolver {
  pub base: FileResolver,
}

pub(crate) unsafe fn null_file_resolver_read_source(
  _this: *mut FileResolver,
  _name: &ModuleName,
) -> Option<SourceCode> {
  None
}

impl NullFileResolver {
  pub fn new() -> Self {
    use crate::records::file_resolver::{
      file_resolver_get_environment_for_module_default,
      file_resolver_get_human_readable_module_name_default, file_resolver_resolve_module_default,
    };

    let vtable = FileResolverVtable {
      read_source: null_file_resolver_read_source,
      resolve_module: file_resolver_resolve_module_default,
      get_human_readable_module_name: file_resolver_get_human_readable_module_name_default,
      get_environment_for_module: file_resolver_get_environment_for_module_default,
    };

    NullFileResolver {
      base: FileResolver {
        vtable,
        require_suggester: None,
      },
    }
  }
}

impl Default for NullFileResolver {
  fn default() -> Self {
    Self::new()
  }
}
