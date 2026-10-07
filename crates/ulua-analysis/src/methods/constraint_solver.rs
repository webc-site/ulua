//! `constraint_solver` 方法汇总：原先按 cpp 符号逐方法拆分的同前缀小文件合并至此，行为逐字保留。

use alloc::vec::Vec;
use core::ptr::NonNull;
use std::panic::panic_any;

use ulua_ast::records::location::Location;
use ulua_common::{fflag, macros::luau_assert::LUAU_ASSERT};

use crate::{
  enums::type_function_instance_state::TypeFunctionInstanceState,
  functions::{
    as_mutable_type::as_mutable_type_id,
    follow_type, follow_type_pack,
    generalize::generalize,
    get_mutable_type, get_type, get_type_pack,
    is_reference_counted_type::is_reference_counted_type,
    simplify_intersection_simplify::simplify_intersection,
    simplify_union::simplify_union,
    to_string_to_string::{to_string_type_id, to_string_type_id_to_string_options_mut},
  },
  records::{
    arena_handle::{alias, alias_opt_mut, alias_ref},
    blocked_constraint_registry::register_constraint,
    blocked_type::BlockedType,
    blocked_type_pack::BlockedTypePack,
    builtin_types::BuiltinTypes,
    constraint::Constraint,
    constraint_graph::ConstraintGraph,
    constraint_solver::ConstraintSolver,
    free_type::FreeType,
    pending_expansion_type::PendingExpansionType,
    scope::Scope,
    time_limit_error::TimeLimitError,
    type_arena::TypeArena,
    type_error::TypeError,
    type_function_instance_type::TypeFunctionInstanceType,
    type_function_instance_type_pack::TypeFunctionInstanceTypePack,
    type_ids::TypeIds,
    unpack_constraint::UnpackConstraint,
    user_cancel_error::UserCancelError,
  },
  type_aliases::{
    blocked_constraint_id::BlockedConstraintId, constraint_v::ConstraintV,
    type_error_data::TypeErrorData, type_id::TypeId, type_pack_id::TypePackId,
    type_variant::TypeVariant,
  },
};

// `ConstraintSolver` 上下文裸指针字段的访问器收口：`cgraph` / `builtin_types` 等
// 由构造方以 NonNull 语义建立、solver 存续期内恒有效（C++ `NotNull<T>` 同契约），
// 解引用统一收进这里，调用点不再出现 `unsafe { (*self.x) }` 样板。

impl ConstraintSolver {
  /// `cgraph` 由构造方以 NonNull 语义建立（见 `check_frontend` 的 ncgraph 存储），
  /// solver 存续期内恒有效且非空，等价 C++ `NotNull<ConstraintGraph>`；
  /// solver 独占 graph，无并发别名。
  pub fn cgraph_mut(&mut self) -> &mut ConstraintGraph {
    alias(self.cgraph)
  }

  /// C++ `NotNull<BuiltinTypes>`：内建类型表随前端上下文存活，solver 存续期内恒有效。
  pub fn builtin_types_ref(&self) -> &BuiltinTypes {
    self.builtin_types.get()
  }

  /// 与 `cgraph` 同契约的 arena 可变访问；与其它字段的
  /// 混合借用场景仍需经裸指针拆分，请优先在单一字段访问处使用本方法。
  pub fn arena_mut(&mut self) -> &mut TypeArena {
    self.arena.get_mut()
  }
}

impl ConstraintSolver {
  pub fn deprecate_d_block(
    &mut self,
    target: BlockedConstraintId,
    constraint: *const Constraint,
  ) -> bool {
    let block_vec = self.deprecated_blocked.entry(target).or_default();

    if block_vec.find(&constraint).is_some() {
      return false;
    }

    block_vec.insert(constraint);

    let count = self
      .deprecated_blocked_constraints
      .entry(constraint)
      .or_insert(0);
    *count += 1;

    true
  }
}

