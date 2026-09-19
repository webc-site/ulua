//! Source: `CodeGen/src/AssemblyBuilderX64.cpp:40` (hand-ported)
// #define REX_W(reg) REX_W_BIT((reg).size == SizeX64::Qword)
#[macro_export]
macro_rules! REX_W {
  ($reg:expr) => {
    $crate::macros::rex_w_bit::REX_W_BIT!(($reg).size == $crate::enums::size_x_64::SizeX64::Qword)
  };
}
pub use REX_W;
