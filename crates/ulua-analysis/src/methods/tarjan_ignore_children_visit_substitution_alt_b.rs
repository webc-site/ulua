use crate::{records::tarjan::Tarjan, type_aliases::type_pack_id::TypePackId};

impl Tarjan {
  /// C++ `Tarjan::ignoreChildrenVisit(TypePackId)` (`Substitution.cpp:567-570`).
  ///
  /// The base forwards to `ignoreChildren`; a few subclasses override the
  /// "visit" variant independently. Dispatches to the subclass override via
  /// the installed
  /// [`SubstitutionVtable`](crate::records::tarjan::SubstitutionVtable) when
  /// present, else falls back to `ignoreChildren` (matching the base default,
  /// and fixing the previous self-recursive stub).
  pub fn ignore_children_visit_type_pack_id(&mut self, tp: TypePackId) -> bool {
    let owner = self.vtable.owner;
    match self.vtable.ignore_children_visit_tp {
      Some(f) => f(owner, tp),
      None => self.ignore_children_type_pack_id(tp),
    }
  }
}
