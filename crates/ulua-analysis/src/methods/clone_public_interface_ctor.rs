use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::solver_mode::SolverMode,
  macros::substitution_vtable,
  records::{
    arena_handle::{Handle, alias},
    builtin_types::BuiltinTypes,
    clone_public_interface::ClonePublicInterface,
    module::Module,
    substitution::Substitution,
    txn_log::TxnLog,
  },
};

// ClonePublicInterface overrides ignoreChildrenVisit in C++, but
// inherits Tarjan::ignoreChildren=false. Replacement must still
// rewrite children of clean cloned parents that were reached
// through dirty internal children.
substitution_vtable!(visit_only, pub(crate) ClonePublicInterface);

impl ClonePublicInterface {
  /// # Safety
  /// 直译 cpp `ClonePublicInterface` 构造：
  /// - `_module` 必须非空（下方 `LUAU_ASSERT!` 亦校验）并指向本次 clone pass 中**被
  ///   独占持有**的 `Module`——其 `interface_types` 会以 `&mut` 借给基类 Substitution，
  ///   直到本对象用完为止不得有其它 `&Module`/`&mut Module` 并存；
  /// - `_log` 必须是与本次替换同寿的存活 `TxnLog` 指针，`_builtin_types` 为构造期注入
  ///   的会话级非空句柄（Handle 编码非空），二者仅作身份保存。
  pub(crate) unsafe fn new(
    _log: *const TxnLog,
    _builtin_types: Handle<BuiltinTypes>,
    _module: *mut Module,
    _solver_mode: SolverMode,
  ) -> Self {
    LUAU_ASSERT!(!_module.is_null());

    let arena = &mut alias(_module).interface_types;
    let base = Substitution::substitution_new(_log, Some(Handle::from_mut(arena)));

    ClonePublicInterface {
      base,
      builtin_types: _builtin_types,
      module: _module,
      solver_mode: _solver_mode,
      internal_type_escaped: false,
    }
  }
}
