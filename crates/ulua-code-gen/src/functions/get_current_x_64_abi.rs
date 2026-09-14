use crate::enums::abix_64::ABIX64;

pub fn get_current_x_64_abi() -> ABIX64 {
  #[cfg(target_os = "windows")]
  {
    ABIX64::Windows
  }
  #[cfg(not(target_os = "windows"))]
  {
    ABIX64::SYSTEM_V
  }
}
