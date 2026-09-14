use crate::records::thread_context::ThreadContext;

pub type ThreadContextProvider = extern "C-unwind" fn() -> *mut ThreadContext;
