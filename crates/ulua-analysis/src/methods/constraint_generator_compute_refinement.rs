use alloc::{collections::BTreeMap, sync::Arc, vec::Vec};
use core::mem::ManuallyDrop;

use ulua_ast::records::location::Location;
use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::table_state::TableState,
  functions::{
    contains_subscripted_definition::contains_subscripted_definition, follow_type, get_type,
  },
  records::{
    conjunction_refinement::Conjunction, constraint_generator::ConstraintGenerator,
    disjunction_refinement::Disjunction, equivalence::Equivalence, negation_refinement::Negation,
    negation_type::NegationType, property_type::Property, proposition_refinement::Proposition,
    refinement_partition::RefinementPartition, scope::Scope, table_type::TableType,
    variadic::Variadic,
  },
  type_aliases::{
    constraint_v::ConstraintV,
    name_type::Name,
    refinement_context::RefinementContext,
    refinement_id_refinement::RefinementId,
    refinement_refinement::{Refinement, RefinementMember},
  },
};
impl ConstraintGenerator {
  // ConstraintGenerator::computeRefinement(const ScopePtr&, Location, RefinementId,
  //     RefinementContext*, bool sense, bool eq, std::vector<ConstraintV>*)
  // (ConstraintGenerator.cpp:565).
  /// # Safety
  /// 各裸指针参数须来自入口 `apply_refinements` 的活借用（对照 cpp
  /// `computeRefinement`，ConstraintGenerator.cpp:663），且在本次调用及其
  /// 整个递归期间保持存活：
  /// - `scope`：`arc_as_mut(&ScopePtr)` 的结果，即某个存活 `Arc<Scope>` 的
  ///   `Arc::as_ptr` 同地址指针，使本函数可用 `Arc::from_raw` + `ManuallyDrop`
  ///   临时重建不减计数的句柄，并允许 `(*scope).level` 读取；
  /// - `refinement`：为空（立即返回）或 `self.refinement_arena` 分配的节点
  ///   地址；`TypedAllocator` 分块地址稳定，遍历期间该细化树不再被分配或
  ///   销毁；Proposition 节点的 `key` 及其非空 `parent` 链指向 module
  ///   `RefinementKeyArena` 驻留节点（`module` Arc 由生成器持有），嵌套子
  ///   `RefinementId` 允许为空；
  /// - `refis`：非空，指向调用方独占存活的 `RefinementContext`；递归传入的
  ///   `&mut lhs_refis`/`&mut rhs_refis` 局部目标须同样在递归返回前不被其他
  ///   借用触碰；
  /// - `constraints`：指向调用方存活的 `Vec<ConstraintV>`，本函数仅透传、
  ///   不解引用；
  /// - `self`：`arena`/`builtin_types` 字段为构造期由 `NonNull` 注入的裸
  ///   指针，所指 Normalizer 的 TypeArena 与 BuiltinTypes 生命周期覆盖生成
  ///   器本身。
  pub unsafe fn compute_refinement(
    &mut self,
    scope: *mut Scope,
    location: Location,
    refinement: RefinementId,
    refis: *mut RefinementContext,
    sense: bool,
    eq: bool,
    constraints: *mut Vec<ConstraintV>,
  ) {
    if refinement.is_null() {
      return;
    }

    // Safety: 入口 `refinement.is_null()` 已排除空指针；`refinement` 指向
    // `self.refinement_arena` 的 `TypedAllocator` 驻留节点，分块地址稳定。
    // 本函数（含递归）只向 `self.arena.as_ptr()`（TypeArena）追加类型，从不触碰
    // refinement arena，故该只读借用与后续 `&mut self` 递归不产生重叠写。
    let refinement_ref: &Refinement = unsafe { &*refinement };

    if let Some(variadic) = <Variadic as RefinementMember>::get_if(refinement_ref) {
      for refi in variadic.refinements.clone() {
        // Safety: `scope`/`refis`/`constraints` 原样透传，有效性不变；子节点
        // `refi` 为 refinement arena 内驻留地址（可能为空，由被调函数入口
        // 判空短路），满足 `compute_refinement` 的逐参数契约。
        unsafe { self.compute_refinement(scope, location, refi, refis, sense, eq, constraints) };
      }
    } else if let Some(negation) = <Negation as RefinementMember>::get_if(refinement_ref) {
      // Safety: 仅 `sense` 取反、子指针换为同 arena 的 `negation.refinement`
      // （arena 的 negation 工厂保证子非空），其余指针与本帧相同仍存活。
      unsafe {
        self.compute_refinement(
          scope,
          location,
          negation.refinement,
          refis,
          !sense,
          eq,
          constraints,
        )
      };
    } else if let Some(conjunction) = <Conjunction as RefinementMember>::get_if(refinement_ref) {
      let (lhs, rhs) = (conjunction.lhs, conjunction.rhs);
      let mut lhs_refis = RefinementContext::default();
      let mut rhs_refis = RefinementContext::default();

      let lhs_target: *mut RefinementContext = if sense { refis } else { &mut lhs_refis };
      // Safety: `lhs` 为 arena 子指针（可空，被调入口判空）；写入目标
      // `lhs_target` 要么是本契约下非空的 `refis`，要么借用本帧局部
      // `lhs_refis`（存活至递归返回，且此刻无人借用该局部）。
      unsafe { self.compute_refinement(scope, location, lhs, lhs_target, sense, eq, constraints) };
      let rhs_target: *mut RefinementContext = if sense { refis } else { &mut rhs_refis };
      // Safety: 与 lhs 递归同理——`rhs_refis` 是独立于 `lhs_refis` 的本帧
      // 局部，两次递归顺序结束，裸指针使用期内无其他活借用。
      unsafe { self.compute_refinement(scope, location, rhs, rhs_target, sense, eq, constraints) };

      if !sense {
        // Safety: `scope` 由调用方 `arc_as_mut(&ScopePtr)` 得来，即活
        // `Arc<Scope>` 的 `Arc::as_ptr` 同地址；`Arc::from_raw` 重建句柄并以
        // `ManuallyDrop` 抑制 Drop，强引用计数不增不减，调用方 Arc 全程存活。
        let sp = ManuallyDrop::new(unsafe { Arc::from_raw(scope as *const Scope) });
        // Safety: `union_refinements` 契约要求 dest 非空且不与 lhs/rhs 借用
        // 重叠——`!sense` 时两路递归均写本帧局部 `lhs_refis`/`rhs_refis`，
        // dest=`refis` 未被递归触碰；`&sp` 指向上行重建的活 Arc 句柄。
        unsafe {
          self.union_refinements(&sp, location, &lhs_refis, &rhs_refis, refis, constraints)
        };
      }
    } else if let Some(disjunction) = <Disjunction as RefinementMember>::get_if(refinement_ref) {
      let (lhs, rhs) = (disjunction.lhs, disjunction.rhs);
      let mut lhs_refis = RefinementContext::default();
      let mut rhs_refis = RefinementContext::default();

      let lhs_target: *mut RefinementContext = if sense { &mut lhs_refis } else { refis };
      // Safety: 与 conjunction 臂同构——`lhs_target` 为活局部 `lhs_refis`
      // 的 `&mut` 裸化或契约非空入参 `refis`，递归仅在调用期间使用该指针。
      unsafe { self.compute_refinement(scope, location, lhs, lhs_target, sense, eq, constraints) };
      let rhs_target: *mut RefinementContext = if sense { &mut rhs_refis } else { refis };
      // Safety: `rhs_refis` 与 `lhs_refis` 互不重叠的两个本帧局部，前一
      // 递归已返回，其借用已结束，本递归独占写入。
      unsafe { self.compute_refinement(scope, location, rhs, rhs_target, sense, eq, constraints) };

      if sense {
        // Safety: 外层 `apply_refinements` 以 `&ScopePtr` 持有本 Scope 的
        // Arc 并存活于整个调用；该指针与 `arc_as_mut` 的 `Arc::as_ptr` 地址
        // 一致，`ManuallyDrop` 包裹使重建句柄不影响引用计数。
        let sp = ManuallyDrop::new(unsafe { Arc::from_raw(scope as *const Scope) });
        // Safety: 此处 `sense` 为真，两路递归写的是局部 `lhs_refis`/
        // `rhs_refis`，dest=`refis` 保持未触碰、与两个只读借用不重叠，
        // 满足 `union_refinements` 的 dest 独占契约。
        unsafe {
          self.union_refinements(&sp, location, &lhs_refis, &rhs_refis, refis, constraints)
        };
      }
    } else if let Some(equivalence) = <Equivalence as RefinementMember>::get_if(refinement_ref) {
      let (lhs, rhs) = (equivalence.lhs, equivalence.rhs);
      // Safety: 两路递归均直写契约非空的 `refis`（顺序递归，独占借用不
      // 重叠），`lhs`/`rhs` 为 refinement arena 子指针或空（入口判空兜底），
      // `eq` 置真不改变任何指针有效性。
      unsafe { self.compute_refinement(scope, location, lhs, refis, sense, true, constraints) };
      // Safety: 与上一行同契约——`rhs` 子指针或空或 arena 驻留，`refis`
      // 仍为本帧独占输出目标，前次递归已返回、借用结束。
      unsafe { self.compute_refinement(scope, location, rhs, refis, sense, true, constraints) };
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
          // Safety: `self.arena.as_ptr()` 为构造期从 Normalizer 注入的非空
          // `*mut TypeArena`，生命周期覆盖生成器；`add_type` 在稳定新地址
          // 上追加节点，不移动既有类型，也不与 `refinement_ref`（指向
          // refinement arena）或 `nt`（else 分支中为 None）重叠。
          discriminant_ty = {
            self.arena.get_mut().add_type(NegationType {
              ty: discriminant_ty,
            })
          };
        }
      }

