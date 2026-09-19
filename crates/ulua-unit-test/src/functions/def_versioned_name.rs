//! SAFETY 封装：`DefId = NotNull<Definition> = *mut SymDef` 的解引用访问。
//! C++ `def->versionedName()` 的 Rust 对应——裸指针字段在 ulua-analysis 侧，
//! 测试侧经此安全包装访问。

use core::ptr::NonNull;

use ulua_analysis::type_aliases::def_id_control_flow_graph::DefId;

/// # Safety（前置条件，调用侧免 unsafe）
/// `def` 须指向仍存活的 `SymDef`（CFG 场景：cfg.allocator 存活，测试期内有效）。
pub fn def_versioned_name(def: DefId) -> String {
  // NotNull 语义保证非空；null 时 panic（比 C++ 非空解引用 UB 更安全）。
  let d = NonNull::new(def).expect("DefId 非空（NotNull 语义）");
  // SAFETY: 见函数级前置条件；NonNull 非空且指向存活 SymDef。
  unsafe { d.as_ref() }.versioned_name()
}
