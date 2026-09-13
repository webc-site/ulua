use alloc::string::String;

use crate::records::{error_handler::ErrorHandler, navigation_context::NavigationContextTrait};

/// 导航过程中的错误（对应 C++ `Navigator::Error`）。
pub(crate) type Error = Option<String>;

/// 路径导航器，持有导航上下文与错误报告器的可变引用。
pub struct Navigator<'ctx> {
  pub(crate) navigation_context: &'ctx mut dyn NavigationContextTrait,
  pub(crate) error_handler: &'ctx mut dyn ErrorHandler,
}
