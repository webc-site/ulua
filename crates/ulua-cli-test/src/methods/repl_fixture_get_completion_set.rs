use alloc::string::ToString;

use ulua_vm::functions::lua_gettop::lua_gettop;

use crate::{
  functions::complete_indexer::complete_indexer,
  records::{completion::Completion, repl_fixture::ReplFixture},
  type_aliases::completion_set::CompletionSet,
};

impl ReplFixture {
  pub fn get_completion_set(&mut self, input_prefix: &str) -> CompletionSet {
    let mut result = CompletionSet::default();

    let top = unsafe { lua_gettop(self.l as *mut _) };

    let mut callback = |completion: &str, display: &str| {
      result.insert(Completion {
        completion: completion.to_string(),
        display: display.to_string(),
      });
    };

    complete_indexer(self.l as *mut _, input_prefix, &mut callback);

    debug_assert!(top == unsafe { lua_gettop(self.l as *mut _) });

    result
  }
}
