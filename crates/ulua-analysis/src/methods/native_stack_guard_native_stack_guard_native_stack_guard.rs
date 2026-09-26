use ulua_common::fflag;

use crate::records::native_stack_guard::NativeStackGuard;

impl NativeStackGuard {
  pub fn native_stack_guard(&mut self) {
    self.high = 0;
    self.low = 0;

    if fflag::LuauUseNativeStackGuard.get() {
      // wasm 无原生线程栈，守卫保持 low=high=0（等效恒通过）
      #[cfg(any(target_os = "windows", target_os = "macos", target_os = "linux"))]
      self.query_native();
    }
  }

  /// 查询当前线程的原生栈边界（对应 C++ NativeTypeArena 的平台分支）。
  #[cfg(any(target_os = "windows", target_os = "macos", target_os = "linux"))]
  fn query_native(&mut self) {
    #[cfg(target_os = "windows")]
    {
      unsafe extern "system" {
        fn GetCurrentThreadStackLimits(low_limit: *mut usize, high_limit: *mut usize);
      }

      unsafe {
        GetCurrentThreadStackLimits(&mut self.low, &mut self.high);
      }
    }

    #[cfg(target_os = "macos")]
    {
      unsafe extern "C" {
        fn pthread_self() -> *mut ();
        fn pthread_get_stackaddr_np(thread: *mut ()) -> *mut ();
        fn pthread_get_stacksize_np(thread: *mut ()) -> usize;
      }

      unsafe {
        let thread = pthread_self();
        let addr = pthread_get_stackaddr_np(thread) as usize;
        let size = pthread_get_stacksize_np(thread);
        self.low = addr.saturating_sub(size);
        self.high = addr;
      }
    }

    // glibc/musl Linux: query the current thread's stack bounds via
    // pthread_getattr_np + pthread_attr_getstack. Unlike macOS
    // (pthread_get_stackaddr_np returns the high end), pthread_attr_getstack
    // returns the LOW address of the stack region plus its size, so
    // low = base, high = base + size. Without this branch `low`/`high` stayed 0
    // on Linux, so the guard's `is_ok` always saw "plenty of stack" and never
    // tripped — runtime_limits_native_stack_guard_prevents_stack_overflows and
    // type_function_user_udtf_areequal_stack_overflow_on_deep_types expected an
    // InternalCompilerError that was therefore never thrown.
    #[cfg(target_os = "linux")]
    {
      unsafe extern "C" {
        fn pthread_self() -> *mut ();
        fn pthread_getattr_np(thread: *mut (), attr: *mut ()) -> i32;
        fn pthread_attr_getstack(
          attr: *const (),
          stackaddr: *mut *mut (),
          stacksize: *mut usize,
        ) -> i32;
        fn pthread_attr_destroy(attr: *mut ()) -> i32;
      }

      // Opaque `pthread_attr_t` storage. glibc's is 56 bytes on x86_64 and
      // musl's is smaller; over-allocate and 16-align to cover both.
      #[repr(C, align(16))]
      struct AttrBuf([u8; 128]);

      unsafe {
        let mut attr = AttrBuf([0u8; 128]);
        let attr_ptr = &mut attr as *mut AttrBuf as *mut ();
        if pthread_getattr_np(pthread_self(), attr_ptr) == 0 {
          let mut stackaddr: *mut () = core::ptr::null_mut();
          let mut stacksize: usize = 0;
          if pthread_attr_getstack(attr_ptr, &mut stackaddr, &mut stacksize) == 0 {
            self.low = stackaddr as usize;
            self.high = (stackaddr as usize).saturating_add(stacksize);
          }
          pthread_attr_destroy(attr_ptr);
        }
      }
    }
  }
}
