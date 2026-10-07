//! `dcr_logger` 方法汇总：原先按 cpp 符号逐方法拆分的同前缀小文件合并至此，行为逐字保留。

use alloc::{string::String, vec::Vec};
use core::mem::take;

use ulua_common::records::{dense_hash_map::DenseHashMap, variant::Variant2};

use crate::{
  functions::{
    snapshot_scope::snapshot_scope,
    snapshot_type_strings::snapshot_type_strings,
    to_string_error::to_string_type_error,
    to_string_to_string::{
      to_string_constraint_to_string_options, to_string_type_id_to_string_options,
      to_string_type_pack_id_to_string_options,
    },
  },
  records::{
    annotation_types_at_location::AnnotationTypesAtLocation, arena_handle::alias_ref,
    boundary_snapshot::BoundarySnapshot, constraint::Constraint, constraint_block::ConstraintBlock,
    constraint_snapshot::ConstraintSnapshot, constraint_step_snapshot::ConstraintStepSnapshot,
    dcr_logger::DcrLogger, error_snapshot::ErrorSnapshot,
    expr_types_at_location::ExprTypesAtLocation, generalize_step_snapshot::GeneralizeStepSnapshot,
    scope::Scope, scope_snapshot::ScopeSnapshot, type_error::TypeError,
  },
  type_aliases::{
    constraint_block_target::ConstraintBlockTarget, module_ptr_module::ModulePtr,
    step_snapshot::StepSnapshot, type_id::TypeId, type_pack_id::TypePackId,
  },
};

impl DcrLogger {
  pub fn capture_boundary_state(
    &mut self,
    target: &mut BoundarySnapshot,
    root_scope: &Scope,
    unsolved_constraints: &[*const Constraint],
  ) {
    target.root_scope = snapshot_scope(root_scope, &mut self.opts);
    target.unsolved_constraints.clear();

    for &c in unsolved_constraints.iter() {
      let constraint_str = to_string_constraint_to_string_options(alias_ref(c), &mut self.opts);
      let location = alias_ref(c).location;
      let blocks = self.snapshot_blocks(c);
      let snapshot = ConstraintSnapshot {
        stringification: constraint_str,
        location,
        blocks,
      };
      *target.unsolved_constraints.get_or_insert(c) = snapshot;
    }

    let Self {
      generation_log,
      opts,
      ..
    } = self;
    snapshot_type_strings(
      &generation_log.expr_type_locations,
      &generation_log.annotation_type_locations,
      &mut target.type_strings,
      opts,
    );
  }
}

impl DcrLogger {
  pub fn capture_final_solver_state(
    &mut self,
    root_scope: &Scope,
    unsolved_constraints: &[*const Constraint],
  ) {
    let mut final_state = take(&mut self.solve_log.final_state);
    self.capture_boundary_state(&mut final_state, root_scope, unsolved_constraints);
    self.solve_log.final_state = final_state;
  }
}

impl DcrLogger {
  pub fn capture_generation_error(&mut self, error: &TypeError) {
    let stringified_error: String = to_string_type_error(error);
    self.generation_log.errors.push(ErrorSnapshot {
      message: stringified_error,
      location: error.location,
    });
  }
}

impl DcrLogger {
  pub fn capture_generation_module(&mut self, module: ModulePtr) {
    let module_ref = &*module;

    self
      .generation_log
      .expr_type_locations
      .reserve(module_ref.ast_types.size());
    for (expr, ty) in module_ref.ast_types.iter() {
      let expr = *expr;
      let mut tys = ExprTypesAtLocation {
        location: alias_ref(expr).base.location,
        ty: *ty,
        expected_ty: None,
      };

      if let Some(expected_ty) = module_ref.ast_expected_types.find(&expr) {
        tys.expected_ty = Some(*expected_ty);
      }

      self.generation_log.expr_type_locations.push(tys);
    }

    self
      .generation_log
      .annotation_type_locations
      .reserve(module_ref.ast_resolved_types.size());
    for (annot, ty) in module_ref.ast_resolved_types.iter() {
      let annot = *annot;
      let tys = AnnotationTypesAtLocation {
        location: alias_ref(annot).base.location,
        resolved_ty: *ty,
      };

      self.generation_log.annotation_type_locations.push(tys);
    }
  }
}

