use ulua_vm::{
  functions::luau_load::luau_load, macros::lua_isfunction::lua_isfunction, records::proto::Proto,
};

use crate::common::records::feedback_vector_fixture::FeedbackVectorFixture;
impl<'a> FeedbackVectorFixture<'a> {
  pub fn load(&mut self) -> *mut Proto {
    let bytecode = self.bcb.get_bytecode();
    let l = self.lua_state();

    // Safety: 测试并行运行下本资源由本用例独占、无共享与并发访问；`self` 在本用例作用域内取得/构造（&mut 再借用、Box::into_raw 或 as_ptr 布线），至本行使用前不释放，故满足被调 unsafe 例程与 C ABI 的前置条件。
    unsafe {
      let res = luau_load(l, "=FeedbackVectorTest", bytecode, 0);

      assert!(res == 0 && lua_isfunction!(l, -1));

      let top = (*(*l).top.sub(1)).as_closure();

      top.inner.l.p
    }
  }
}
