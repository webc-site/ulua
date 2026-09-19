//! 对照 C++ `cpp/tests/TypeInfer.tryUnify.test.cpp`（TryUnifyFixture）与
//! `TypeInfer.union` 收窄语义，补充老求解器 `Unifier::can_unify` 的
//! 类型收窄与 unify 边界测试。错误消息文本是输出契约，此处只验证结构
//! （TypeErrorData 种类与 wanted/given），不改写文本。
//!
//! fixture 复刻 `check_type_match` 的 Old 分支：独立 arena + builtin +
//! normalize=false（避开归一化开销），只驱动 unify 本体。

use std::vec::Vec;

use ulua_analysis::{
  enums::{solver_mode::SolverMode, variance::Variance},
  functions::follow_type::follow_type_id,
  records::{
    boolean_singleton::BooleanSingleton,
    builtin_types::BuiltinTypes,
    count_mismatch::CountMismatchContext,
    internal_error_reporter::InternalErrorReporter,
    normalizer::Normalizer,
    primitive_type::{PrimitiveType, Type as PrimKind},
    scope::Scope,
    singleton_type::SingletonType,
    txn_log::TxnLog,
    type_arena::TypeArena,
    unifier::Unifier,
    unifier_shared_state::UnifierSharedState,
    union_type::UnionType,
  },
  type_aliases::{
    singleton_variant::SingletonVariant, type_error_data::TypeErrorData, type_id::TypeId,
  },
};
use ulua_ast::records::location::Location;

/// 持有一组自引用裸指针（types/builtin/scope/shared/normalizer 互指）。
/// 各协作对象装在 `Box` 里保证地址稳定（与 C++ 里它们作为固定成员同理），
/// 因此本结构体被 move 后指针仍然有效。
struct UnifyFixture {
  builtin: Box<BuiltinTypes>,
  _ice: Box<InternalErrorReporter>,
  shared: Box<UnifierSharedState>,
  arena: Box<TypeArena>,
  scope: Box<Scope>,
  normalizer: Box<Normalizer>,
}

impl UnifyFixture {
  fn new() -> Self {
    let mut builtin = Box::new(BuiltinTypes::new());
    let mut ice = Box::new(InternalErrorReporter::default());
    let mut shared = Box::new(UnifierSharedState::new(
      &mut *ice as *mut InternalErrorReporter,
    ));
    let mut arena = Box::new(TypeArena::default());
    let empty_tp = builtin.empty_type_pack;
    let scope = Box::new(Scope::scope_type_pack_id(empty_tp));
    let normalizer = Box::new(Normalizer::new(
      &mut *arena as *mut TypeArena,
      &mut *builtin as *mut BuiltinTypes,
      &mut *shared as *mut UnifierSharedState,
      SolverMode::Old,
      false,
    ));
    Self {
      builtin,
      _ice: ice,
      shared,
      arena,
      scope,
      normalizer,
    }
  }

  /// 复刻 `check_type_match` Old 分支的 Unifier：normalize / check_inhabited 关闭。
  fn unifier(&mut self) -> Unifier {
    Unifier {
      types: &mut *self.arena as *mut TypeArena,
      builtin_types: &mut *self.builtin as *mut BuiltinTypes,
      normalizer: &mut *self.normalizer as *mut Normalizer,
      scope: &mut *self.scope as *mut Scope,
      log: TxnLog::new(),
      failure: false,
      errors: Vec::new(),
      location: Location::default(),
      variance: Variance::Covariant,
      normalize: false,
      check_inhabited: false,
      ctx: CountMismatchContext::Arg,
      shared_state: &mut *self.shared as *mut UnifierSharedState,
      blocked_types: Vec::new(),
      blocked_type_packs: Vec::new(),
      first_pack_error_pos: None,
    }
  }

  fn number(&self) -> TypeId {
    self.builtin.number_type
  }
  fn string(&self) -> TypeId {
    self.builtin.string_type
  }
  fn boolean(&self) -> TypeId {
    self.builtin.boolean_type
  }
  fn nil(&self) -> TypeId {
    self.builtin.nil_type
  }
  fn never(&self) -> TypeId {
    self.builtin.never_type
  }
  fn any(&self) -> TypeId {
    self.builtin.any_type
  }

  /// 在本 fixture 的 arena 里铸造一个新基本类型（区别于 builtin 持久类型，
  /// 用于验证“结构相同但指针不同”的两类型 unify）。
  fn mint_primitive(&mut self, p: PrimKind) -> TypeId {
    self.arena.add_type(PrimitiveType {
      r#type: p,
      metatable: None,
    })
  }

  fn union_of(&mut self, options: Vec<TypeId>) -> TypeId {
    self.arena.add_type(UnionType { options })
  }

  fn bool_singleton_true(&mut self) -> TypeId {
    self.arena.add_type(SingletonType {
      variant: SingletonVariant::V0(BooleanSingleton::new(true)),
    })
  }
}

