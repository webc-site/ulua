use crate::records::dynamic_property_lookup_on_extern_types_unsafe::DynamicPropertyLookupOnExternTypesUnsafe;

impl DynamicPropertyLookupOnExternTypesUnsafe {
  #[inline]
  pub fn operator_eq(&self, rhs: &DynamicPropertyLookupOnExternTypesUnsafe) -> bool {
    self.ty == rhs.ty
  }
}
