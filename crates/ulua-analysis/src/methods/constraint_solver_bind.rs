use alloc::string::String;
use core::ptr::{NonNull, from_ref};

use ulua_common::{fflag, macros::luau_assert::LUAU_ASSERT};

use crate::{
  enums::{occurs_check_result::OccursCheckResult, polarity::Polarity},
  functions::{
    as_mutable_type::as_mutable_type_id, as_mutable_type_pack::as_mutable_type_pack, follow_type,
    follow_type_pack, fresh_type::fresh_type, get_type, get_type_pack,
    occurs_check_type_utils::occurs_check_type_pack_id_type_pack_id,
    track_interior_free_type::track_interior_free_type,
  },
  methods::unifiable::{emplace_type_pack, unifiable_bound_type_id_emplace_type_bound_type},
  records::{
    arena_handle::{alias, alias_ref},
    blocked_type::BlockedType,
    blocked_type_pack::BlockedTypePack,
    constraint::Constraint,
    constraint_solver::ConstraintSolver,
    free_type::FreeType,
    free_type_pack::FreeTypePack,
    internal_error::InternalError,
    pending_expansion_type::PendingExpansionType,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId, type_variant::TypeVariant},
};

impl ConstraintSolver {
  /// 把 `ty` 绑定到 `bound_to`，`constraint` 为正在派发的约束（Blocked 节点
  /// 的 owner 键）。
  ///
  /// 对应 cpp `ConstraintSolver::bind(NotNull<const Constraint>, TypeId, TypeId)`
  /// （`Analysis/src/ConstraintSolver.cpp:870-892`）。参数契约：
  /// - `constraint`：正在派发的约束（C++ `NotNull` 直传）。本函数只读它的
  ///   `scope`/`location` 两字段，并把其地址用作 BlockedType owner 的同一性
  ///   比较（`can_mutate_type_id` 只比对、不延长借用）。
  /// - `ty`：follow 后必须是 BlockedType、FreeType 或 PendingExpansionType 的
  ///   arena 驻留节点；若为 BlockedType，其 `owner` 必须恰等于 `constraint`
  ///   （入口 `LUAU_ASSERT!(can_mutate_type_id(..))` 校验，release 下由派发层
  ///   不变量保证）——本函数会原地写该节点的 `.ty` 槽，owner 不符即篡夺其它
  ///   约束独占的节点；且调用期间不得有其它并存的可变借用（solver 单线程独占）。
  /// - `bound_to`：有效 arena TypeId；本函数只 follow 并读取，不写该节点。
  pub(crate) fn bind(&mut self, constraint: &Constraint, ty: TypeId, bound_to: TypeId) {
    LUAU_ASSERT!(
      get_type::get::<BlockedType>(ty).is_some()
        || get_type::get::<FreeType>(ty).is_some()
        || get_type::get::<PendingExpansionType>(ty).is_some()
    );
    LUAU_ASSERT!(can_mutate_type_id(ty, constraint));

    let bound_to = follow_type::follow(bound_to);
    // scope 为存活约束登记的 NotNull<Scope> 裸指针，此处仅值复制；需要引用
    // 处经 alias_ref 收口物化，只用于给下方 fresh 类型锚定绑定作用域。
    let scope = constraint.scope;
    // location 仅 Copy 出值参与 unblock/报错定位，不持有对 constraint 的借用。
    let location = constraint.location;

    if fflag::LuauOccursCheckForAllBindings.get() {
      // This follow shouldn't be needed, but if for some reason we end up
      // with a bound type, we want to also follow it when doing this
      // occurence check.
      if follow_type::follow(ty) == bound_to {
        let fresh_ty = fresh_type(
          // Handle::get_mut/get 均以 `&self` 物化借用，两字段并存借用无需 unsafe。
          self.arena.get_mut(),
          self.builtin_types.get(),
          Some(alias_ref(scope)),
          Polarity::Mixed,
        );
        let mutable_ty = as_mutable_type_id(ty);
        let mut fresh_arg = fresh_ty;
        // alias 为 arena 节点别名的 crate 收口点：以独占引用写
        // `.ty = Bound(fresh_ty)`，作用域止于本调用，对应 cpp
        // `emplaceType<BoundType>`。
        unifiable_bound_type_id_emplace_type_bound_type(alias(mutable_ty), &mut fresh_arg);
        track_interior_free_type(scope, fresh_ty);
        self.unblock_type_id_location(ty, location);
        return;
      }
    } else if get_type::get::<BlockedType>(ty).is_some() && ty == bound_to {
      // DEPRECATED_emplace<FreeType>(constraint, ty, scope, never_type, unknown_type, Polarity::Mixed)
      // FIXME?  Is this the right polarity?
      let free_ty = FreeType::free_type_scope_type_id_type_id_polarity(
        scope,
        self.builtin_types_ref().never_type,
        self.builtin_types_ref().unknown_type,
        Polarity::Mixed,
      );
      let mutable_ty = as_mutable_type_id(ty);
      // 本分支进入前提是 `ty` 为 BlockedType 且 `bound_to` 就是它自己，
      // 入口 can_mutate 断言已钉住其 owner 等于 `constraint`，故有权替换该
      // 槽位：把 `.ty` 从 Blocked 改写为刚构造的 FreeType（cpp
      // DEPRECATED_emplace<FreeType> 的原地 writeback），solver 单线程独占，
      // 写点无并存可变借用。
      alias(mutable_ty).ty = TypeVariant::Free(free_ty);
      self.unblock_type_id_location(ty, location);
      track_interior_free_type(scope, ty);
      return;
    }

    let mutable_ty = as_mutable_type_id(ty);
    let mut bound_arg = bound_to;
    // bind 主路径：`mutable_ty` 指向入口校验过（BlockedType 时 owner 恰为
    // `constraint`）的 arena 存活节点，独占可变引用只用于写
    // `.ty = Bound(bound_to)` 一次，与 cpp `emplaceType<BoundType>(asMutable(ty), boundTo)`
    // 同契约同范围。
    unifiable_bound_type_id_emplace_type_bound_type(alias(mutable_ty), &mut bound_arg);

    if !fflag::LuauConstraintGraph.get() {
      // `unblock` will "shift references" under the hood.
      self.deprecate_d_shift_references(ty, bound_to);
    }

    self.unblock_type_id_location(ty, location);
  }

