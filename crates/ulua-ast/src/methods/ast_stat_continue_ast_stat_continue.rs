use crate::records::{ast_stat::AstStat, ast_stat_continue::AstStatContinue, location::Location};

impl_ast_node_new!(AstStatContinue, AstStat, location: Location);
