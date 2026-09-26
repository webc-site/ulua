//! 编译目标特性常量族（r7-macros98 合并票：逐字保真自原一文件一宏碎片）。

#[cfg(all(target_arch = "aarch64", not(target_os = "windows")))]
pub const CODEGEN_TARGET_A64: bool = true;

#[cfg(not(all(target_arch = "aarch64", not(target_os = "windows"))))]
pub const CODEGEN_TARGET_A64: bool = false;

#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
pub const CODEGEN_TARGET_X64: bool = true;

#[cfg(not(any(target_arch = "x86_64", target_arch = "x86")))]
pub const CODEGEN_TARGET_X64: bool = false;
