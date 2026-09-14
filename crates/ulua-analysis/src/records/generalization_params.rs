use crate::enums::polarity::Polarity;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GeneralizationParams {
  pub found_outside_functions: bool,
  pub use_count: usize,
  pub polarity: Polarity,
}

impl Default for GeneralizationParams {
  fn default() -> Self {
    Self {
      found_outside_functions: false,
      use_count: 0,
      polarity: Polarity::None,
    }
  }
}
