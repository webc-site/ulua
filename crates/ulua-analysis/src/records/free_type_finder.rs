//! `cpp/Analysis/src/Generalization.cpp` 的 `FreeTypeFinder`（iterative 分支）：
//! 从给定根类型出发，收集本 arena 中可达的自由类型。
//!
//! 与 C++ 一致：跳过 Bound 链（`skip_bound_types`），遇 Free 记录自身并继续
//! 深入其上下界（含嵌套在 union/intersection 中的成员），在
//! Table/Metatable/Function/Extern 处停止下降；跨 arena 的类型直接剪枝。

use crate::{
  records::{
    arena_handle::Handle, arena_id::ArenaId, extern_type::ExternType, free_type::FreeType,
    function_type::FunctionType, iterative_type_visitor::IterativeTypeVisitor,
    metatable_type::MetatableType, table_type::TableType, type_arena::TypeArena, type_ids::TypeIds,
  },
  type_aliases::type_id::TypeId,
};

#[derive(Debug, Clone)]
pub struct FreeTypeFinder {
  pub base: IterativeTypeVisitor,
  pub arena: Handle<TypeArena>,
  pub free_tys: TypeIds,
}

impl FreeTypeFinder {
  pub fn new(arena: Handle<TypeArena>) -> Self {
    let mut base = IterativeTypeVisitor::default();
    base.iterative_type_visitor_string_bool_bool("FreeTypeFinder", true, true);
    Self {
      base,
      arena,
      free_tys: TypeIds::new(),
    }
  }

  pub fn visit_type_id_free_type(&mut self, ty: TypeId, _ftv: &FreeType) -> bool {
    if type_owning_arena(ty) != self.arena.get().arena_id {
      return false;
    }
    self.free_tys.insert_type_id(ty);
    true
  }

  pub fn visit_type_id_table_type(&mut self, _ty: TypeId, _ttv: &TableType) -> bool {
    false
  }

  pub fn visit_type_id_metatable_type(&mut self, _ty: TypeId, _mtv: &MetatableType) -> bool {
    false
  }

  pub fn visit_type_id_function_type(&mut self, _ty: TypeId, _ftv: &FunctionType) -> bool {
    false
  }

  pub fn visit_type_id_extern_type(&mut self, _ty: TypeId, _etv: &ExternType) -> bool {
    false
  }
}

/// C++ `ty->owningArena`（Generalization.cpp:793）：TypeId 句柄解引用收口在
/// 私有 helper，公共方法保持 safe（同 `clone_public_interface_is_dirty_module`
/// 的先例）。读出 [`ArenaId`] 值，比较不含指针解引用。
fn type_owning_arena(ty: TypeId) -> ArenaId {
  // SAFETY: `ty` 有效性由遍历器交接契约保证（本 arena 存活节点），仅读一次。
  unsafe { (*ty).owning_arena }
}
