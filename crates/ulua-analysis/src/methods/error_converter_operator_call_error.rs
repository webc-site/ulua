use alloc::string::String;

use ulua_common::FInt;

use crate::{
  enums::context_error::Context,
  functions::{
    follow_type::follow_type_id, get_definition_module_name::get_definition_module_name,
    get_type_alt_j::get_type_id,
    to_string_error_alt_k::to_string_type_error_type_error_to_string_options,
    to_string_to_string_alt_c::to_string_type_id,
  },
  records::{
    error_converter::ErrorConverter, file_resolver::FileResolver, never_type::NeverType,
    type_error_to_string_options::TypeErrorToStringOptions, type_mismatch::TypeMismatch,
  },
};
impl ErrorConverter {
  pub fn operator_call_40(&self, tm: &TypeMismatch) -> String {
    let given_type_name = to_string_type_id(tm.given_type);
    let wanted_type_name = to_string_type_id(tm.wanted_type);

    let mut result = String::new();

    let quote = |s: &str| -> String { format!("'{}'", s) };

    let construct_error_message = |given_type: String,
                                   wanted_type: String,
                                   given_module: Option<String>,
                                   wanted_module: Option<String>|
     -> String {
      let given = if let Some(ref gm) = given_module {
        format!("{} from {}", quote(&given_type), quote(gm))
      } else {
        quote(&given_type)
      };

      let wanted = if let Some(ref wm) = wanted_module {
        format!("{} from {}", quote(&wanted_type), quote(wm))
      } else {
        quote(&wanted_type)
      };

      let luau_indent_type_mismatch_max_type_length =
        FInt::LuauIndentTypeMismatchMaxTypeLength.get() as usize;

      let follow_wanted = follow_type_id(tm.wanted_type);
      let wanted_never = get_type_id::<NeverType>(follow_wanted);
      if !wanted_never.is_none() {
        if given_type.len() <= luau_indent_type_mismatch_max_type_length {
          return format!("Expected this to be unreachable, but got {}", given);
        } else {
          return format!("Expected this to be unreachable, but got\n\t{}", given);
        }
      }

      if tm.context == Context::InvariantContext {
        if given_type.len() <= luau_indent_type_mismatch_max_type_length
          || wanted_type.len() <= luau_indent_type_mismatch_max_type_length
        {
          return format!("Expected this to be exactly {}, but got {}", wanted, given);
        } else {
          return format!(
            "Expected this to be exactly\n\t{}\nbut got\n\t{}",
            wanted, given
          );
        }
      }

      if given_type.len() <= luau_indent_type_mismatch_max_type_length
        || wanted_type.len() <= luau_indent_type_mismatch_max_type_length
      {
        format!("Expected this to be {}, but got {}", wanted, given)
      } else {
        format!("Expected this to be\n\t{}\nbut got\n\t{}", wanted, given)
      }
    };

    if given_type_name == wanted_type_name {
      let given_definition_module = get_definition_module_name(tm.given_type);
      let wanted_definition_module = get_definition_module_name(tm.wanted_type);

      if let (Some(ref given_mod), Some(ref wanted_mod)) =
        (given_definition_module, wanted_definition_module)
      {
        if !self.file_resolver.is_null() {
          let given_module_name =
            unsafe { FileResolver::get_human_readable_module_name(self.file_resolver, given_mod) };
          let wanted_module_name =
            unsafe { FileResolver::get_human_readable_module_name(self.file_resolver, wanted_mod) };

          result = construct_error_message(
            given_type_name.clone(),
            wanted_type_name.clone(),
            Some(given_module_name),
            Some(wanted_module_name),
          );
        } else {
          result = construct_error_message(
            given_type_name.clone(),
            wanted_type_name.clone(),
            Some(given_mod.clone()),
            Some(wanted_mod.clone()),
          );
        }
      }
    }

    if result.is_empty() {
      result = construct_error_message(
        given_type_name.clone(),
        wanted_type_name.clone(),
        None,
        None,
      );
    }

    if let Some(ref err) = tm.error {
      result.push_str("\ncaused by:\n  ");

      if !tm.reason.is_empty() {
        result.push_str(&tm.reason);
        result.push('\n');
      }

      let opts = TypeErrorToStringOptions {
        file_resolver: self.file_resolver,
      };
      result.push_str(&to_string_type_error_type_error_to_string_options(
        err, opts,
      ));
    } else if !tm.reason.is_empty() {
      result.push_str("; ");
      result.push_str(&tm.reason);
    }

    result
  }
}
