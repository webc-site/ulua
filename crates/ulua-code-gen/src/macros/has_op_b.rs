//! Source: `CodeGen/include/Luau/IrData.h:1218` (hand-ported)
// #define HAS_OP_B(inst) (1 < (inst).ops.size() && (inst).ops[1].kind() != IrOpKind::None)
#[macro_export]
macro_rules! HAS_OP_B {
  ($inst:expr) => {
    1 < ($inst).ops.size() as usize
      && ($inst).ops[1].kind() != $crate::enums::ir_op_kind::IrOpKind::None
  };
}
pub use HAS_OP_B;
