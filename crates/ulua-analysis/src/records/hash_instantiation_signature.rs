use ulua_common::{collections::fast_hash, records::dense_hash_table::DenseHasher};

use crate::records::instantiation_signature::InstantiationSignature;

// C++ (ConstraintSolver.h:65-68): a hash functor over InstantiationSignature;
#[derive(Debug, Clone, Copy, Default)]
pub struct HashInstantiationSignature;

impl HashInstantiationSignature {
  #[inline]
  pub fn operator_call(&self, signature: &InstantiationSignature) -> usize {
    // Hash the function type
    let mut hash = fast_hash(&signature.fn_sig.r#type);

    // Hash type parameters
    for p in &signature.fn_sig.type_params {
      hash ^= fast_hash(&p.ty) << 1;
    }

    // Hash type pack parameters
    for p in &signature.fn_sig.type_pack_params {
      hash ^= fast_hash(&p.tp) << 1;
    }

    // Hash arguments
    for a in &signature.arguments {
      hash ^= fast_hash(a) << 1;
    }

    // Hash pack arguments
    for a in &signature.pack_arguments {
      hash ^= fast_hash(a) << 1;
    }

    hash
  }
}

impl DenseHasher<InstantiationSignature> for HashInstantiationSignature {
  #[inline]
  fn hash(&self, key: &InstantiationSignature) -> usize {
    self.operator_call(key)
  }
}
