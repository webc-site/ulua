#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum TypeKind {
  Unknown,
  /// primitive type supported by VM - boolean/userdata/etc. No differentiation between types of userdata.
  Primitive,
  /// TODO: deprecated and not set, but read in 'visit'
  Vector,
  /// custom userdata type
  Userdata,
}
