use alloc::{collections::BTreeMap, vec::Vec};
use core::ptr::from_ref;

use ulua_ast::records::location::Location;
use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::table_state::TableState,
  functions::{
    contains_subscripted_definition::contains_subscripted_definition, follow_type, get_type,
  },
  records::{
    arena_handle::{alias_opt, alias_ref},
    conjunction_refinement::Conjunction,
    constraint_generator::ConstraintGenerator,
    disjunction_refinement::Disjunction,
    equivalence::Equivalence,
    negation_refinement::Negation,
    negation_type::NegationType,
    property_type::Property,
    proposition_refinement::Proposition,
    refinement_partition::RefinementPartition,
    table_type::TableType,
    variadic::Variadic,
  },
  type_aliases::{
    constraint_v::ConstraintV, name_type::Name, refinement_context::RefinementContext,
    refinement_id_refinement::RefinementId, refinement_refinement::RefinementMember,
    scope_ptr_type::ScopePtr,
  },
};
impl ConstraintGenerator {
  // ConstraintGenerator::computeRefinement(const ScopePtr&, Location, RefinementId,
  //     RefinementContext*, bool sense, bool eq, std::vector<ConstraintV>*)
  // (ConstraintGenerator.cpp:565).
  /// 形参链已引用化（原 `# Safety` 契约由签名承担）：
  /// - `scope`：调用方持有的存活 `Arc<Scope>` 共享借用（cpp `const ScopePtr&`）；
  /// - `refinement`：可空 `RefinementId`（null 即立即返回）；非空时指向
  ///   `self.refinement_arena` 分配的驻留节点（TypedAllocator 分块地址稳定，
  ///   遍历期间该细化树不再被分配或销毁）；Proposition 节点的 `key` 及其非空
  ///   `parent` 链指向 module `RefinementKeyArena` 驻留节点；
  /// - `refis`：调用方独占存活的输出上下文；递归传入的 `&mut lhs_refis`/
  ///   `&mut rhs_refis` 局部目标同样在递归返回前不被其他借用触碰；
  /// - `constraints`：调用方存活的输出队列，本函数仅透传；
  /// - `self`：`arena`/`builtin_types` 为构造期注入的存活句柄。
  pub fn compute_refinement(
    &mut self,
    scope: &ScopePtr,
    location: Location,
    refinement: RefinementId,
    refis: &mut RefinementContext,
    sense: bool,
    eq: bool,
    constraints: &mut Vec<ConstraintV>,
  ) {
    let Some(refinement_ref) = alias_opt(refinement) else {
      return;
    };

    if let Some(variadic) = <Variadic as RefinementMember>::get_if(refinement_ref) {
      for refi in variadic.refinements.clone() {
        // 子节点 `refi` 为 refinement arena 内驻留地址（可能为空，由本函数
        // 入口判空短路），契约同本帧。
        self.compute_refinement(scope, location, refi, refis, sense, eq, constraints);
      }
    } else if let Some(negation) = <Negation as RefinementMember>::get_if(refinement_ref) {
      // 仅 `sense` 取反、子指针换为同 arena 的 `negation.refinement`
      // （arena 的 negation 工厂保证子非空）。
      self.compute_refinement(
        scope,
        location,
        negation.refinement,
        refis,
        !sense,
        eq,
        constraints,
      );
    } else if let Some(conjunction) = <Conjunction as RefinementMember>::get_if(refinement_ref) {
      let (lhs, rhs) = (conjunction.lhs, conjunction.rhs);
      let mut lhs_refis = RefinementContext::default();
      let mut rhs_refis = RefinementContext::default();

      // 写入目标：`sense` 时直写调用方 `refis`，否则借用本帧局部（两次
      // 递归顺序进行，借用窗口互不重叠）。
      let lhs_target: &mut RefinementContext = if sense { refis } else { &mut lhs_refis };
      self.compute_refinement(scope, location, lhs, lhs_target, sense, eq, constraints);
      let rhs_target: &mut RefinementContext = if sense { refis } else { &mut rhs_refis };
      self.compute_refinement(scope, location, rhs, rhs_target, sense, eq, constraints);

      if !sense {
        // `!sense` 时两路递归均写本帧局部 `lhs_refis`/`rhs_refis`，
        // dest=`refis` 未被递归触碰。
        self.union_refinements(scope, location, &lhs_refis, &rhs_refis, refis, constraints);
      }
    } else if let Some(disjunction) = <Disjunction as RefinementMember>::get_if(refinement_ref) {
      let (lhs, rhs) = (disjunction.lhs, disjunction.rhs);
      let mut lhs_refis = RefinementContext::default();
      let mut rhs_refis = RefinementContext::default();

      // 与 conjunction 臂同构：`sense` 时写本帧局部、否则直写 `refis`。
      let lhs_target: &mut RefinementContext = if sense { &mut lhs_refis } else { refis };
      self.compute_refinement(scope, location, lhs, lhs_target, sense, eq, constraints);
      let rhs_target: &mut RefinementContext = if sense { &mut rhs_refis } else { refis };
      self.compute_refinement(scope, location, rhs, rhs_target, sense, eq, constraints);

      if sense {
        // 此处 `sense` 为真，两路递归写的是局部 `lhs_refis`/`rhs_refis`，
        // dest=`refis` 保持未触碰、与两个只读借用不重叠。
        self.union_refinements(scope, location, &lhs_refis, &rhs_refis, refis, constraints);
      }
    } else if let Some(equivalence) = <Equivalence as RefinementMember>::get_if(refinement_ref) {
      let (lhs, rhs) = (equivalence.lhs, equivalence.rhs);
      // 两路递归均直写 `refis`（顺序递归，独占借用不重叠），`eq` 置真。
      self.compute_refinement(scope, location, lhs, refis, sense, true, constraints);
      self.compute_refinement(scope, location, rhs, refis, sense, true, constraints);
    } else if let Some(proposition) = <Proposition as RefinementMember>::get_if(refinement_ref) {
      let mut discriminant_ty = proposition.discriminant_ty;
      let prop_key = proposition.key;
      let implicit_from_call = proposition.implicit_from_call;

      // if we have a negative sense, then we need to negate the discriminant
      // 对照 C++：`if (auto nt = get<NegationType>(follow(discriminantTy))) discriminantTy = nt->ty;`
      if !sense {
        if let Some(nt) = get_type::get::<NegationType>(follow_type::follow(discriminant_ty)) {
          discriminant_ty = nt.ty;
        } else {
          // arena 为构造期注入的存活句柄；`add_type` 在稳定新地址上追加节点，
          // 不移动既有类型，也不与 refinement arena 重叠。
          discriminant_ty = {
            self.arena.get_mut().add_type(NegationType {
              ty: discriminant_ty,
            })
          };
        }
      }

      if eq {
        // builtin_types 单例存活至生成器之后；`singleton_func` 借用仅供
        // create_type_function_instance 在本次调用内读。
        let singleton_func = { &self.builtin_types.get().type_functions.singleton_func };
        discriminant_ty = self.create_type_function_instance(
          singleton_func,
          alloc::vec![discriminant_ty],
          alloc::vec![],
          scope,
          location,
        );
      }

      let mut key = prop_key;
      while let Some(k) = alias_opt(key) {
        let key_def = k.def;

        // insert 保证 `key_def` 已在，随后 get_mut().expect() 必取得 Some。
        refis.insert(key_def, RefinementPartition::default());
        refis
          .get_mut(&key_def)
          .expect("上一行刚 insert(key_def)，get_mut 必命中")
          .discriminant_types
          .push(discriminant_ty);

        // Reached leaf node
        let prop_name = k.prop_name.clone();
        let prop_name = match prop_name {
          Some(n) => n,
          None => break,
        };

        let mut props: BTreeMap<Name, Property> = BTreeMap::new();
        props.insert(prop_name, Property::readonly(discriminant_ty));

        // `scope.level` 为 Copy 只读；TableType 记录的 scope 字段保持裸指针
        // 布局（记录布局不改），仅在此处一次性还原地址（cpp
        // `table->scope = scope.get()`）；arena 句柄追加稳定地址节点。
        let tt = TableType::table_type_props_optional_table_indexer_type_level_scope_table_state(
          &props,
          None,
          scope.level,
          from_ref(&**scope).cast_mut(),
          TableState::Sealed,
        );
        let next_discriminant_ty = self.arena.get_mut().add_type(tt);

        discriminant_ty = next_discriminant_ty;

        // `k.parent` 是同一稳定 RefinementKey 节点的字段读，或为链上下一
        // 驻留节点、或为 null 由循环条件收敛。
        key = k.parent;
      }

      // When the top-level expression is `t[x]`, we want to refine it into `nil`, not `never`.
      // 进入 Proposition 分支即 refinement 非空，而 arena 的 proposition 工厂
      // 拒绝空 key，故 `prop_key` 恒非空且指向稳定 RefinementKeyArena 节点；
      // refis 上方循环已插入 `prop_def`，get_mut().expect() 必命中。
      let prop_def = alias_ref(prop_key).def;
      LUAU_ASSERT!(refis.get(&prop_def).is_some());
      refis
        .get_mut(&prop_def)
        .expect("上方 LUAU_ASSERT 已确认 prop_def 键在")
        .should_append_nil_type =
        (sense || !eq) && contains_subscripted_definition(prop_def) && !implicit_from_call;
    }
  }
}
