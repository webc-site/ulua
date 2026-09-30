use ulua_analysis::{
  records::{builtin_types::BuiltinTypes, txn_log::TxnLog, type_arena::TypeArena},
  type_aliases::{scope_ptr_type::ScopePtr, type_id::TypeId},
};

#[derive(Debug)]
pub struct TxnLogFixture {
  pub log: TxnLog,
  pub log2: TxnLog,
  pub arena: TypeArena,
  pub builtin_types: BuiltinTypes,
  pub global_scope: ScopePtr,
  pub child_scope: ScopePtr,
  pub a: TypeId,
  pub b: TypeId,
  pub c: TypeId,
  pub g: TypeId,
}
