#[cfg(all(target_arch = "aarch64", not(target_os = "windows")))]
pub const CODEGEN_TARGET_A64: bool = true;

#[cfg(not(all(target_arch = "aarch64", not(target_os = "windows"))))]
pub const CODEGEN_TARGET_A64: bool = false;
