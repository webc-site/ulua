use crate::records::{property_type::Property, type_stringifier::TypeStringifier};

impl TypeStringifier {
  pub fn type_stringifier_new_stringify(&mut self, name: &str, prop: &Property) {
    let mut comma = false;

    if prop.is_shared() {
      self.emit_key(name);
      if let Some(read_ty) = prop.read_ty {
        unsafe { self.stringify_type_id(read_ty) };
      }
      return;
    }

    if let Some(read_ty) = prop.read_ty {
      let state = unsafe { &mut *self.state };
      state.emit("read ");
      self.emit_key(name);
      unsafe { self.stringify_type_id(read_ty) };
      comma = true;
    }

    if let Some(write_ty) = prop.write_ty {
      if comma {
        let state = unsafe { &mut *self.state };
        state.emit(",");
        state.newline();
      }

      let state = unsafe { &mut *self.state };
      state.emit("write ");
      self.emit_key(name);
      unsafe { self.stringify_type_id(write_ty) };
    }
  }
}
