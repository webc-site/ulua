use crate::records::json_emitter::JsonEmitter;

/// 借用父 `JsonEmitter` 的数组写入器;`Drop` 时自动 `finish()`。
#[derive(Debug)]
pub struct ArrayEmitter<'a> {
  pub(crate) emitter: &'a mut JsonEmitter,
  pub(crate) comma: bool,
  pub(crate) finished: bool,
}
