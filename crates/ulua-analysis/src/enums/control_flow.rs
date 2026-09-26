use bitflags::bitflags;

bitflags! {
  /// C++ `ControlFlow`（`Analysis/include/Luau/ControlFlow.h:21-29`）。
  #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
  pub struct ControlFlow: u32 {
    const None = 1 << 0;
    const Returns = 1 << 1;
    const Throws = 1 << 2;
    const Breaks = 1 << 3;
    const Continues = 1 << 4;
  }
}
