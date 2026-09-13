use crate::{
  functions::to_pointer_id_dcr_logger::to_pointer_id, records::json_emitter::JsonEmitter,
};

pub fn write_json_emitter_t<T>(emitter: &mut JsonEmitter, ptr: *const T) {
  let id = to_pointer_id(ptr);
  emitter.write_raw_string_view(&id);
}
