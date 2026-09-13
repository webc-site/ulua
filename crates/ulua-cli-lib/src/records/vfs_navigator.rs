use alloc::string::String;

#[derive(Debug, Clone, Default)]
pub struct VfsNavigator {
  pub(crate) real_path: String,
  pub(crate) absolute_real_path: String,
  pub(crate) absolute_path_prefix: String,
  pub(crate) module_path: String,
  pub(crate) absolute_module_path: String,
}
