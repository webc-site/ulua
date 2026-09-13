//! Source: `CodeGen/include/Luau/IrData.h:1221` (hand-ported)
// #define HAS_OP_E(inst) (4 < (inst).ops.size() && (inst).ops[4].kind() != IrOpKind::None)
#[macro_export]
macro_rules! HAS_OP_E {
  ($inst:expr) => {
    4 < ($inst).ops.size() as usize
      && ($inst).ops[4].kind() != $crate::enums::ir_op_kind::IrOpKind::None
  };
}
pub use HAS_OP_E;
