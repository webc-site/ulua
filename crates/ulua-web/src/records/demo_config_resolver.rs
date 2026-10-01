//! Port of `DemoConfigResolver : Luau::ConfigResolver` (`CLI/src/Web.cpp:49-62`)。
//!
//! Rust 化（punch #15 / review.md §10）：原形态是 C++ 继承的镜像——`#[repr(C)]`
//! 结构以 `base: ConfigResolver` 作首字段，vtable thunk 把 `*const ConfigResolver`
//! 回铸成 `*const DemoConfigResolver` 再解引用读 `default_config`。该形态的唯一
//! 存在理由是「配置挂在具体 receiver 上」；而 demo 配置恒为同一份默认值（cpp
//! `getConfig` 忽略两个实参、返回成员 `defaultConfig`），故把配置升格为进程级
//! 冻结单例：适配器不再解引用/回铸任何指针，`this` 形态的 vtable 布线随之消失。
//!
//! 剩余的唯一边界形状是槽位类型本身：`ConfigResolver::get_config` 是
//! ulua-analysis（本任务范围外）建模 C++ 纯虚类的函数指针替身，签名即其契约。
//! 本模块以安全构造器 [`demo_config_resolver`] 收口，调用点零 unsafe。

use std::sync::OnceLock;

use ulua_analysis::{
  records::{config_resolver::ConfigResolver, type_check_limits::TypeCheckLimits},
  type_aliases::module_name_type::ModuleName,
};
use ulua_ast::enums::mode::Mode;
use ulua_config::records::config::Config;

/// 冻结的 demo 默认配置包装：一次性初始化后只读，永不变更。
///
/// # Safety（`unsafe impl Sync` 的论证）
/// `Config::default()` 的形态为全静态初值：`Vec`/`DenseHashMap` 均为空表，
/// `!Sync` 仅来自 `ParseOptions` 内部对宿主 AST 裸指针的 `PhantomData` 标记
/// （默认配置下无任何指向对象）。实例经 [`demo_default_config`] 一次落定后
/// 再无 `&mut` 路径（私有、无写访问器、只经 `&Config` 共享），跨线程只读
/// 共享与 `&T` 同构，不触及其标记所防范的「跨线程可变裸指针」情形。
struct FrozenConfig(Config);

// Safety: 上方不变量——初始化后只读、内容无真实可变指针状态。
// `Send` 随之成立（`Sync` 类型对只读共享即 `Send` 的退化情形由
// `&FrozenConfig: Send` 提供，见 `demo_default_config` 的单例形态）。
unsafe impl Sync for FrozenConfig {}
unsafe impl Send for FrozenConfig {}

/// demo 默认配置（进程级冻结单例，对应 cpp `DemoConfigResolver::defaultConfig`）。
///
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
fn demo_default_config() -> &'static Config {
  // 单例形态为 `OnceLock<&'static FrozenConfig>`：初始化一次性泄漏冻结配置本体
  // （playground 全生命周期常驻，无析构路径可回收，泄漏即预期），此后仅有 `Copy`
  // 的共享引用流转；`FrozenConfig` 为本地类型，`Sync`/`Send` 由上方论证手工成立。
  static CONFIG: OnceLock<&'static FrozenConfig> = OnceLock::new();
  let frozen: &FrozenConfig = CONFIG.get_or_init(|| {
    let leaked: &'static mut FrozenConfig = Box::leak(Box::new(FrozenConfig(Config {
      mode: Mode::Nonstrict,
      ..Default::default()
    })));
    leaked // &mut → & 隐式再借用：一次性降级为共享引用，此后全程序无该本体的 `&mut` 路径
  });
  &frozen.0
}

/// demo resolver 的安全构造器（cpp `DemoConfigResolver()` 构造的对应物）：
/// 返回的 `ConfigResolver` 无接收者状态——配置在 [`demo_default_config`] 冻结
/// 单例上，因此不再有 `#[repr(C)]` 的 `base` 子对象与回cast。
pub(crate) fn demo_config_resolver() -> ConfigResolver {
  ConfigResolver {
    get_config: Some(demo_get_config),
  }
}

/// `getConfig` 槽（vtable 替身）的适配器：三枚指针实参一律忽略（demo 配置不随
/// 模块名/预算/接收者变化），返回进程级冻结配置的地址。
///
/// # Safety
/// 仅履行 [`ConfigResolver::get_config`] 的槽位契约：返回指针在任意请求寿命内
/// 有效（`OnceLock` 冻结单例，只读共享）；不解引用 `this`/`name`/`limits`，
/// 故对其存续期无假设。
unsafe fn demo_get_config(
  _this: *const ConfigResolver,
  _name: *const ModuleName,
  _limits: *const TypeCheckLimits,
) -> *const Config {
  demo_default_config()
}
