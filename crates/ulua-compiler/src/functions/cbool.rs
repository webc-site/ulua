use crate::records::constant::Constant;

pub(crate) fn cbool(v: bool) -> Constant {
  Constant::Boolean(v)
}
