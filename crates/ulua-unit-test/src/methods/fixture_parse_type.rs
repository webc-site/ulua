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
    // `Box<Frontend>` → 宿主借用重借用：`parse_type` 取整器 `&mut self`，与
    // `ice_handler` 字段的 `&mut` 借用无法在 Box 上拆分，故沿用下方同源裸句柄。
    let frontend: &mut Frontend = frontend;
    let frontend_ptr = frontend as *mut Frontend;

    // Safety: frontend_ptr 由上方 Box 堆地址（`new_boxed` 钉址、非空稳定）借出的
    // `&mut Frontend` 转铸；ice_handler 取同 Frontend 内字段地址（&mut 合法
    // addr-of）；self.allocator/name_table/arena 为 fixture 独立字段，借用区域互异，与 cpp f->parseType(&allocator,...) 一致。
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
