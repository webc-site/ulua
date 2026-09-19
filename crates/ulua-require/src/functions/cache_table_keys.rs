/// 存储显式注册模块的注册表键（C++ `registeredCacheTableKey`）。
/// Lua 字符串是字节串，内部一律以 `&[u8]` 传递，仅在真 FFI 边界
/// （`with_c_str`）补 NUL。
pub(crate) const REGISTERED_CACHE_TABLE_KEY: &[u8] = b"_REGISTEREDMODULES";

/// 存储 require 调用结果的注册表键（C++ `requiredCacheTableKey`）。
pub(crate) const REQUIRED_CACHE_TABLE_KEY: &[u8] = b"_MODULES";
