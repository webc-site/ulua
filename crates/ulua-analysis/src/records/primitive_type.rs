use crate::type_aliases::type_id::TypeId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Type {
  NilType,
  Boolean,
  Number,
  Integer,
  String,
  Thread,
  Function,
  Table,
  Buffer,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PrimitiveType {
  pub r#type: Type,
  pub metatable: Option<TypeId>,
}

impl PrimitiveType {
  pub const NIL_TYPE: Type = Type::NilType;
  pub const BOOLEAN: Type = Type::Boolean;
  pub const NUMBER: Type = Type::Number;
  pub const INTEGER: Type = Type::Integer;
  pub const STRING: Type = Type::String;
  pub const THREAD: Type = Type::Thread;
  pub const FUNCTION: Type = Type::Function;
  pub const TABLE: Type = Type::Table;
  pub const BUFFER: Type = Type::Buffer;
}
