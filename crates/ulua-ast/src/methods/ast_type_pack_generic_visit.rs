use crate::{
  records::ast_type_pack_generic::AstTypePackGeneric,
  visit::{AstNodeRefMut, AstVisitable},
};

impl_visitable!(AstTypePackGeneric, TypePackGeneric);
