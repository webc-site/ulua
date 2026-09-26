use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::{
  records::{
    arena_handle::Handle, instantiation_2::Instantiation2, scope::Scope,
    substitution::Substitution, subtyping::Subtyping, tarjan::SubstitutionVtable, txn_log::TxnLog,
    type_arena::TypeArena,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

fn inst2_is_dirty_ty(owner: *mut (), ty: TypeId) -> bool {
  // Safety: owner 由 `install_substitution_vtable` 写入的 `self as *mut Instantiation2 as
  // *mut ()`（见 tarjan::SubstitutionVtable 文档），只在本 Instantiation2 存活期内、单线程
  // substitute 遍历中回调派发；回转并借 & 做只读 is_dirty_type_id（&self 安全方法）无别名冲突，
  // 入参 ty 为遍历中存活的 arena 类型句柄。
  unsafe { (*(owner as *mut Instantiation2)).is_dirty_type_id(ty) }
}

fn inst2_is_dirty_tp(owner: *mut (), tp: TypePackId) -> bool {
  // Safety: 同 inst2_is_dirty_ty——owner 回转 *mut Instantiation2 后借 & 调用只读的
  // is_dirty_type_pack_id（&self 安全方法），遍历单线程且此刻无并存 &mut；tp 为存活 arena 句柄。
  unsafe { (*(owner as *mut Instantiation2)).is_dirty_type_pack_id(tp) }
}

fn inst2_clean_ty(owner: *mut (), ty: TypeId) -> TypeId {
  // Safety: owner 由本类型 install 写入，回转 *mut Instantiation2 后重建 &mut 调用 clean_type_id
  // （&mut self 安全方法）——遍历单线程、回调点无并存借用故无别名冲突；ty 为存活 arena 句柄，
  // clean_type_id 内部只经构造接线且比遍历长寿的 arena/log 读写。
  unsafe { (*(owner as *mut Instantiation2)).clean_type_id(ty) }
}

fn inst2_clean_tp(owner: *mut (), tp: TypePackId) -> TypePackId {
  // Safety: 同 inst2_clean_ty——owner 回转 *mut Instantiation2 后独占重建 &mut 调用
  // clean_type_pack_id（&mut self 安全方法），单线程回调点无并存别名；tp 为存活 arena 句柄。
  unsafe { (*(owner as *mut Instantiation2)).clean_type_pack_id(tp) }
}

fn inst2_found_dirty_ty(owner: *mut (), ty: TypeId) {
  unsafe {
    // Safety: owner 回转 *mut Instantiation2 的 &mut 仅在单线程 substitute 回调期独占重建、
    // 无并存别名；`.base.found_dirty_type_id` 为 unsafe fn(&mut)，其入参 ty 为遍历中存活的
    // arena 类型句柄，内部只经构造接线且长寿的 log 读写依赖。
    (*(owner as *mut Instantiation2))
      .base
      .found_dirty_type_id(ty)
  }
}

fn inst2_found_dirty_tp(owner: *mut (), tp: TypePackId) {
  unsafe {
    // Safety: 同 inst2_found_dirty_ty——owner 回转 *mut Instantiation2 的 &mut 在单线程回调期内
    // 独占重建、无并存别名；`.base.found_dirty_type_pack_id` 为 unsafe fn(&mut)，tp 为存活 arena
    // 类型包句柄，内部只经构造接线且长寿的 log 读写。
    (*(owner as *mut Instantiation2))
      .base
      .found_dirty_type_pack_id(tp)
  }
}

fn inst2_ignore_children_ty(owner: *mut (), ty: TypeId) -> bool {
  // Safety: owner 回转 *mut Instantiation2 后借 & 调用只读的 ignore_children（&self 安全方法），
  // 单线程回调点无并存借用故无别名冲突；ty 为遍历中存活的 arena 类型句柄。
  unsafe { (*(owner as *mut Instantiation2)).ignore_children(ty) }
}

fn inst2_ignore_children_tp(_owner: *mut (), _tp: TypePackId) -> bool {
  false
}

impl Instantiation2 {
  pub fn instantiation_2_type_arena_dense_hash_map_type_id_type_id_dense_hash_map_type_pack_id_type_pack_id_not_null_subtyping_not_null_scope(
    arena: Handle<TypeArena>,
    generic_substitutions: DenseHashMap<TypeId, TypeId>,
    generic_pack_substitutions: DenseHashMap<TypePackId, TypePackId>,
    subtyping: *mut Subtyping,
    scope: *mut Scope,
  ) -> Self {
    Instantiation2 {
      base: Substitution::substitution_new(TxnLog::empty(), Some(arena)),
      generic_substitutions,
      generic_pack_substitutions,
      subtyping,
      scope,
    }
  }

  fn install_substitution_vtable(&mut self) {
    let owner = self as *mut Instantiation2 as *mut ();
    self.base.base.vtable = SubstitutionVtable {
      owner,
      is_dirty_ty: Some(inst2_is_dirty_ty),
      is_dirty_tp: Some(inst2_is_dirty_tp),
      clean_ty: Some(inst2_clean_ty),
      clean_tp: Some(inst2_clean_tp),
      found_dirty_ty: Some(inst2_found_dirty_ty),
      found_dirty_tp: Some(inst2_found_dirty_tp),
      ignore_children_ty: Some(inst2_ignore_children_ty),
      ignore_children_tp: Some(inst2_ignore_children_tp),
      ignore_children_visit_ty: Some(inst2_ignore_children_ty),
      ignore_children_visit_tp: Some(inst2_ignore_children_tp),
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
