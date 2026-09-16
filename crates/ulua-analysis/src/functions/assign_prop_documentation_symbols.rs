use crate::type_aliases::props_type::Props;

pub fn assign_prop_documentation_symbols(props: &mut Props, base_name: &str) {
  for (_name, prop) in props.iter_mut() {
    prop.documentation_symbol = Some(format!("{}.{}", base_name, _name));
  }
}
