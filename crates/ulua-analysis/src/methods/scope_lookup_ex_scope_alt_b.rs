use crate::records::{binding::Binding, scope::Scope, symbol::Symbol};

impl Scope {
  pub fn lookup_ex_symbol(&mut self, sym: Symbol) -> Option<(*mut Binding, *mut Scope)> {
    let mut s: *mut Scope = self as *mut Scope;

    loop {
      let bindings = unsafe { &(*s).bindings };
      if let Some(binding) = bindings.get(&sym) {
        return Some((binding as *const Binding as *mut Binding, s));
      }

      let parent = unsafe { &(*s).parent };
      match parent {
        Some(parent_scope) => {
          s = parent_scope.as_ref() as *const Scope as *mut Scope;
        }
        None => {
          return None;
        }
      }
    }
  }
}
