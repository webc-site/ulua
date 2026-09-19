use alloc::vec::Vec;

use crate::records::{error_handler::ErrorHandler, navigation_context::NavigationContextTrait};

/// 导航过程中的错误（对应 C++ `Navigator::Error = std::optional<std::string>`）：
/// 消息内嵌路径/别名字节，故用 `Vec<u8>` 保持与 cpp 一致的字节拼接语义。
pub(crate) type Error = Option<Vec<u8>>;

/// 路径导航器，持有导航上下文与错误报告器的可变引用。
/// 用泛型参数替代 `dyn`，静态分派消除虚调用开销。
pub struct Navigator<'ctx, C: NavigationContextTrait, E: ErrorHandler> {
  pub(crate) navigation_context: &'ctx mut C,
  pub(crate) error_handler: &'ctx mut E,
}
