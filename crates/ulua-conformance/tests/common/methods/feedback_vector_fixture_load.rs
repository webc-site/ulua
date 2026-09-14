use core::ffi::c_char;

use ulua_vm::{
  functions::luau_load::luau_load,
  macros::{clvalue::clvalue, lua_isfunction::lua_isfunction},
  records::Proto::Proto,
};

use crate::common::records::feedback_vector_fixture::FeedbackVectorFixture;
impl FeedbackVectorFixture {
  pub fn load(&mut self) -> *mut Proto {
    let bytecode = self.bcb.get_bytecode();
    let l = self.lua_state();

    unsafe {
      let res = luau_load(
        l,
        c"=FeedbackVectorTest".as_ptr(),
        bytecode.as_ptr() as *const c_char,
        bytecode.len(),
        0,
      );

      assert!(res == 0 && lua_isfunction!(l, -1));

      let top = clvalue!((*l).top.sub(1));

      (*top).inner.l.p
    }
  }
}
