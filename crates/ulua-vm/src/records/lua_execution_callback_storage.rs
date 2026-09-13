// C++ lstate.h: LUA_EXECUTION_CALLBACK_STORAGE
pub const LUA_EXECUTION_CALLBACK_STORAGE: usize = 512;

#[derive(Debug, Clone, Copy)]
#[repr(C, align(16))]
pub struct LuaExecutionCallbackStorage {
  pub bytes: [u8; LUA_EXECUTION_CALLBACK_STORAGE],
}

impl LuaExecutionCallbackStorage {
  pub fn as_mut_ptr(&mut self) -> *mut u8 {
    self.bytes.as_mut_ptr()
  }
}
