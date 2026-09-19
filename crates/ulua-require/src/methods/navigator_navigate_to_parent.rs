use crate::{
  enums::navigate_result::NavigateResult,
  functions::navigate_error::{ambiguous, quoted, suffixed},
  records::{
    error_handler::ErrorHandler,
    navigation_context::NavigationContextTrait,
    navigator::{Error, Navigator},
  },
};

/// 带组件名时的消息前缀（cpp `"could not get parent of component \"" + c + "\""`）。
const PARENT_OF_PREFIX: &[u8] = b"could not get parent of component ";
/// 无前置组件时的固定消息。
const PARENT_OF_REQUIRER: &[u8] = b"could not get parent of requiring context";

impl<C: NavigationContextTrait, E: ErrorHandler> Navigator<'_, C, E> {
  pub(crate) fn navigate_to_parent(&mut self, previous_component: Option<&[u8]>) -> Error {
    let result = self.navigation_context.to_parent();
    if result == NavigateResult::Success {
      return None;
    }

    let suffix = ambiguous(result);
    Some(match previous_component {
      Some(component) => quoted(PARENT_OF_PREFIX, component, suffix),
      None => suffixed(PARENT_OF_REQUIRER, suffix),
    })
  }
}
