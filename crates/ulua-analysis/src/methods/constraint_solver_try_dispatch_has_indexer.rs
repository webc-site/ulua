use alloc::vec::Vec;
use core::ptr::null;

use ulua_common::{fflag, fint, records::dense_hash_set::DenseHashSet};

use crate::{
  enums::{polarity::Polarity, table_state::TableState},
  functions::{
    as_mutable_type::as_mutable_type_id,
    begin_type::{begin_intersection_type, begin_union_type},
    follow_type,
    fresh_type::fresh_type,
    get_mutable_type, get_type,
    track_interior_free_type::track_interior_free_type,
  },
  records::{
    any_type::AnyType, blocked_type::BlockedType, constraint::Constraint,
    constraint_solver::ConstraintSolver, extern_type::ExternType, free_type::FreeType,
    intersection_builder::IntersectionBuilder, intersection_type::IntersectionType,
    metatable_type::MetatableType, never_type::NeverType, recursion_limiter::RecursionLimiter,
    set::Set, table_indexer::TableIndexer, table_type::TableType, type_level::TypeLevel,
    union_builder::UnionBuilder, union_type::UnionType,
  },
  type_aliases::{error_type::ErrorType, type_id::TypeId, type_variant::TypeVariant},
};
impl ConstraintSolver {
  /// 对应 cpp `tryDispatchHasIndexer(int&, NotNull<const Constraint>, TypeId,
  /// TypeId, TypeId, Set<TypeId>&)`（`Analysis/src/ConstraintSolver.cpp:2069`，
  /// 入口断言 2087-2088）。各参数的运行期前置为求解器内部不变量，由下游
  /// `bind`/`unify` 的 `LUAU_ASSERT` 在派发链中校验（`TypeId` 在本 crate 一律
  /// 视作 arena 驻留句柄，读/写解引用集中在带 `# Safety` 注释的 arena 封装内）：
  /// - `recursion_depth`：cpp `int& recursionDepth`（`RecursionLimiter` 深度
  ///   槽：cpp:2078 构造即 ++、超 `FInt::LuauSolverRecursionLimit` 抛
  ///   `RecursionLimitException`、Drop 回退），须为整条递归链内存活的可变借用。
  /// - `constraint`：正在派发的 HasIndexer 约束（C++ `NotNull`），整调用内
  ///   有效且从不被本函数写；其地址被用作 bind/unify 与 BlockedType
  ///   `set_owner` 的身份键。
  /// - `subject_type` / `index_type`：arena 驻留 TypeId，入口各自 follow 后
  ///   按变体匹配/参与 unify，无所有权要求。
  /// - `result_type`：当前应是 arena 驻留 BlockedType 节点且其 owner 恰等于
  ///   `constraint`（cpp `LUAU_ASSERT(get<BlockedType>(resultType))` 与
  ///   `LUAU_ASSERT(canMutate(resultType, constraint))`）。本函数会经 `bind`
  ///   覆写该节点的 `.ty`，或在 intersection/union 多结果分支就地改写为复合
  ///   变体。
  /// - `_seen`：subject 环保护集，沿同一递归链共享（本函数会插入），否则
  ///   自引用 subject 会无限递归。
  pub fn constraint_solver_try_dispatch_has_indexer(
    &mut self,
    recursion_depth: &mut i32,
    constraint: &Constraint,
    subject_type: TypeId,
    index_type: TypeId,
    mut result_type: TypeId,
    _seen: &mut DenseHashSet<TypeId>,
  ) -> bool {
    // cpp:2078 `RecursionLimiter _rl{"ConstraintSolver::tryDispatchHasIndexer",
    // &recursionDepth, FInt::LuauSolverRecursionLimit}`——metatable 链自引用时
    // 由深度槽截断；槽由调用方持有（cpp:2375 同位局部），limiter 存裸指针，
    // 递归传递处再经 &mut 重借用。
    let _rl = RecursionLimiter::new(
      "ConstraintSolver::tryDispatchHasIndexer",
      recursion_depth,
      fint::LuauSolverRecursionLimit.get(),
    );

    // bind/unify/set_owner 以 `&Constraint` 的地址为身份键：安全封装在参数处
    // 隐式指针化，业务调用点无需再持裸指针局部量。
    let subject_type = follow_type::follow(subject_type);
    let index_type = follow_type::follow(index_type);

    if _seen.contains(&subject_type) {
      return false;
    }
    _seen.insert(subject_type);

    if get_type::get::<AnyType>(subject_type).is_some() {
      // cpp:2092——把本约束独占的 Blocked 结果节点绑到内建 any。
      self.bind(constraint, result_type, self.builtin_types_ref().any_type);
      return true;
    }

    if let Some(free_type) = get_mutable_type::get_mutable::<FreeType>(subject_type) {
      let upper_bound = follow_type::follow(free_type.upper_bound);

      if let Some(table) = get_type::get::<TableType>(upper_bound) {
        if let Some(indexer) = &table.indexer {
          self.constraint_solver_unify(constraint, index_type, indexer.index_type);
          // cpp:2101——`result_type` 尚未写回、Blocked/owner 前置仍成立；
          // `indexer.index_result_type` 是上方共享读出的 arena 句柄。
          self.bind(constraint, result_type, indexer.index_result_type);
          return true;
        }
      } else if let Some(metatable) = get_type::get::<MetatableType>(upper_bound) {
        // 对应 cpp:2105 尾递归改指元表 table 面：`metatable.table()` 复制自
        // 共享读的 arena 节点、`result_type` 与约束对象原样续传
        // （Blocked/owner 前置未动），`_seen` 已含本层 subject 继续环保护。
        return self.constraint_solver_try_dispatch_has_indexer(
          recursion_depth,
          constraint,
          metatable.table(),
          index_type,
          result_type,
          _seen,
        );
      }

      let scope = free_type.scope;
      let free_result = fresh_type(
        // Safety: 为 FreeType 的 `ft.scope` 追加新 FreeType 节点（cpp:2109）——
        // arena 是构造期 NotNull 布线，恒非空且随 solver 存活，独占借用止于
        // 本次调用；`free_type`/`table` 等 arena 句柄指向固定分配节点，
        // 追加不移动它们。访问器对无法同调用混用，故此处拆裸指针。
        { self.arena.get_mut() },
        // Safety: builtin_types 只读取 never/unknown 种子；与 arena 分属两块
        // 不相交分配，并存借用无别名冲突。
        { self.builtin_types.get() },
        scope,
        Polarity::Mixed,
      );
      track_interior_free_type(scope, free_result);
      // cpp:2111——首次写回正主 `result_type`：入口不变量保证它是本约束独占的
      // Blocked 节点，绑定目标为刚 fresh 的 FreeType；随后本地把 `result_type`
      // 改指 free_result（cpp:2113「follow 即可」约定）。
      self.bind(constraint, result_type, free_result);
      result_type = free_result;

      let mut table = TableType::table_type_table_state_type_level_scope(
        TableState::Unsealed,
        TypeLevel::default(),
        scope,
      );
      table.indexer = Some(TableIndexer {
        index_type,
        index_result_type: free_result,
        is_read_only: false,
      });

      let upper_bound = self.arena_mut().add_type(table);
      let simplified = self.simplify_intersection_not_null_scope_location_type_id_type_id(
        constraint.scope,
        constraint.location,
        free_type.upper_bound,
        upper_bound,
      );

      if get_type::get::<NeverType>(simplified).is_some() {
        // `result_type` 已被上方重指向 free_result——一个刚 fresh 的 FreeType，
        // 满足 bind 入口断言；simplify 出 Never 即索引类型与上界冲突，按
        // cpp:2128 绑定 error（error 为只读内建句柄）。
        self.bind(constraint, result_type, self.builtin_types_ref().error_type);
      } else {
        free_type.upper_bound = simplified;
      }

      return true;
    }

    if let Some(table) = get_mutable_type::get_mutable::<TableType>(subject_type) {
      if let Some(indexer) = &table.indexer {
        self.constraint_solver_unify(constraint, index_type, indexer.index_type);
        // cpp:2139——subject 表自带索引器直绑：`result_type` 仍保持入口
        // Blocked/owner==约束 前置（本路径首次写回），绑定值是表索引器结果
        // 的 arena 句柄。
        self.bind(constraint, result_type, indexer.index_result_type);
        return true;
      }

      if table.state == TableState::Unsealed {
        let scope = table.scope;
        let free_result = fresh_type(
          // Safety: unsealed 表按 `tt->scope`（cpp:2149）grow 出索引结果类型；
          // 与 free 分支同理，arena NotNull 恒非空，本调用独占可变借用，
          // 持存的 `table` 可变句柄指向固定分配节点，追加不影响其地址。
          { self.arena.get_mut() },
          // Safety: 只读 never/unknown 种子，与 arena 互不相交的另一块分配。
          { self.builtin_types.get() },
          scope,
          Polarity::Mixed,
        );
        track_interior_free_type(scope, free_result);
        // cpp:2151——`result_type` 仍是入口不变量的本约束独占 Blocked 节点
        // （本路径首写），刚 fresh 的 free_result 为 arena 驻留 FreeType。
        self.bind(constraint, result_type, free_result);
        table.indexer = Some(TableIndexer {
          index_type,
          index_result_type: result_type,
          is_read_only: false,
        });
        return true;
      }
    }

    if let Some(metatable) = get_type::get::<MetatableType>(subject_type) {
      // 对齐 C++（ConstraintSolver.cpp:2194-2195）`else if (auto mt = ...)
      // return tryDispatchHasIndexer(...)`：递归结果必须直接返回。
      // 丢失 return 会让 blocked（false）路径跌落到函数尾部的
      // `bind(result_type, error)`，把已被递归绑定的 Bound result_type 再
      // bind，触发断言崩溃（metatable_table_assertion_crash）。
      // 对应 cpp:2165 的尾递归直传：`metatable.table()` 复制自共享读的 arena
      // 节点；约束、`result_type`（Blocked/owner 前置原样未动）与 `_seen` 均
      // 续传，写回只发生在递归内部完成。
      return self.constraint_solver_try_dispatch_has_indexer(
        recursion_depth,
        constraint,
        metatable.table(),
        index_type,
        result_type,
        _seen,
      );
    }

    let mut extern_type = get_type::get::<ExternType>(subject_type);
    while let Some(et) = extern_type {
      if let Some(indexer) = &et.indexer {
        self.constraint_solver_unify(constraint, index_type, indexer.index_type);
        // cpp:2171——沿 ExternType parent 链找到的索引器直绑：`result_type`
        // 至此仍未被写回，入口 Blocked/owner==约束 前置成立；`et.indexer` 为
        // 共享读出的 arena 句柄。
        self.bind(constraint, result_type, indexer.index_result_type);
        return true;
      }

      extern_type = et.parent.and_then(get_type::get::<ExternType>);
    }

    if let Some(it) = get_type::get::<IntersectionType>(subject_type) {
      // Indexing into an intersection of types is roughly akin to overload
      // selection: for every type in the intersection where it is well typed
      // to index into _that_ type, we construct an intersection of said result
      // types.
      if fflag::LuauRemoveConstraintSolverEmplace.get() {
        let mut ib = IntersectionBuilder::new(self.arena, self.builtin_types);
        let mut success = false;

        // C++ `for (TypeId part : it)`——IntersectionTypeIterator 展平
        // 嵌套 intersection 并 follow,裸遍历 parts 会漏掉嵌套成员。
        let parts: Vec<TypeId> = begin_intersection_type(it).collect();
        for part in parts {
          let r = self.arena_mut().add_type(BlockedType::default());
          // C++ `getMutable<BlockedType>(r)->setOwner(...)`：r 刚 emplace 为
          // BlockedType，下转必命中。
          get_mutable_type::get_mutable::<BlockedType>(r)
            .expect("fresh blocked type")
            .set_owner(constraint as *const Constraint);

          // cpp:2194-2197 的每部分临时结果槽：`r` 是刚 add_type 并以本约束
          // 为 owner 的 Blocked 节点，逐字满足被调函数对 `result_type` 的入口
          // 前置；`part` 来自展平迭代器（已 follow）。
          let ok = self.constraint_solver_try_dispatch_has_indexer(
            recursion_depth,
            constraint,
            part,
            index_type,
            r,
            _seen,
          );
          // If we've cut a recursive loop short, skip it.
          if !ok {
            continue;
          }

          let r = follow_type::follow(r);
          if get_type::get::<ErrorType>(r).is_none() {
            success = true;
            ib.add(r);
          }
        }

        // We need to distinguish between the empty case (there
        // were no valid indexable types) and the bottom type (one of the
        // indexable result types was never). UnionBuilder will opt to
        // only record that its seen a top type as an optimization. we
        // add a flag to distinguish these cases.
        if success {
          let built = ib.build();
          // cpp:2216——循环里只 bind 过各 `r` 槽，正主 `result_type` 依然满足
          // 入口 Blocked/owner 前置；`built` 为 IntersectionBuilder 现造的
          // arena 节点。
          self.bind(constraint, result_type, built);
        } else {
          // cpp:2218——无任何可索引部分：`result_type` 未被触碰，前置仍成立；
          // error 经 `builtin_types_ref()` 安全只读取得。
          self.bind(constraint, result_type, self.builtin_types_ref().error_type);
        }
      } else {
        let mut parts: Set<TypeId> = Set::new(null());
        // C++ `for (TypeId part : it)`——迭代器展平嵌套 intersection 并 follow。
        for part in begin_intersection_type(it) {
          parts.insert(&follow_type::follow(part));
        }

        let mut results: Set<TypeId> = Set::new(null());

        let parts_iter: Vec<TypeId> = parts.iter().copied().collect();
        for part in parts_iter {
          let r = self.arena_mut().add_type(BlockedType::default());
          // C++ `getMutable<BlockedType>(r)->setOwner(...)`：r 刚 emplace 为
          // BlockedType，下转必命中。
          get_mutable_type::get_mutable::<BlockedType>(r)
            .expect("fresh blocked type")
            .set_owner(constraint as *const Constraint);

          // cpp:2231-2234 legacy emplace 分支的同一手法：per-part 新 Blocked
          // 槽 `r` 的 owner 上一行刚置为本约束，被调契约成立；`part` 为已
          // follow 入 Set 的展平成员句柄。
          let ok = self.constraint_solver_try_dispatch_has_indexer(
            recursion_depth,
            constraint,
            part,
            index_type,
            r,
            _seen,
          );
          // If we've cut a recursive loop short, skip it.
          if !ok {
            continue;
          }

          let r = follow_type::follow(r);
          if get_type::get::<ErrorType>(r).is_none() {
            results.insert(&r);
          }
        }

        if results.size() == 0 {
          // cpp:2245 legacy 空结果：`result_type` 从未被本路径写回，入口
          // Blocked/owner 前置原样成立。
          self.bind(constraint, result_type, self.builtin_types_ref().error_type);
        } else if results.size() == 1 {
          // `results.size() == 1` 蕴含唯一元素。
          let first = *results
            .iter()
            .next()
            .expect("results.size()==1 判据蕴含首元素必在");
          // cpp:2247 单结果直绑：`first` 是 follow 后收入 Set 的 arena 句柄；
          // `result_type` 仍是本约束独占的 Blocked 节点。
          self.bind(constraint, result_type, first);
        } else {
          let parts_vec: Vec<TypeId> = results.iter().copied().collect();
          let mutable_ty = { as_mutable_type_id(result_type) };
          // Safety: 对应 cpp:2249 `DEPRECATED_emplace<IntersectionType>`——
          // `mutable_ty` 指向入口契约钉住的 Blocked 结果节点（owner==约束，
          // can_mutate 前置成立），把 `.ty` 就地从 Blocked 改写为刚收集的
          // Intersection；`as_mutable_type_id` 仅 reinterpret 地址（C++
          // const_cast 同义），parts_vec 已收集完毕故迭代借用不再活跃；
          // 紧随的 unblock 把等待该节点的约束链接到覆写值，写点无并存可变借用。
          unsafe {
            (*mutable_ty).ty = TypeVariant::Intersection(IntersectionType { parts: parts_vec });
          }
          self.unblock_type_id_location(result_type, constraint.location);
        }
      }

      return true;
    }

    if let Some(ut) = get_type::get::<UnionType>(subject_type) {
      // Indexing into a union of types means constructing a union of
      // results: we don't know _which_ type it could be.
      if fflag::LuauRemoveConstraintSolverEmplace.get() {
        let mut ub = UnionBuilder::new(self.arena, self.builtin_types);
        let mut success = false;

        // C++ `for (TypeId option : ut)`——UnionTypeIterator 展平嵌套
        // union 并 follow,裸遍历 options 会漏掉嵌套成员。
        let options: Vec<TypeId> = begin_union_type(ut).collect();
        for option in options {
          let r = self.arena_mut().add_type(BlockedType::default());
          // C++ `getMutable<BlockedType>(r)->setOwner(...)`：r 刚 emplace 为
          // BlockedType，下转必命中。
          get_mutable_type::get_mutable::<BlockedType>(r)
            .expect("fresh blocked type")
            .set_owner(constraint as *const Constraint);

          // cpp:2266-2269 union 逐选项版：per-option Blocked 槽 `r` 刚以本约束
          // 为 owner 建立，满足递归 `result_type` 前置；`option` 为展平后已
          // follow 的 arena 成员句柄。
          let ok = self.constraint_solver_try_dispatch_has_indexer(
            recursion_depth,
            constraint,
            option,
            index_type,
            r,
            _seen,
          );
          // If we've cut a recursive loop short, skip it.
          if !ok {
            continue;
          }

          let r = follow_type::follow(r);
          success = true;
          ub.add(r);
        }

        // We need to distinguish between the empty case (there
        // were no valid indexable types) and the top type (one of the
        // indexable result types was unknown). UnionBuilder will opt to
        // only record that its seen a top type as an optimization. we
        // add a flag to distinguish these cases.
        if success {
          let built = ub.build();
          // cpp:2285——与 intersection FFlag 分支同式但收集的是 union 结果：
          // `result_type` 尚未写回（循环只 bind `r`），`built` 为 UnionBuilder
          // 现造 arena 节点。
          self.bind(constraint, result_type, built);
        } else {
          // cpp:2287——无成功选项时绑 error：目标 `result_type` 保持入口
          // Blocked/owner 前置。
          self.bind(constraint, result_type, self.builtin_types_ref().error_type);
        }
      } else {
        let mut parts: Set<TypeId> = Set::new(null());
        // C++ `for (TypeId part : ut)`——迭代器展平嵌套 union 并 follow。
        for part in begin_union_type(ut) {
          parts.insert(&follow_type::follow(part));
        }

        let mut results: Set<TypeId> = Set::new(null());

        let parts_iter: Vec<TypeId> = parts.iter().copied().collect();
        for part in parts_iter {
          let r = self.arena_mut().add_type(BlockedType::default());
          // C++ `getMutable<BlockedType>(r)->setOwner(...)`：r 刚 emplace 为
          // BlockedType，下转必命中。
          get_mutable_type::get_mutable::<BlockedType>(r)
            .expect("fresh blocked type")
            .set_owner(constraint as *const Constraint);

          // cpp:2300-2303 union legacy 版：per-part Blocked 槽 `r` 的 owner
          // 上一行置为本约束，被调 `result_type` 前置逐字满足；`part` 为 Set
          // 中 follow 后的成员句柄。
          let ok = self.constraint_solver_try_dispatch_has_indexer(
            recursion_depth,
            constraint,
            part,
            index_type,
            r,
            _seen,
          );
          // If we've cut a recursive loop short, skip it.
          if !ok {
            continue;
          }

          let r = follow_type::follow(r);
          results.insert(&r);
        }

        if results.size() == 0 {
          // cpp:2313 union legacy 空结果：`result_type` 未被本路径写过，
          // Blocked/owner==约束 前置原样成立；error 走安全只读访问器。
          self.bind(constraint, result_type, self.builtin_types_ref().error_type);
        } else if results.size() == 1 {
          // `results.size() == 1` 同判据蕴含唯一元素。
          let first_result = *results
            .iter()
            .next()
            .expect("results.size()==1 判据蕴含首元素必在");
          if !fflag::LuauConstraintGraph.get() {
            // bind will already shift references.
            self.deprecate_d_shift_references(result_type, first_result);
          }
          // cpp:2317 单结果直绑（shift-references 不改 result_type 的 owner
          // 身份）：`first_result` 为 Set 中 follow 后的 arena 句柄。
          self.bind(constraint, result_type, first_result);
        } else {
          let options_vec: Vec<TypeId> = results.iter().copied().collect();
          let mutable_ty = { as_mutable_type_id(result_type) };
          // Safety: cpp:2320 `DEPRECATED_emplace<UnionType>` 的直译：目标
          // `mutable_ty` 是入口契约保证「Blocked 且 owner==本约束」的结果
          // 节点，故有权把 `.ty` 由 Blocked 覆为刚收集的 Union；results 已
          // collect 进新 Vec，Set 借用不再活跃；随后 unblock 接续 C++
          // emplace 后的解阻语义，单线程下写点无并存可变借用。
          unsafe {
            (*mutable_ty).ty = TypeVariant::Union(UnionType {
              options: options_vec,
            });
          }
          self.unblock_type_id_location(result_type, constraint.location);
        }
      }

      return true;
    }

    // 函数尾部兜底（cpp:2326）：subject 无任何可索引形态时把仍满足入口
    // Blocked/owner 前置的 `result_type` 绑到 error。
    self.bind(constraint, result_type, self.builtin_types_ref().error_type);
    true
  }
}
