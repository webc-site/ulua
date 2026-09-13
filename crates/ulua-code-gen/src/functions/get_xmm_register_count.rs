use crate::enums::abix_64::ABIX64;

pub const K_SYSTEM_VUSABLE_XMM_REGS: u8 = 16;
pub const K_WINDOWS_USABLE_XMM_REGS: u8 = 10;

#[inline]
pub fn get_xmm_register_count(abi: ABIX64) -> u8 {
  if abi == ABIX64::SYSTEM_V {
    K_SYSTEM_VUSABLE_XMM_REGS
  } else {
    K_WINDOWS_USABLE_XMM_REGS
  }
}
