use core::ffi::{c_char, c_void};

use crate::{
  enums::{luarequire_navigate_result::luarequire_NavigateResult, navigate_result::NavigateResult},
  functions::{c_str_prefix::with_c_str, convert_navigate_result::convert_navigate_result},
  records::runtime_navigation_context::RuntimeNavigationContext,
};

/// 带一个 C 字符串输入的导航回调形态（jump_to_alias/to_alias_override/
/// to_alias_fallback/to_child 共用，对应 cpp Navigation.cpp 同一调用样板）。
pub(crate) type NavWithInputFn =
  unsafe extern "C-unwind" fn(*mut c_void, *mut c_void, *const c_char) -> luarequire_NavigateResult;

impl RuntimeNavigationContext<'_> {
  /// C 字符串入参的 C 回调公共调用：配置为空或回调缺失时返回 NotFound。
  /// 输入按字节串传递，只在真 FFI 边界经 `with_c_str` 补 NUL（短名零分配）。
  pub(crate) fn call_with_c_str(
    &self,
    nav: Option<NavWithInputFn>,
    input: &[u8],
  ) -> NavigateResult {
    unsafe { self.config.as_ref() }
      .and(nav)
      .map_or(NavigateResult::NotFound, |nav| {
        convert_navigate_result(with_c_str(input, |input| unsafe {
          nav(self.l, self.ctx, input)
        }))
      })
  }
}
