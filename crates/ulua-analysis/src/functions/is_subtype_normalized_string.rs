use crate::records::normalized_string_type::NormalizedStringType;

pub fn is_subtype_normalized_string(
  sub_str: &NormalizedStringType,
  super_str: &NormalizedStringType,
) -> bool {
  match (sub_str.is_union(), super_str.is_union()) {
    (true, true) => sub_str
      .singletons
      .keys()
      .all(|name| super_str.singletons.contains_key(name)),
    (true, false) => sub_str
      .singletons
      .keys()
      .all(|name| !super_str.singletons.contains_key(name)),
    (false, true) => false,
    (false, false) => super_str
      .singletons
      .keys()
      .all(|name| sub_str.singletons.contains_key(name)),
  }
}
