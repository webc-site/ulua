use crate::records::{
  error_handler::ErrorHandler, navigation_context::NavigationContextTrait, navigator::Navigator,
};

impl Navigator<'_> {
  pub fn new<'ctx>(
    navigation_context: &'ctx mut dyn NavigationContextTrait,
    error_handler: &'ctx mut dyn ErrorHandler,
  ) -> Navigator<'ctx> {
    Navigator {
      navigation_context,
      error_handler,
    }
  }
}
