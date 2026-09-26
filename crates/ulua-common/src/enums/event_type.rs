//! Source: `Common/include/Luau/TimeTrace.h`
//!
//! cpp 镜像工件（`EventType`），sync-cpp 维护；仅随 `records::event::Event`
//! 在本 crate 打点机制内部流转，降 `pub(crate)`。

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub(crate) enum EventType {
  Enter = 0,
  Leave = 1,
  ArgName = 2,
  ArgValue = 3,
}
