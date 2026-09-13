use crate::{
  records::{scope::Scope, symbol::Symbol},
  type_aliases::type_id::TypeId,
};

impl Scope {
  pub fn lookup_symbol(&self, sym: Symbol) -> Option<TypeId> {
    let mutable_self = self as *const Scope as *mut Scope;
    let r = unsafe { (*mutable_self).lookup_ex_symbol(sym) };

    if let Some((binding, _)) = r {
      Some(unsafe { (*binding).type_id })
    } else {
      None
    }
  }
}
