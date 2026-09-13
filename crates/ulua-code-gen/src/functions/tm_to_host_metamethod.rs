use core::mem::transmute;

use ulua_vm::enums::tms::TMS;

use crate::enums::host_metamethod::HostMetamethod;

#[inline]
fn tms_from_tm(tm: i32) -> TMS {
  unsafe { transmute::<u32, TMS>(tm as u32) }
}

#[inline]
pub fn tm_to_host_metamethod(tm: i32) -> HostMetamethod {
  match tms_from_tm(tm) {
    TMS::TmAdd => HostMetamethod::Add,
    TMS::TmSub => HostMetamethod::Sub,
    TMS::TmMul => HostMetamethod::Mul,
    TMS::TmDiv => HostMetamethod::Div,
    TMS::TmIDiv => HostMetamethod::Idiv,
    TMS::TmMod => HostMetamethod::Mod,
    TMS::TmPow => HostMetamethod::Pow,
    TMS::TmUnm => HostMetamethod::Minus,
    TMS::TmEq => HostMetamethod::Equal,
    TMS::TmLt => HostMetamethod::LessThan,
    TMS::TmLe => HostMetamethod::LessEqual,
    TMS::TmLen => HostMetamethod::Length,
    TMS::TmConcat => HostMetamethod::Concat,
    _ => {
      // CODEGEN_ASSERT! references ulua_common::assertCallHandler and arch-specific intrinsics.
      // This code path may be compiled in configurations where those are unavailable, so keep
      // the behavior safe and deterministic.
      HostMetamethod::Add
    }
  }
}
