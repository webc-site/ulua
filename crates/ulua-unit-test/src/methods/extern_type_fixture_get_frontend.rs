use ulua_analysis::records::frontend::Frontend;

use crate::{
  functions::register_extern_type_fixture_types::register_extern_type_fixture_types,
  records::extern_type_fixture::ExternTypeFixture,
};

impl ExternTypeFixture {
  pub fn get_frontend(&mut self) -> &mut Frontend {
    if self.vector2_instance_type.is_null() {
      // (a) 类 `as *mut Frontend` + `unsafe { &mut *ptr }` 绕道已消除：
      // base.get_frontend() 即 `&mut Frontend`，直传 register 独占登记
      // （cpp Frontend& 同形），句尾自然归还。
      let (vector2_type, vector2_instance_type) =
        register_extern_type_fixture_types(self.base.get_frontend());
      self.vector2_type = vector2_type;
      self.vector2_instance_type = vector2_instance_type;
    }

    self.base.get_frontend()
  }
}
