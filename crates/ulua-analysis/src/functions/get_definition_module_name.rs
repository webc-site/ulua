use crate::{
  functions::{follow_type, get_type},
  records::{extern_type::ExternType, function_type::FunctionType, table_type::TableType},
  type_aliases::{module_name_type::ModuleName, type_id::TypeId},
};

pub fn get_definition_module_name(type_: TypeId) -> Option<ModuleName> {
  let type_ = follow_type::follow(type_);

  if let Some(ttv) = get_type::get::<TableType>(type_) {
    let def_mod_name = ttv.definition_module_name.clone();
    if !def_mod_name.is_empty() {
      return Some(def_mod_name);
    }
  }

  if let Some(ftv) = get_type::get::<FunctionType>(type_)
    && let Some(def) = ftv.definition.as_ref()
    && let Some(module_name) = def.definition_module_name.clone()
  {
    return Some(module_name);
  }

  if let Some(etv) = get_type::get::<ExternType>(type_) {
    let def_mod_name = etv.definition_module_name.clone();
    if !def_mod_name.is_empty() {
      return Some(def_mod_name);
    }
  }

  None
}