impl DcrLogger {
  pub fn capture_initial_solver_state(
    &mut self,
    root_scope: &Scope,
    unsolved_constraints: &[*const Constraint],
  ) {
    let mut initial_state = take(&mut self.solve_log.initial_state);
    self.capture_boundary_state(&mut initial_state, root_scope, unsolved_constraints);
    self.solve_log.initial_state = initial_state;
  }
}

impl DcrLogger {
  pub fn capture_type_check_error(&mut self, error: &TypeError) {
    let stringified_error = to_string_type_error(error);
    let snapshot = ErrorSnapshot {
      message: stringified_error,
      location: error.location,
    };

    self.check_log.errors.push(snapshot);
  }
}

impl DcrLogger {
  pub fn commit_step_snapshot(&mut self, snapshot: StepSnapshot) {
    if let Variant2::V1(eg) = &snapshot
      && eg.before == eg.after
    {
      return;
    }

    self.solve_log.step_states.push(snapshot);
  }
}

impl DcrLogger {
  pub fn pop_block_type_id(&mut self, block: TypeId) {
    for (_, list) in self.constraint_blocks.iter_mut() {
      list.retain(|target| {
        if let ConstraintBlockTarget::V0(target_block) = target {
          *target_block != block
        } else {
          true
        }
      });
    }
  }

  pub fn pop_block_type_pack_id(&mut self, block: TypePackId) {
    for (_, list) in self.constraint_blocks.iter_mut() {
      list.retain(|target| {
        if let ConstraintBlockTarget::V1(target_block) = target {
          *target_block != block
        } else {
          true
        }
      });
    }
  }

  pub fn pop_block_not_null_constraint(&mut self, block: *const Constraint) {
    for (_, list) in self.constraint_blocks.iter_mut() {
      list.retain(|target| {
        if let ConstraintBlockTarget::V2(target_block) = target {
          *target_block != block
        } else {
          true
        }
      });
    }
  }
}

impl DcrLogger {
  /// `GeneralizeStepSnapshot DcrLogger::prepareGeneralizationSnapshot(...)`
  /// (`Analysis/src/DcrLogger.cpp:455-483`).
  pub fn prepare_generalization_snapshot(
    &mut self,
    before: String,
    root_scope: &Scope,
    unsolved_constraints: &[*const Constraint],
  ) -> GeneralizeStepSnapshot {
    let (root_scope, unsolved_constraints, type_strings) =
      self.prepare_snapshot_parts(root_scope, unsolved_constraints);

    GeneralizeStepSnapshot {
      before,
      // /*after*/ "", // to be filled in
      after: String::new(),
      unsolved_constraints,
      root_scope,
      type_strings,
    }
  }
}

// `DcrLogger::prepareStepSnapshot` 与 `prepareGeneralizationSnapshot`
// （DcrLogger.cpp:424-483）共用的三段式快照骨架：scope 快照、未解约束快照表、
// 生成期类型串表。两入口原先各抄一份逐字相同的实现，收口于此。

