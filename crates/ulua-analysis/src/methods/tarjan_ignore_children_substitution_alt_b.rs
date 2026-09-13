use crate::{records::tarjan::Tarjan, type_aliases::type_pack_id::TypePackId};

impl Tarjan {
  /// C++ `Tarjan::ignoreChildren(TypePackId)` (`Substitution.cpp:556-559`).
  ///
  /// The base returns `false`; concrete subclasses override it. Dispatches to
  /// the subclass override via the installed
  /// [`SubstitutionVtable`](crate::records::tarjan::SubstitutionVtable) when
  /// present, else the base-class default.
  pub fn ignore_children_type_pack_id(&mut self, tp: TypePackId) -> bool {
    let owner = self.vtable.owner;
    match self.vtable.ignore_children_tp {
      Some(f) => f(owner, tp),
      None => false,
    }
  }
}
