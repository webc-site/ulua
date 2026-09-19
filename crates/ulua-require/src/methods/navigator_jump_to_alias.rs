use crate::{
  enums::navigate_result::NavigateResult,
  functions::navigate_error::{ambiguous, quoted},
  records::{
    error_handler::ErrorHandler,
    navigation_context::NavigationContextTrait,
    navigator::{Error, Navigator},
  },
};

/// cpp 原文案前缀：`"could not jump to alias \"" + path + "\""`。
const JUMP_PREFIX: &[u8] = b"could not jump to alias ";

impl<C: NavigationContextTrait, E: ErrorHandler> Navigator<'_, C, E> {
  pub(crate) fn jump_to_alias(&mut self, alias_path: &[u8]) -> Error {
    let result = self.navigation_context.jump_to_alias(alias_path);
    if result == NavigateResult::Success {
      return None;
    }

    Some(quoted(JUMP_PREFIX, alias_path, ambiguous(result)))
  }
}
