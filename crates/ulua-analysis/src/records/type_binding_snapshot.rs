use alloc::string::String;
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct TypeBindingSnapshot {
  pub type_id: String,
  pub type_string: String,
}
