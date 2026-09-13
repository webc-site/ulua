use ulua_analysis::{
  records::{
    frontend::Frontend, internal_error_reporter::InternalErrorReporter,
    type_check_limits::TypeCheckLimits,
  },
  type_aliases::type_id::TypeId,
};

use crate::records::fixture::Fixture;

impl Fixture {
  pub fn parse_type(&mut self, src: &str) -> TypeId {
    self
      .name_table
      .rebind_allocator(&mut self.allocator as *mut _);
    self.get_frontend();

    let frontend = self
      .frontend
      .as_mut()
      .expect("frontend should be initialized");
    let frontend_ptr = frontend as *mut Frontend;

    unsafe {
      let ice_handler = &mut (*frontend_ptr).ice_handler as *mut InternalErrorReporter;
      (*frontend_ptr).parse_type(
        &mut self.allocator,
        &mut self.name_table,
        &mut *ice_handler,
        TypeCheckLimits::default(),
        &mut self.arena,
        src,
      )
    }
  }
}
