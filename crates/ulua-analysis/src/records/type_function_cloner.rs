//! Generated skeleton item.
//! Node: `cxx:Record:Luau.Analysis:Analysis/src/TypeFunctionRuntime.cpp:2541:type_function_cloner`
//! Source: `Analysis/src/TypeFunctionRuntime.cpp`
//! Graph edges:
//! - declared_by: source_file Analysis/src/TypeFunctionRuntime.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/TypeFunctionRuntime.h
//!   - includes -> source_file Ast/include/Luau/Allocator.h
//!   - includes -> source_file Common/include/Luau/Common.h
//!   - includes -> source_file Ast/include/Luau/Lexer.h
//!   - includes -> source_file Bytecode/include/Luau/BytecodeBuilder.h
//!   - includes -> source_file Ast/include/Luau/ParseResult.h
//!   - includes -> source_file Compiler/include/Luau/Compiler.h
//!   - includes -> source_file Common/include/Luau/DenseHash.h
//!   - includes -> source_file Common/include/Luau/StringUtils.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file Analysis/include/Luau/TypeFunction.h
//!   - includes -> source_file Analysis/include/Luau/TypeFunctionRuntimeBuilder.h
//!   - includes -> source_file Analysis/include/Luau/RecursionCounter.h
//!   - includes -> source_file VM/include/lua.h
//!   - includes -> source_file VM/include/lualib.h
//! - incoming:
//!   - declares <- source_file Analysis/src/TypeFunctionRuntime.cpp
//!   - type_ref <- method TypeFunctionCloner::TypeFunctionCloner (Analysis/src/TypeFunctionRuntime.cpp)
//!   - type_ref <- method TypeFunctionCloner::clone (Analysis/src/TypeFunctionRuntime.cpp)
//!   - type_ref <- method TypeFunctionCloner::clone (Analysis/src/TypeFunctionRuntime.cpp)
//!   - type_ref <- method TypeFunctionCloner::hasExceededIterationLimit (Analysis/src/TypeFunctionRuntime.cpp)
//!   - type_ref <- method TypeFunctionCloner::run (Analysis/src/TypeFunctionRuntime.cpp)
//!   - type_ref <- method TypeFunctionCloner::find (Analysis/src/TypeFunctionRuntime.cpp)
//!   - type_ref <- method TypeFunctionCloner::find (Analysis/src/TypeFunctionRuntime.cpp)
//!   - type_ref <- method TypeFunctionCloner::find (Analysis/src/TypeFunctionRuntime.cpp)
//!   - type_ref <- method TypeFunctionCloner::shallowClone (Analysis/src/TypeFunctionRuntime.cpp)
//!   - type_ref <- method TypeFunctionCloner::shallowClone (Analysis/src/TypeFunctionRuntime.cpp)
//!   - type_ref <- method TypeFunctionCloner::cloneChildren (Analysis/src/TypeFunctionRuntime.cpp)
//!   - type_ref <- method TypeFunctionCloner::cloneChildren (Analysis/src/TypeFunctionRuntime.cpp)
//!   - type_ref <- method TypeFunctionCloner::cloneChildren (Analysis/src/TypeFunctionRuntime.cpp)
//!   - type_ref <- method TypeFunctionCloner::cloneChildren (Analysis/src/TypeFunctionRuntime.cpp)
//!   - type_ref <- method TypeFunctionCloner::cloneChildren (Analysis/src/TypeFunctionRuntime.cpp)
//!   - type_ref <- method TypeFunctionCloner::cloneChildren (Analysis/src/TypeFunctionRuntime.cpp)
//!   - type_ref <- method TypeFunctionCloner::cloneChildren (Analysis/src/TypeFunctionRuntime.cpp)
//!   - type_ref <- method TypeFunctionCloner::cloneChildren (Analysis/src/TypeFunctionRuntime.cpp)
//!   - type_ref <- method TypeFunctionCloner::cloneChildren (Analysis/src/TypeFunctionRuntime.cpp)
//!   - type_ref <- method TypeFunctionCloner::cloneChildren (Analysis/src/TypeFunctionRuntime.cpp)
//!   - type_ref <- method TypeFunctionCloner::cloneChildren (Analysis/src/TypeFunctionRuntime.cpp)
//!   - type_ref <- method TypeFunctionCloner::cloneChildren (Analysis/src/TypeFunctionRuntime.cpp)
//!   - type_ref <- method TypeFunctionCloner::cloneChildren (Analysis/src/TypeFunctionRuntime.cpp)
//!   - type_ref <- method TypeFunctionCloner::cloneChildren (Analysis/src/TypeFunctionRuntime.cpp)
//!   - type_ref <- method TypeFunctionCloner::cloneChildren (Analysis/src/TypeFunctionRuntime.cpp)
//!   - type_ref <- method TypeFunctionCloner::cloneChildren (Analysis/src/TypeFunctionRuntime.cpp)
//!   - type_ref <- method TypeFunctionCloner::cloneChildren (Analysis/src/TypeFunctionRuntime.cpp)
//!   - type_ref <- method TypeFunctionCloner::cloneChildren (Analysis/src/TypeFunctionRuntime.cpp)
//! - outgoing:
//!   - type_ref -> method TypeFunctionCloner::TypeFunctionCloner (Analysis/src/TypeFunctionRuntime.cpp)
//!   - type_ref -> record TypeFunctionRuntime (Analysis/include/Luau/TypeFunctionRuntime.h)
//!   - type_ref -> type_alias TypeFunctionKind (Analysis/include/Luau/TypeFunctionRuntimeBuilder.h)
//!   - type_ref -> type_alias SeenTypes (Analysis/src/TypeFunctionRuntime.cpp)
//!   - type_ref -> type_alias SeenTypePacks (Analysis/src/TypeFunctionRuntime.cpp)
//!   - translates_to -> rust_item TypeFunctionCloner

use alloc::vec::Vec;

use crate::{
  records::type_function_runtime::TypeFunctionRuntime,
  type_aliases::{
    seen_type_packs_type_function_runtime::SeenTypePacks,
    seen_types_type_function_runtime::SeenTypes, type_function_kind::TypeFunctionKind,
  },
};

#[derive(Debug, Clone)]
pub struct TypeFunctionCloner {
  pub(crate) type_function_runtime: *mut TypeFunctionRuntime,
  pub(crate) queue: Vec<(TypeFunctionKind, TypeFunctionKind)>,
  pub(crate) types: SeenTypes,
  pub(crate) packs: SeenTypePacks,
  pub(crate) steps: i32,
}
