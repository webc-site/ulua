use crate::{
  enums::type_constant_folding::Type,
  records::constant::{Constant, ConstantData},
};

pub(crate) fn cbool(v: bool) -> Constant {
  Constant {
    r#type: Type::Boolean,
    string_length: 0,
    data: ConstantData { value_boolean: v },
  }
}
