use crate::type_aliases::{scope_ptr_type::ScopePtr, type_id::TypeId};

#[derive(Debug, Clone)]
pub struct FunctionSignature {
  /// The type of the function.
  pub signature: TypeId,
  /// The scope that encompasses the function's signature. May be nullptr
  /// if there was no need for a signature scope (the function has no
  /// generics).
  pub signature_scope: ScopePtr,
  /// The scope that encompasses the function's body. Is a child scope of
  /// signatureScope, if present.
  pub body_scope: ScopePtr,
}
