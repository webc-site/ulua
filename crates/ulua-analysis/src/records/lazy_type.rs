use crate::records::r#type::Type;
#[derive(Debug, Clone)]
pub struct LazyType {
  pub(crate) unwrap: Option<fn(&mut LazyType)>,
  pub(crate) unwrapped: *const Type,
}
