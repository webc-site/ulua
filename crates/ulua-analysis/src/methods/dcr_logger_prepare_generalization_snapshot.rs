use alloc::string::String;
use core::{ffi::c_void, ptr::null};

use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::{
  functions::{
    snapshot_scope::snapshot_scope, snapshot_type_strings::snapshot_type_strings,
    to_string_to_string_alt_q::to_string_constraint_to_string_options,
  },
  records::{
    constraint::Constraint, constraint_snapshot::ConstraintSnapshot, dcr_logger::DcrLogger,
    generalize_step_snapshot::GeneralizeStepSnapshot, scope::Scope, scope_snapshot::ScopeSnapshot,
  },
};
impl DcrLogger {
  /// `GeneralizeStepSnapshot DcrLogger::prepareGeneralizationSnapshot(...)`
  /// (`Analysis/src/DcrLogger.cpp:455-483`).
  pub fn prepare_generalization_snapshot(
    &mut self,
    before: String,
    root_scope: &Scope,
    unsolved_constraints: &[*const Constraint],
  ) -> GeneralizeStepSnapshot {
    let scope_snapshot: ScopeSnapshot = snapshot_scope(root_scope, &mut self.opts);
    let mut constraints: DenseHashMap<*const Constraint, ConstraintSnapshot> =
      DenseHashMap::new(null());

    for &c in unsolved_constraints.iter() {
      let stringification = to_string_constraint_to_string_options(unsafe { &*c }, &mut self.opts);
      let location = unsafe { (*c).location };
      let blocks = self.snapshot_blocks(c);
      *constraints.get_or_insert(c) = ConstraintSnapshot {
        stringification,
        location,
        blocks,
      };
    }

    let mut type_strings: DenseHashMap<*const c_void, String> = DenseHashMap::new(null());
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

    GeneralizeStepSnapshot {
      before,
      // /*after*/ "", // to be filled in
      after: String::new(),
      unsolved_constraints: constraints,
      root_scope: scope_snapshot,
      type_strings,
    }
  }
}
