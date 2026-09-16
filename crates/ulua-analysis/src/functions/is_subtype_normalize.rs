use crate::{
  functions::is_subtype_normalized_string::is_subtype_normalized_string,
  records::normalized_string_type::NormalizedStringType,
};

pub fn is_subtype_normalized_string_type_normalized_string_type(
  sub_str: &NormalizedStringType,
  super_str: &NormalizedStringType,
) -> bool {
  is_subtype_normalized_string(sub_str, super_str)
}
