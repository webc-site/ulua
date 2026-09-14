use ulua_common::records::variant::Variant2;

use crate::{
  functions::{
    write_dcr_logger_alt_r::write_json_emitter_constraint_step_snapshot,
    write_dcr_logger_alt_s::write_json_emitter_generalize_step_snapshot,
  },
  records::json_emitter::JsonEmitter,
  type_aliases::step_snapshot::StepSnapshot,
};

pub fn write_json_emitter_step_snapshot(emitter: &mut JsonEmitter, snap: &StepSnapshot) {
  match snap {
    Variant2::V0(s) => write_json_emitter_constraint_step_snapshot(emitter, s),
    Variant2::V1(s) => write_json_emitter_generalize_step_snapshot(emitter, s),
  }
}
