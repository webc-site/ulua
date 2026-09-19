use alloc::vec::Vec;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct ClassShape {
  pub class_name: i32,
  pub property_names: Vec<i32>,
  pub method_names: Vec<i32>,
}
