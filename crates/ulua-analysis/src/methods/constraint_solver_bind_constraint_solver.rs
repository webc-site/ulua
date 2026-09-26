use alloc::string::String;
use std::ptr::eq;

use ulua_common::{fflag, macros::luau_assert::LUAU_ASSERT};

use crate::{
  enums::{occurs_check_result::OccursCheckResult, polarity::Polarity},
  functions::{
    as_mutable_type::as_mutable_type_id, as_mutable_type_pack::as_mutable_type_pack, follow_type,
    follow_type_pack, fresh_type::fresh_type, get_type, get_type_pack,
    occurs_check_type_utils::occurs_check_type_pack_id_type_pack_id,
    track_interior_free_type::track_interior_free_type,
  },
  methods::{
    unifiable_bound_type_id_emplace_type_bound_type::unifiable_bound_type_id_emplace_type_bound_type,
    unifiable_bound_type_pack_id_emplace_type_pack_bound_type_pack::emplace_type_pack,
  },
  records::{
    blocked_type::BlockedType, blocked_type_pack::BlockedTypePack, constraint::Constraint,
    constraint_solver::ConstraintSolver, free_type::FreeType, free_type_pack::FreeTypePack,
    internal_error::InternalError, pending_expansion_type::PendingExpansionType,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId, type_variant::TypeVariant},
};

impl ConstraintSolver {
  /// 安全封装（crate 内部门面）：把正在派发的 `&Constraint` 作为 owner 键，将 `ty` 绑定到 `bound_to`。
  ///
  /// 裸指针转换收敛于此一处——下游 [`ConstraintSolver::bind_not_null_constraint_type_id_type_id`]
  /// 的 `NotNull<const Constraint>` 参数只按同一性比较该指针并 Copy 出 `scope`/`location`，
  /// 引用语义天然保证非空且整调用存活，因此业务调用点无需再包 `unsafe`。
  ///
  /// 可见性收口为 `pub(crate)`：门面消费方全在本 crate 的约束派发层；对外 API 面仍是带
  /// `# Safety` 契约的 unsafe `bind_not_null_*` 裸指针形态。若以 `pub` 安全函数把 `TypeId`/
  /// `TypePackId`（裸指针句柄）实参转发给 unsafe 被调方，即构成 deny-by-default 的
  /// `not_unsafe_ptr_arg_deref`（公共安全函数不得可能解引用裸指针参数）。
  pub(crate) fn bind(&mut self, constraint: &Constraint, ty: TypeId, bound_to: TypeId) {
    // # Safety: `constraint` 为活的共享引用（非空、对齐、整调用存活），满足下游
    // unsafe fn 对 NotNull 参数的契约；`ty`/`bound_to` 的可改性与 arena 存活前提由调用点保证。
    unsafe {
      self.bind_not_null_constraint_type_id_type_id(constraint as *const Constraint, ty, bound_to)
    };
  }

  /// 安全封装：[`ConstraintSolver::bind`] 的 TypePack 版（可见性收口理由同 [`ConstraintSolver::bind`]）。
  pub(crate) fn bind_pack(
    &mut self,
    constraint: &Constraint,
    tp: TypePackId,
    bound_to: TypePackId,
  ) {
    // # Safety: 同 [`ConstraintSolver::bind`]——`&Constraint` 引用非空且整调用存活。
    unsafe {
      self.bind_not_null_constraint_type_pack_id_type_pack_id(
        constraint as *const Constraint,
        tp,
        bound_to,
      )
    };
  }

