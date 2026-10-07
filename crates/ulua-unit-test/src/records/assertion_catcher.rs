use core::sync::atomic::AtomicI32;

use ulua_common::type_aliases::assert_handler::AssertHandler;

pub static ASSERTION_CATCHER_TRIPPED: AtomicI32 = AtomicI32::new(0);

/// 安装/还原 `LUAU_ASSERT` 钩子的 RAII 夹具：进入时把断言计数清零并挂上自己的
/// 处理器，退出时还原被替换掉的旧处理器。
#[derive(Debug, Clone)]
pub struct AssertionCatcher {
  /// 被本夹具替换掉的旧断言处理器，`Drop` 时原样还原。
  pub previous: AssertHandler,
}