  /// [`ConstraintSolver::bind`] 的裸指针（cpp `NotNull` 直传）兼容边界，
  /// 仅供尚未迁移的 `*const Constraint` 调用点（bidirectional type pusher、
  /// iterable 派发分支等）原样续传；新调用点一律直接用安全形态。
  ///
  /// # Safety
  /// `constraint`：非空、对齐且整调用期间指向存活的 `Constraint`（即正在
  /// 派发的那条约束）；`ty`/`bound_to` 前提同 [`ConstraintSolver::bind`]。
  pub(crate) unsafe fn bind_not_null_constraint_type_id_type_id(
    &mut self,
    constraint: *const Constraint,
    ty: TypeId,
    bound_to: TypeId,
  ) {
    // SAFETY: 契约已钉住 `constraint` 非空且活过本次调用；alias_ref 是
    // 全 crate 裸指针别名物化的收口点，仅恢复共享引用后转发安全实现。
    self.bind(alias_ref(constraint), ty, bound_to);
  }
}

// C++ `[[maybe_unused]] static bool canMutate(TypeId ty, NotNull<const Constraint> constraint)`
// (`Analysis/src/ConstraintSolver.cpp:88-98`), used only in asserts.
// BlockedType.owner 存的是裸指针身份，故仅在指针值层面比对，不解引用。
fn can_mutate_type_id(ty: TypeId, constraint: &Constraint) -> bool {
  if let Some(blocked) = get_type::get::<BlockedType>(ty) {
    let owner = blocked.get_owner();
    LUAU_ASSERT!(!owner.is_null());
    return owner == from_ref(constraint);
  }
  true
}

