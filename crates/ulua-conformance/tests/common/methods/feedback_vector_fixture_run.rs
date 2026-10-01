use crate::common::{
  functions::safe_api::resume, records::feedback_vector_fixture::FeedbackVectorFixture,
};
impl<'a> FeedbackVectorFixture<'a> {
  pub fn run(&mut self) {
    let l = self.lua_state();

    // Safety: `global` 为该状态挂接的全局状态（结构不变量），仅写回调表一个槽位。
    unsafe {
      (*(*l).global).ecb.inlinefunction = self.on_inline;
    }

    let status = resume(l, None, 0);
    assert_eq!(status, 0);
  }
}
