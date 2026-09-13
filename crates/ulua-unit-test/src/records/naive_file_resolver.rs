use ulua_analysis::records::null_file_resolver::NullFileResolver;

use crate::methods::naive_file_resolver_resolve_module::naive_file_resolver_resolve_module_vtable;

#[derive(Debug)]
#[repr(C)]
pub struct NaiveFileResolver {
  pub base: NullFileResolver,
}

impl Default for NaiveFileResolver {
  fn default() -> Self {
    let mut base = NullFileResolver::new();
    base.base.vtable.resolve_module = naive_file_resolver_resolve_module_vtable;

    Self { base }
  }
}
