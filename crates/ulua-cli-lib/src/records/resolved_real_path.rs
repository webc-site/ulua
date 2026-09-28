use std::path::PathBuf;

use crate::enums::navigation_status::NavigationStatus;

#[derive(Debug, Clone)]
pub(crate) struct ResolvedRealPath {
  pub(crate) status: NavigationStatus,
  pub(crate) real_path: PathBuf,
}

impl ResolvedRealPath {
  pub const fn new(status: NavigationStatus, real_path: PathBuf) -> Self {
    Self { status, real_path }
  }
}
