#[cfg(feature = "luau_enable_time_trace")]
use crate::FFlag::DebugLuauTimeTracing;
use crate::records::thread_context::ThreadContext;

#[derive(Debug)]
pub struct Scope {
  pub(crate) context: *mut ThreadContext,
}

impl Drop for Scope {
  fn drop(&mut self) {
    // feature 关闭时 context 字段无读取路径；此读取保持 dead_code 检查通过。
    let _ = self.context;
    #[cfg(feature = "luau_enable_time_trace")]
    if DebugLuauTimeTracing.get() {
      unsafe {
        (*self.context).event_leave();
      }
    }
  }
}
