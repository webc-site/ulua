use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::solver_mode::SolverMode,
  records::{
    arena_handle::Handle, builtin_types::BuiltinTypes,
    clone_public_interface::ClonePublicInterface, module::Module, substitution::Substitution,
    tarjan::SubstitutionVtable, txn_log::TxnLog,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

// 以下 `cpi_*` 均为 `SubstitutionVtable` 的 fn 指针槽：由 `install_substitution_vtable`
// 把 `self as *mut ClonePublicInterface as *mut ()` 存入 `vtable.owner`，仅在该
// 对象存活、处于 Substitution 遍历期时被回调，回调参数 `ty`/`tp` 皆为遍历传入的 arena
// 存活句柄。故每处 `owner`→`*mut ClonePublicInterface` 反引用与其 `&mut self` 再借用，
// 都在本 crate「单线程、遍历期裸指针重入」不变量下不产生并存别名。

fn cpi_is_dirty_ty(owner: *mut (), ty: TypeId) -> bool {
  // Safety: owner 为 install_substitution_vtable 装入的存活 ClonePublicInterface 句柄，
  // is_dirty_type_id 取 &mut self 的借用只覆盖本次回调；ty 是遍历传入的存活 TypeId。
  unsafe { (*(owner as *mut ClonePublicInterface)).is_dirty_type_id(ty) }
}

fn cpi_is_dirty_tp(owner: *mut (), tp: TypePackId) -> bool {
  // Safety: 同上，owner 存活非空；is_dirty_type_pack_id 本身为 unsafe fn，其前置「tp 为
  // arena 存活 TypePackId」由 vtable 分派的遍历实参满足，再借用不超出回调窗口。
  unsafe { (*(owner as *mut ClonePublicInterface)).is_dirty_type_pack_id(tp) }
}

fn cpi_clean_ty(owner: *mut (), ty: TypeId) -> TypeId {
  // Safety: owner 为遍历期存活的 ClonePublicInterface；clean_type_id 以 &mut self 就地
  // 改写其 Substitution 状态，借用覆盖本次调用，无并发持有者；ty 存活。
  unsafe { (*(owner as *mut ClonePublicInterface)).clean_type_id(ty) }
}

fn cpi_clean_tp(owner: *mut (), tp: TypePackId) -> TypePackId {
  // Safety: 同 clean_ty——owner 存活非空，&mut self 再借用只活过这次 clean_type_pack_id
  // 调用；tp 为遍历传入的 arena 存活 TypePackId。
  unsafe { (*(owner as *mut ClonePublicInterface)).clean_type_pack_id(tp) }
}

fn cpi_found_dirty_ty(owner: *mut (), ty: TypeId) {
  // Safety: owner 存活；`.base` 借用其 Substitution 记录 dirty，found_dirty_type_id 为
  // unsafe fn，其「ty 存活、Substitution 处于遍历中」前置由遍历实参满足，借用止于回调。
  unsafe {
    (*(owner as *mut ClonePublicInterface))
      .base
      .found_dirty_type_id(ty)
  }
}

fn cpi_found_dirty_tp(owner: *mut (), tp: TypePackId) {
  // Safety: 同上，经 owner 短借其 Substitution.base 记录 dirty TypePackId；tp 存活，
  // unsafe fn 前置由 vtable 分派的遍历持有，无并存可变借用。
  unsafe {
    (*(owner as *mut ClonePublicInterface))
      .base
      .found_dirty_type_pack_id(tp)
  }
}

fn cpi_ignore_children_visit_ty(owner: *mut (), ty: TypeId) -> bool {
  // Safety: owner 存活非空，ignore_children_visit_type_id 为 unsafe fn，其前置（ty 存活、
  // 遍历独占该对象）由回调实参满足；&mut self 再借用止于本次判定返回。
  unsafe { (*(owner as *mut ClonePublicInterface)).ignore_children_visit_type_id(ty) }
}

fn cpi_ignore_children_visit_tp(owner: *mut (), tp: TypePackId) -> bool {
  // Safety: 同 ignore_children_visit_ty——owner 为遍历期存活句柄，tp 存活，unsafe fn
  // 前置成立，借用不跨越回调窗口，故无非别名冲突。
  unsafe { (*(owner as *mut ClonePublicInterface)).ignore_children_visit_type_pack_id(tp) }
}

impl ClonePublicInterface {
  /// # Safety
  /// 直译 cpp `ClonePublicInterface` 构造：
  /// - `_module` 必须非空（下方 `LUAU_ASSERT!` 亦校验）并指向本次 clone pass 中**被
  ///   独占持有**的 `Module`——其 `interface_types` 会以 `&mut` 借给基类 Substitution，
  ///   直到本对象用完为止不得有其它 `&Module`/`&mut Module` 并存；
  /// - `_log` 必须是与本次替换同寿的存活 `TxnLog` 指针，`_builtin_types` 为构造期注入
  ///   的会话级非空句柄（Handle 编码非空），二者仅作身份保存。
  pub unsafe fn new(
    _log: *const TxnLog,
    _builtin_types: Handle<BuiltinTypes>,
    _module: *mut Module,
    _solver_mode: SolverMode,
  ) -> Self {
    LUAU_ASSERT!(!_module.is_null());

    // Safety: `_module` 经上面断言非空，且按函数级契约为本次 pass 独占存活的 Module；
    // 取 `&mut (*_module).interface_types` 的 arena 可变借用与传者的独占持有不冲突。
    let arena = unsafe { &mut (*_module).interface_types };
    let base = Substitution::substitution_new(_log, Some(Handle::from_mut(arena)));

    ClonePublicInterface {
      base,
      builtin_types: _builtin_types,
      module: _module,
      solver_mode: _solver_mode,
      internal_type_escaped: false,
    }
  }

  pub(crate) fn install_substitution_vtable(&mut self) {
    let owner = self as *mut ClonePublicInterface as *mut ();
    self.base.base.vtable = SubstitutionVtable {
      owner,
      is_dirty_ty: Some(cpi_is_dirty_ty),
      is_dirty_tp: Some(cpi_is_dirty_tp),
      clean_ty: Some(cpi_clean_ty),
      clean_tp: Some(cpi_clean_tp),
      found_dirty_ty: Some(cpi_found_dirty_ty),
      found_dirty_tp: Some(cpi_found_dirty_tp),
      // ClonePublicInterface overrides ignoreChildrenVisit in C++, but
      // inherits Tarjan::ignoreChildren=false. Replacement must still
      // rewrite children of clean cloned parents that were reached
      // through dirty internal children.
      ignore_children_ty: None,
      ignore_children_tp: None,
      ignore_children_visit_ty: Some(cpi_ignore_children_visit_ty),
      ignore_children_visit_tp: Some(cpi_ignore_children_visit_tp),
    };
  }
}
