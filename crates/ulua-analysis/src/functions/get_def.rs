//! Source: `Analysis/include/Luau/Def.h:73-77` (hand-ported)
/// C++ `template<typename T> const T* get(DefId def)`.
use core::ptr::null;

use crate::{
  records::def_registry::def_as,
  type_aliases::{def_id_def::DefId, variant::VariantMember},
};

/// 句柄 → 变体成员指针下转。签名保留 cpp `get<T>(DefId)` 的指针返回形态，
/// 供本 crate 判空逻辑与 `ulua-unit-test` 夹具（`get_phi`）源码兼容；crate 内
/// 新代码应直接用安全的 [`def_as`]。
///
/// # Safety
/// `def` 须为 [`DefId::NULL`] 或本线程 `DefArena` 分配点发放的存活句柄
/// （注册表契约见 `records::def_registry`）；命中返回的指针只读有效。
pub unsafe fn get_def_id<T: VariantMember>(def: DefId) -> *const T {
  def_as::<T>(def).map_or(null(), |r| r as *const T)
}
