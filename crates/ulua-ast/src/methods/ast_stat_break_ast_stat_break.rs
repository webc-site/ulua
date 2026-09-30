use crate::records::{ast_stat::AstStat, ast_stat_break::AstStatBreak, location::Location};

impl_ast_node_new!(AstStatBreak, AstStat, location: Location);
