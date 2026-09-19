use alloc::format;

use crate::{
  functions::{
    get_mutable_type::get_mutable_type_id, persist_type::persist,
    to_string_symbol::to_string_symbol,
  },
  records::table_type::TableType,
  type_aliases::scope_ptr_type::ScopePtr,
};

pub fn finalize_global_bindings(scope: ScopePtr) {
  for (symbol, binding) in scope.bindings.iter() {
    persist(binding.type_id);

    if let Some(ttv) = get_mutable_type_id::<TableType>(binding.type_id)
      && ttv.name.is_none()
    {
      let name = format!("typeof({})", to_string_symbol(symbol));
      ttv.name = Some(name);
    }
  }
}
