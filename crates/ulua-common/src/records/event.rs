use crate::enums::event_type::EventType;

/// cpp `struct Event`（`Common/include/Luau/TimeTrace.h:44-54`）。
///
/// 上游的 `data` 是 `{ uint32_t microsec; uint32_t dataPos; }` 匿名 union：两臂
/// 同为 `u32`，Enter/Leave 写入的是时间戳、ArgName/ArgValue 写入的是 `data`
/// 缓冲区偏移，读取哪一"臂"完全由 `type` 决定，union 本身不提供任何布局收益。
/// Rust 用单一 `data: u32` 承载同一语义，按 `type` 解释，消除"读未写臂"的风险。
///
/// cpp 镜像工件，sync-cpp 维护；仅本 crate 打点缓冲与落盘路径消费，降 `pub(crate)`。
#[derive(Debug, Clone, Copy)]
pub(crate) struct Event {
  pub(crate) r#type: EventType,
  pub(crate) token: u16,
  /// `EventType::Enter`/`Leave` 时为微秒时间戳（1 小时回绕一次，与 cpp 同为
  /// `uint32_t`），`ArgName`/`ArgValue` 时为 `ThreadContext::data` 内的字节偏移。
  pub(crate) data: u32,
}
