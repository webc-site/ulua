use alloc::{string::String, sync::Arc};

use ulua_analysis::{
  enums::polarity::Polarity,
  functions::fresh_type::fresh_type,
  records::{
    builtin_types::BuiltinTypes, generic_type::GenericType, scope::Scope, txn_log::TxnLog,
    type_arena::TypeArena,
  },
};

use crate::records::txn_log_fixture::TxnLogFixture;

impl TxnLogFixture {
  pub fn new() -> Self {
    let mut arena = TypeArena::default();
    let builtin_types = BuiltinTypes::new();
    let global_scope = Arc::new(Scope::scope_type_pack_id(builtin_types.any_type_pack()));
    let child_scope = Arc::new(Scope::new(&global_scope, 0));

    let global_scope_ptr = Arc::as_ptr(&global_scope) as *mut Scope;
    let child_scope_ptr = Arc::as_ptr(&child_scope) as *mut Scope;

    let a = fresh_type(
      &mut arena,
      &builtin_types,
      global_scope_ptr,
      Polarity::Unknown,
    );
    let b = fresh_type(
      &mut arena,
      &builtin_types,
      global_scope_ptr,
      Polarity::Unknown,
    );
    let c = fresh_type(
      &mut arena,
      &builtin_types,
      child_scope_ptr,
      Polarity::Unknown,
    );

    let generic_name = String::from("G");
    let g = arena.add_type(GenericType::generic_type_name_polarity(
      &generic_name,
      Polarity::Mixed,
    ));

    Self {
      log: TxnLog::new(),
      log2: TxnLog::new(),
      arena,
      builtin_types,
      global_scope,
      child_scope,
      a,
      b,
      c,
      g,
    }
  }
}

impl Default for TxnLogFixture {
  fn default() -> Self {
    Self::new()
  }
}
