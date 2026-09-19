use alloc::vec::Vec;

use crate::{
  enums::status_require_navigator::Status,
  functions::split_path::{PATH_SEPARATOR, PATH_SEPARATOR_ALT},
  records::{
    error_handler::ErrorHandler, navigation_context::NavigationContextTrait, navigator::Navigator,
  },
};

impl<C: NavigationContextTrait, E: ErrorHandler> Navigator<'_, C, E> {
  /// `impl AsRef<[u8]>` 同时接受 `&[u8]`、`Vec<u8>`、`&str` 与 `String`
  /// （Lua 路径是字节串，ulua-analyze-cli 传 `String`），内部零拷贝借用。
  pub fn navigate(&mut self, path: impl AsRef<[u8]>) -> Status {
    let path = path.as_ref();

    // 对应 cpp `std::replace(path.begin(), path.end(), '\\', '/')`：等宽字节替换。
    // 无反斜杠时零拷贝借用原字节串，避免重新分配；有则单次遍历一次性生成新字节串。
    let error = if path.contains(&PATH_SEPARATOR_ALT) {
      let normalized = path
        .iter()
        .map(|&b| {
          if b == PATH_SEPARATOR_ALT {
            PATH_SEPARATOR
          } else {
            b
          }
        })
        .collect::<Vec<u8>>();
      self.navigate_impl(&normalized)
    } else {
      self.navigate_impl(path)
    };

    if let Some(error) = error {
      self.error_handler.report_error(error);
      return Status::ErrorReported;
    }

    Status::Success
  }
}
