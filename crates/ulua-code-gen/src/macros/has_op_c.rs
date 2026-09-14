//! Source: `CodeGen/include/Luau/IrData.h:1219` (hand-ported)
// #define HAS_OP_C(inst) (2 < (inst).ops.size() && (inst).ops[2].kind() != IrOpKind::None)
#[macro_export]
macro_rules! HAS_OP_C {
  ($inst:expr) => {
    2 < ($inst).ops.size() as usize
      && ($inst).ops[2].kind() != $crate::enums::ir_op_kind::IrOpKind::None
  };
}
pub use HAS_OP_C;
