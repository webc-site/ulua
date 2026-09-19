use ulua_common::functions::escape::escape;

use crate::{
  records::{singleton_type::SingletonType, type_stringifier::TypeStringifier},
  type_aliases::{singleton_variant::SingletonVariant, type_id::TypeId},
};

impl TypeStringifier {
  pub fn operator_call_18(&mut self, _ty: TypeId, stv: &SingletonType) {
    unsafe {
      match stv.variant {
        SingletonVariant::V0(ref bs) => {
          if bs.value {
            (*self.state).emit_string("true");
          } else {
            (*self.state).emit_string("false");
          }
        }
        SingletonVariant::V1(ref ss) => {
          (*self.state).emit_string("\"");
          let escaped = escape(&ss.value, false);
          (*self.state).emit_string(&escaped);
          (*self.state).emit_string("\"");
        }
      }
    }
  }
}
