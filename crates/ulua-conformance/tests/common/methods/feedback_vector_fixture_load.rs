use ulua_vm::records::proto::Proto;

use crate::common::{
  functions::safe_api::{load_bytes, state_mut},
  records::feedback_vector_fixture::FeedbackVectorFixture,
};
impl<'a> FeedbackVectorFixture<'a> {
  pub fn load(&mut self) -> *mut Proto {
    let bytecode = self.bcb.get_bytecode();
    let l = self.lua_state();

    let res = load_bytes(l, "=FeedbackVectorTest", bytecode, 0);

    assert!(res == 0 && state_mut(l).is_function(-1));

    // Safety: `top` 为栈顶闭包槽（上一行断言为函数）；`sub(1)` 即 top-1 槽位，
    // as_closure 取其闭包指针（VM 内部结构边界，feedback_vector_api 门面契约）。
    unsafe { (*(*l).top.sub(1)).as_closure().inner.l.p }
  }
}
