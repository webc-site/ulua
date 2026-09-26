use crate::{functions::metamethod_names::METAMETHODS, type_aliases::name_type::Name};

/// cpp `isMetamethod(const Name&)`（TypeInfer.cpp:194）。
pub fn is_metamethod(name: &Name) -> bool {
  METAMETHODS.contains(&name.as_str())
}
