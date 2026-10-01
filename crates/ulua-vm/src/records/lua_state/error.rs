use super::LuaState;
use crate::functions::{lua_l_argerror_l::lua_l_argerror_l, lua_l_typeerror_l::lua_l_typeerror_l};

impl LuaState {
  #[inline(always)]
  pub fn type_error(&mut self, narg: i32, tname: &str) -> ! {
    // SAFETY: `self.as_mut_ptr()` 为存活 LuaState 有效指针，被调方 `# Safety` 其余前提由调用方按文档保证。
    unsafe { lua_l_typeerror_l(self.as_mut_ptr(), narg, tname) }
  }

  #[inline(always)]
  pub fn arg_error(&mut self, narg: i32, extramsg: &str) -> ! {
    // SAFETY: `self.as_mut_ptr()` 为存活 LuaState 有效指针，被调方 `# Safety` 其余前提由调用方按文档保证。
    unsafe { lua_l_argerror_l(self.as_mut_ptr(), narg, extramsg) }
  }

  #[inline(always)]
  pub fn arg_check(&mut self, cond: bool, narg: i32, extramsg: &str) {
    if !cond {
      self.arg_error(narg, extramsg);
    }
  }

  #[inline(always)]
  pub fn arg_expected(&mut self, cond: bool, narg: i32, tname: &str) {
    if !cond {
      self.type_error(narg, tname);
    }
  }
}