      if eq {
        // Safety: `scope` 与活 `Arc<Scope>` 的 `Arc::as_ptr` 同址（契约），
        // `ManuallyDrop` 重建只作临时句柄、计数守恒。
        let sp = ManuallyDrop::new(unsafe { Arc::from_raw(scope as *const Scope) });
        // Safety: `self.builtin_types.as_ptr()` 由构造期 `NonNull<BuiltinTypes>` 裸化
        // 注入，所指全局 BuiltinTypes 存活至生成器之后；`singleton_func` 借
        // 用仅供下方安全函数 `create_type_function_instance` 在本次调用内读。
        let singleton_func = { &self.builtin_types.get().type_functions.singleton_func };
        discriminant_ty = self.create_type_function_instance(
          singleton_func,
          alloc::vec![discriminant_ty],
          alloc::vec![],
          &sp,
          location,
        );
      }

      let mut key = prop_key;
      while !key.is_null() {
        // Safety: 循环条件已判非空；`key` 沿 `Proposition.key` 的 `parent`
        // 链，指向 module `RefinementKeyArena` 的 `TypedAllocator` 驻留节点
        // （生成器持有 `module` Arc，链上地址全程稳定），此处只读 `def`。
        let key_def = unsafe { (*key).def };

        // Safety: `refis` 按契约为非空输出上下文；`insert` 保证 `key_def`
        // 已在，随后 `get_mut().expect()` 必取得 Some，两次裸指针借用顺序
        // 发生、不同时存活。
        unsafe {
          (*refis).insert(key_def, RefinementPartition::default());
          (*refis)
            .get_mut(&key_def)
            .expect("上一行刚 insert(key_def)，get_mut 必命中")
            .discriminant_types
            .push(discriminant_ty);
        }

        // Reached leaf node
        // Safety: 同一活 `key` 节点读取 `prop_name` 并 clone 为拥有值，借用
        // 随语句结束。
        let prop_name = unsafe { (*key).prop_name.clone() };
        let prop_name = match prop_name {
          Some(n) => n,
          None => break,
        };

        let mut props: BTreeMap<Name, Property> = BTreeMap::new();
        props.insert(prop_name, Property::readonly(discriminant_ty));

        // Safety: `&props` 是本帧局部（构造器内 `props.clone()`，仅借用后
        // 立即失效）；`(*scope).level` 从活 `Arc<Scope>` 内层读取字段；传入
        // 的 `scope` 裸指针存入 `TableType.scope`，其有效性由调用方持有的
        // Arc 覆盖（对应 cpp `table->scope = scope.get()`）；`self.arena.as_ptr()` 为
        // 构造期注入的非空 TypeArena，`add_type` 追加稳定地址节点。
        let next_discriminant_ty = unsafe {
          let tt = TableType::table_type_props_optional_table_indexer_type_level_scope_table_state(
            &props,
            None,
            (*scope).level,
            scope,
            TableState::Sealed,
          );
          self.arena.get_mut().add_type(tt)
        };

        discriminant_ty = next_discriminant_ty;

        // Safety: `(*key).parent` 是同一稳定 `RefinementKey` 节点的字段读，
        // 结果或为链上下一驻留节点、或为 null 由 `while` 条件收敛。
        key = unsafe { (*key).parent };
      }

      // When the top-level expression is `t[x]`, we want to refine it into `nil`, not `never`.
      // Safety: 进入 Proposition 分支即 `refinement` 非空，而 arena 的
      // `proposition` 工厂拒绝空 key，故 `prop_key` 恒非空且指向稳定
      // `RefinementKeyArena` 节点；`refis` 非空且上方循环已插入 `prop_def`，
      // 故 `get`/`get_mut().expect()` 必命中，写标志位为该上下文独占。
      let prop_def = unsafe { (*prop_key).def };
      // Safety: `refis` 非空契约成立、上方循环已插入 `prop_def`，`get` 为只读
      // 查询仅用于断言。
      LUAU_ASSERT!(unsafe { (*refis).get(&prop_def) }.is_some());
      // Safety: `refis` 非空契约成立，且 assert 已确认 `prop_def` 键在，
      // `get_mut().expect()` 不会失败；该裸指针可变借用随本语句结束。
      unsafe {
        (*refis)
          .get_mut(&prop_def)
          .expect("上方 LUAU_ASSERT 已确认 prop_def 键在")
          .should_append_nil_type =
          (sense || !eq) && contains_subscripted_definition(prop_def) && !implicit_from_call;
      }
    }
  }
}
