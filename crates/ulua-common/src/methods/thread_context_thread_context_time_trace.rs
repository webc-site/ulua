#[cfg(feature = "luau_enable_time_trace")]
use crate::functions::create_thread::create_thread;
use crate::{
  functions::get_global_context::get_global_context, records::thread_context::ThreadContext,
};

impl ThreadContext {
  pub fn new() -> Self {
    #[cfg(not(feature = "luau_enable_time_trace"))]
    {
      ThreadContext {
        global_context: get_global_context(),
        thread_id: 0,
        events: Vec::new(),
        data: Vec::new(),
      }
    }

    #[cfg(feature = "luau_enable_time_trace")]
    {
      // C++: `ThreadContext() : globalContext(getGlobalContext()) { threadId = createThread(*globalContext, this); }`
      let global_context = get_global_context();
      let mut result = ThreadContext {
        global_context: global_context.clone(),
        thread_id: 0,
        events: Vec::new(),
        data: Vec::new(),
      };

      // `createThread(*globalContext, this)` — 注册身份是返回的 `thread_id`
      // （指针在按值返回后不稳定，见 `create_thread` 的偏差说明）；单例
      // 上下文自身以 Mutex 保护，共享 `&` 即可。
      result.thread_id = create_thread(&global_context);

      result
    }
  }
}

impl Default for ThreadContext {
  fn default() -> Self {
    Self::new()
  }
}
