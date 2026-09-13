use crate::records::bytecode_type_info::BytecodeTypeInfo;

const LBC_TYPE_ANY: u8 = 15;

pub fn has_typed_parameters(type_info: &BytecodeTypeInfo) -> bool {
  for &el in &type_info.argument_types {
    if el != LBC_TYPE_ANY {
      return true;
    }
  }
  false
}
