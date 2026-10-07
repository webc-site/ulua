use crate::records::r#type::Type;

#[derive(Debug, Clone)]
pub struct PendingType {
  pub(crate) pending: Type,
  pub(crate) dead: bool,
}
