use ulua_ast::records::location::Location;
use ulua_common::records::{
  dense_hash_map::DenseHashMap, dense_hash_table::DenseDefault, variant::Variant2,
};

use crate::{
  functions::to_pointer_id_dcr_logger::{to_pointer_id, to_pointer_id_not_null_constraint},
  methods::object_emitter_write_pair::WriteJson,
  records::{
    annotation_types_at_location::AnnotationTypesAtLocation, binding_snapshot::BindingSnapshot,
    boundary_snapshot::BoundarySnapshot, constraint_block::ConstraintBlock,
    constraint_generation_log::ConstraintGenerationLog, constraint_snapshot::ConstraintSnapshot,
    constraint_step_snapshot::ConstraintStepSnapshot, error_snapshot::ErrorSnapshot,
    expr_types_at_location::ExprTypesAtLocation, generalize_step_snapshot::GeneralizeStepSnapshot,
    json_emitter::JsonEmitter, scope_snapshot::ScopeSnapshot,
    type_binding_snapshot::TypeBindingSnapshot, type_check_log::TypeCheckLog,
    type_solve_log::TypeSolveLog,
  },
  type_aliases::{constraint_block_target::ConstraintBlockTarget, step_snapshot::StepSnapshot},
};

pub fn write_json_emitter_t<T>(emitter: &mut JsonEmitter, ptr: *const T) {
  let id = to_pointer_id(ptr);
  emitter.write_raw_string_view(&id);
}

/// `void write(JsonEmitter& emitter, const Location& location)`
/// (`Analysis/src/DcrLogger.cpp:37-45`).
pub fn write_json_emitter_location(emitter: &mut JsonEmitter, location: &Location) {
  let mut a = emitter.write_array();
  a.write_value(location.begin.line);
  a.write_value(location.begin.column);
  a.write_value(location.end.line);
  a.write_value(location.end.column);
  a.finish();
}

pub fn write_json_emitter_error_snapshot(emitter: &mut JsonEmitter, snapshot: &ErrorSnapshot) {
  let mut o = emitter.write_object();
  o.write_pair("message", &snapshot.message);
  o.write_pair("location", snapshot.location);
  o.finish();
}

pub fn write_json_emitter_binding_snapshot(emitter: &mut JsonEmitter, snapshot: &BindingSnapshot) {
  let mut o = emitter.write_object();
  o.write_pair("typeId", &snapshot.type_id);
  o.write_pair("typeString", &snapshot.type_string);
  o.write_pair("location", snapshot.location);
  o.finish();
}

pub fn write_json_emitter_type_binding_snapshot(
  emitter: &mut JsonEmitter,
  snapshot: &TypeBindingSnapshot,
) {
  let mut o = emitter.write_object();
  o.write_pair("typeId", &snapshot.type_id);
  o.write_pair("typeString", &snapshot.type_string);
  o.finish();
}

// Source: `Analysis/src/DcrLogger.cpp` (lines 72-79, faithful port)
//
// C++ template:
// ```cpp
// template<typename K, typename V>
// void write(JsonEmitter& emitter, const DenseHashMap<const K*, V>& map)
// {
//     ObjectEmitter o = emitter.writeObject();
//     for (const auto& [k, v] : map)
//         o.writePair(toPointerId(k), v);
//     o.finish();
// }
// ```
//
// Writes a pointer-keyed `DenseHashMap` as a JSON object whose member names are
// each key's pointer id (`toPointerId`) and whose values are the mapped `V`.
pub fn write_json_emitter_dense_hash_map_k_v<K, V: WriteJson + DenseDefault>(
  emitter: &mut JsonEmitter,
  map: &DenseHashMap<*const K, V>,
) {
  let mut o = emitter.write_object();

  for (k, v) in map.iter() {
    o.write_pair(&to_pointer_id(*k), v);
  }

  o.finish();
}

pub fn write_json_emitter_expr_types_at_location(
  emitter: &mut JsonEmitter,
  tys: &ExprTypesAtLocation,
) {
  let mut o = emitter.write_object();
  o.write_pair("location", tys.location);
  o.write_pair("ty", to_pointer_id(tys.ty as *const _));
  if let Some(expected_ty) = tys.expected_ty {
    o.write_pair("expectedTy", to_pointer_id(expected_ty as *const _));
  }
  o.finish();
}

pub fn write_json_emitter_annotation_types_at_location(
  emitter: &mut JsonEmitter,
  tys: &AnnotationTypesAtLocation,
) {
  let mut o = emitter.write_object();
  o.write_pair("location", tys.location);
  o.write_pair("resolvedTy", to_pointer_id(tys.resolved_ty as *const _));
  o.finish();
}

