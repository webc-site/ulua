//! arena 类型节点与约束节点的槽位写门面（`review.md` §2 的「非空指针 →
//! 引用/句柄」收口点）。
//!
//! cpp 侧两类惯用写法在 Rust 里原本是「裸句柄 + 就地解引用」：
//!
//! - `(*asMutable(ty)).ty = Bound{target}`：把 BlockedType 占位绑定到目标类型
//!   （`TypeId = *const Type`，可变访问靠 `const_cast`），散布在 visit 各分支；
//! - `constraint->deprecatedDependencies.push(other)`：约束图关闭时的退化
//!   依赖登记，两侧都是 generator 持有的 `Box::into_raw` 泄漏指针。
//!
//! 两者的解引用前提（目标由 arena / generator 保活、单线程顺序改写）在 cpp
//! 里是语言层面隐含的，在 Rust 里必须由 `unsafe` 表达；本模块把它收拢成
//! 具名动作，业务调用点恢复为一次函数调用，不再出现 `(*p).field = ..` 形态。
//!
//! 越界方向一律取安全侧：句柄为 null 属契约违例（cpp 同处直接 UB），由
//! [`crate::records::arena_handle::Handle`] 的 panic 语义兜住，不静默跳过。

use core::ptr::from_ref;

use ulua_ast::records::ast_expr_function::AstExprFunction;

use crate::{
  functions::{
    as_mutable_type::as_mutable_type_id, for_each_constraint::for_each_constraint,
    get_mutable_type::get_mutable,
    propagate_deprecated_attribute_to_constraint::propagate_deprecated_attribute_to_constraint,
  },
  records::{
    arena_handle::{alias, alias_ref},
    blocked_type::BlockedType,
    checkpoint::Checkpoint,
    constraint::Constraint,
    constraint_generator::ConstraintGenerator,
    r#type::Type,
  },
  type_aliases::{constraint_v::ConstraintV, type_id::TypeId, type_variant::TypeVariant},
};

/// arena `TypeId` → 可变 `Type` 节点：cpp `asMutable<Type>(ty)` 后解引用的收口。
#[inline]
pub(crate) fn type_slot(ty: TypeId) -> &'static mut Type {
  alias(as_mutable_type_id(ty))
}

/// cpp `(*asMutable(ty)).ty = Bound{target}`：把占位（BlockedType 等）绑定到目标。
#[inline]
pub(crate) fn bind_type(ty: TypeId, target: TypeId) {
  type_slot(ty).ty = TypeVariant::Bound(target);
}

/// generator 持有的约束节点 → 可变借用（`constraints: Vec<*mut Constraint>`
/// 里的元素由 `Box::into_raw` 泄漏、随 generator 存活）。
#[inline]
pub(crate) fn constraint_slot(constraint: *mut Constraint) -> &'static mut Constraint {
  alias(constraint)
}

/// 约束节点的只读视图（cpp `const Constraint&`，用于匹配 `ConstraintV`）。
#[inline]
pub(crate) fn constraint_view(constraint: *mut Constraint) -> &'static Constraint {
  alias_ref(constraint)
}

/// 向约束追加一条退化依赖（cpp `c->deprecatedDependencies.push_back(dep)`）。
#[inline]
pub(crate) fn add_deprecated_dependency(constraint: *mut Constraint, dep: *mut Constraint) {
  constraint_slot(constraint)
    .deprecated_dependencies
    .push(dep);
}

/// cpp `getMutable<BlockedType>(ty)->setOwner(gc)`：让 BlockedType 占位指向负责
/// 泛化它的约束（`set_owner` 只保存地址、不解引用 owner，见
/// `methods/blocked_type_set_owner.rs`）。
///
/// 句柄未命中 BlockedType 时（cpp 同处对 `getMutable` 的返回值直接解引用，
/// 即 UB）取安全方向：跳过写入。
#[inline]
pub(crate) fn block_owner_at(ty: TypeId, constraint: *mut Constraint) {
  if let Some(blocked) = get_mutable::<BlockedType>(ty) {
    blocked.set_owner(constraint.cast_const());
  }
}

/// cpp 约束图关闭（`LuauConstraintGraph=false`）时的「链式退化依赖」：
/// `[start, end)` 区间内每条约束都排在 `constraint` 之前派发，且其中
/// `PackSubtype{returns}` 的约束按出现顺序两两串联。
///
/// 对应 C++ ConstraintGenerator.cpp:1546/2230 等四处同款循环；遍历区间内
/// 元素均出自 `generator.constraints`（`Box::into_raw` 泄漏、随 generator
/// 存活），故解引用只发生在 [`constraint_slot`]/[`add_deprecated_dependency`]
/// 这两个收口点内。
pub(crate) fn chain_deprecated_dependencies(
  cg: &ConstraintGenerator,
  start: Checkpoint,
  end: Checkpoint,
  constraint: *mut Constraint,
) {
  let mut previous = None;
  for_each_constraint(start, end, cg, |run| {
    add_deprecated_dependency(constraint, run);
    if let ConstraintV::PackSubtype(psc) = &constraint_view(run).c
      && psc.returns
    {
      if let Some(previous) = previous {
        add_deprecated_dependency(run, previous);
      }
      previous = Some(run);
    }
  });
}

/// [`propagate_deprecated_attribute_to_constraint`] 的安全包装：cpp 形态
/// （约束句柄 + 存活函数节点）在门面内折算为引用，调用点无 `unsafe`。
pub(crate) fn propagate_deprecated_attribute(constraint: *mut Constraint, func: &AstExprFunction) {
  // Safety: `constraint` 为紧邻 add_constraint 返回、由 generator 持有的有效
  // 节点；`func` 由调用方自 parse arena 交出的存活节点借用还原地址，与 cpp
  // 传入的 `AstExprFunction*` 同一目标（被调方只读取其 attributes）。
  // 解引用 `&mut (*c).c` 与 cpp `&c->c` 同一形态。
  let body = &mut constraint_slot(constraint).c;
  // Safety: `body` 为上方存活约束的约束体；`from_ref(func)` 只交出地址（被调方
  // 形参即 `*const AstExprFunction`），沿该节点只读遍历 attributes。
  unsafe { propagate_deprecated_attribute_to_constraint(body, from_ref(func)) };
}
