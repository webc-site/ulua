pub fn is_valid_class_metamethod(name: &str) -> bool {
  matches!(
    name,
    "__call"
      | "__concat"
      | "__unm"
      | "__add"
      | "__sub"
      | "__mul"
      | "__div"
      | "__mod"
      | "__pow"
      | "__tostring"
      | "__eq"
      | "__lt"
      | "__le"
      | "__iter"
      | "__len"
      | "__idiv"
  )
}
