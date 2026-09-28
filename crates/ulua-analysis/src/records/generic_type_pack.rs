use crate::{
  enums::polarity::Polarity,
  records::{scope::Scope, type_level::TypeLevel},
  type_aliases::name_type::Name,
};

#[derive(Debug, Clone)]
pub struct GenericTypePack {
  pub(crate) index: i32,
  pub(crate) level: TypeLevel,
  pub(crate) scope: *mut Scope,
  pub(crate) name: Name,
  pub(crate) explicit_name: bool,
  pub(crate) polarity: Polarity,
}
