use crate::{
  enums::type_constant_folding::Type,
  records::constant::{Constant, ConstantData},
};

pub(crate) fn cnum(v: f64) -> Constant {
  Constant {
    r#type: Type::Number,
    string_length: 0,
    data: ConstantData { value_number: v },
  }
}
