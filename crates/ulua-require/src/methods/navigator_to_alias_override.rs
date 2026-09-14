use crate::{
  enums::navigate_result::NavigateResult,
  records::{
    error_handler::ErrorHandler,
    navigation_context::NavigationContextTrait,
    navigator::{Error, Navigator},
  },
};

impl<C: NavigationContextTrait, E: ErrorHandler> Navigator<'_, C, E> {
  pub fn to_alias_override(&mut self, alias_unprefixed: &str) -> (Error, bool) {
    match self.navigation_context.to_alias_override(alias_unprefixed) {
      NavigateResult::Success => (None, true),
      NavigateResult::NotFound => (None, false),
      NavigateResult::Ambiguous => (
        Some(format!(
          "@{} is not a valid alias (ambiguous)",
          alias_unprefixed
        )),
        false,
      ),
    }
  }
}
