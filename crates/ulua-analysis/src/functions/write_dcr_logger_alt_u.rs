use crate::records::{json_emitter::JsonEmitter, type_solve_log::TypeSolveLog};

pub fn write_json_emitter_type_solve_log(emitter: &mut JsonEmitter, log: &TypeSolveLog) {
  let mut object_emitter = emitter.write_object();
  object_emitter.write_pair("initialState", &log.initial_state);
  object_emitter.write_pair("stepStates", &log.step_states);
  object_emitter.write_pair("finalState", &log.final_state);
  object_emitter.finish();
}