impl ConstraintSolver {
  pub fn deprecate_d_is_blocked(&self, constraint: *const Constraint) -> bool {
    let blocked_it = self.deprecated_blocked_constraints.get(&constraint);
    blocked_it.is_some_and(|blocked| *blocked > 0)
  }
}

impl ConstraintSolver {
  pub fn deprecate_d_shift_references(&mut self, source: TypeId, target: TypeId) {
    LUAU_ASSERT!(!fflag::LuauConstraintGraph.get());
    let target = follow_type::follow(target);

    // if the target isn't a reference counted type, there's nothing to do.
    // this stops us from keeping unnecessary counts for e.g. primitive types.
    if !is_reference_counted_type(target) {
      return;
    }

    if source == target {
      return;
    }

    if let Some(sourcerefs) = self.deprecated_type_to_constraint_set.get(&source).cloned() {
      for constraint in sourcerefs.iter() {
        // For every constraint that the source might be modified by,
        // add that constraint to the set of constraints the target
        // might be modified by.
        let targetrefs = self
          .deprecated_type_to_constraint_set
          .entry(target)
          .or_default();
        targetrefs.insert(*constraint);

        // Additionally, note that said constraint now may modify the target.
        let (it, _) = self
          .deprecated_constraint_to_mutated_types
          .try_insert(*constraint, TypeIds::new());
        it.insert_type_id(target);
      }
    }
  }
}

impl ConstraintSolver {
  pub fn deprecate_d_unblock_(&mut self, progressed: BlockedConstraintId) {
    LUAU_ASSERT!(!fflag::LuauConstraintGraph.get());
    if let Some(blocked_constraints) = self.deprecated_blocked.remove(&progressed) {
      for unblocked_constraint in blocked_constraints.iter() {
        let count = self
          .deprecated_blocked_constraints
          .get_mut(unblocked_constraint)
          // Safety: 与 deprecated_blocked 成对登记的键，constraints 表必命中。
          .expect("blocked_constraints 与 constraints 成对登记，键必命中");
        LUAU_ASSERT!(*count > 0);
        *count -= 1;
      }
    }
  }

  pub fn constraint_solver_deprecate_d_unblock(&mut self, progressed: *const Constraint) {
    if let Some(logger) = alias_opt_mut(self.logger) {
      logger.pop_block_not_null_constraint(progressed);
    }
    self.deprecate_d_unblock_(BlockedConstraintId::V2(register_constraint(progressed)));
  }
}

impl ConstraintSolver {
  pub fn fill_in_discriminant_types(
    &mut self,
    constraint: &Constraint,
    discriminant_types: &[Option<TypeId>],
  ) {
    for ty_opt in discriminant_types.iter() {
      let ty = match ty_opt {
        Some(ty) => *ty,
        None => continue,
      };

      let follow_ty = follow_type::follow(ty);

      if self.is_blocked_type_id(follow_ty) {
        let mutable_ty = { as_mutable_type_id(follow_ty) };
        alias(mutable_ty).ty = TypeVariant::Bound(self.builtin_types_ref().no_refine_type);
      }

      self.unblock_type_id_location(ty, constraint.location);
    }
  }
}

impl ConstraintSolver {
  pub fn generalize_one_type(&mut self, ty: TypeId) {
    let ty = follow_type::follow(ty);
    let free_ty = get_type::get::<FreeType>(ty);

    let saveme = if fflag::DebugLuauLogSolver.get() {
      to_string_type_id_to_string_options_mut(ty, self.opts.clone())
    } else {
      "[FFlag::DebugLuauLogSolver Off]".to_string()
    };

    let Some(free_ty) = free_ty else {
      return;
    };

    let function_type = self.constraint_set.scope_to_function.find(&free_ty.scope);

    if let Some(function_type) = function_type {
      let result_ty = generalize(
        self.arena,
        self.builtin_types,
        // free_ty.scope 为 FreeType 记录内 NotNull 语义裸指针字段，alias_ref 换共享引用。
        alias_ref(free_ty.scope),
        &mut self.generalized_types_,
        *function_type,
        Some(ty),
      );

      if fflag::DebugLuauLogSolver.get() {
        let current_ty_str = to_string_type_id(ty);
        let result_ty_str = result_ty
          .map(to_string_type_id)
          .unwrap_or_else(|| to_string_type_id(*function_type));

        println!(
          "Eagerly generalized {} (now {})\n\tin function {}",
          saveme, current_ty_str, result_ty_str
        );
      }
    }
  }
}

