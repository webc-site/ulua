extern crate alloc;

use alloc::string::String;

use crate::{
  FFlag::{LuauIntegerType2, LuauUdtfTypeIsSubtypeOf},
  functions::embedded_builtin_raw_const::embedded_builtin_raw_const,
};

pub fn get_type_function_definition_source() -> String {
  let mut result = String::new();

  if LuauUdtfTypeIsSubtypeOf.get() {
    result.push_str(embedded_builtin_raw_const(
      "kBuiltinDefinitionTypeMethodSrc",
    ));
  } else if LuauIntegerType2.get() {
    result.push_str(embedded_builtin_raw_const(
      "kBuiltinDefinitionTypeMethodSrc_DEPRECATED",
    ));
  } else {
    result.push_str(embedded_builtin_raw_const(
      "kBuiltinDefinitionTypeMethodSrc_NOINTEGER",
    ));
  }

  if LuauIntegerType2.get() {
    result.push_str(embedded_builtin_raw_const("kBuiltinDefinitionTypesLibSrc"));
  } else {
    result.push_str(embedded_builtin_raw_const(
      "kBuiltinDefinitionTypesLibSrc_NOINTEGER",
    ));
  }

  result
}