impl DcrLogger {
  pub(crate) fn prepare_snapshot_parts(
    &mut self,
    root_scope: &Scope,
    unsolved_constraints: &[*const Constraint],
  ) -> (
    ScopeSnapshot,
    DenseHashMap<*const Constraint, ConstraintSnapshot>,
    DenseHashMap<*const (), String>,
  ) {
    let scope_snapshot: ScopeSnapshot = snapshot_scope(root_scope, &mut self.opts);
    let mut constraints: DenseHashMap<*const Constraint, ConstraintSnapshot> =
      DenseHashMap::default();

    for &c in unsolved_constraints.iter() {
      let stringification = to_string_constraint_to_string_options(alias_ref(c), &mut self.opts);
      let location = alias_ref(c).location;
      let blocks = self.snapshot_blocks(c);
      *constraints.get_or_insert(c) = ConstraintSnapshot {
        stringification,
        location,
        blocks,
      };
    }

    let mut type_strings: DenseHashMap<*const (), String> = DenseHashMap::default();
    let Self {
      generation_log,
      opts,
      ..
    } = self;
    snapshot_type_strings(
      &generation_log.expr_type_locations,
      &generation_log.annotation_type_locations,
      &mut type_strings,
      opts,
    );

    (scope_snapshot, constraints, type_strings)
  }
}

impl DcrLogger {
  /// `ConstraintStepSnapshot DcrLogger::prepareStepSnapshot(...)`
  /// (`Analysis/src/DcrLogger.cpp:424-453`).
  pub fn prepare_step_snapshot(
    &mut self,
    root_scope: &Scope,
    current: *const Constraint,
    force: bool,
    unsolved_constraints: &[*const Constraint],
  ) -> ConstraintStepSnapshot {
    let (root_scope, unsolved_constraints, type_strings) =
      self.prepare_snapshot_parts(root_scope, unsolved_constraints);

    ConstraintStepSnapshot {
      current_constraint: current,
      forced: force,
      unsolved_constraints,
      root_scope,
      type_strings,
    }
  }
}

impl DcrLogger {
  pub fn push_block_not_null_constraint_type_id(
    &mut self,
    constraint: *const Constraint,
    block: TypeId,
  ) {
    self
      .constraint_blocks
      .get_or_insert(constraint)
      .push(ConstraintBlockTarget::V0(block));
  }

  pub fn push_block_not_null_constraint_type_pack_id(
    &mut self,
    constraint: *const Constraint,
    block: TypePackId,
  ) {
    self
      .constraint_blocks
      .get_or_insert(constraint)
      .push(ConstraintBlockTarget::V1(block));
  }

  pub fn push_block_not_null_constraint_not_null_constraint(
    &mut self,
    constraint: *const Constraint,
    block: *const Constraint,
  ) {
    self
      .constraint_blocks
      .get_or_insert(constraint)
      .push(ConstraintBlockTarget::V2(block));
  }
}

impl DcrLogger {
  /// `std::vector<ConstraintBlock> DcrLogger::snapshotBlocks(NotNull<const Constraint> c)`
  /// (`Analysis/src/DcrLogger.cpp:510-550`).
  pub fn snapshot_blocks(&self, c: *const Constraint) -> Vec<ConstraintBlock> {
    // The hash from `c` is independent of `opts`, so a `&self` shared borrow
    // suffices for the lookup; stringification clones `opts` internally.
    let mut opts = self.opts.clone();

    let it = match self.constraint_blocks.find(&c) {
      Some(list) => list,
      None => return Vec::new(),
    };

    let mut snapshot: Vec<ConstraintBlock> = Vec::new();

    for target in it.iter() {
      if let Some(ty) = target.get_if::<TypeId>() {
        snapshot.push(ConstraintBlock {
          target: target.clone(),
          stringification: to_string_type_id_to_string_options(*ty, &mut opts),
        });
      } else if let Some(tp) = target.get_if::<TypePackId>() {
        snapshot.push(ConstraintBlock {
          target: target.clone(),
          stringification: to_string_type_pack_id_to_string_options(*tp, &mut opts),
        });
      } else if let Some(c) = target.get_if::<*const Constraint>() {
        snapshot.push(ConstraintBlock {
          target: target.clone(),
          stringification: to_string_constraint_to_string_options(alias_ref(*c), &mut opts),
        });
      } else {
        debug_assert!(false);
      }
    }

    snapshot
  }
}
