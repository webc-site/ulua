//! Port of `CliConfigResolver`（`CLI/src/Analyze.cpp:231-321`）。

use alloc::{string::String, vec::Vec};
use core::cell::UnsafeCell;
use std::collections::HashMap;

use ulua_analysis::records::config_resolver::ConfigResolver;
use ulua_config::records::config::Config;

/// Port of `struct CliConfigResolver : Luau::ConfigResolver` (`CLI/src/Analyze.cpp:231-321`).
///
/// Like the analysis `ConfigResolver` (a struct with a fn-pointer vtable slot for
/// the single pure virtual `getConfig`), this concrete subclass is `#[repr(C)]`
/// with `base: ConfigResolver` first so that a `*const ConfigResolver` (the vtable
/// receiver) can be cast back to `*const CliConfigResolver`.
///
/// C++ members:
/// - `Luau::Config defaultConfig;`
/// - `mutable std::unordered_map<std::string, Luau::Config> configCache;`
/// - `mutable std::vector<std::pair<std::string, std::string>> configErrors;`
///
/// `config_cache`/`config_errors` mirror C++ `mutable`：`getConfig` 逻辑 const
/// 但填充缓存。用 `UnsafeCell` 表达内部可变（返回缓存内引用需要稳定地址，
/// RefCell 的 borrow 无法跨返回存活）。
/// 契约与上游一致：resolver 单线程使用，回调期间无并发/重叠可变借用。
#[repr(C)]
#[derive(Debug)]
pub struct CliConfigResolver {
  pub base: ConfigResolver,
  pub default_config: Config,
  pub config_cache: UnsafeCell<HashMap<String, Config>>,
  pub config_errors: UnsafeCell<Vec<(String, String)>>,
}
