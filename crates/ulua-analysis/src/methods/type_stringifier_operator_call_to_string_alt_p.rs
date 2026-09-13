use crate::{
  records::type_stringifier::TypeStringifier,
  type_aliases::{error_type::ErrorType, type_id::TypeId},
};

impl TypeStringifier {
  pub fn operator_call_11(&mut self, _ty: TypeId, tv: &ErrorType) {
    unsafe {
      (*(*self.state).result).error = true;
    }

    if let Some(synthetic) = &tv.synthetic {
      unsafe {
        (*self.state).emit_string("*error-type<");
      }
      unsafe { self.stringify_type_id(*synthetic) };
      unsafe {
        (*self.state).emit_string(">*");
      }
    } else {
      unsafe {
        (*self.state).emit_string("*error-type*");
      }
    }
  }
}
