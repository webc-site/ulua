use alloc::vec::Vec;

use crate::{
  records::{
    function_definition::FunctionDefinition, function_type::FunctionType, type_level::TypeLevel,
  },
  type_aliases::{tags::Tags, type_pack_id::TypePackId},
};

impl FunctionType {
  /// C++ `FunctionType(TypePackId argTypes, TypePackId retTypes, std::optional<FunctionDefinition> defn = {}, bool hasSelf = false)`
  /// — the global monomorphic function constructor (Type.cpp:610). All other members take their
  /// C++ default member initializers (Type.h:350-47).
  pub fn function_type_new(
    arg_types: TypePackId,
    ret_types: TypePackId,
    defn: Option<FunctionDefinition>,
    has_self: bool,
  ) -> Self {
    FunctionType {
      definition: defn,
      generics: Vec::new(),
      generic_packs: Vec::new(),
      arg_names: Vec::new(),
      tags: Tags::new(),
      level: TypeLevel::default(),
      arg_types,
      ret_types,
      magic: None,
      has_self,
      has_no_free_or_generic_types: false,
      is_checked_function: false,
      is_deprecated_function: false,
      deprecated_info: None,
    }
  }
}
