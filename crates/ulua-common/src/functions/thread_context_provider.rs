use core::ptr::{addr_of_mut, null_mut};

use crate::{
  records::thread_context::ThreadContext,
  type_aliases::thread_context_provider::ThreadContextProvider,
};

pub fn thread_context_provider() -> &'static mut ThreadContextProvider {
  static mut HANDLER: ThreadContextProvider = {
    extern "C-unwind" fn default_provider() -> *mut ThreadContext {
      null_mut()
    }
    default_provider
  };

  // 安全性说明：在原 C++ 代码中，这是函数局部静态变量。
  // 虽然 C++11 保证静态变量初始化的线程安全，但不保证对对象本身的并发访问安全。
  // Luau 中该全局 provider 通常在启动时配置一次或在单线程环境中使用。
  unsafe { &mut *addr_of_mut!(HANDLER) }
}
