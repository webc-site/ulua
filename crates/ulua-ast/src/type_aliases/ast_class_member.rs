use ulua_common::records::variant::Variant2;

use crate::records::{ast_class_method::AstClassMethod, ast_class_property::AstClassProperty};

pub type AstClassMember = Variant2<AstClassProperty, AstClassMethod>;
