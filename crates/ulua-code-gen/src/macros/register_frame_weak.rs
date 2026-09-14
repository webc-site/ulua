#[cfg(not(target_os = "windows"))]
pub const REGISTER_FRAME_WEAK: &str = "weak";

#[cfg(target_os = "windows")]
pub const REGISTER_FRAME_WEAK: () = ();
