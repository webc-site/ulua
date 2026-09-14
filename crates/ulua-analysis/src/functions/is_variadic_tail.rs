use crate::{
  records::{
    generic_type_pack::GenericTypePack, txn_log::TxnLog, variadic_type_pack::VariadicTypePack,
  },
  type_aliases::type_pack_id::TypePackId,
};

pub fn is_variadic_tail(tp: TypePackId, log: &TxnLog, include_hidden_variadics: bool) -> bool {
  if log.txn_log_is::<GenericTypePack, TypePackId>(tp) {
    return true;
  }

  if let Some(vtp) = unsafe { log.txn_log_get::<VariadicTypePack, TypePackId>(tp).as_ref() }
    && (include_hidden_variadics || !vtp.hidden)
  {
    return true;
  }

  false
}