pub fn write_json_emitter_constraint_generation_log(
  emitter: &mut JsonEmitter,
  log: &ConstraintGenerationLog,
) {
  let mut o = emitter.write_object();
  o.write_pair("source", &log.source);
  o.write_pair("errors", &log.errors);
  o.write_pair("exprTypeLocations", &log.expr_type_locations);
  o.write_pair("annotationTypeLocations", &log.annotation_type_locations);
  o.finish();
}

pub fn write_json_emitter_scope_snapshot(emitter: &mut JsonEmitter, snapshot: &ScopeSnapshot) {
  let mut o = emitter.write_object();
  o.write_pair("bindings", &snapshot.bindings);
  o.write_pair("typeBindings", &snapshot.type_bindings);
  o.write_pair("typePackBindings", &snapshot.type_pack_bindings);
  o.write_pair("children", &snapshot.children);
  o.finish();
}

pub fn write_json_emitter_constraint_block(emitter: &mut JsonEmitter, block: &ConstraintBlock) {
  let mut o = emitter.write_object();
  o.write_pair("stringification", &block.stringification);

  let target = &block.target;

  let kind = match target {
    ConstraintBlockTarget::V0(_) => "type",
    ConstraintBlockTarget::V1(_) => "typePack",
    ConstraintBlockTarget::V2(_) => "constraint",
  };

  let ptr_id = match target {
    ConstraintBlockTarget::V0(ty) => to_pointer_id(*ty),
    ConstraintBlockTarget::V1(tp) => to_pointer_id(*tp),
    ConstraintBlockTarget::V2(c) => to_pointer_id_not_null_constraint(*c),
  };

  o.write_pair("id", &ptr_id);
  o.write_pair("kind", kind);

  o.finish();
}

pub fn write_json_emitter_constraint_snapshot(
  emitter: &mut JsonEmitter,
  snapshot: &ConstraintSnapshot,
) {
  let mut o = emitter.write_object();
  o.write_pair("stringification", &snapshot.stringification);
  o.write_pair("location", snapshot.location);
  o.write_pair("blocks", &snapshot.blocks);
  o.finish();
}

pub fn write_json_emitter_boundary_snapshot(
  emitter: &mut JsonEmitter,
  snapshot: &BoundarySnapshot,
) {
  let mut o = emitter.write_object();
  o.write_pair("rootScope", &snapshot.root_scope);
  o.write_pair("unsolvedConstraints", &snapshot.unsolved_constraints);
  o.write_pair("typeStrings", &snapshot.type_strings);
  o.finish();
}

pub fn write_json_emitter_constraint_step_snapshot(
  emitter: &mut JsonEmitter,
  snapshot: &ConstraintStepSnapshot,
) {
  let mut o = emitter.write_object();
  o.write_pair("type", "constraint");
  o.write_pair("currentConstraint", snapshot.current_constraint);
  o.write_pair("forced", snapshot.forced);
  o.write_pair("unsolvedConstraints", &snapshot.unsolved_constraints);
  o.write_pair("rootScope", &snapshot.root_scope);
  o.write_pair("typeStrings", &snapshot.type_strings);
  o.finish();
}

pub fn write_json_emitter_generalize_step_snapshot(
  emitter: &mut JsonEmitter,
  eg: &GeneralizeStepSnapshot,
) {
  let mut o = emitter.write_object();
  o.write_pair("type", "generalize");
  o.write_pair("before", &eg.before);
  o.write_pair("after", &eg.after);
  o.write_pair("unsolvedConstraints", &eg.unsolved_constraints);
  o.write_pair("rootScope", &eg.root_scope);
  o.write_pair("typeStrings", &eg.type_strings);
  o.finish();
}

pub fn write_json_emitter_step_snapshot(emitter: &mut JsonEmitter, snap: &StepSnapshot) {
  match snap {
    Variant2::V0(s) => write_json_emitter_constraint_step_snapshot(emitter, s),
    Variant2::V1(s) => write_json_emitter_generalize_step_snapshot(emitter, s),
  }
}

pub fn write_json_emitter_type_solve_log(emitter: &mut JsonEmitter, log: &TypeSolveLog) {
  let mut object_emitter = emitter.write_object();
  object_emitter.write_pair("initialState", &log.initial_state);
  object_emitter.write_pair("stepStates", &log.step_states);
  object_emitter.write_pair("finalState", &log.final_state);
  object_emitter.finish();
}

pub fn write_json_emitter_type_check_log(emitter: &mut JsonEmitter, log: &TypeCheckLog) {
  let mut o = emitter.write_object();
  o.write_pair("errors", &log.errors);
  o.finish();
}
