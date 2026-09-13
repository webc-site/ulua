use ulua_ast::records::location::Location;

use crate::{records::symbol::Symbol, type_aliases::variant::Variant as VariantAlias};

#[derive(Debug, Clone)]
pub struct Def {
  pub v: VariantAlias,
  pub name: Symbol,
  pub location: Location,
}
