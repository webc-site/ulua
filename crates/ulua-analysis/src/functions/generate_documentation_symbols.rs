use alloc::string::String;

use crate::{
  functions::{as_mutable_type::as_mutable_type_id, get_mutable_type},
  records::{
    arena_handle::{alias, alias_opt},
    extern_type::ExternType,
    table_type::TableType,
  },
  type_aliases::type_id::TypeId,
};

pub(crate) fn generate_documentation_symbols(ty: TypeId, root_name: String) {
  let Some(ty_ref) = alias_opt(ty) else {
    return;
  };

  if ty_ref.persistent {
    return;
  }

  let mutable_type_ptr = as_mutable_type_id(ty);
  if mutable_type_ptr.is_null() {
    return;
  }

  alias(mutable_type_ptr).documentation_symbol = Some(root_name.clone());

  if let Some(table_type) = get_mutable_type::get_mutable::<TableType>(ty) {
    for (name, prop) in &mut table_type.props {
      let mut n = String::with_capacity(root_name.len() + 1 + name.len());
      n.push_str(&root_name);
      n.push('.');
      n.push_str(name);
      prop.documentation_symbol = Some(n);
    }
  } else if let Some(extern_type) = get_mutable_type::get_mutable::<ExternType>(ty) {
    for (name, prop) in &mut extern_type.props {
      let mut n = String::with_capacity(root_name.len() + 1 + name.len());
      n.push_str(&root_name);
      n.push('.');
      n.push_str(name);
      prop.documentation_symbol = Some(n);
    }
  }
}
