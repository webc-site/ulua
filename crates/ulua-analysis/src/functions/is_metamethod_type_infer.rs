use crate::type_aliases::name_type::Name;

pub fn is_metamethod(name: &Name) -> bool {
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
