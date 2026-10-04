//! Port of `CliConfigResolver`（`CLI/src/Analyze.cpp:231-321`）。

use alloc::{
  rc::Rc,
  string::String,
  vec::Vec,
};
use core::cell::{RefCell, UnsafeCell};

use ulua_analysis::type_aliases::collections::HashMap;
use ulua_config::records::config::Config;

/// Port of `struct CliConfigResolver : Luau::ConfigResolver` (`CLI/src/Analyze.cpp:231-321`).
///
/// 配置解析改经 ulua-analysis 侧 `ConfigResolver` trait（`impl` 见
/// `cli_config_resolver_get_config.rs`）；`Frontend` 以 `Box<dyn ConfigResolver>`
/// 独占持有本类型实例后，宿主侧只需在移交前克隆 [`Self::config_errors`] 的 `Rc`
/// 句柄即可读回分析期收集的错误，不再有 `#[repr(C)]` 的 `base` 首字段与
/// `*const ConfigResolver`→`*const CliConfigResolver` 的 container-of 回cast。
///
/// C++ members:
/// - `Luau::Config defaultConfig;`
/// - `mutable std::unordered_map<std::string, Luau::Config> configCache;`
/// - `mutable std::vector<std::pair<std::string, std::string>> configErrors;`
///
/// `config_cache`/`config_errors` mirror C++ `mutable`：`getConfig` 逻辑 const
/// 但填充缓存。`config_cache` 用 `UnsafeCell` 表达内部可变（返回缓存内引用需要稳定地址，
/// RefCell 的 borrow 无法跨返回存活）；`config_errors` 用 `Rc<RefCell<..>>` 承载
/// 内部可变并向宿主共享（所有权移交 frontend 后仍可经克隆句柄读回）。
/// 契约与上游一致：resolver 单线程使用，回调期间无并发/重叠可变借用。
#[derive(Debug)]
pub struct CliConfigResolver {
  pub default_config: Config,
  /// 值走 `Box` 堆上定址：哈希表 insert 触发 grow 时会 memmove 全部内联值，
  /// 已返回的 `&Config` 即悬垂；Box 使值地址与表结构解耦，
  /// 等价 cpp `std::unordered_map` 的节点式地址稳定（缓存项永不删除）。
  /// 表用工作区统一的 foldhash `FixedState` 别名（先例
  /// `ulua-common/src/collections.rs`）：仅 get/insert，无迭代，
  /// 仅去掉 std 默认 SipHash。
  pub config_cache: UnsafeCell<HashMap<String, Box<Config>>>,
  /// `Rc` 共享槽：所有权移交 frontend 后，宿主经移交前克隆的句柄读回错误列表。
  pub config_errors: Rc<RefCell<Vec<(String, String)>>>,
}
