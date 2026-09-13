use crate::{functions::is_metamethod_type_infer::is_metamethod, type_aliases::name_type::Name};

pub fn is_metamethod_mut(name: &Name) -> bool {
  if is_metamethod(name) {
    return true;
  }

  matches!(
    name.as_str(),
    "__index"
      | "__newindex"
      | "__call"
      | "__concat"
      | "__unm"
      | "__add"
      | "__sub"
      | "__mul"
      | "__div"
      | "__mod"
      | "__pow"
      | "__tostring"
      | "__metatable"
      | "__eq"
      | "__lt"
      | "__le"
      | "__mode"
      | "__iter"
      | "__len"
      | "__idiv"
  )
}
