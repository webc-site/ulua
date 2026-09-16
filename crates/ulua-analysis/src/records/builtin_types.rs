use alloc::boxed::Box;

use crate::{
  records::{builtin_type_functions::BuiltinTypeFunctions, type_arena::TypeArena},
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

#[derive(Debug)]
pub struct BuiltinTypes {
  pub(crate) arena: Box<TypeArena>,
  pub(crate) debug_freeze_arena: bool,
  pub type_functions: Box<BuiltinTypeFunctions>,
  pub nil_type: TypeId,
  pub number_type: TypeId,
  pub integer_type: TypeId,
  pub string_type: TypeId,
  pub boolean_type: TypeId,
  pub thread_type: TypeId,
  pub buffer_type: TypeId,
  pub function_type: TypeId,
  pub extern_type: TypeId,
  pub object_type: TypeId,
  pub class_type: TypeId,
  pub table_type: TypeId,
  pub empty_table_type: TypeId,
  pub true_type: TypeId,
  pub false_type: TypeId,
  pub any_type: TypeId,
  pub unknown_type: TypeId,
  pub never_type: TypeId,
  pub error_type: TypeId,
  pub no_refine_type: TypeId,
  pub falsy_type: TypeId,
  pub truthy_type: TypeId,
  pub not_nil_type: TypeId,
  pub optional_number_type: TypeId,
  pub optional_string_type: TypeId,
  pub empty_type_pack: TypePackId,
  pub any_type_pack: TypePackId,
  pub unknown_type_pack: TypePackId,
  pub never_type_pack: TypePackId,
  pub uninhabitable_type_pack: TypePackId,
  pub error_type_pack: TypePackId,
}

impl Clone for BuiltinTypes {
  fn clone(&self) -> Self {
    panic!("BuiltinTypes is not cloneable in C++ (copy constructor is deleted)");
  }
}
