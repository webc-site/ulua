//! Port of `DemoConfigResolver : Luau::ConfigResolver` (`CLI/src/Web.cpp:49-62`)。
//!
//! Rust 化（punch #15 / review.md §10）：原形态是 C++ 继承的镜像——`#[repr(C)]`
//! 结构以 `base: ConfigResolver` 作首字段，vtable thunk 把 `*const ConfigResolver`
//! 回铸成 `*const DemoConfigResolver` 再解引用读 `default_config`。该形态的后续
//! 变体把配置升格为进程级冻结单例（`OnceLock` + `Box::leak` + 手工 `unsafe impl
//! Sync/Send`），只为绕开「无接收者可返回 `&Config`」这一点。
//!
//! review.md §2/§7 收形：`ConfigResolver` 的 trait 契约本就是「返回引用指向实现方
//! 持有、在借用期内存活的 `Config`」，故把配置收回成 [`DemoConfigResolver`] 自身的
//! 字段——`get_config` 直接交出 `&self.default_config`，生命周期由接收者供给。
//! 进程级单例、`'static` 泄漏与两枚 `unsafe impl` 随之全部消失，所有权仍经
//! `Box<dyn ConfigResolver>` 移交 `Frontend` 独占；不再有 `this` 形态的 vtable
//! 布线、回cast或调用点 `unsafe`。

use ulua_analysis::{
  records::{config_resolver::ConfigResolver, type_check_limits::TypeCheckLimits},
  type_aliases::module_name_type::ModuleName,
};
use ulua_ast::enums::mode::Mode;
use ulua_config::records::config::Config;

/// demo 配置解析器（cpp `DemoConfigResolver` 的对应物）：配置由本类型持有，
/// 所有权经 `Box<dyn ConfigResolver>` 移交 `Frontend` 独占。
pub(crate) struct DemoConfigResolver {
  /// cpp `DemoConfigResolver::defaultConfig`：demo 全程只读的一份默认配置。
  default_config: Config,
}

impl Default for DemoConfigResolver {
  /// cpp `DemoConfigResolver()` 构造段（`CLI/src/Web.cpp:51-54`）硬置
  /// `Mode::Strict`：
  ///
  /// ```cpp
  /// DemoConfigResolver() { defaultConfig.mode = Luau::Mode::Strict; }
  /// ```
  ///
  /// DELIBERATE DEVIATION from `Web.cpp`：playground 默认 `Nonstrict`，让每段脚本
  /// 自己的 `--!strict` / `--!nonstrict` 模式注释说了算（与 `ulua-analyze` CLI 同向）。
  /// 如此才不至于把未标注的示例脚本按 strict 检查、向访问者报出「干净代码被判有错」
  /// 的困惑诊断。
  fn default() -> Self {
    Self {
      default_config: Config {
        mode: Mode::Nonstrict,
        ..Default::default()
      },
    }
  }
}

impl ConfigResolver for DemoConfigResolver {
  /// cpp `getConfig` 忽略两个实参、恒返回成员 `defaultConfig`：此处交出本类型自持
  /// 字段的借用，其生命周期即 `&self` 的借用期（`Frontend` 独占持有本解析器，故
  /// 与该引用的可用窗口一致）。
  fn get_config(&self, _name: &ModuleName, _limits: &TypeCheckLimits) -> &Config {
    &self.default_config
  }
}
