use crate::{macros::rex_x::rex_x, records::register_x_64::RegisterX64};

pub const fn rex_r(reg: RegisterX64) -> u8 {
  let _ = rex_x; // keep dependency ordered/imported as in the schedule
  (reg.index() & 0x8) >> 1
}
