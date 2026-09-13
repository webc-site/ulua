use core::ptr::addr_of_mut;

use crate::type_aliases::assert_handler::AssertHandler;

pub fn assert_handler() -> &'static mut AssertHandler {
  static mut HANDLER: AssertHandler = None;

  // 安全性说明：在原 C++ 代码中，这是函数局部静态变量。
  // 虽然 C++11 保证静态变量初始化的线程安全，但不保证对对象本身的并发访问安全。
  // Luau 中该全局处理器通常在启动时配置一次或在单线程测试上下文中使用。
  unsafe { &mut *addr_of_mut!(HANDLER) }
}
