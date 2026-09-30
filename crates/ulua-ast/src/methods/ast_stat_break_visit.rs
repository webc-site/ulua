use crate::{
  records::ast_stat_break::AstStatBreak,
  visit::{AstNodeRefMut, AstVisitable},
};

impl_visitable!(AstStatBreak, StatBreak);
