/// 存储显式注册模块的注册表键（C++ `registeredCacheTableKey`）。
/// Lua 字符串是字节串，内部一律以 `&[u8]` 传递；只有 `lua_l_findtable` 这类
/// 按 C 路径指针推进的 ulua-vm C ABI 在门面内补 NUL（见 `registry_table`）。
pub(crate) const REGISTERED_CACHE_TABLE_KEY: &[u8] = b"_REGISTEREDMODULES";

/// 存储 require 调用结果的注册表键（C++ `requiredCacheTableKey`）。
pub(crate) const REQUIRED_CACHE_TABLE_KEY: &[u8] = b"_MODULES";

/// 记录占位表是否已实际交给循环 require 方的注册表键
/// （C++ `cyclicPlaceholderProvidedKey`）。
pub(crate) const CYCLIC_PLACEHOLDER_PROVIDED_KEY: &[u8] = b"_CYCLIC_PLACEHOLDER_PROVIDED";
