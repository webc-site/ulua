use alloc::string::String;

use crate::{
  functions::{
    follow_type::follow_type_id, get_type_alt_j::get_type_id,
    to_string_to_string_alt_c::to_string_type_id,
  },
  records::{
    error_converter::ErrorConverter, extern_type::ExternType,
    unknown_prop_but_found_like_prop::UnknownPropButFoundLikeProp,
  },
};

impl ErrorConverter {
  pub fn operator_call_42(&self, e: &UnknownPropButFoundLikeProp) -> String {
    let mut candidates_suggestion = String::from("Did you mean ");
    if e.candidates().len() != 1 {
      candidates_suggestion.push_str("one of ");
    }

    let mut first = true;
    for name in e.candidates() {
      if first {
        first = false;
      } else {
        candidates_suggestion.push_str(", ");
      }
      candidates_suggestion.push('\'');
      candidates_suggestion.push_str(name);
      candidates_suggestion.push('\'');
    }

    let mut s = String::from("Key '");
    s.push_str(e.key());
    s.push_str("' not found in ");

    let t = follow_type_id(e.table());
    if get_type_id::<ExternType>(t).is_none() {
      s.push_str("table");
    } else {
      s.push_str("external type");
    }

    s.push_str(" '");
    s.push_str(&to_string_type_id(e.table()));
    s.push_str("'.  ");
    s.push_str(&candidates_suggestion);
    s.push('?');

    s
  }
}
