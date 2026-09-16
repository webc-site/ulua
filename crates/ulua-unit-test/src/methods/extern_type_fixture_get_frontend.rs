use ulua_analysis::records::frontend::Frontend;

use crate::{
  functions::register_extern_type_fixture_types::register_extern_type_fixture_types,
  records::extern_type_fixture::ExternTypeFixture,
};

impl ExternTypeFixture {
  pub fn get_frontend(&mut self) -> &mut Frontend {
    let frontend_ptr = self.base.get_frontend() as *mut Frontend;

    if self.vector2_instance_type.is_null() {
      let (vector2_type, vector2_instance_type) =
        register_extern_type_fixture_types(unsafe { &mut *frontend_ptr });
      self.vector2_type = vector2_type;
      self.vector2_instance_type = vector2_instance_type;
    }

    unsafe { &mut *frontend_ptr }
  }
}
