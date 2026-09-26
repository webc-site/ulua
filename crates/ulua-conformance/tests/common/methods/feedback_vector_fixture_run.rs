use core::ptr::null_mut;

use ulua_vm::functions::lua_resume::lua_resume;

use crate::common::records::feedback_vector_fixture::FeedbackVectorFixture;
impl<'a> FeedbackVectorFixture<'a> {
  pub fn run(&mut self) {
    let l = self.lua_state();

    // Safety: 测试并行运行下本资源由本用例独占、无共享与并发访问；`self` 在本用例作用域内取得/构造（&mut 再借用、Box::into_raw 或 as_ptr 布线），至本行使用前不释放，故满足被调 unsafe 例程与 C ABI 的前置条件。
    unsafe {
      (*(*l).global).ecb.inlinefunction = self.on_inline;

      // FFI: c-API 要求 NULL
      let status = lua_resume(l, null_mut(), 0);
      assert_eq!(status, 0);
    }
  }
}
