//! Source: `VM/src/lstate.h` — no-op unless HARDSTACKTESTS (off in default builds)
#[macro_export]
macro_rules! condhardstacktests {
  ($x:expr) => {
    ()
  };
}
pub use condhardstacktests;
