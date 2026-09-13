use crate::type_aliases::singleton_variant::SingletonVariant;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SingletonType {
  pub variant: SingletonVariant,
}

unsafe impl Send for SingletonType {}
unsafe impl Sync for SingletonType {}
