use crate::records::{
  error_handler::ErrorHandler, navigation_context::NavigationContextTrait, navigator::Navigator,
};

impl<'ctx, C: NavigationContextTrait, E: ErrorHandler> Navigator<'ctx, C, E> {
  pub fn new(navigation_context: &'ctx mut C, error_handler: &'ctx mut E) -> Navigator<'ctx, C, E> {
    Navigator {
      navigation_context,
      error_handler,
    }
  }
}
