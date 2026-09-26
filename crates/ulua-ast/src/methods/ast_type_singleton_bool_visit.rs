use crate::{
  records::ast_type_singleton_bool::AstTypeSingletonBool,
  visit::{AstNodeRefMut, AstVisitable},
};

impl_visitable!(AstTypeSingletonBool, TypeSingletonBool);
