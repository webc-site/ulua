use alloc::string::String;

/// 错误报告接口，用于导航过程中报告错误（对应 C++ `ErrorHandler`）。
pub trait ErrorHandler {
  fn report_error(&mut self, message: String);
}
