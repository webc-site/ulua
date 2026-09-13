//! Source: `CodeGen/src/AssemblyBuilderX64.cpp:39` (hand-ported)
// #define REX_W_BIT(value) (value ? 0x8 : 0x0)
#[macro_export]
macro_rules! REX_W_BIT {
  ($value:expr) => {
    if $value { 0x8u8 } else { 0x0u8 }
  };
}
pub use REX_W_BIT;
