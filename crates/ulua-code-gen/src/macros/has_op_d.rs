//! Source: `CodeGen/include/Luau/IrData.h:1220` (hand-ported)
// #define HAS_OP_D(inst) (3 < (inst).ops.size() && (inst).ops[3].kind() != IrOpKind::None)
#[macro_export]
macro_rules! HAS_OP_D {
  ($inst:expr) => {
    3 < ($inst).ops.size() as usize
      && ($inst).ops[3].kind() != $crate::enums::ir_op_kind::IrOpKind::None
  };
}
pub use HAS_OP_D;
