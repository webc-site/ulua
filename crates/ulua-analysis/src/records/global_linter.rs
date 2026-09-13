/// C++ `LintContext::Global` (`Analysis/src/Linter.cpp:22`).
///
/// ```cpp
/// struct Global
/// {
///     TypeId type = nullptr;
///     std::optional<const char*> deprecated;
/// };
/// ```
use alloc::string::String;
use core::ptr::null;

use crate::type_aliases::type_id::TypeId;
#[derive(Debug, Clone)]
pub struct Global {
  pub(crate) r#type: TypeId,
  pub(crate) deprecated: Option<String>,
}

impl Default for Global {
  fn default() -> Self {
    Self {
      r#type: null(),
      deprecated: None,
    }
  }
}
