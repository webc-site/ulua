use crate::enums::block_kind::BlockKind;

pub fn block_kind_name(kind: BlockKind) -> &'static str {
  match kind {
    BlockKind::Entry => "entry",
    BlockKind::Linear => "linear",
    BlockKind::Condition => "condition",
  }
}
