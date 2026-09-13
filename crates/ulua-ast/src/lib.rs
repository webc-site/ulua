extern crate alloc;

use core::sync::atomic::AtomicBool;

pub mod enums;
pub mod functions;
pub mod methods;
pub mod records;
pub mod rtti;
pub mod type_aliases;
pub mod visit;

// 解析器遥测开关：返回类型带类型后缀的变参（对应 C++ 的全局 bool）
pub static LUAU_TELEMETRY_PARSED_RETURN_TYPE_VARIADIC_WITH_TYPE_SUFFIX: AtomicBool =
  AtomicBool::new(false);