/// 统一成功（无错误）。
fn unifies(f: &mut UnifyFixture, sub: TypeId, sup: TypeId) {
  let errors = f.unifier().can_unify_type_id_type_id(sub, sup);
  assert!(
    errors.is_empty(),
    "期望可统一 {sub:p} <: {sup:p}，实际产生 {} 个错误",
    errors.len()
  );
}

/// 统一失败且首个错误为 TypeMismatch。
fn mismatch(f: &mut UnifyFixture, sub: TypeId, sup: TypeId) {
  let errors = f.unifier().can_unify_type_id_type_id(sub, sup);
  assert!(!errors.is_empty(), "期望不可统一 {sub:p} <: {sup:p}");
  assert!(
    matches!(errors[0].data, TypeErrorData::TypeMismatch(_)),
    "期望 TypeMismatch，实际 {:?}",
    errors[0].data
  );
}

#[test]
fn identical_primitive_pointers_unify() {
  // C++ `primitives_unify`：同一 number 自身 → 无错
  let mut f = UnifyFixture::new();
  let n = f.number();
  unifies(&mut f, n, n);
}

#[test]
fn structurally_equal_primitives_unify() {
  // 两个不同 arena 实例的 number 仍按结构统一（走 tryUnifyPrimitives 的相等分支）
  let mut f = UnifyFixture::new();
  let a = f.mint_primitive(PrimKind::Number);
  let b = f.mint_primitive(PrimKind::Number);
  assert_ne!(a, b, "应为不同 arena 节点");
  unifies(&mut f, a, b);
}

#[test]
fn distinct_primitives_do_not_unify() {
  // number <: string → TypeMismatch
  let mut f = UnifyFixture::new();
  let (n, s) = (f.number(), f.string());
  mismatch(&mut f, n, s);
}

#[test]
fn never_is_subtype_of_any_concrete() {
  // never 是 bottom：never <: string 通过（走 tryUnifyWithAny(neverType) 分支）
  let mut f = UnifyFixture::new();
  let (nv, s) = (f.never(), f.string());
  unifies(&mut f, nv, s);
}

#[test]
fn any_is_not_subtype_of_concrete_without_normalization() {
  // normalize=false 时 any <: string 直接 failure（与 C++ 一致：置 failure 后走 withAny 分支）
  let mut f = UnifyFixture::new();
  let (a, s) = (f.any(), f.string());
  let mut u = f.unifier();
  u.try_unify_type_id_type_id_bool_bool_literal_properties_entry(a, s, false, false, None);
  assert!(
    u.failure,
    "any <: string 应失败（normalize 关闭走 failure 分支）"
  );
}

#[test]
fn union_supertype_accepts_member() {
  // 收窄核心：number <: number|string 通过（super-union 命中 option）
  let mut f = UnifyFixture::new();
  let (n, s) = (f.number(), f.string());
  let sup = f.union_of(vec![n, s]);
  unifies(&mut f, n, sup);
}

#[test]
fn union_supertype_rejects_non_member() {
  // nil 不在 number|string 中 → 失败
  let mut f = UnifyFixture::new();
  let (n, s, nil) = (f.number(), f.string(), f.nil());
  let sup = f.union_of(vec![n, s]);
  mismatch(&mut f, nil, sup);
}

#[test]
fn union_subtype_requires_all_members() {
  // (number|string) <: string 失败：number 分支不兼容（Not all union options）
  let mut f = UnifyFixture::new();
  let (n, s, b) = (f.number(), f.string(), f.boolean());
  let sub = f.union_of(vec![n, s]);
  mismatch(&mut f, sub, b);
}

#[test]
fn union_subtype_of_matching_union() {
  // (number|nil) <: (number|string|nil)：每个成员都命中超类型 option → 通过
  //（union 至少两成员，单元素 union 在 add_type 被 LUAU_ASSERT 拒绝）
  let mut f = UnifyFixture::new();
  let (n, s, nil) = (f.number(), f.string(), f.nil());
  let sub = f.union_of(vec![n, nil]);
  let sup = f.union_of(vec![n, s, nil]);
  unifies(&mut f, sub, sup);
}

#[test]
fn boolean_singleton_is_subtype_of_boolean_primitive() {
  // 协变下 singleton true <: boolean 通过（tryUnifySingletons 协变分支）
  let mut f = UnifyFixture::new();
  let t = f.bool_singleton_true();
  let b = f.boolean();
  unifies(&mut f, t, b);
}

#[test]
fn primitive_wanted_given_are_reported() {
  // 验证 TypeMismatch 的 wanted/given 指回入参（输出契约的字段绑定）
  let mut f = UnifyFixture::new();
  let (n, s) = (f.number(), f.string());
  let errors = f.unifier().can_unify_type_id_type_id(n, s);
  let TypeErrorData::TypeMismatch(tm) = &errors[0].data else {
    panic!("期望 TypeMismatch");
  };
  assert_eq!(follow_type_id(tm.wanted_type), s);
  assert_eq!(follow_type_id(tm.given_type), n);
}
