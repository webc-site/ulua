use crate::records::{
  ast_stat::AstStat, ast_stat_block::AstStatBlock, location::Location, node_handle::Nodes,
};

impl_ast_node_new!(AstStatBlock, AstStat, location: Location, body: Nodes<AstStat>, has_end: bool);
