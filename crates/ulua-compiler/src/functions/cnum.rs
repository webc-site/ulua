use crate::records::constant::Constant;

pub(crate) fn cnum(v: f64) -> Constant {
  Constant::Number(v)
}
