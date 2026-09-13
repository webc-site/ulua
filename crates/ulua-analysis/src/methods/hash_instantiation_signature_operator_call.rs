use std::{
  collections::hash_map::DefaultHasher,
  hash::{Hash, Hasher},
};

use ulua_common::records::dense_hash_table::DenseHasher;

use crate::records::{
  hash_instantiation_signature::HashInstantiationSignature,
  instantiation_signature::InstantiationSignature,
};

impl HashInstantiationSignature {
  pub fn operator_call(&self, signature: &InstantiationSignature) -> usize {
    // Hash the function type
    let mut hasher = DefaultHasher::new();
    signature.fn_sig.r#type.hash(&mut hasher);
    let mut hash = hasher.finish() as usize;

    // Hash type parameters
    for p in &signature.fn_sig.type_params {
      let mut h = DefaultHasher::new();
      p.ty.hash(&mut h);
      hash ^= (h.finish() as usize) << 1;
    }

    // Hash type pack parameters
    for p in &signature.fn_sig.type_pack_params {
      let mut h = DefaultHasher::new();
      p.tp.hash(&mut h);
      hash ^= (h.finish() as usize) << 1;
    }

    // Hash arguments
    for a in &signature.arguments {
      let mut h = DefaultHasher::new();
      a.hash(&mut h);
      hash ^= (h.finish() as usize) << 1;
    }

    // Hash pack arguments
    for a in &signature.pack_arguments {
      let mut h = DefaultHasher::new();
      a.hash(&mut h);
      hash ^= (h.finish() as usize) << 1;
    }

    hash
  }
}

impl DenseHasher<InstantiationSignature> for HashInstantiationSignature {
  fn hash(&self, key: &InstantiationSignature) -> usize {
    self.operator_call(key)
  }
}
