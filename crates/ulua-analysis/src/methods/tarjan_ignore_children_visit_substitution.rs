use crate::{records::tarjan::Tarjan, type_aliases::type_id::TypeId};

impl Tarjan {
  /// C++ `Tarjan::ignoreChildrenVisit(TypeId)` (`Substitution.cpp:562-565`).
  ///
  /// The base forwards to `ignoreChildren`; a few subclasses override the
  /// "visit" variant independently. Dispatches to the subclass override via
  /// the installed
  /// [`SubstitutionVtable`](crate::records::tarjan::SubstitutionVtable) when
  /// present, else falls back to `ignoreChildren` (matching the base default,
  /// and fixing the previous self-recursive stub).
  pub fn ignore_children_visit_type_id(&mut self, ty: TypeId) -> bool {
    let owner = self.vtable.owner;
    match self.vtable.ignore_children_visit_ty {
      Some(f) => f(owner, ty),
      None => self.ignore_children_type_id(ty),
    }
  }
}