impl ConstraintSolver {
  pub fn has_unresolved_constraints(&mut self, ty: TypeId) -> bool {
    if fflag::LuauConstraintGraph.get() {
      let ty = follow_type::follow(ty);
      alias(self.cgraph).has_unsolved_dependencies(BlockedConstraintId::V0(ty))
    } else {
      let ty = follow_type::follow(ty);
      if let Some(set) = self.deprecated_type_to_constraint_set.get(&ty) {
        !set.is_empty()
      } else {
        false
      }
    }
  }
}

impl ConstraintSolver {
  pub fn inherit_blocks(&mut self, source: *const Constraint, addition: *const Constraint) {
    if fflag::LuauConstraintGraph.get() {
      alias(self.cgraph).inherit_blocks(
        BlockedConstraintId::V2(register_constraint(source)),
        BlockedConstraintId::V2(register_constraint(addition)),
      )
    } else {
      // Anything that is blocked on this constraint must also be blocked on our
      // synthesized constraints.
      let blocked_constraints: Vec<*const Constraint> = match self
        .deprecated_blocked
        .get(&BlockedConstraintId::V2(register_constraint(source)))
      {
        Some(blocked_set) => blocked_set.iter().copied().collect(),
        None => Vec::new(),
      };
      for blocked_constraint in blocked_constraints {
        self.block_not_null_constraint_not_null_constraint(addition, blocked_constraint);
      }
    }
  }
}

impl ConstraintSolver {
  pub fn is_blocked_type_id(&self, ty: TypeId) -> bool {
    // FIXME CLI-180636: Eventually this should use the same logic as
    // `SubtypingUnifier`, which is that blocked types are only based
    // on their type and any additional state, rather than looking at
    // `uninhabitedTypeFunctions`.
    let ty = follow_type::follow(ty);

    if let Some(tfit) = get_type::get::<TypeFunctionInstanceType>(ty) {
      if tfit.state != TypeFunctionInstanceState::Unsolved {
        return false;
      }

      return !self.uninhabited_type_functions.contains(&(ty as *const ()));
    }

    get_type::get::<BlockedType>(ty).is_some()
      || get_type::get::<PendingExpansionType>(ty).is_some()
  }

  pub(crate) fn is_blocked_type_pack_id(&self, tp: TypePackId) -> bool {
    let tp = follow_type_pack::follow(tp);

    if get_type_pack::get::<TypeFunctionInstanceTypePack>(tp).is_some() {
      return !self.uninhabited_type_functions.contains(&(tp as *const ()));
    }

    get_type_pack::get::<BlockedTypePack>(tp).is_some()
  }
}

impl ConstraintSolver {
  pub fn is_done(&self) -> bool {
    self.unsolved_constraints.is_empty()
  }
}

impl ConstraintSolver {
  pub fn randomize(&mut self, seed: u32) {
    if self.unsolved_constraints.is_empty() {
      return;
    }

    let mut rng = seed;

    for i in (1..self.unsolved_constraints.len()).rev() {
      let j = (rng as usize) % (i + 1);

      self.unsolved_constraints.swap(i, j);

      rng = rng.wrapping_mul(1664525).wrapping_add(1013904223);
    }
  }
}

