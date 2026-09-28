//! Source: `Analysis/include/Luau/ConfigResolver.h:12` (hand-ported)
//! C++ abstract interface — modeled as a struct with a fn-pointer vtable slot
//! (the project convention for pure-virtual classes).

use ulua_config::records::config::Config;

use crate::{
  records::type_check_limits::TypeCheckLimits, type_aliases::module_name_type::ModuleName,
};

#[derive(Debug)]
pub struct ConfigResolver {
  /// virtual const Config& getConfig(const ModuleName&, const TypeCheckLimits&) const
  ///
  /// # Safety
  /// 槽内 `unsafe fn` 是 C++ 纯虚 `getConfig` 的 vtable 替身：调用方传入的 `this`
  /// 须为持有本槽位的 `ConfigResolver` 自身指针（vtable self），`name`/`limits`
  /// 须为非空且指向存活 `ModuleName`/`TypeCheckLimits` 的借用；返回的 `*const
  /// Config` 须指向在该借用读取期内保持存活的 `Config`（调用点立即 `&*`/`clone`
  /// 它）。实现方须为无捕获状态的静态函数；槽位为 `None` 时不可调用（调用点以
  /// `expect` 兜底）。
  pub get_config: Option<
    unsafe fn(
      this: *const ConfigResolver,
      name: *const ModuleName,
      limits: *const TypeCheckLimits,
    ) -> *const Config,
  >,
}
