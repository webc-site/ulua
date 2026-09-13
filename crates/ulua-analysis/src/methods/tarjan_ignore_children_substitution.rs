use crate::{records::tarjan::Tarjan, type_aliases::type_id::TypeId};

impl Tarjan {
  /// C++ `Tarjan::ignoreChildren(TypeId)` (`Substitution.cpp:551-554`).
  ///
  /// The base returns `false`; concrete subclasses override it. Dispatches to
  /// the subclass override via the installed
  /// [`SubstitutionVtable`](crate::records::tarjan::SubstitutionVtable) when
  /// present, else the base-class default.
  pub fn ignore_children_type_id(&mut self, ty: TypeId) -> bool {
    let owner = self.vtable.owner;
    match self.vtable.ignore_children_ty {
      Some(f) => f(owner, ty),
      None => false,
    }
  }
}
