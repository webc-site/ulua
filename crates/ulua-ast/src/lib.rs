extern crate alloc;

use core::sync::atomic::AtomicBool;

pub mod enums;
pub mod functions;

// `#[macro_use]`：visit.rs 的 `ast_node_table!`（全节点分发表，单一事实来源）与
// `impl_visitable!` 进入 crate 文本作用域，供其后声明的 `methods` 与
// `records::ast_visitor` 消费（macro_rules 按文本序可见，故本模块先于 methods）。
#[macro_use]
pub mod visit;

// `#[macro_use]`：rtti.rs 的 `impl_cst_node_class!`（CST 节点 CLASS_INDEX 一行
// 登记）进入 crate 文本作用域，供其后声明的 `records` 消费。
#[macro_use]
pub mod rtti;

pub mod methods;

pub mod records;
pub mod type_aliases;

// 解析器遥测开关：返回类型带类型后缀的变参（对应 C++ 的全局 bool）
pub static LUAU_TELEMETRY_PARSED_RETURN_TYPE_VARIADIC_WITH_TYPE_SUFFIX: AtomicBool =
  AtomicBool::new(false);
