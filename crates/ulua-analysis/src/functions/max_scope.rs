use crate::{functions::subsumes_scope::subsumes, records::scope::Scope};

/// `inline Scope* max(Scope* left, Scope* right)` (Scope.h:126-132).
/// Returns the inner (more-specific) of two scopes.
pub fn max(left: *mut Scope, right: *mut Scope) -> *mut Scope {
  if subsumes(left, right) { right } else { left }
}