impl ConstraintSolver {
  pub fn report_error_type_error_data_location(
    &mut self,
    data: TypeErrorData,
    location: &Location,
  ) {
    self
      .errors
      .push(TypeError::type_error_location_type_error_data(
        *location, data,
      ));
    if let Some(ref module) = self.module {
      let name = module.name.clone();
      if let Some(last) = self.errors.last_mut() {
        last.module_name = name;
      }
    }
  }

  pub fn report_error_type_error(&mut self, e: TypeError) {
    {
      self.errors.push(e);
      // Safety: 紧邻上方 push 刚入队，last_mut 必命中。
      let last_error = self.errors.last_mut().expect("紧邻 push 之后取尾，必命中");
      if let Some(ref module) = self.module {
        last_error.module_name = module.name.clone();
      }
    }
  }
}

impl ConstraintSolver {
  pub fn simplify_intersection_not_null_scope_location_type_id_type_id(
    &mut self,
    _scope: &Scope,
    _location: Location,
    left: TypeId,
    right: TypeId,
  ) -> TypeId {
    simplify_intersection(self.builtin_types, self.arena, left, right).result
  }

  pub fn simplify_intersection_not_null_scope_location_type_ids(
    &mut self,
    _scope: &Scope,
    _location: Location,
    parts: TypeIds,
  ) -> TypeId {
    let left = parts.front();
    let mut parts = parts;
    parts.erase_type_id(left);
    let right = parts.front();

    simplify_intersection(self.builtin_types, self.arena, left, right).result
  }
}

impl ConstraintSolver {
  pub(crate) fn simplify_union(
    &mut self,
    _scope: &Scope,
    _location: Location,
    left: TypeId,
    right: TypeId,
  ) -> TypeId {
    simplify_union(self.builtin_types, self.arena, left, right).result
  }
}

impl ConstraintSolver {
  pub fn constraint_solver_throw_time_limit_error(&self) {
    // 载荷携带类型而非渲染串：跨 `catch_unwind` 后 `downcast_ref::<TimeLimitError>()`
    // 才可能命中（消费方见 `ulua-analyze-cli` 的 ICE 收口）。文案由该类型的
    // `Display`（转发 `base.message`）单源产生，与旧 `panic!("{}", ..)` 逐字一致。
    panic_any(TimeLimitError::time_limit_error_time_limit_error(""));
  }
}

impl ConstraintSolver {
  pub fn constraint_solver_throw_user_cancel_error(&self) {
    // 同上；旧形态是 `panic!("{:?}", ..)`，把 Debug 文本当消息抛出，既丢了类型
    // 也丢了 cpp `what()` 文案，现统一为类型化载荷 + `Display` 渲染。
    panic_any(UserCancelError::new(String::new()));
  }
}

impl ConstraintSolver {
  pub fn unpack_and_assign(
    &mut self,
    dest_types: Vec<TypeId>,
    src_types: TypePackId,
    constraint: NonNull<Constraint>,
  ) -> NonNull<Constraint> {
    // constraint 为 NonNull 句柄（类型不变量恒非空），指向 Box 堆上存活的
    // Constraint（push_constraint 以 `&mut *c` 出裸地址，Vec 移动 Box 不影响
    // 堆地址，solver 存活期内稳定）。此处仅 Copy 读取 scope/location 字段。
    let constraint_scope = alias_ref(constraint.as_ptr()).scope;
    let constraint_location = alias_ref(constraint.as_ptr()).location;

    let c = self.push_constraint(
      NonNull::new(constraint_scope)
        .expect("Constraint.scope 按 cpp NotNull<Scope> 构造登记，恒非空"),
      constraint_location,
      ConstraintV::Unpack(UnpackConstraint {
        result_pack: dest_types.clone(),
        source_pack: src_types,
      }),
    );

    for t in dest_types {
      // C++ `LUAU_ASSERT(bt)` 后 `bt->replaceOwner(...)`：dest 均为 blocked。
      get_mutable_type::get_mutable::<BlockedType>(t)
        .expect("unpack dest must be blocked")
        .replace_owner(c.as_ptr());
    }

    c
  }
}
