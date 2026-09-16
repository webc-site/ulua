use crate::{
  functions::as_mutable_type_pack::as_mutable_type_pack_id,
  records::type_cloner::TypeCloner,
  type_aliases::{
    bound_type_pack::BoundTypePack, error_type_pack::ErrorTypePack, type_pack_id::TypePackId,
    type_pack_variant::TypePackVariant,
  },
};

impl TypeCloner {
  /// `void TypeCloner::cloneChildren(TypePackId tp)`. Reference: `Clone.cpp:236-245`.
  ///
  /// C++: `visit([&](auto&& t){ return cloneChildren(&t); }, asMutable(tp)->ty);`
  /// — dispatches over the `TypePackVariant` to the matching concrete overload.
  pub fn clone_children_type_pack_id(&mut self, tp: TypePackId) {
    let tv: &mut TypePackVariant = unsafe { &mut (*as_mutable_type_pack_id(tp)).ty };
    match tv {
      // `Bound<TypePackId>` is repr-transparent over its single field.
      TypePackVariant::Bound(inner) => unsafe {
        self.clone_children_bound_type_pack(inner as *mut TypePackId as *mut BoundTypePack)
      },
      TypePackVariant::Error(inner) => {
        self.clone_children_error_type_pack(inner as *mut ErrorTypePack)
      }
      TypePackVariant::Free(inner) => self.clone_children_free_type_pack(inner),
      TypePackVariant::Generic(inner) => self.clone_children_generic_type_pack(inner),
      TypePackVariant::TypePack(inner) => unsafe { self.clone_children_type_pack(inner) },
      TypePackVariant::Variadic(inner) => unsafe { self.clone_children_variadic_type_pack(inner) },
      TypePackVariant::Blocked(inner) => self.clone_children_blocked_type_pack(inner),
      TypePackVariant::TypeFunctionInstance(inner) => unsafe {
        self.clone_children_type_function_instance_type_pack(inner)
      },
    }
  }
}
