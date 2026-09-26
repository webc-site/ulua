extern crate alloc;

use alloc::string::String;

use crate::{
  records::sym_def_registry::resolve_sym_def, type_aliases::def_id_control_flow_graph::DefId,
};

/// 对应 C++ `static std::string dumpDef(Definition* def)` (`cpp/Analysis/src/DumpCFG.cpp:23`)。
///
/// `def` 为 `sym_def_registry` 发放的句柄：解析命中即读 `versioned_name`，
/// 未命中（含空哨兵，对应 cpp `nullptr` 的 `"?"` 分支）输出 `"?"`。解引用
/// 收口在注册表，本函数为 safe（§2）。
pub fn dump_def(def: DefId) -> String {
  match resolve_sym_def(def) {
    Some(sym_def) => sym_def.versioned_name(),
    None => String::from("?"),
  }
}
