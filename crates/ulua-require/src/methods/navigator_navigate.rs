use alloc::string::String;

use crate::{
  enums::status_require_navigator::Status,
  records::{
    error_handler::ErrorHandler, navigation_context::NavigationContextTrait, navigator::Navigator,
  },
};

impl<C: NavigationContextTrait, E: ErrorHandler> Navigator<'_, C, E> {
  pub fn navigate(&mut self, mut path: String) -> Status {
    // 对应 cpp `std::replace(path.begin(), path.end(), '\\', '/')`；
    // 无反斜杠时跳过重新分配。
    if path.contains('\\') {
      path = path.replace('\\', "/");
    }

    if let Some(error) = self.navigate_impl(&path) {
      self.error_handler.report_error(error);
      return Status::ErrorReported;
    }

    Status::Success
  }
}
