#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
pub const CODEGEN_TARGET_X64: bool = true;

#[cfg(not(any(target_arch = "x86_64", target_arch = "x86")))]
pub const CODEGEN_TARGET_X64: bool = false;
