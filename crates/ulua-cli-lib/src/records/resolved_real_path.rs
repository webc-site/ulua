use alloc::string::String;

use crate::enums::navigation_status::NavigationStatus;

#[derive(Debug, Clone)]
pub struct ResolvedRealPath {
  pub(crate) status: NavigationStatus,
  pub(crate) real_path: String,
}

impl ResolvedRealPath {
  pub const fn new(status: NavigationStatus, real_path: String) -> Self {
    Self { status, real_path }
  }
}
