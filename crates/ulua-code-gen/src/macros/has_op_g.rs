//! Source: `CodeGen/include/Luau/IrData.h:1223` (hand-ported)
// #define HAS_OP_G(inst) (6 < (inst).ops.size() && (inst).ops[6].kind() != IrOpKind::None)
#[macro_export]
macro_rules! HAS_OP_G {
  ($inst:expr) => {
    6 < ($inst).ops.size() as usize
      && ($inst).ops[6].kind() != $crate::enums::ir_op_kind::IrOpKind::None
  };
}
pub use HAS_OP_G;
