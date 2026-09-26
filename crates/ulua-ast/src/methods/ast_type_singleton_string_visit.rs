use crate::{
  records::ast_type_singleton_string::AstTypeSingletonString,
  visit::{AstNodeRefMut, AstVisitable},
};

impl_visitable!(AstTypeSingletonString, TypeSingletonString);
