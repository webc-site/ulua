extern crate alloc;

use alloc::string::String;

use crate::{
  FFlag::{
    DebugLuauUserDefinedClasses, LuauAllowGlobalDeclarationToBeCalledClass, LuauIntegerLibrary,
    LuauIntegerType2,
  },
  functions::embedded_builtin_raw_const::embedded_builtin_raw_const,
};

pub fn get_builtin_definition_source() -> String {
  let mut result = String::from(embedded_builtin_raw_const("kBuiltinDefinitionBaseSrc"));

  result.push_str(embedded_builtin_raw_const("kBuiltinDefinitionBit32Src"));
  result.push_str(embedded_builtin_raw_const("kBuiltinDefinitionMathSrc"));
  result.push_str(embedded_builtin_raw_const("kBuiltinDefinitionOsSrc"));
  result.push_str(embedded_builtin_raw_const("kBuiltinDefinitionCoroutineSrc"));
  result.push_str(embedded_builtin_raw_const("kBuiltinDefinitionTableSrc"));
  result.push_str(embedded_builtin_raw_const("kBuiltinDefinitionDebugSrc"));
  result.push_str(embedded_builtin_raw_const("kBuiltinDefinitionUtf8Src"));

  if LuauIntegerType2.get() && LuauIntegerLibrary.get() {
    result.push_str(embedded_builtin_raw_const("kBuiltinDefinitionBufferSrc"));
  } else {
    result.push_str(embedded_builtin_raw_const(
      "kBuiltinDefinitionBufferSrc_NOINTEGER",
    ));
  }

  result.push_str(embedded_builtin_raw_const("kBuiltinDefinitionVectorSrc"));

  if LuauIntegerType2.get() && LuauIntegerLibrary.get() {
    result.push_str(embedded_builtin_raw_const("kBuiltinDefinitionIntegerSrc"));
  }

  if DebugLuauUserDefinedClasses.get() && LuauAllowGlobalDeclarationToBeCalledClass.get() {
    result.push_str(embedded_builtin_raw_const("kBuiltinDefinitionClassSrc"));
  }

  result
}
