use crate::{
  records::singleton_type::SingletonType, type_aliases::singleton_variant::SingletonVariant,
};

impl SingletonType {
  pub fn new(variant: SingletonVariant) -> Self {
    Self { variant }
  }
}
