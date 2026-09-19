use alloc::string::String;

use crate::{
  functions::to_string_to_string_alt_c::to_string_type_id,
  records::{
    error_converter::ErrorConverter,
    missing_properties::{Context, MissingProperties},
  },
};
impl ErrorConverter {
  pub fn operator_call_31(&self, e: &MissingProperties) -> String {
    let sub_type_str = to_string_type_id(e.sub_type());
    let super_type_str = to_string_type_id(e.super_type());

    let mut s = String::from("Table type '");
    s.push_str(&sub_type_str);
    s.push_str("' not compatible with type '");
    s.push_str(&super_type_str);
    s.push_str("' because the former");

    match e.context() {
      Context::Missing => {
        s.push_str(" is missing field");
      }
      Context::Extra => {
        s.push_str(" has extra field");
      }
    }

    if e.properties().len() > 1 {
      s.push('s');
    }

    s.push(' ');

    let properties = e.properties();
    let last = properties.len().saturating_sub(1);
    for (i, prop) in properties.iter().enumerate() {
      if i > 0 {
        s.push_str(", ");
      }

      if i > 0 && i == last {
        s.push_str("and ");
      }

      s.push('\'');
      s.push_str(prop);
      s.push('\'');
    }

    s
  }
}