  /// # Safety
  /// 对应 cpp `ConstraintSolver::bind(NotNull<const Constraint>, TypeId, TypeId)`
  /// （`Analysis/src/ConstraintSolver.cpp:870-892`）的参数契约：
  /// - `constraint`：非空、对齐且在本次调用期间存活的 `Constraint` 指针（C++
  ///   `NotNull` 直传，通常取正在派发的那条约束）。本函数只读它的 `scope`/
  ///   `location` 两个字段，并把指针值用作 BlockedType owner 的同一性比较
  ///   （`can_mutate_type_id` 只比对、不解引用）。
  /// - `ty`：follow 后必须是 BlockedType、FreeType 或 PendingExpansionType 的
  ///   arena 驻留节点；若为 BlockedType，其 `owner` 必须恰等于 `constraint`
  ///   （入口 `LUAU_ASSERT!(can_mutate_type_id(..))` 校验，release 下由派发层
  ///   不变量保证）——本函数会原地写该节点的 `.ty` 槽，owner 不符即篡夺其它
  ///   约束独占的节点；且调用期间不得有其它并存的可变借用（solver 单线程独占）。
  /// - `bound_to`：有效 arena TypeId；本函数只 follow 并读取，不写该节点。
  pub unsafe fn bind_not_null_constraint_type_id_type_id(
    &mut self,
    constraint: *const Constraint,
    ty: TypeId,
    bound_to: TypeId,
  ) {
    LUAU_ASSERT!(
      get_type::get::<BlockedType>(ty).is_some()
        || get_type::get::<FreeType>(ty).is_some()
        || get_type::get::<PendingExpansionType>(ty).is_some()
    );
    LUAU_ASSERT!(can_mutate_type_id(ty, constraint));

    let bound_to = follow_type::follow(bound_to);
    // Safety: 契约参数 `constraint` 按 NotNull 直传语义非空且整调用存活，
    // 读取 scope 仅用于给下方 fresh 类型锚定绑定作用域。
    let scope = unsafe { (*constraint).scope };
    // Safety: 同一存活约束对象再读 `location`：仅 Copy 出 Location 值参与
    // unblock/报错定位，不持有对 constraint 的引用，无别名延长。
    let location = unsafe { (*constraint).location };

    if fflag::LuauOccursCheckForAllBindings.get() {
      // This follow shouldn't be needed, but if for some reason we end up
      // with a bound type, we want to also follow it when doing this
      // occurence check.
      if follow_type::follow(ty) == bound_to {
        let fresh_ty = fresh_type(
          // Safety: arena 为构造期 NotNull 布线的独占分配；自绑定兜底分支要
          // fresh 一个新 FreeType，写借用只覆盖本次调用，无并存借用者。
          { self.arena.get_mut() },
          // Safety: builtin_types 同为构造期独立分配，此处只读取 never/unknown
          // 种子；与 arena 分配互不交叠，故并存借用成立。
          { self.builtin_types.get() },
          scope,
          Polarity::Mixed,
        );
        let mutable_ty = as_mutable_type_id(ty);
        let mut fresh_arg = fresh_ty;
        unifiable_bound_type_id_emplace_type_bound_type(
          // Safety: `mutable_ty` 是对入口已校验可改性的 `ty` 做 const→mut reinterpret，
          // 指向 arena 内存活类型节点；emplace（安全函数）以独占引用写 `.ty =
          // Bound(fresh_ty)`，作用域止于本调用，对应 cpp `emplaceType<BoundType>`。
          unsafe { &mut *mutable_ty },
          &mut fresh_arg,
        );
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
      // Safety: 本分支进入前提是 `ty` 为 BlockedType 且 `bound_to` 就是它自己，
      // 入口 can_mutate 断言已钉住其 owner 等于 `constraint`，故有权替换该槽位：
      // 把 `.ty` 从 Blocked 改写为刚构造的 FreeType（cpp DEPRECATED_emplace<FreeType>
      // 的原地 writeback），solver 单线程独占，写点无并存可变借用。
      unsafe {
        (*mutable_ty).ty = TypeVariant::Free(free_ty);
      }
      self.unblock_type_id_location(ty, location);
      track_interior_free_type(scope, ty);
      return;
    }

    let mutable_ty = as_mutable_type_id(ty);
    let mut bound_arg = bound_to;
    unifiable_bound_type_id_emplace_type_bound_type(
      // Safety: bind 主路径：`mutable_ty` 指向入口校验过（BlockedType 时 owner 恰为
      // `constraint`）的 arena 存活节点，独占可变引用只用于写 `.ty = Bound(bound_to)`
      // 一次，与 cpp `emplaceType<BoundType>(asMutable(ty), boundTo)` 同契约同范围。
      unsafe { &mut *mutable_ty },
      &mut bound_arg,
    );

    if !fflag::LuauConstraintGraph.get() {
      // `unblock` will "shift references" under the hood.
      self.deprecate_d_shift_references(ty, bound_to);
    }

    self.unblock_type_id_location(ty, location);
  }
}

// C++ `[[maybe_unused]] static bool canMutate(TypeId ty, NotNull<const Constraint> constraint)`
// (`Analysis/src/ConstraintSolver.cpp:88-98`), used only in asserts.
fn can_mutate_type_id(ty: TypeId, constraint: *const Constraint) -> bool {
  if let Some(blocked) = get_type::get::<BlockedType>(ty) {
    let owner = blocked.get_owner();
    LUAU_ASSERT!(!owner.is_null());
    return owner == constraint;
  }
  true
}

impl ConstraintSolver {
  /// # Safety
  /// 对应 cpp `ConstraintSolver::bind(NotNull<const Constraint>, TypePackId,
  /// TypePackId)`（`Analysis/src/ConstraintSolver.cpp:894-917`）的参数契约：
  /// - `constraint`：非空、对齐、整调用存活的 `Constraint` 指针（C++ `NotNull`
  ///   直传）；仅读取 `location` 用于报错与 unblock 定位，并作 BlockedTypePack
  ///   owner 的同一性比较（`can_mutate_type_pack_id` 只比对指针值）。
  /// - `tp`：follow 前即须为 BlockedTypePack 或 FreeTypePack 的 arena 驻留节点
  ///   （入口断言）；若为 BlockedTypePack，其 `owner` 必须等于 `constraint`——
  ///   本函数经 `emplace_type_pack` 原地覆写该 pack 的 `.ty` 槽，owner 不符会
  ///   破坏另一约束的独占 writeback；调用期间不得有并存可变借用。
  /// - `bound_to`：有效 arena TypePackId；follow 后必须不同于 `tp`（入口断言，
  ///   拒绝自环绑定）。
  pub unsafe fn bind_not_null_constraint_type_pack_id_type_pack_id(
    &mut self,
    constraint: *const Constraint,
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

    // Safety: `constraint` 是本 unsafe fn 契约中的 NotNull 直传参数，调用期间
    // 指向存活的约束对象；只 Copy 出 location 值，不派生长期引用。
    let location = unsafe { (*constraint).location };

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
      // Safety: `emplace_type_pack` 是 unsafe fn，要求 pack 指针存活可写：
      // `mutable_tp` 由入口校验过 owner（或本就不是 Blocked）的 `tp` 做 const→mut
      // reinterpret 而来，occurs-check 失败路径把该槽覆写为 Bound(errorTypePack)
      // （cpp 同分支），覆写期间无其它借用者。
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
}

// C++ `[[maybe_unused]] static bool canMutate(TypePackId tp, NotNull<const Constraint> constraint)`
// (`Analysis/src/ConstraintSolver.cpp:101-111`), used only in asserts.
fn can_mutate_type_pack_id(tp: TypePackId, constraint: *const Constraint) -> bool {
  if let Some(blocked) = get_type_pack::get::<BlockedTypePack>(tp) {
    let owner = blocked.owner;
    LUAU_ASSERT!(!owner.is_null());
    return eq(owner, constraint);
  }
  true
}
