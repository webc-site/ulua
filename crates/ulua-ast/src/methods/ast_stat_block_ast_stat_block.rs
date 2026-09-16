use crate::{
  records::{
    ast_array::AstArray, ast_node::AstNode, ast_stat::AstStat, ast_stat_block::AstStatBlock,
    location::Location,
  },
  rtti::AstNodeClass,
};

impl AstStatBlock {
  pub fn new(location: Location, body: AstArray<*mut AstStat>, has_end: bool) -> Self {
    Self {
      base: AstStat {
        base: AstNode {
          class_index: Self::CLASS_INDEX,
          location,
        },
        has_semicolon: false,
      },
      body,
      has_end,
    }
  }
}

pub fn ast_stat_block_ast_stat_block(
  location: Location,
  body: AstArray<*mut AstStat>,
  has_end: bool,
) -> AstStatBlock {
  AstStatBlock::new(location, body, has_end)
}
