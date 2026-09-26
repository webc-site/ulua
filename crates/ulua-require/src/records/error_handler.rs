use alloc::vec::Vec;

/// 错误报告接口，用于导航过程中报告错误（对应 C++ `ErrorHandler`）。
/// 消息是字节串（对应 cpp `std::string`），可能含非 UTF-8 的路径字节。
pub trait ErrorHandler {
  fn report_error(&mut self, message: Vec<u8>);
}
