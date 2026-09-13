use alloc::string::String;

use crate::enums::status_require_impl::Status;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ResolvedRequire {
  pub(crate) status: Status,
  pub(crate) chunkname: String,
  pub(crate) loadname: String,
  pub(crate) cache_key: String,
  pub(crate) error: String,
}
