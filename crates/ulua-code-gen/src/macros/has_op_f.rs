//! Source: `CodeGen/include/Luau/IrData.h:1222` (hand-ported)
// #define HAS_OP_F(inst) (5 < (inst).ops.size() && (inst).ops[5].kind() != IrOpKind::None)
#[macro_export]
macro_rules! HAS_OP_F {
  ($inst:expr) => {
    5 < ($inst).ops.size() as usize
      && ($inst).ops[5].kind() != $crate::enums::ir_op_kind::IrOpKind::None
  };
}
pub use HAS_OP_F;
