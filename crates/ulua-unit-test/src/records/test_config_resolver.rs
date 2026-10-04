//! Source: `tests/Fixture.h`

use std::{cell::UnsafeCell, rc::Rc};

use hashbrown::HashMap;
use ulua_analysis::records::{config_resolver::ConfigResolver, type_check_limits::TypeCheckLimits};
use ulua_config::{records::config::Config, type_aliases::module_name::ModuleName};

/// C++ `TestConfigResolver : ConfigResolver`（`tests/Fixture.h`）。
///
/// `Frontend` 以 `Box<dyn ConfigResolver>` 独占持有本类型的克隆
/// （`get_frontend` 移交所有权）后，夹具句柄与 frontend 内实例读写的是同一份
/// 配置状态：`default_config` / `config_files` 经 `Rc<UnsafeCell<_>>` 共享槽承载
/// （等价 cpp「Frontend 存裸指针、宿主直写字段」的可见性）。`getConfig` 返回
/// `&Config` 需稳定地址且跨返回存活，故用 `UnsafeCell` 而非 `RefCell`（手法对齐
/// `CliConfigResolver::config_cache`）。契约：resolver 单线程使用，宿主改写
/// （`default_config_mut` / `config_files_mut`）与 frontend 读取（`get_config`）
/// 的借用窗口不重叠。
#[derive(Debug, Clone)]
pub struct TestConfigResolver {
  default_config: Rc<UnsafeCell<Config>>,
  config_files: Rc<UnsafeCell<HashMap<ModuleName, Config>>>,
}

impl Default for TestConfigResolver {
  fn default() -> Self {
    Self {
      default_config: Rc::new(UnsafeCell::new(Config::default())),
      config_files: Rc::new(UnsafeCell::new(HashMap::default())),
    }
  }
}

impl TestConfigResolver {
  /// 夹具侧对共享默认配置的可变访问：`&mut self` 保证夹具内不并存 `&mut`，
  /// 对 frontend 侧的可见互通靠底层 `Rc` 同一目标。
  ///
  /// # Safety
  /// 依赖 [`TestConfigResolver`] 类型注的单线程契约：本可变借用存续期内
  /// frontend 未在同一 `Rc` 目标上调用 `get_config`（两者借用窗口不重叠）。
  pub fn default_config_mut(&mut self) -> &mut Config {
    unsafe { &mut *self.default_config.get() }
  }

  /// 夹具侧对共享 `config_files` 表的可变访问，契约同 [`Self::default_config_mut`]。
  ///
  /// # Safety
  /// 同 [`Self::default_config_mut`]：单线程、与 `get_config` 读窗口不重叠。
  pub fn config_files_mut(&mut self) -> &mut HashMap<ModuleName, Config> {
    unsafe { &mut *self.config_files.get() }
  }
}

impl ConfigResolver for TestConfigResolver {
  /// C++ `const Config& getConfig(const ModuleName&, const TypeCheckLimits&) const`：
  /// `config_files` 命中则返回该项，否则回落共享 `default_config`（`_limits` 不触碰）。
  fn get_config(&self, name: &ModuleName, _limits: &TypeCheckLimits) -> &Config {
    // Safety: 单线程契约见类型注——读窗口内宿主未持有同一目标的 `&mut`
    //（`default_config_mut` / `config_files_mut` 调用均发生在检查之前）。Rc 令
    // 底层 `Config`/表至少与本次 `&self` 借用同寿，故返回引用可按 `&self` 生命周期
    // 物化，语义同 cpp `getConfig` 返回成员引用。
    let files = unsafe { &*self.config_files.get() };
    files
      .get(name)
      .unwrap_or_else(|| unsafe { &*self.default_config.get() })
  }
}
