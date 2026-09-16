use alloc::string::String;

use crate::{
  functions::to_string_to_string_alt_c::to_string_type_id,
  records::{
    error_converter::ErrorConverter,
    explicit_function_annotation_recommended::ExplicitFunctionAnnotationRecommended,
  },
};

impl ErrorConverter {
  pub fn operator_call_8(&self, e: &ExplicitFunctionAnnotationRecommended) -> String {
    let recommended_return = to_string_type_id(e.recommended_return());
    let mut arg_annotations = String::new();

    for (arg, type_id) in e.recommended_args() {
      if !arg_annotations.is_empty() {
        arg_annotations.push_str(", ");
      }
      arg_annotations.push_str(arg);
      arg_annotations.push_str(": ");
      arg_annotations.push_str(&to_string_type_id(*type_id));
    }

    if arg_annotations.is_empty() {
      String::from("Consider annotating the return with ") + &recommended_return
    } else {
      String::from("Consider placing the following annotations on the arguments: ")
        + &arg_annotations
        + " or instead annotating the return as "
        + &recommended_return
    }
  }
}
