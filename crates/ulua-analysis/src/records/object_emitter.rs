use crate::records::json_emitter::JsonEmitter;

#[derive(Debug, Clone)]
pub struct ObjectEmitter {
  pub(crate) emitter: *mut JsonEmitter,
  pub(crate) comma: bool,
  pub(crate) finished: bool,
}
