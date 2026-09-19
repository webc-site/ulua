use core::ptr::null_mut;

use ulua_vm::functions::lua_resume::lua_resume;

use crate::common::records::feedback_vector_fixture::FeedbackVectorFixture;
impl FeedbackVectorFixture {
  pub fn run(&mut self) {
    let l = self.lua_state();

    unsafe {
      (*(*l).global).ecb.inlinefunction = self.on_inline;

      let status = lua_resume(l, null_mut(), 0);
      assert_eq!(status, 0);
    }
  }
}
