//! 句柄访问封装：`DefId = SymDefId`（u32 句柄，见 ulua-analysis
//! `records::sym_def_registry`）的只读解引用。
//! C++ `def->versionedName()` 的 Rust 对应——测试侧经注册表解析取
//! `Option<&SymDef>`，不再接触裸指针。

use ulua_analysis::{
  records::sym_def_registry::resolve_sym_def, type_aliases::def_id_control_flow_graph::DefId,
};

/// NotNull 语义保证句柄指向存活 SymDef；空哨兵/越界解析失败时 panic
/// （比 C++ 非空解引用 UB 更安全）。
pub fn def_versioned_name(def: DefId) -> String {
  resolve_sym_def(def)
    .expect("DefId 非空（NotNull 语义）")
    .versioned_name()
}
