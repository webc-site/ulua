use crate::{
  records::ast_stat_continue::AstStatContinue,
  visit::{AstNodeRefMut, AstVisitable},
};

impl_visitable!(AstStatContinue, StatContinue);
