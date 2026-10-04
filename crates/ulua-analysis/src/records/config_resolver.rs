//! Source: `Analysis/include/Luau/ConfigResolver.h:12` (hand-ported)
//! C++ abstract interface — modeled as a Rust trait (the project convention for
//! pure-virtual classes, aligned with [`FileResolver`](crate::records::file_resolver::FileResolver)).

use ulua_config::records::config::Config;

use crate::{
  records::type_check_limits::TypeCheckLimits, type_aliases::module_name_type::ModuleName,
};

/// C++ `Luau::ConfigResolver` 虚基类：纯虚 `getConfig` → trait 方法。实现方以
/// `dyn ConfigResolver`（trait object）交由 `Frontend` 独占持有，替代原先
/// `#[repr(C)]` 手写 vtable + `unsafe fn` 指针槽（收 `this: *const`
/// 裸地址、靠 container-of 回铸宿主具体类型）的形态。
pub trait ConfigResolver {
  /// `getConfig`：C++ `virtual const Config& getConfig(const ModuleName&,
  /// const TypeCheckLimits&) const`，纯虚。返回引用指向实现方持有、在借用期内
  /// 存活的 `Config`（cpp 同款所有权）。
  fn get_config(&self, name: &ModuleName, limits: &TypeCheckLimits) -> &Config;
}
