use alloc::string::String;

use ulua_analysis::records::type_check_limits::TypeCheckLimits;

#[derive(Debug, Clone)]
pub struct LuauConfigInterruptInfo {
  pub(crate) limits: TypeCheckLimits,
  pub(crate) module: String,
}
