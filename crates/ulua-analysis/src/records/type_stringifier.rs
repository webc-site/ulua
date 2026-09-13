use crate::records::stringifier_state::StringifierState;
#[derive(Debug, Clone)]
pub struct TypeStringifier {
  pub(crate) state: *mut StringifierState,
}
