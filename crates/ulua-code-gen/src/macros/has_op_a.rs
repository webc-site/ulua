//! Source: `CodeGen/include/Luau/IrData.h:1217` (hand-ported)
// #define HAS_OP_A(inst) (0 < (inst).ops.size() && (inst).ops[0].kind() != IrOpKind::None)
#[macro_export]
macro_rules! HAS_OP_A {
  ($inst:expr) => {
    0 < ($inst).ops.size() as usize
      && ($inst).ops[0].kind() != $crate::enums::ir_op_kind::IrOpKind::None
  };
}
pub use HAS_OP_A;
