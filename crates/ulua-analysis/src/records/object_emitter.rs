use crate::records::json_emitter::JsonEmitter;

/// 借用父 `JsonEmitter` 的对象写入器;`Drop` 时自动 `finish()`,
/// 生命周期保证父 emitter 在写入器存活期间不可被移动或另用。
#[derive(Debug)]
pub struct ObjectEmitter<'a> {
  pub(crate) emitter: &'a mut JsonEmitter,
  pub(crate) comma: bool,
  pub(crate) finished: bool,
}