impl ConstraintSolver {
  /// 把类型 pack `tp` 绑定到 `bound_to`，`constraint` 为正在派发的约束。
  ///
  /// 对应 cpp `ConstraintSolver::bind(NotNull<const Constraint>, TypePackId,
  /// TypePackId)`（`Analysis/src/ConstraintSolver.cpp:894-917`）。参数契约：
  /// - `constraint`：正在派发的约束（C++ `NotNull` 直传）；仅读取 `location`
  ///   用于报错与 unblock 定位，并作 BlockedTypePack owner 的同一性比较
  ///   （`can_mutate_type_pack_id` 只比对地址）。
  /// - `tp`：follow 前即须为 BlockedTypePack 或 FreeTypePack 的 arena 驻留节点
  ///   （入口断言）；若为 BlockedTypePack，其 `owner` 必须等于 `constraint`——
  ///   本函数经 `emplace_type_pack` 原地覆写该 pack 的 `.ty` 槽，owner 不符会
  ///   破坏另一约束的独占 writeback；调用期间不得有并存可变借用。
  /// - `bound_to`：有效 arena TypePackId；follow 后必须不同于 `tp`（入口断言，
  ///   拒绝自环绑定）。
  pub(crate) fn bind_pack(
    &mut self,
    constraint: &Constraint,
    tp: TypePackId,
    bound_to: TypePackId,
  ) {
    LUAU_ASSERT!(
      get_type_pack::get::<BlockedTypePack>(tp).is_some()
        || get_type_pack::get::<FreeTypePack>(tp).is_some()
    );
    LUAU_ASSERT!(can_mutate_type_pack_id(tp, constraint));

    let bound_to = follow_type_pack::follow(bound_to);
    LUAU_ASSERT!(tp != bound_to);

    // 仅 Copy 出 location 值参与报错/unblock，不派生长期借用。
    let location = constraint.location;

    if fflag::LuauOccursCheckForAllBindings.get()
      && occurs_check_type_pack_id_type_pack_id(tp, bound_to) == OccursCheckResult::Fail
    {
      self.report_error_type_error_data_location(
        InternalError {
          message: String::from("Attempted to create a type pack cycle"),
        }
        .into(),
        &location,
      );
      let mutable_tp = as_mutable_type_pack(tp);
      let mut err_arg = self.builtin_types_ref().error_type_pack;
      // Safety: `emplace_type_pack` 是 unifiable 层原地覆写 arena pack 节点的
      // 最小契约边界（其文件未在本次迁移范围）：`mutable_tp` 由入口校验过
      // owner（或本就不是 Blocked）的 `tp` 取得，occurs-check 失败路径把该
      // 槽覆写为 Bound(errorTypePack)（cpp 同分支），覆写期间无其它借用者。
      unsafe { emplace_type_pack(mutable_tp, &mut err_arg) };
    } else {
      let mutable_tp = as_mutable_type_pack(tp);
      let mut bound_arg = bound_to;
      // Safety: 正常绑定路径覆写 `.ty = Bound(bound_to)`；`tp`/`bound_to` 均为
      // arena 驻留 pack 句柄且上方已排除 tp==bound_to 自环，callee 契约
      // （目标指针有效、实参 pack 存活）成立。
      unsafe { emplace_type_pack(mutable_tp, &mut bound_arg) };
    }

    self.unblock_type_pack_id_location(tp, location);
  }

  /// [`ConstraintSolver::bind_pack`] 的裸指针（cpp `NotNull` 直传）兼容边界，
  /// 仅供尚未迁移的 `*const Constraint` 调用点原样续传。
  ///
  /// # Safety
  /// `constraint`：非空、对齐且整调用期间指向存活的 `Constraint`；
  /// `tp`/`bound_to` 前提同 [`ConstraintSolver::bind_pack`]。
  pub(crate) unsafe fn bind_not_null_constraint_type_pack_id_type_pack_id(
    &mut self,
    constraint: *const Constraint,
    tp: TypePackId,
    bound_to: TypePackId,
  ) {
    // SAFETY: 契约钉住非空与存活；alias_ref 收口物化共享引用后转发安全实现。
    self.bind_pack(alias_ref(constraint), tp, bound_to);
  }
}

// C++ `[[maybe_unused]] static bool canMutate(TypePackId tp, NotNull<const Constraint> constraint)`
// (`Analysis/src/ConstraintSolver.cpp:101-111`), used only in asserts.
// 指针同一性比较：等价原 `std::ptr::eq(owner, constraint)`（NonNull 按地址判等）。
fn can_mutate_type_pack_id(tp: TypePackId, constraint: &Constraint) -> bool {
  if let Some(blocked) = get_type_pack::get::<BlockedTypePack>(tp) {
    let owner = blocked.owner;
    LUAU_ASSERT!(owner.is_some());
    return owner == Some(NonNull::from(constraint));
  }
  true
}
