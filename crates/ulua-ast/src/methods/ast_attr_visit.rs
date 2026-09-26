use crate::{
  records::ast_attr::AstAttr,
  visit::{AstNodeRefMut, AstVisitable},
};

impl_visitable!(AstAttr, Attr);
