use ulua_common::macros::luau_unreachable::LUAU_UNREACHABLE;

use crate::{
  enums::kind_a_64::KindA64,
  records::{ir_reg_alloc_a_64::IrRegAllocA64, set::Set},
};

impl IrRegAllocA64 {
  pub(crate) fn get_set(&mut self, kind: KindA64) -> &mut Set {
    match kind {
      KindA64::X | KindA64::W => &mut self.gpr,

      KindA64::S | KindA64::D | KindA64::Q => &mut self.simd,

      _ => {
        debug_assert!(false, "Unexpected register kind");
        LUAU_UNREACHABLE!();
      }
    }
  }
}
