use crate::enums::abix_64::ABIX64;

const K_WINDOWS_FIRST_NON_VOL_XMM_REG: u8 = 6;

pub fn get_non_vol_xmm_storage_size(abi: ABIX64, xmm_reg_count: u8) -> u32 {
  if abi == ABIX64::SYSTEM_V {
    return 0;
  }

  // First 6 are volatile
  if xmm_reg_count <= K_WINDOWS_FIRST_NON_VOL_XMM_REG {
    return 0;
  }

  assert!(xmm_reg_count <= 16);
  u32::from(xmm_reg_count - K_WINDOWS_FIRST_NON_VOL_XMM_REG) * 16
}
