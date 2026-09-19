use crate::{
  enums::navigate_result::NavigateResult,
  functions::navigate_error::{ambiguous, quoted},
  records::{
    error_handler::ErrorHandler,
    navigation_context::NavigationContextTrait,
    navigator::{Error, Navigator},
  },
};

/// cpp 原文案前缀：`"could not resolve child component \"" + component + "\""`。
const CHILD_PREFIX: &[u8] = b"could not resolve child component ";

impl<C: NavigationContextTrait, E: ErrorHandler> Navigator<'_, C, E> {
  pub(crate) fn navigate_to_child(&mut self, component: &[u8]) -> Error {
    let result = self.navigation_context.to_child(component);
    if result == NavigateResult::Success {
      return None;
    }

    Some(quoted(CHILD_PREFIX, component, ambiguous(result)))
  }
}
