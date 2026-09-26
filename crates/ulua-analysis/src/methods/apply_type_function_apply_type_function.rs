use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::{
  records::{
    apply_type_function::ApplyTypeFunction, arena_handle::Handle, substitution::Substitution,
    tarjan::SubstitutionVtable, txn_log::TxnLog, type_arena::TypeArena,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};
fn apply_type_function_is_dirty_ty(owner: *mut (), ty: TypeId) -> bool {
  // Safety: `owner` 由 `install_substitution_vtable` 接线为 `self as *mut ApplyTypeFunction
  // as *mut ()`，故它恒指向存活的 `ApplyTypeFunction`；回调仅在 `substitute_*` 期间同步
  // 触发、实例借出期内有效，`as *mut ApplyTypeFunction` 是无损还原（非空、对齐、类型正确）。
  unsafe { (*(owner as *mut ApplyTypeFunction)).is_dirty_type_id(ty) }
}

fn apply_type_function_is_dirty_tp(owner: *mut (), tp: TypePackId) -> bool {
  // Safety: 同 `is_dirty_ty`——`owner` 是 vtable 接线的 `ApplyTypeFunction` 类型擦除指针，
  // 回调期实例存活，向下转型还原原类型。
  unsafe { (*(owner as *mut ApplyTypeFunction)).is_dirty_type_pack_id(tp) }
}

fn apply_type_function_clean_ty(owner: *mut (), ty: TypeId) -> TypeId {
  // Safety: 同上——`owner` 经 vtable 保证指向存活 `ApplyTypeFunction`，还原类型后只读方法。
  unsafe { (*(owner as *mut ApplyTypeFunction)).clean_type_id(ty) }
}

fn apply_type_function_clean_tp(owner: *mut (), tp: TypePackId) -> TypePackId {
  // Safety: 同上——`owner` 为 vtable 接线的 `ApplyTypeFunction`，回调期有效，向下转型无损。
  unsafe { (*(owner as *mut ApplyTypeFunction)).clean_type_pack_id(tp) }
}

fn apply_type_function_found_dirty_ty(owner: *mut (), ty: TypeId) {
  // Safety: 同上——`owner` 指向存活 `ApplyTypeFunction`，此处经还原类型改写其 base 状态，
  // 借用期为该次同步回调独占。
  unsafe {
    (*(owner as *mut ApplyTypeFunction))
      .base
      .found_dirty_type_id(ty)
  }
}

fn apply_type_function_found_dirty_tp(owner: *mut (), tp: TypePackId) {
  // Safety: 同上——`owner` 指向存活 `ApplyTypeFunction`，同步回调期内独占改写 base 状态。
  unsafe {
    (*(owner as *mut ApplyTypeFunction))
      .base
      .found_dirty_type_pack_id(tp)
  }
}

fn apply_type_function_ignore_children_ty(owner: *mut (), ty: TypeId) -> bool {
  // Safety: 同上——`owner` 经 vtable 指向存活 `ApplyTypeFunction`，还原类型后只读判定。
  unsafe { (*(owner as *mut ApplyTypeFunction)).ignore_children_type_id(ty) }
}

fn apply_type_function_ignore_children_tp(owner: *mut (), tp: TypePackId) -> bool {
  // Safety: 同上——`owner` 经 vtable 指向存活 `ApplyTypeFunction`，还原类型后只读判定。
  unsafe { (*(owner as *mut ApplyTypeFunction)).ignore_children_type_pack_id(tp) }
}

impl ApplyTypeFunction {
  pub fn new(arena: Handle<TypeArena>) -> Self {
    Self {
      base: Substitution::substitution_new(TxnLog::empty(), Some(arena)),
      encountered_forwarded_type: false,
      type_arguments: DenseHashMap::default(),
      type_pack_arguments: DenseHashMap::default(),
    }
  }

  fn install_substitution_vtable(&mut self) {
    let owner = self as *mut ApplyTypeFunction as *mut ();
    self.base.base.vtable = SubstitutionVtable {
      owner,
      is_dirty_ty: Some(apply_type_function_is_dirty_ty),
      is_dirty_tp: Some(apply_type_function_is_dirty_tp),
      clean_ty: Some(apply_type_function_clean_ty),
      clean_tp: Some(apply_type_function_clean_tp),
      found_dirty_ty: Some(apply_type_function_found_dirty_ty),
      found_dirty_tp: Some(apply_type_function_found_dirty_tp),
      ignore_children_ty: Some(apply_type_function_ignore_children_ty),
      ignore_children_tp: Some(apply_type_function_ignore_children_tp),
      ignore_children_visit_ty: Some(apply_type_function_ignore_children_ty),
      ignore_children_visit_tp: Some(apply_type_function_ignore_children_tp),
    };
  }

  pub fn substitute_type_id(&mut self, ty: TypeId) -> Option<TypeId> {
    self.install_substitution_vtable();
    self.base.substitute_type_id(ty)
  }

  pub fn substitute_type_pack_id(&mut self, tp: TypePackId) -> Option<TypePackId> {
    self.install_substitution_vtable();
    self.base.substitute_type_pack_id(tp)
  }
}
