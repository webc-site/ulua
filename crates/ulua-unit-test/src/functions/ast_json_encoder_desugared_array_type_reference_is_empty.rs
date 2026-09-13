use ulua_common::FFlag;
pub fn desugared_array_type_reference_is_empty<'a>(enabled: &'a str, disabled: &'a str) -> &'a str {
  if FFlag::DesugaredArrayTypeReferenceIsEmpty.get() {
    enabled
  } else {
    disabled
  }
}
