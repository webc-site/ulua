use core::any::Any;

/// 保留 `Box<dyn Any + Send>`：入参即 `catch_unwind` 返回的错误形态（载荷类型
/// 集合运行期开放），测试断言按 `&'static str` / `String` 逐一下探。
pub fn assert_code_allocator_testing_panic(payload: Box<dyn Any + Send>) {
  if let Some(message) = payload.downcast_ref::<&'static str>() {
    assert_eq!(*message, "testing");
  } else if let Some(message) = payload.downcast_ref::<String>() {
    assert_eq!(message, "testing");
  } else {
    panic!("unexpected panic payload");
  }
}
