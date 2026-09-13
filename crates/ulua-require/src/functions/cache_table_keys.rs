use core::ffi::CStr;

/// 存储显式注册模块的注册表键（C++ `registeredCacheTableKey`）。
pub(crate) const REGISTERED_CACHE_TABLE_KEY: &CStr = c"_REGISTEREDMODULES";

/// 存储 require 调用结果的注册表键（C++ `requiredCacheTableKey`）。
pub(crate) const REQUIRED_CACHE_TABLE_KEY: &CStr = c"_MODULES";
